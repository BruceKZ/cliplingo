use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

#[cfg(target_os = "macos")]
use core_foundation::runloop::CFRunLoop;
#[cfg(target_os = "macos")]
use core_graphics::event::{
    CGEventFlags, CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement,
    CGEventType, CallbackResult, EventField, KeyCode,
};
#[cfg(not(target_os = "macos"))]
use rdev::{listen, Event, EventType, Key};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{
    Builder as GlobalShortcutBuilder, Code, Modifiers, Shortcut, ShortcutState,
};

use crate::{
    services::clipboard::{read_clipboard_text, ClipboardServiceError, ClipboardText},
    MAIN_WINDOW_LABEL, POPUP_WINDOW_LABEL,
};

pub const TRANSLATION_TRIGGER_EVENT: &str = "trigger:translation-requested";
pub const DEFAULT_FALLBACK_SHORTCUT: &str = "CmdOrCtrl+Shift+Y";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TriggerSource {
    DoubleCopy,
    FallbackShortcut,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerDispatchPayload {
    pub source: TriggerSource,
    pub text: String,
    pub character_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerServiceError {
    Clipboard(ClipboardServiceError),
    ShortcutRegistration(String),
}

impl std::fmt::Display for TriggerServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clipboard(error) => write!(f, "{error}"),
            Self::ShortcutRegistration(error) => {
                write!(f, "failed to register fallback shortcut: {error}")
            }
        }
    }
}

impl std::error::Error for TriggerServiceError {}

#[derive(Debug)]
pub struct CopyTriggerStateMachine {
    window: Duration,
    last_copy_at: Option<Instant>,
}

impl CopyTriggerStateMachine {
    pub fn new(window: Duration) -> Self {
        Self {
            window,
            last_copy_at: None,
        }
    }

    pub fn register_copy(&mut self, now: Instant) -> bool {
        if let Some(previous) = self.last_copy_at {
            if now.duration_since(previous) <= self.window {
                self.last_copy_at = None;
                return true;
            }
        }

        self.last_copy_at = Some(now);
        false
    }
}

#[cfg(not(target_os = "macos"))]
#[derive(Debug, Default, Clone, Copy)]
struct ModifierState {
    control_pressed: bool,
    meta_pressed: bool,
}

#[cfg(not(target_os = "macos"))]
impl ModifierState {
    fn apply_key_press(&mut self, key: Key) {
        match key {
            Key::ControlLeft | Key::ControlRight => self.control_pressed = true,
            Key::MetaLeft | Key::MetaRight => self.meta_pressed = true,
            _ => {}
        }
    }

    fn apply_key_release(&mut self, key: Key) {
        match key {
            Key::ControlLeft | Key::ControlRight => self.control_pressed = false,
            Key::MetaLeft | Key::MetaRight => self.meta_pressed = false,
            _ => {}
        }
    }

    fn copy_modifier_active(self) -> bool {
        self.control_pressed || self.meta_pressed
    }
}

pub fn initialize_trigger_services(app: &AppHandle, shortcut: &str, double_copy_window_ms: u64) {
    if let Err(error) = register_fallback_shortcut(app, shortcut) {
        eprintln!("{error}");
        if shortcut != DEFAULT_FALLBACK_SHORTCUT {
            let _ = register_fallback_shortcut(app, DEFAULT_FALLBACK_SHORTCUT)
                .map_err(|fallback_error| eprintln!("{fallback_error}"));
        }
    }

    spawn_double_copy_listener(app.clone(), double_copy_window_ms);
}

