// 按钮命令
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

