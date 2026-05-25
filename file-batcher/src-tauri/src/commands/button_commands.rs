// 按钮命令
use crate::file_sys::file_info::rename_file;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[allow(dead_code)]
const FILE_LIST_PROGRESS_EVENT: &str = "rename:folder-loading-progress";

#[allow(dead_code)]
#[derive(Clone, Serialize)]
struct FileListProgressPayload {
    request_id: String,
    current: usize,
    total: usize,
    percentage: u8,
    file_name: Option<String>,
}

#[allow(dead_code)]
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
pub fn rename_file_command(
    file_dir: &str,
    old_file_name: &str,
    new_file_name: &str,
) -> Result<String, String> {
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
            log::error!(
                "convert renamed file info to json failed, the error is {}",
                e
            );
            return Err(e.to_string());
        }
    };
    log::info!("rename file command success, the file info is {}", json);
    Ok(json)
}
