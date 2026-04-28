use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

use crate::windows;

/// Get the path to a bundled resource file
fn resource_path(app: &AppHandle, name: &str) -> std::path::PathBuf {
    app.path()
        .resource_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join(name)
}

fn resource_candidates(app: &AppHandle, name: &str) -> Vec<std::path::PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(path) = app.path().resolve(
        format!("assets/{name}"),
        tauri::path::BaseDirectory::Resource,
    ) {
        candidates.push(path);
    }
    if let Ok(path) = app.path().resolve(
        format!("_up_/assets/{name}"),
        tauri::path::BaseDirectory::Resource,
    ) {
        candidates.push(path);
    }
    if let Ok(dir) = app.path().resource_dir() {
        candidates.push(dir.join("assets").join(name));
        candidates.push(dir.join("_up_").join("assets").join(name));
    }
    if let Ok(cwd) = std::env::current_dir() {
        candidates.push(cwd.join("assets").join(name));
        candidates.push(cwd.join("../assets").join(name));
    }

    candidates.push(resource_path(app, name));
    candidates
}

fn load_icon(path: std::path::PathBuf) -> Result<tauri::image::Image<'static>, image::ImageError> {
    let img = image::open(path)?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    Ok(tauri::image::Image::new_owned(
        rgba.into_raw(),
        width,
        height,
    ))
}

fn load_resource_icon(app: &AppHandle, name: &str) -> Option<tauri::image::Image<'static>> {
    resource_candidates(app, name)
        .into_iter()
        .find_map(|path| load_icon(path).ok())
}

pub fn create_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let is_macos = cfg!(target_os = "macos");

    // Build context menu
    let open_item = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
    let refresh_item = MenuItem::with_id(app, "refresh", "Refresh", true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;

    let menu = Menu::with_items(
        app,
        &[
            &open_item,
            &refresh_item,
            &separator,
            &settings_item,
            &separator,
            &quit_item,
        ],
    )?;

    let default_icon = load_resource_icon(app, "outlook_macOS.png")
        .or_else(|| app.default_window_icon().cloned())
        .ok_or("No tray icon available")?;

    let _tray = TrayIconBuilder::with_id("main")
        .icon(default_icon)
        .icon_as_template(is_macos)
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "open" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "refresh" => {
                let _ = windows::refresh_page(app.clone());
            }
            "settings" => {
                let _ = windows::open_settings(app.clone());
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
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
        let icon = if is_macos {
            if has_unread {
                load_resource_icon(&app, "outlook_macOS_unread.png")
            } else {
                load_resource_icon(&app, "outlook_macOS.png")
            }
        } else {
            if has_unread {
                load_resource_icon(&app, "outlook_linux_unread.png")
            } else {
                app.path()
                    .resolve("icons/512x512.png", tauri::path::BaseDirectory::Resource)
                    .ok()
                    .and_then(|path| load_icon(path).ok())
                    .or_else(|| load_resource_icon(&app, "outlook_macOS.png"))
            }
        };

        if let Some(icon) = icon {
            let _ = tray.set_icon(Some(icon));
            let _ = tray.set_icon_as_template(is_macos);
        }
    }

    Ok(())
}
