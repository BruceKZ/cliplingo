mod models;
mod services;
mod storage;

use models::config::{AppConfigRecord, ProviderConfigRecord, ProviderDirectoryRecord};
use services::{
    clipboard::{read_clipboard_text, read_clipboard_text_with_limits, ClipboardLimits},
    config::{ConfigService, ProviderSecretStatus},
    history::{HistoryEntryRecord, HistoryRepository},
    language::{LanguageAnalysis, LanguageDetectionService, LanguageRouter},
    logging::{LogEvent, LoggingService},
    translation::{
        TranslateTextInput, TranslationExecutionOutput, TranslationOrchestrator,
        TranslationOrchestratorError,
    },
    trigger::{
        dispatch_translation_trigger, initialize_trigger_services, TriggerDispatchPayload,
        TriggerSource,
    },
};
use storage::{fs_paths::ProjectPathProvider, secure_storage::KeyringSecretStore};
use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, WindowEvent,
};

fn to_json<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| "<serialize failed>".to_string())
}

const MAIN_WINDOW_LABEL: &str = "main";
const SETTINGS_WINDOW_LABEL: &str = "settings";

const TRAY_SHOW_MAIN_ID: &str = "tray-show-main";
const TRAY_SHOW_SETTINGS_ID: &str = "tray-show-settings";

fn app_url() -> WebviewUrl {
    WebviewUrl::App("index.html".into())
}

fn config_service() -> ConfigService<ProjectPathProvider, KeyringSecretStore> {
    ConfigService::new(ProjectPathProvider, KeyringSecretStore)
}

fn history_repository() -> HistoryRepository<ProjectPathProvider> {
    HistoryRepository::new(ProjectPathProvider)
}

fn logging_service() -> LoggingService<ProjectPathProvider> {
    LoggingService::new(ProjectPathProvider)
}

fn bind_hide_on_close(window: &WebviewWindow) {
    let window = window.clone();
    let hide_window = window.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            // Keep services alive in background; closing behaves like minimizing to tray.
            if hide_window.hide().is_err() {
                let _ = hide_window.minimize();
            }
        }
    });
}

fn reveal_window(window: &WebviewWindow) -> tauri::Result<()> {
    if window.is_minimized()? {
        window.unminimize()?;
    }

    if !window.is_visible()? {
        window.show()?;
    }

    window.set_focus()?;
    Ok(())
}

fn ensure_window<F>(app: &AppHandle, label: &str, create: F) -> tauri::Result<WebviewWindow>
where
    F: FnOnce() -> tauri::Result<WebviewWindow>,
{
    if let Some(window) = app.get_webview_window(label) {
        Ok(window)
    } else {
        create()
    }
}

fn create_main_window(app: &AppHandle) -> tauri::Result<WebviewWindow> {
    let window = WebviewWindowBuilder::new(app, MAIN_WINDOW_LABEL, app_url())
        .title("ClipLingo")
        .inner_size(1120.0, 760.0)
        .min_inner_size(960.0, 640.0)
        .build()?;
    bind_hide_on_close(&window);
    Ok(window)
}

fn create_settings_window(app: &AppHandle) -> tauri::Result<WebviewWindow> {
    let window = WebviewWindowBuilder::new(app, SETTINGS_WINDOW_LABEL, app_url())
        .title("ClipLingo Settings")
        .inner_size(980.0, 720.0)
        .min_inner_size(840.0, 600.0)
        .visible(false)
        .build()?;
    bind_hide_on_close(&window);
    Ok(window)
}

fn present_main_window(app: &AppHandle) -> tauri::Result<()> {
    let window = ensure_window(app, MAIN_WINDOW_LABEL, || create_main_window(app))?;
    reveal_window(&window)
}

fn present_settings_window(app: &AppHandle) -> tauri::Result<()> {
    let window = ensure_window(app, SETTINGS_WINDOW_LABEL, || create_settings_window(app))?;
    reveal_window(&window)
}

fn hide_window_by_label(app: &AppHandle, label: &str) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window(label) {
        window.hide()?;
    }

    Ok(())
}

#[tauri::command]
async fn show_main_window(app: AppHandle) -> tauri::Result<()> {
    present_main_window(&app)
}

#[tauri::command]
async fn show_settings_window(app: AppHandle) -> tauri::Result<()> {
    present_settings_window(&app)
}

#[tauri::command]
async fn hide_window(app: AppHandle, label: String) -> tauri::Result<()> {
    hide_window_by_label(&app, &label)
}

#[tauri::command]
async fn toggle_main_window(app: AppHandle) -> tauri::Result<()> {
    let window = ensure_window(&app, MAIN_WINDOW_LABEL, || {
        Err(tauri::Error::WindowNotFound)
    })?;

    if window.is_visible()? {
        window.hide()?;
    } else {
        reveal_window(&window)?;
    }

    Ok(())
}

