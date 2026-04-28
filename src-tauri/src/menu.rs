use tauri::{
    menu::{Menu, MenuBuilder, MenuItem, PredefinedMenuItem, SubmenuBuilder},
    AppHandle,
};

pub fn create_menu(app: &AppHandle) -> Result<Menu<tauri::Wry>, Box<dyn std::error::Error>> {
    let is_macos = cfg!(target_os = "macos");

    // Edit menu with standard copy/paste shortcuts (all platforms)
    let edit_menu = SubmenuBuilder::new(app, "Edit")
        .item(&PredefinedMenuItem::undo(app, None)?)
        .item(&PredefinedMenuItem::redo(app, None)?)
        .separator()
        .item(&PredefinedMenuItem::cut(app, None)?)
        .item(&PredefinedMenuItem::copy(app, None)?)
        .item(&PredefinedMenuItem::paste(app, None)?)
        .separator()
        .item(&PredefinedMenuItem::select_all(app, None)?)
        .build()?;

    // View menu with Refresh
    let refresh_item = MenuItem::with_id(app, "refresh", "Refresh", true, Some("CmdOrCtrl+R"))?;
    let view_menu = SubmenuBuilder::new(app, "View")
        .item(&refresh_item)
        .build()?;

    if is_macos {
        // macOS Application menu
        let app_menu = SubmenuBuilder::new(app, "Application")
            .item(&PredefinedMenuItem::about(app, Some("Freelook"), None)?)
            .separator()
            .item(&PredefinedMenuItem::quit(app, Some("Quit"))?)
            .build()?;

        let menu = MenuBuilder::new(app)
            .item(&app_menu)
            .item(&edit_menu)
            .item(&view_menu)
            .build()?;

        Ok(menu)
    } else {
        // Non-macOS: Edit menu ensures Ctrl+C/V/X work
        let menu = MenuBuilder::new(app)
            .item(&edit_menu)
            .item(&view_menu)
            .build()?;

        Ok(menu)
    }
}
