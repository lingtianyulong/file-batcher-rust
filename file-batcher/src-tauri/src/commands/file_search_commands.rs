use crate::file_search::filesystem_provider::FileSystemProvider;
use crate::file_search::win::windows_provider::*;

#[tauri::command]
pub async fn scan_files_command() -> Result<String, String> {
    log::info!("scan_files_command");
    let windows_provider = WindowsProvider::new();
    windows_provider.metadata().await;
    Ok(String::from("scan_files_command success"))
}