pub fn dispatch_translation_trigger(
    app: &AppHandle,
    source: TriggerSource,
) -> Result<TriggerDispatchPayload, TriggerServiceError> {
    let clipboard = read_clipboard_text(app.clone()).map_err(TriggerServiceError::Clipboard)?;
    let payload = build_payload(source, clipboard);

    let popup_window = if let Some(window) = app.get_webview_window(POPUP_WINDOW_LABEL) {
        window
    } else {
        WebviewWindowBuilder::new(
            app,
            POPUP_WINDOW_LABEL,
            WebviewUrl::App("index.html".into()),
        )
        .title("ClipLingo Translation")
        .inner_size(560.0, 380.0)
        .min_inner_size(420.0, 280.0)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .visible(false)
        .build()
        .map_err(|error| TriggerServiceError::ShortcutRegistration(error.to_string()))?
    };
    let _ = popup_window.show();
    let _ = popup_window.set_focus();

    app.emit(TRANSLATION_TRIGGER_EVENT, &payload)
        .map_err(|error| TriggerServiceError::ShortcutRegistration(error.to_string()))?;

    if let Some(main_window) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        let _ = main_window.emit(TRANSLATION_TRIGGER_EVENT, &payload);
    }

    Ok(payload)
}

fn build_payload(source: TriggerSource, clipboard: ClipboardText) -> TriggerDispatchPayload {
    TriggerDispatchPayload {
        source,
        text: clipboard.text,
        character_count: clipboard.character_count,
    }
}

fn register_fallback_shortcut(app: &AppHandle, shortcut: &str) -> Result<(), TriggerServiceError> {
    let fallback_shortcut = parse_shortcut(shortcut).unwrap_or_else(default_shortcut);
    let app_handle = app.clone();

    app.plugin(
        GlobalShortcutBuilder::new()
            .with_shortcuts([fallback_shortcut])
            .map_err(|error| TriggerServiceError::ShortcutRegistration(error.to_string()))?
            .with_handler(move |_app, triggered_shortcut, event| {
                if event.state() == ShortcutState::Pressed
                    && triggered_shortcut == &fallback_shortcut
                {
                    let _ =
                        dispatch_translation_trigger(&app_handle, TriggerSource::FallbackShortcut);
                }
            })
            .build(),
    )
    .map_err(|error: tauri::Error| TriggerServiceError::ShortcutRegistration(error.to_string()))
}

#[cfg(not(target_os = "macos"))]
fn spawn_double_copy_listener(app: AppHandle, double_copy_window_ms: u64) {
    let state_machine = Arc::new(Mutex::new(CopyTriggerStateMachine::new(
        Duration::from_millis(normalized_window_ms(double_copy_window_ms)),
    )));
    let modifiers = Arc::new(Mutex::new(ModifierState::default()));

    thread::spawn(move || {
        let state_machine = Arc::clone(&state_machine);
        let modifiers = Arc::clone(&modifiers);

        if let Err(error) = listen(move |event| {
            handle_global_event(&app, &state_machine, &modifiers, event);
        }) {
            eprintln!("double-copy listener unavailable: {error:?}");
        }
    });
}

#[cfg(target_os = "macos")]
fn spawn_double_copy_listener(app: AppHandle, double_copy_window_ms: u64) {
    let state_machine = Arc::new(Mutex::new(CopyTriggerStateMachine::new(
        Duration::from_millis(normalized_window_ms(double_copy_window_ms)),
    )));

    thread::spawn(move || {
        if let Err(error) = run_macos_double_copy_listener(app, state_machine) {
            eprintln!("double-copy listener unavailable: {error}");
        }
    });
}

