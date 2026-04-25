use tauri::{
    menu::{Menu, MenuBuilder, PredefinedMenuItem, SubmenuBuilder},
    AppHandle,
};

pub fn create_menu(app: &AppHandle) -> Result<Menu<tauri::Wry>, Box<dyn std::error::Error>> {
    let is_macos = cfg!(target_os = "macos");

    if is_macos {
        // macOS Application menu
        let app_menu = SubmenuBuilder::new(app, "Application")
            .item(&PredefinedMenuItem::about(app, Some("Freelook"), None)?)
            .separator()
            .item(&PredefinedMenuItem::quit(app, Some("Quit"))?)
            .build()?;

        // Edit menu with standard shortcuts
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

        let menu = MenuBuilder::new(app)
            .item(&app_menu)
            .item(&edit_menu)
            .build()?;

        Ok(menu)
    } else {
        // Non-macOS: use default menu
        let menu = MenuBuilder::new(app).build()?;
        Ok(menu)
    }
}
