use crate::file_sys::directory_info::{ DirFileInfo };
use serde_json;

#[tauri::command]
pub fn get_dir_info_command(dir_path: &str) -> Result<String, String> {
    log::info!("get_dir_info_command, the dir path is {}", dir_path);
    let dir_info = match DirFileInfo::get_dir_info(dir_path) {
        Ok(dir_info) => dir_info,
        Err(e) => {
            log::error!("get dir info failed, the error is {}", e);
            return Err(e.to_string());
        }
    };
    let json = match serde_json::to_string(&dir_info) {
        Ok(json) => json,
        Err(e) => {
            log::error!("convert dir info to json failed, the error is {}", e);
            return Err(e.to_string());
        }
    };
    log::info!("get dir info command success, the dir info is {}", json);
    Ok(json)
}