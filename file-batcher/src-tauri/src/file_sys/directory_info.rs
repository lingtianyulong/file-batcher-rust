use std::{os::windows::fs::MetadataExt, path::Path};
use serde::{Deserialize, Serialize};
use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_HIDDEN;


#[derive(Debug, Serialize, Deserialize)]
pub struct DirFileInfo {
    name: String,           // 目录名称
    path: String,           // 目录路径
    is_dir: bool,           // 是否是目录
    has_sub_dir: bool,      // 是否包含子目录
    sub_dirs: Vec<String>,  // 子目录
}

impl DirFileInfo {
    pub fn get_dir_info(dir_path: &str) -> Result<DirFileInfo, Box<dyn std::error::Error + 'static>> {
        log::info!("get_dir_info, the dir path is {}", dir_path);
        let path = Path::new(dir_path).join(r"\");
        if !path.is_dir() {
            log::error!("get_dir_info, the dir path not found, the dir path is {}", dir_path);
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Directory not found")));
        }

        let dir_name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| path.display().to_string());

        let mut sub_dirs = Vec::new();
        let mut has_sub_dir = false;
        let mut is_dir = false;

        for entry in std::fs::read_dir(path)? {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    log::error!("get_dir_info, the entry not found, the error is {}", e);
                    continue;
                }
            };

            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(e) => {
                    log::error!("get_dir_info, the metadata not found, the error is {}", e);
                    continue;
                }
            };

            let attrs = metadata.file_attributes();
            if attrs & FILE_ATTRIBUTE_HIDDEN != 0 {
                // 过滤隐藏文件
                continue;
            }

            let entry_path = entry.path();
            sub_dirs.push(entry_path.display().to_string());
            if entry_path.is_dir() {
                has_sub_dir = true;
                is_dir = true;
            }

        }

        log::info!(
            "get_dir_info, path={}, child_count={}, has_sub_dir={}",
            dir_path,
            sub_dirs.len(),
            has_sub_dir
        );

        Ok(DirFileInfo {
            name: dir_name.clone(),
            path: dir_name.clone(),
            is_dir,
            has_sub_dir,
            sub_dirs,
        })
    }
}