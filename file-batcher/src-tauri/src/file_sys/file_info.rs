use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use humansize::{format_size, BINARY};
use std::fs;
use std::path::Path;
use std::time::SystemTime;


#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub file_name: Option<String>,
    pub file_path: Option<String>,
    pub file_type: Option<String>,
    pub file_size: Option<String>,
    pub file_create_time: Option<String>,
    pub file_modify_time: Option<String>,
}

/**
 * 获取文件信息
 * @param file_path: &str
 * @return: Result<FileInfo, Box<dyn std::error::Error + 'static>>
 */
pub fn get_file_info(file_path: &str) -> Result<FileInfo, Box<dyn std::error::Error + 'static>> {
    log::info!("get_file_info: {}", file_path);
    let path = Path::new(file_path);
    if !path.exists() {
        let error = std::io::Error::new(std::io::ErrorKind::NotFound, format!("File not found: {}", file_path));
        return Err(Box::new(error));
    }

    let file_name = match path.file_name() {
        Some(file_name) => file_name.to_string_lossy().to_string(),
        None => {
            let error = std::io::Error::new(std::io::ErrorKind::NotFound, format!("File name not found: {}", file_path));
            return Err(Box::new(error));
        }
    };

    let file_path = match path.parent() {
        Some(file_path) => file_path.to_string_lossy().to_string(),
        None => {
            let error = std::io::Error::new(std::io::ErrorKind::NotFound, format!("File path not found: {}", file_path));
            return Err(Box::new(error));
        }
    };

    let file_type = match path.extension() {
        Some(file_type) => file_type.to_string_lossy().to_string(),
        None => {
            let error = std::io::Error::new(std::io::ErrorKind::NotFound, format!("File type not found: {}", file_path));
            return Err(Box::new(error));
        }
    };

    let file_size = match fs::metadata(path) {
        Ok(metadata) => format_size(metadata.len(), BINARY),
        Err(_) => {
            let error = std::io::Error::new(std::io::ErrorKind::NotFound, format!("File size not found: {}", file_path));
            return Err(Box::new(error));
        }
    };

    let file_create_time = match fs::metadata(path) {
        Ok(metadata) => {
            let create_time = metadata.created().unwrap_or(SystemTime::now());
            let create_time: DateTime<Local> = create_time.into();
            create_time.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        Err(_) => {
            let error = std::io::Error::new(std::io::ErrorKind::NotFound, format!("File create time not found: {}", file_path));
            return Err(Box::new(error));
        }
    };
    let file_modify_time = match fs::metadata(path) {
        Ok(metadata) => {
            let modify_time = metadata.modified().unwrap_or(SystemTime::now());
            let modify_time: DateTime<Local> = modify_time.into();
            modify_time.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        Err(_) => {
            let error = std::io::Error::new(std::io::ErrorKind::NotFound, format!("File modify time not found: {}", file_path));
            return Err(Box::new(error));
        }
    };

    Ok(FileInfo {
        file_name: Some(file_name),
        file_path: Some(file_path),
        file_type: Some(file_type),
        file_size: Some(file_size),
        file_create_time: Some(file_create_time),
        file_modify_time: Some(file_modify_time),
    })
}

/**
 * 获取文件列表
 * @param file_path: &str
 * @return: Result<Vec<FileInfo>, Box<dyn std::error::Error + 'static>>
 */
pub fn get_file_list(file_path: &str) -> Result<Vec<FileInfo>, Box<dyn std::error::Error + 'static>> {
    log::info!("get_file_list: {}", file_path);
    let path = Path::new(file_path);
    if !path.exists() {
        let error = std::io::Error::new(std::io::ErrorKind::NotFound, format!("File not found: {}", file_path));
        return Err(Box::new(error));
    }
    let files = fs::read_dir(path)?;
    let mut file_list = Vec::new();
    for file in files {
        let file = file?;
        let file_path = file.path();
        let file_info = match get_file_info(&file_path.to_string_lossy()) {
            Ok(file_info) => file_info,
            Err(e) => {
                log::error!("get file info failed, the error is {}", e.to_string());
                continue;
            }
        };
        file_list.push(file_info);
    }
    Ok(file_list)
}