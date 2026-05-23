use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::{LogicalPosition, Window};


#[tauri::command]
pub fn show_contextmenu_command(window: Window, x: f64, y: f64) -> Result<(), String> {
    let menu = Menu::new(&window).map_err(|e| e.to_string())?;
    let open_item = MenuItem::with_id(&window, "open", "打开文件", true, Option::<&str>::None)
        .map_err(|e| e.to_string())?;
    let open_folder_item = MenuItem::with_id(&window, "open_folder", "打开文件夹", true, Option::<&str>::None)
        .map_err(|e| e.to_string())?;
    let separator = PredefinedMenuItem::separator(&window).map_err(|e| e.to_string())?;
    let cut_item = MenuItem::with_id(&window, "cut", "剪切", true, Option::<&str>::None).map_err(|e| e.to_string())?;
    let copy_item = MenuItem::with_id(&window, "copy", "复制", true, Option::<&str>::None).map_err(|e| e.to_string())?;
    let paste_item = MenuItem::with_id(&window, "paste", "粘贴", true, Option::<&str>::None).map_err(|e| e.to_string())?;

    let rename_item = MenuItem::with_id(&window, "rename", "重命名", true, Option::<&str>::None).map_err(|e| e.to_string())?;
    let delete_item = MenuItem::with_id(&window, "delete", "删除", true, Option::<&str>::None).map_err(|e| e.to_string())?;

    menu.append_items(&[
        &open_item,
        &open_folder_item,
        &separator,
        &cut_item,
        &copy_item,
        &paste_item,
        &separator,
        &rename_item,
        &delete_item,
    ]).map_err(|e| e.to_string())?;
    window
        .popup_menu_at(&menu, LogicalPosition::new(x, y))
        .map_err(|e| e.to_string())?;
    Ok(())
}
