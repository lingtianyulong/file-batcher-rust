#[tauri::command]
pub async fn scan_files_command() -> Result<String, String> {
    log::info!("scan_files_command");
    Ok(String::from("scan_files_command success"))
}
