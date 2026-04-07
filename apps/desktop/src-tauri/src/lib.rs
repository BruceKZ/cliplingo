mod models;
mod services;
mod storage;

use models::config::{AppConfigRecord, ProviderConfigRecord};
use services::{
    clipboard::{read_clipboard_text, read_clipboard_text_with_limits, ClipboardLimits},
    config::{ConfigService, ProviderSecretStatus},
    trigger::{
        dispatch_translation_trigger, initialize_trigger_services, TriggerDispatchPayload,
        TriggerSource,
    },
};
use storage::{fs_paths::ProjectPathProvider, secure_storage::KeyringSecretStore};
use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, WindowEvent,
};

const MAIN_WINDOW_LABEL: &str = "main";
const SETTINGS_WINDOW_LABEL: &str = "settings";
const POPUP_WINDOW_LABEL: &str = "translation-popup";

const TRAY_SHOW_MAIN_ID: &str = "tray-show-main";
const TRAY_SHOW_SETTINGS_ID: &str = "tray-show-settings";

fn app_url() -> WebviewUrl {
    WebviewUrl::App("index.html".into())
}

fn config_service() -> ConfigService<ProjectPathProvider, KeyringSecretStore> {
    ConfigService::new(ProjectPathProvider, KeyringSecretStore)
}

fn bind_hide_on_close(window: &WebviewWindow) {
    let window = window.clone();
    let hide_window = window.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            let _ = hide_window.hide();
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

fn create_popup_window(app: &AppHandle) -> tauri::Result<WebviewWindow> {
    let window = WebviewWindowBuilder::new(app, POPUP_WINDOW_LABEL, app_url())
        .title("ClipLingo Translation")
        .inner_size(560.0, 380.0)
        .min_inner_size(420.0, 280.0)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
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

fn present_popup_window(app: &AppHandle) -> tauri::Result<()> {
    let window = ensure_window(app, POPUP_WINDOW_LABEL, || create_popup_window(app))?;
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
async fn show_translation_popup(app: AppHandle) -> tauri::Result<()> {
    present_popup_window(&app)
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
    config_service().load().map_err(|error| error.to_string())
}

#[tauri::command]
async fn save_app_config(config: AppConfigRecord) -> Result<AppConfigRecord, String> {
    config_service()
        .save(config)
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn upsert_provider_config(provider: ProviderConfigRecord) -> Result<AppConfigRecord, String> {
    config_service()
        .upsert_provider(provider)
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn remove_provider_config(provider_id: String) -> Result<AppConfigRecord, String> {
    config_service()
        .remove_provider(&provider_id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn set_active_provider(provider_id: Option<String>) -> Result<AppConfigRecord, String> {
    config_service()
        .set_active_provider(provider_id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn set_provider_api_key(
    provider_id: String,
    api_key: String,
) -> Result<ProviderSecretStatus, String> {
    config_service()
        .set_provider_secret(&provider_id, &api_key)
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn get_provider_api_key_status(provider_id: String) -> Result<ProviderSecretStatus, String> {
    config_service()
        .get_provider_secret_status(&provider_id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn delete_provider_api_key(provider_id: String) -> Result<ProviderSecretStatus, String> {
    config_service()
        .delete_provider_secret(&provider_id)
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
        .show_menu_on_left_click(true)
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
            let main_window = if let Some(window) = handle.get_webview_window(MAIN_WINDOW_LABEL) {
                window
            } else {
                create_main_window(&handle)?
            };
            bind_hide_on_close(&main_window);

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
            show_translation_popup,
            hide_window,
            toggle_main_window,
            read_system_clipboard,
            load_app_config,
            save_app_config,
            upsert_provider_config,
            remove_provider_config,
            set_active_provider,
            set_provider_api_key,
            get_provider_api_key_status,
            delete_provider_api_key,
            trigger_translation_from_fallback_shortcut
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
