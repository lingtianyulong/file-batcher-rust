use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
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

fn format_file_size(size: u64) -> String {
    std::cfg_select! {
        target_os = "windows" => {
            // 添加 MB、GB 的转换逻辑
            if size >= 1024 * 1024 * 1024 {
                let gb = (size as f64) / (1024_f64 * 1024_f64 * 1024_f64);
                format!("{:.2} GB", gb)
            } else if size >= 1024 * 1024 {
                let mb = (size as f64) / (1024_f64 * 1024_f64);
                format!("{:.2} MB", mb)
            } else if size >= 1024 {
                let kb = (size as f64) / 1024_f64;
                format!("{:.2} KB", kb)
            } else {
                let kb = (size + 1023) / 1024;  // 向上取整
                format!("{} KB", kb)
            }
        },
        any(target_os = "linux", target_os = "macos") => {
            use humansize::{format_size, BINARY};
            format_size(size, BINARY)
        },
        _ => {
            format!("{} B", size)
        }
    }
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
        let error = std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", file_path),
        );
        return Err(Box::new(error));
    }

    let file_name = match path.file_name() {
        Some(file_name) => file_name.to_string_lossy().to_string(),
        None => {
            let error = std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File name not found: {}", file_path),
            );
            return Err(Box::new(error));
        }
    };

    let file_path = match path.parent() {
        Some(file_path) => file_path.to_string_lossy().to_string(),
        None => {
            let error = std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File path not found: {}", file_path),
            );
            return Err(Box::new(error));
        }
    };

    let file_type = match path.extension() {
        Some(file_type) => file_type.to_string_lossy().to_lowercase(),
        None => {
            let error = std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File type not found: {}", file_path),
            );
            return Err(Box::new(error));
        }
    };

    let file_size = match fs::metadata(path) {
        Ok(metadata) => format_file_size(metadata.len()),
        Err(_) => {
            let error = std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File size not found: {}", file_path),
            );
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
            let error = std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File create time not found: {}", file_path),
            );
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
            let error = std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File modify time not found: {}", file_path),
            );
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
pub fn get_file_list(
    file_path: &str,
) -> Result<Vec<FileInfo>, Box<dyn std::error::Error + 'static>> {
    log::info!("get_file_list: {}", file_path);
    let path = Path::new(file_path);
    if !path.exists() {
        let error = std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", file_path),
        );
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

#[allow(dead_code)]
pub fn get_file_list_with_progress<F>(
    file_path: &str,
    mut on_progress: F,
) -> Result<Vec<FileInfo>, Box<dyn std::error::Error + 'static>>
where
    F: FnMut(usize, usize, Option<String>),
{
    log::info!("get_file_list_with_progress: {}", file_path);
    let path = Path::new(file_path);
    if !path.exists() {
        let error = std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", file_path),
        );
        return Err(Box::new(error));
    }

    let entries = fs::read_dir(path)?.collect::<Result<Vec<_>, std::io::Error>>()?;
    let total = entries.len();
    if total == 0 {
        on_progress(0, 0, None);
        return Ok(Vec::new());
    }

    let mut file_list = Vec::new();
    for (index, file) in entries.into_iter().enumerate() {
        let file_path = file.path();
        let file_name = file_path
            .file_name()
            .map(|name| name.to_string_lossy().to_string());
        let file_info = match get_file_info(&file_path.to_string_lossy()) {
            Ok(file_info) => file_info,
            Err(e) => {
                log::error!("get file info failed, the error is {}", e.to_string());
                on_progress(index + 1, total, file_name);
                continue;
            }
        };
        file_list.push(file_info);
        on_progress(index + 1, total, file_name);
    }
    Ok(file_list)
}

/**
 * 重命名文件
 * @param file_dir: &str
 * @param old_file_name: &str
 * @param new_file_name: &str
 * @return: Result<FileInfo, Box<dyn std::error::Error + 'static>>
 */
pub fn rename_file(
    file_dir: &str,
    old_file_name: &str,
    new_file_name: &str,
) -> Result<FileInfo, Box<dyn std::error::Error + 'static>> {
    log::info!(
        "rename_file: file_dir={}, old_file_name={}, new_file_name={}",
        file_dir,
        old_file_name,
        new_file_name
    );

    let new_file_name = new_file_name.trim();
    if new_file_name.is_empty() {
        let error = std::io::Error::new(std::io::ErrorKind::InvalidInput, "New file name is empty");
        return Err(Box::new(error));
    }

    let old_file_path = PathBuf::from(file_dir).join(old_file_name);
    if !old_file_path.exists() {
        let error = std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", old_file_path.to_string_lossy()),
        );
        return Err(Box::new(error));
    }

    let new_file_path = PathBuf::from(file_dir).join(new_file_name);
    if new_file_path.exists() {
        let error = std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("File already exists: {}", new_file_path.to_string_lossy()),
        );
        return Err(Box::new(error));
    }

    fs::rename(&old_file_path, &new_file_path)?;
    get_file_info(&new_file_path.to_string_lossy())
}
