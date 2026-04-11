// 按钮命令
use crate::models::file_info::FileInfo;
use chrono::{DateTime, Local};
use humansize::{format_size, BINARY};
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[tauri::command]
pub fn get_file_info(file_path: &str) -> Result<String, String> {
    log::info!("get_file_info: {}", file_path);
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let file_name = match path.file_name() {
        Some(file_name) => file_name.to_string_lossy().to_string(),
        None => return Err(format!("File name not found: {}", file_path)),
    };

    let file_path = match path.parent() {
        Some(file_path) => file_path.to_string_lossy().to_string(),
        None => return Err(format!("File path not found: {}", file_path)),
    };

    let file_type = match path.extension() {
        Some(file_type) => file_type.to_string_lossy().to_string(),
        None => return Err(format!("File type not found: {}", file_path)),
    };

    let file_size = match fs::metadata(path) {
        Ok(metadata) => format_size(metadata.len(), BINARY),
        Err(_) => return Err(format!("File size not found: {}", file_path)),
    };

    let file_create_time = match fs::metadata(path) {
        Ok(metadata) => {
            let create_time = metadata.created().unwrap_or(SystemTime::now());
            let create_time: DateTime<Local> = create_time.into();
            create_time.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        Err(_) => return Err(format!("File create time not found: {}", file_path)),
    };
    let file_modify_time = match fs::metadata(path) {
        Ok(metadata) => {
            let modify_time = metadata.modified().unwrap_or(SystemTime::now());
            let modify_time: DateTime<Local> = modify_time.into();
            modify_time.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        Err(_) => return Err(format!("File modify time not found: {}", file_path)),
    };

    let json = serde_json::json!(FileInfo {
        file_name: Some(file_name),
        file_path: Some(file_path),
        file_type: Some(file_type),
        file_size: Some(file_size),
        file_create_time: Some(file_create_time),
        file_modify_time: Some(file_modify_time),
    });
    let json = json.to_string();
    log::info!("get_file_info result: {}", json);
    Ok(json)
}
