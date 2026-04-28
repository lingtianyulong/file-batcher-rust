// 按钮命令
use crate::file_sys::file_info::{ get_file_info, get_file_list, get_file_list_with_progress, rename_file };
use serde::Serialize;
use tauri::{AppHandle, Emitter};

const FILE_LIST_PROGRESS_EVENT: &str = "rename:folder-loading-progress";

#[derive(Clone, Serialize)]
struct FileListProgressPayload {
    request_id: String,
    current: usize,
    total: usize,
    percentage: u8,
    file_name: Option<String>,
}

fn emit_file_list_progress(
    app: &AppHandle,
    request_id: &str,
    current: usize,
    total: usize,
    file_name: Option<String>,
) {
    let percentage = if total == 0 {
        100
    } else {
        (((current as f64 / total as f64) * 100_f64).round() as u8).min(100)
    };
    let payload = FileListProgressPayload {
        request_id: request_id.to_string(),
        current,
        total,
        percentage,
        file_name,
    };
    // 向前端 UI 发送文件列表进度事件
    if let Err(e) = app.emit(FILE_LIST_PROGRESS_EVENT, payload) {
        log::error!("emit file list progress failed, the error is {}", e);
    }
}

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
pub fn get_file_list_command(app: AppHandle, file_path: &str, request_id: Option<String>) -> Result<String, String> {
    log::info!("get_file_list_command, the file path is {}", file_path);
    let request_id = request_id.unwrap_or_default();
    let file_list_result = if request_id.is_empty() {
        get_file_list(file_path)
    } else {
        get_file_list_with_progress(file_path, |current, total, file_name| {
            emit_file_list_progress(&app, &request_id, current, total, file_name);
        })
    };
    let file_list = match file_list_result {
        Ok(file_list) => file_list,
        Err(e) => {
            log::error!("get file list failed, the error is {}", e);
            return Err(e.to_string());
        }
    };
    if !request_id.is_empty() {
        emit_file_list_progress(&app, &request_id, file_list.len(), file_list.len(), None);
    }
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

#[tauri::command]
pub fn rename_file_command(file_dir: &str, old_file_name: &str, new_file_name: &str) -> Result<String, String> {
    log::info!(
        "rename_file_command, file_dir is {}, old_file_name is {}, new_file_name is {}",
        file_dir,
        old_file_name,
        new_file_name
    );
    let file_info = match rename_file(file_dir, old_file_name, new_file_name) {
        Ok(file_info) => file_info,
        Err(e) => {
            log::error!("rename file failed, the error is {}", e);
            return Err(e.to_string());
        }
    };

    let json = match serde_json::to_string(&file_info) {
        Ok(json) => json,
        Err(e) => {
            log::error!("convert renamed file info to json failed, the error is {}", e);
            return Err(e.to_string());
        }
    };
    log::info!("rename file command success, the file info is {}", json);
    Ok(json)
}