#[cfg(target_os = "macos")]
fn run_macos_double_copy_listener(
    app: AppHandle,
    state_machine: Arc<Mutex<CopyTriggerStateMachine>>,
) -> Result<(), String> {
    CGEventTap::with_enabled(
        CGEventTapLocation::HID,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::ListenOnly,
        vec![CGEventType::KeyDown],
        move |_proxy, _event_type, event| {
            let keycode = event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE) as u16;
            if keycode != KeyCode::ANSI_C {
                return CallbackResult::Keep;
            }

            let modifier_flags = event.get_flags();
            let copy_modifier_active = modifier_flags
                .intersects(CGEventFlags::CGEventFlagCommand | CGEventFlags::CGEventFlagControl);
            if !copy_modifier_active {
                return CallbackResult::Keep;
            }

            let is_repeat =
                event.get_integer_value_field(EventField::KEYBOARD_EVENT_AUTOREPEAT) != 0;
            if is_repeat {
                return CallbackResult::Keep;
            }

            let mut trigger_state = state_machine.lock().expect("trigger state lock");
            if trigger_state.register_copy(Instant::now()) {
                let _ = dispatch_translation_trigger(&app, TriggerSource::DoubleCopy);
            }

            CallbackResult::Keep
        },
        CFRunLoop::run_current,
    )
    .map_err(|_| {
        "failed to install macOS event tap (check Input Monitoring permission)".to_string()
    })
}

#[cfg(not(target_os = "macos"))]
fn handle_global_event(
    app: &AppHandle,
    state_machine: &Arc<Mutex<CopyTriggerStateMachine>>,
    modifiers: &Arc<Mutex<ModifierState>>,
    event: Event,
) {
    match event.event_type {
        EventType::KeyPress(key) => {
            let mut modifier_state = modifiers.lock().expect("modifier state lock");
            modifier_state.apply_key_press(key);

            if key == Key::KeyC && modifier_state.copy_modifier_active() {
                let mut trigger_state = state_machine.lock().expect("trigger state lock");
                if trigger_state.register_copy(Instant::now()) {
                    let _ = dispatch_translation_trigger(app, TriggerSource::DoubleCopy);
                }
            }
        }
        EventType::KeyRelease(key) => {
            modifiers
                .lock()
                .expect("modifier state lock")
                .apply_key_release(key);
        }
        _ => {}
    }
}

fn normalized_window_ms(value: u64) -> u64 {
    value.clamp(250, 800)
}

fn parse_shortcut(input: &str) -> Option<Shortcut> {
    let trimmed = input.trim();
    if trimmed.eq_ignore_ascii_case("CmdOrCtrl+Shift+Y") {
        return Some(default_shortcut());
    }

    None
}

fn default_shortcut() -> Shortcut {
    #[cfg(target_os = "macos")]
    let modifiers = Modifiers::META | Modifiers::SHIFT;
    #[cfg(not(target_os = "macos"))]
    let modifiers = Modifiers::CONTROL | Modifiers::SHIFT;

    Shortcut::new(Some(modifiers), Code::KeyY)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_machine_only_triggers_for_second_copy_within_window() {
        let start = Instant::now();
        let mut machine = CopyTriggerStateMachine::new(Duration::from_millis(450));

        assert!(!machine.register_copy(start));
        assert!(machine.register_copy(start + Duration::from_millis(300)));
        assert!(!machine.register_copy(start + Duration::from_millis(700)));
        assert!(!machine.register_copy(start + Duration::from_millis(1_200)));
    }

    #[test]
    fn state_machine_resets_when_second_copy_is_too_late() {
        let start = Instant::now();
        let mut machine = CopyTriggerStateMachine::new(Duration::from_millis(450));

        assert!(!machine.register_copy(start));
        assert!(!machine.register_copy(start + Duration::from_millis(500)));
        assert!(machine.register_copy(start + Duration::from_millis(850)));
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn modifier_state_tracks_copy_modifiers() {
        let mut modifiers = ModifierState::default();

        modifiers.apply_key_press(Key::ControlLeft);
        assert!(modifiers.copy_modifier_active());

        modifiers.apply_key_release(Key::ControlLeft);
        assert!(!modifiers.copy_modifier_active());

        modifiers.apply_key_press(Key::MetaLeft);
        assert!(modifiers.copy_modifier_active());
    }

    #[test]
    fn normalizes_window_bounds() {
        assert_eq!(normalized_window_ms(100), 250);
        assert_eq!(normalized_window_ms(450), 450);
        assert_eq!(normalized_window_ms(900), 800);
    }
}