#[tauri::command]
async fn read_system_clipboard(
    app: AppHandle,
    max_chars: Option<usize>,
) -> Result<services::clipboard::ClipboardText, String> {
    match max_chars {
        Some(max_chars) => {
            let limits = ClipboardLimits::new(max_chars).map_err(|error| error.to_string())?;
            read_clipboard_text_with_limits(app, limits).map_err(|error| error.to_string())
        }
        None => read_clipboard_text(app).map_err(|error| error.to_string()),
    }
}

#[tauri::command]
async fn load_app_config() -> Result<AppConfigRecord, String> {
    eprintln!("[tauri] load_app_config:start");
    let loaded = config_service().load().map_err(|error| error.to_string())?;
    eprintln!("[tauri] load_app_config:done payload={}", to_json(&loaded));
    Ok(loaded)
}

#[tauri::command]
async fn list_providers() -> Result<ProviderDirectoryRecord, String> {
    eprintln!("[tauri] list_providers:start");
    let directory = config_service()
        .load()
        .map(|config| config.provider_directory())
        .map_err(|error| error.to_string())?;
    eprintln!("[tauri] list_providers:done payload={}", to_json(&directory));
    Ok(directory)
}

#[tauri::command]
async fn save_app_config(config: AppConfigRecord) -> Result<AppConfigRecord, String> {
    eprintln!("[tauri] save_app_config:start payload={}", to_json(&config));
    let saved = config_service()
        .save(config)
        .map_err(|error| error.to_string())?;
    eprintln!("[tauri] save_app_config:done payload={}", to_json(&saved));
    Ok(saved)
}

#[tauri::command]
async fn upsert_provider_config(
    provider: ProviderConfigRecord,
) -> Result<ProviderDirectoryRecord, String> {
    eprintln!(
        "[tauri] upsert_provider_config:start provider={}",
        to_json(&provider)
    );
    let directory = config_service()
        .upsert_provider(provider)
        .map(|config| config.provider_directory())
        .map_err(|error| error.to_string())?;
    eprintln!(
        "[tauri] upsert_provider_config:done directory={}",
        to_json(&directory)
    );
    Ok(directory)
}

#[tauri::command]
async fn remove_provider_config(provider_id: String) -> Result<ProviderDirectoryRecord, String> {
    eprintln!(
        "[tauri] remove_provider_config:start provider_id={}",
        provider_id
    );
    let directory = config_service()
        .remove_provider(&provider_id)
        .map(|config| config.provider_directory())
        .map_err(|error| error.to_string())?;
    eprintln!(
        "[tauri] remove_provider_config:done directory={}",
        to_json(&directory)
    );
    Ok(directory)
}

#[tauri::command]
async fn set_active_provider(provider_id: Option<String>) -> Result<ProviderDirectoryRecord, String> {
    eprintln!("[tauri] set_active_provider:start provider_id={:?}", provider_id);
    let directory = config_service()
        .set_active_provider(provider_id)
        .map(|config| config.provider_directory())
        .map_err(|error| error.to_string())?;
    eprintln!(
        "[tauri] set_active_provider:done directory={}",
        to_json(&directory)
    );
    Ok(directory)
}

#[tauri::command]
async fn set_provider_api_key(
    provider_id: String,
    api_key: String,
) -> Result<ProviderSecretStatus, String> {
    eprintln!(
        "[tauri] set_provider_api_key:start provider_id={} api_key_len={}",
        provider_id,
        api_key.chars().count()
    );
    let status = config_service()
        .set_provider_secret(&provider_id, &api_key)
        .map_err(|error| error.to_string())?;
    eprintln!("[tauri] set_provider_api_key:done payload={}", to_json(&status));
    Ok(status)
}

#[tauri::command]
async fn get_provider_api_key_status(provider_id: String) -> Result<ProviderSecretStatus, String> {
    eprintln!(
        "[tauri] get_provider_api_key_status:start provider_id={}",
        provider_id
    );
    let status = config_service()
        .get_provider_secret_status(&provider_id)
        .map_err(|error| error.to_string())?;
    eprintln!(
        "[tauri] get_provider_api_key_status:done payload={}",
        to_json(&status)
    );
    Ok(status)
}

#[tauri::command]
async fn delete_provider_api_key(provider_id: String) -> Result<ProviderSecretStatus, String> {
    eprintln!(
        "[tauri] delete_provider_api_key:start provider_id={}",
        provider_id
    );
    let status = config_service()
        .delete_provider_secret(&provider_id)
        .map_err(|error| error.to_string())?;
    eprintln!(
        "[tauri] delete_provider_api_key:done payload={}",
        to_json(&status)
    );
    Ok(status)
}

#[tauri::command]
async fn analyze_language_routing(
    text: String,
    rule: Option<models::config::LanguageRoutingRuleRecord>,
) -> Result<LanguageAnalysis, String> {
    let detector = LanguageDetectionService::new();
    let router = LanguageRouter;
    let routing_rule = if let Some(rule) = rule {
        rule
    } else {
        config_service()
            .load()
            .map_err(|error| error.to_string())?
            .translation
            .routing_rule
    };

    Ok(router.resolve_for_text(&routing_rule, &detector, &text))
}

