// 按钮命令
use crate::file_sys::file_info::{ get_file_info, get_file_list };

#[tauri::command]
pub fn get_file_info_command(file_path: &str) -> Result<String, String> {
    log::info!("get_file_info_command, the file path is {}", file_path);
    let file_info = match get_file_info(file_path) {
        Ok(file_info) => file_info,
        Err(e) => {
            log::error!("get file info failed, the error is {}", e);
            return Err(e.to_string());
        }
    };

    let json = match serde_json::to_string(&file_info) {
        Ok(json) => json,
        Err(e) => {
            log::error!("convert file info to json failed, the error is {}", e);
            return Err(e.to_string());
        }
    };
    log::info!("get file info command success, the file info is {}", json);
    Ok(json)
}

#[tauri::command]
pub fn get_file_list_command(file_path: &str) -> Result<String, String> {
    log::info!("get_file_list_command, the file path is {}", file_path);
    let file_list = match get_file_list(file_path) {
        Ok(file_list) => file_list,
        Err(e) => {
            log::error!("get file list failed, the error is {}", e);
            return Err(e.to_string());
        }
    };
    let json = match serde_json::to_string(&file_list) {
        Ok(json) => json,
        Err(e) => {
            log::error!("convert file list to json failed, the error is {}", e);
            return Err(e.to_string());
        }
    };
    log::info!("get file list command success, the file list is {}", json);
    Ok(json)
}
