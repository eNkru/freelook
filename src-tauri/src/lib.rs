mod commands;
mod config;
mod menu;
mod network;
mod tray;
mod windows;

use tauri::{Emitter, Manager};

/// Set up main window event handlers (CSS injection, unread observer)
fn setup_main_window(app: &tauri::AppHandle) {
    if let Some(main_window) = app.get_webview_window("main") {
        let app_handle = app.clone();

        let main_win_clone = main_window.clone();
        main_window.on_window_event(move |event| {
            if let tauri::WindowEvent::Focused(focused) = event {
                if *focused {
                    let _ = windows::apply_main_settings(&app_handle);

                    // Inject unread observer JS
                    let unread_js = windows::get_unread_js(&app_handle);
                    if !unread_js.is_empty() {
                        let _ = main_win_clone.eval(&unread_js);
                    }
                }
            }
        });

        // Show main window
        let _ = main_window.show();
        let _ = main_window.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::set_config,
            commands::delete_config,
            commands::get_configs,
            commands::check_network,
            commands::reconnect,
            commands::reconnect_and_launch,
            commands::update_unread,
            commands::show_main_window,
            commands::reset_window_frame,
            commands::open_settings,
            commands::restart_app,
            commands::css_inject,
            commands::submit_login,
        ])
        .setup(|app| {
            // Create tray icon
            let app_handle = app.handle().clone();
            tray::create_tray(&app_handle).expect("Failed to create tray");

            // Set up native menu
            let menu = menu::create_menu(&app_handle).expect("Failed to create menu");
            app.set_menu(menu).expect("Failed to set menu");

            // Start loading Outlook immediately. Network probing only controls the splash fallback.
            let app_handle2 = app.handle().clone();
            let _ = windows::create_main_window(&app_handle2);
            setup_main_window(&app_handle2);
            let _ = windows::apply_main_settings(&app_handle2);

            tauri::async_runtime::spawn(async move {
                let online = network::check_network().await.unwrap_or(false);

                if online {
                    // Destroy splash window
                    if let Some(splash) = app_handle2.get_webview_window("splash") {
                        let _ = splash.close();
                    }
                } else {
                    // Send connect-timeout event to splash
                    if let Some(splash) = app_handle2.get_webview_window("splash") {
                        let _ = splash.emit("connect-timeout", ());
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    let label = window.label();
                    if label == "main" || label == "settings" {
                        // Close-to-tray: hide instead of closing
                        api.prevent_close();
                        let _ = window.hide();
                    }
                }
                _ => {}
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            if let tauri::RunEvent::Reopen { .. } = event {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        });
}
