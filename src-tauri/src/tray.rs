use tauri::{
    AppHandle, Manager,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    menu::{Menu, MenuItem, PredefinedMenuItem},
};

use crate::windows;

/// Get the path to a bundled resource file
fn resource_path(app: &AppHandle, name: &str) -> std::path::PathBuf {
    app.path()
        .resource_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join(name)
}

pub fn create_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let is_macos = cfg!(target_os = "macos");

    // Build context menu
    let open_item = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;

    let menu = Menu::with_items(app, &[&open_item, &separator, &settings_item, &separator, &quit_item])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .icon_as_template(is_macos)
        .menu(&menu)
        .on_menu_event(move |app, event| {
            match event.id().as_ref() {
                "open" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "settings" => {
                    let _ = windows::open_settings(app.clone());
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            let app = tray.app_handle();
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}

pub fn update_unread(app: AppHandle, has_unread: bool) -> Result<(), String> {
    let is_macos = cfg!(target_os = "macos");

    if let Some(tray) = app.tray_by_id("main") {
        let icon_path = if is_macos {
            if has_unread {
                resource_path(&app, "outlook_macOS_unread.png")
            } else {
                resource_path(&app, "outlook_macOS.png")
            }
        } else {
            if has_unread {
                resource_path(&app, "outlook_linux_unread.png")
            } else {
                resource_path(&app, "512x512.png")
            }
        };

        // Load and set the tray icon using the image crate
        if let Ok(img) = image::open(&icon_path) {
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();
            let icon = tauri::image::Image::new_owned(rgba.into_raw(), width, height);
            let _ = tray.set_icon(Some(icon));
            let _ = tray.set_icon_as_template(is_macos);
        }
    }

    Ok(())
}
