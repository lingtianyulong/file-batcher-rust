use tauri::menu::{Menu, MenuItem};
use tauri::{LogicalPosition, Window};

#[tauri::command]
pub fn show_contextmenu_command(window: Window, x: f64, y: f64) -> Result<(), String> {
    let menu = Menu::new(&window).map_err(|e| e.to_string())?;
    let open_item =
        MenuItem::with_id(&window, "open", "Open", true, Option::<&str>::None).map_err(|e| e.to_string())?;
    let close_item =
        MenuItem::with_id(&window, "close", "Close", true, Option::<&str>::None).map_err(|e| e.to_string())?;
    menu.append_items(&[&open_item, &close_item])
        .map_err(|e| e.to_string())?;
    window
        .popup_menu_at(&menu, LogicalPosition::new(x, y))
        .map_err(|e| e.to_string())?;
    Ok(())
}