#[tauri::command]
async fn translate_text(input: TranslateTextInput) -> Result<TranslationExecutionOutput, String> {
    eprintln!("[tauri] translate_text:start input={}", to_json(&input));
    let config = config_service().load().map_err(|error| error.to_string())?;
    let resolved_provider = config_service()
        .resolve_provider_config(input.provider_id.as_deref())
        .map_err(|error| error.to_string())?;
    eprintln!(
        "[tauri] translate_text:resolved_provider provider_id={} provider_name={} has_secret={}",
        resolved_provider.provider.id,
        resolved_provider.provider.name,
        resolved_provider.api_key.is_some()
    );
    let orchestrator = TranslationOrchestrator::default();

    match orchestrator
        .execute(resolved_provider, config, input.clone())
        .await
    {
        Ok(output) => {
            eprintln!("[tauri] translate_text:success output={}", to_json(&output));
            let config = config_service().load().map_err(|error| error.to_string())?;
            let _ = history_repository().append_translation(&config.history, &output);
            let _ = logging_service().log(
                &config.debug,
                LogEvent {
                    level: "info",
                    event: "translation-succeeded",
                    message: format!(
                        "provider={} targets={}",
                        output.provider_id,
                        output.target_languages.join(",")
                    ),
                    text_preview: Some(output.source_text.clone()),
                },
            );

            Ok(output)
        }
        Err(error) => {
            let safe_output = TranslationExecutionOutput::from_safe_error(&input, &error);
            if let TranslationOrchestratorError::Provider(provider_error) = &error {
                eprintln!(
                    "[tauri] translate_text:error provider_code={:?} status={:?} message={} details={}",
                    provider_error.code,
                    provider_error.status,
                    provider_error.message,
                    provider_error.details.as_deref().unwrap_or("")
                );
            } else {
                eprintln!("[tauri] translate_text:error error={}", error);
            }
            eprintln!(
                "[tauri] translate_text:error safe_output={}",
                to_json(&safe_output)
            );
            if let Ok(config) = config_service().load() {
                let _ = logging_service().log(
                    &config.debug,
                    LogEvent {
                        level: "error",
                        event: "translation-failed",
                        message: error.to_string(),
                        text_preview: Some(safe_output.source_text.clone()),
                    },
                );
            }

            Ok(safe_output)
        }
    }
}

#[tauri::command]
async fn list_translation_history() -> Result<Vec<HistoryEntryRecord>, String> {
    history_repository()
        .list()
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn clear_translation_history() -> Result<(), String> {
    history_repository()
        .clear()
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn trigger_translation_from_fallback_shortcut(
    app: AppHandle,
) -> Result<TriggerDispatchPayload, String> {
    dispatch_translation_trigger(&app, TriggerSource::FallbackShortcut)
        .map_err(|error| error.to_string())
}

fn build_tray(app: &AppHandle) -> tauri::Result<()> {
    let show_main = MenuItem::with_id(
        app,
        TRAY_SHOW_MAIN_ID,
        "Show Main Window",
        true,
        None::<&str>,
    )?;
    let show_settings =
        MenuItem::with_id(app, TRAY_SHOW_SETTINGS_ID, "Settings", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = PredefinedMenuItem::quit(app, Some("Quit"))?;

    let menu = Menu::with_items(app, &[&show_main, &show_settings, &separator, &quit])?;

    TrayIconBuilder::with_id("cliplingo-tray")
        .tooltip("ClipLingo")
        .icon(Image::from_bytes(include_bytes!("../icons/icon.png"))?.to_owned())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event({
            let app_handle = app.clone();
            move |_tray, event| {
                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } = event
                {
                    let _ = present_main_window(&app_handle);
                }
            }
        })
        .on_menu_event({
            let app_handle = app.clone();
            move |_app, event| match event.id().as_ref() {
                TRAY_SHOW_MAIN_ID => {
                    let _ = present_main_window(&app_handle);
                }
                TRAY_SHOW_SETTINGS_ID => {
                    let _ = present_settings_window(&app_handle);
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let handle = app.handle().clone();
            let config = config_service().load()?;
            if let Some(window) = handle.get_webview_window(MAIN_WINDOW_LABEL) {
                bind_hide_on_close(&window);
            } else {
                let _ = create_main_window(&handle)?;
            }

            build_tray(&handle)?;
            initialize_trigger_services(
                &handle,
                &config.trigger.fallback_shortcut,
                config.trigger.double_copy_window_ms,
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            show_main_window,
            show_settings_window,
            hide_window,
            toggle_main_window,
            read_system_clipboard,
            load_app_config,
            list_providers,
            save_app_config,
            upsert_provider_config,
            remove_provider_config,
            set_active_provider,
            set_provider_api_key,
            get_provider_api_key_status,
            delete_provider_api_key,
            analyze_language_routing,
            translate_text,
            list_translation_history,
            clear_translation_history,
            trigger_translation_from_fallback_shortcut
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
