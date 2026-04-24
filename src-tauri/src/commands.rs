use tauri::{AppHandle, Manager};

// Re-export all commands from their respective modules
use crate::config;
use crate::network;
use crate::tray;
use crate::windows;

// Config commands
#[tauri::command]
pub fn get_config(app: AppHandle, key: String, default: Option<String>) -> Result<String, String> {
    config::get_config(app, key, default)
}

#[tauri::command]
pub fn set_config(app: AppHandle, key: String, value: String) -> Result<(), String> {
    config::set_config(app, key, value)
}

#[tauri::command]
pub fn delete_config(app: AppHandle, key: String) -> Result<(), String> {
    config::delete_config(app, key)
}

#[tauri::command]
pub fn get_configs(app: AppHandle) -> Result<serde_json::Value, String> {
    config::get_configs(app)
}

// Network commands
#[tauri::command]
pub async fn check_network() -> Result<bool, String> {
    network::check_network().await
}

#[tauri::command]
pub async fn reconnect() -> Result<bool, String> {
    network::reconnect().await
}

/// Reconnect and, if successful, create the main window and close splash
#[tauri::command]
pub async fn reconnect_and_launch(app: AppHandle) -> Result<bool, String> {
    let online = network::reconnect().await?;
    if online {
        // Create main window
        let _ = windows::create_main_window(&app);

        // Set up main window event handlers (CSS injection, unread observer)
        if let Some(main_window) = app.get_webview_window("main") {
            let app_handle = app.clone();
            let main_win_clone = main_window.clone();
            main_window.on_window_event(move |event| {
                if let tauri::WindowEvent::Focused(focused) = event {
                    if *focused {
                        let css = windows::get_main_css(&app_handle);
                        let _ = main_win_clone.eval(&format!(
                            "document.head.insertAdjacentHTML('beforeend', `<style>{}</style>`)",
                            css
                        ));
                        let unread_js = windows::get_unread_js(&app_handle);
                        if !unread_js.is_empty() {
                            let _ = main_win_clone.eval(&unread_js);
                        }
                    }
                }
            });
            let _ = main_window.show();
            let _ = main_window.set_focus();
        }

        // Close splash window
        if let Some(splash) = app.get_webview_window("splash") {
            let _ = splash.close();
        }
    }
    Ok(online)
}

// Tray commands
#[tauri::command]
pub fn update_unread(app: AppHandle, has_unread: bool) -> Result<(), String> {
    tray::update_unread(app, has_unread)
}

// Window commands
#[tauri::command]
pub fn show_main_window(app: AppHandle) -> Result<(), String> {
    windows::show_main_window(app)
}

#[tauri::command]
pub fn reset_window_frame(app: AppHandle) -> Result<(), String> {
    windows::reset_window_frame(app)
}

#[tauri::command]
pub fn open_settings(app: AppHandle) -> Result<(), String> {
    windows::open_settings(app)
}

#[tauri::command]
pub fn restart_app(app: AppHandle) -> Result<(), String> {
    windows::restart_app(app)
}

#[tauri::command]
pub fn css_inject(app: AppHandle, webview_label: String, css: String) -> Result<(), String> {
    windows::css_inject(app, webview_label, css)
}

// Login command
#[tauri::command]
pub fn submit_login(credentials: serde_json::Value) -> Result<serde_json::Value, String> {
    Ok(credentials)
}