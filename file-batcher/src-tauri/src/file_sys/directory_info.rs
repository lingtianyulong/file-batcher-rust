use std::{os::windows::fs::MetadataExt, path::Path};
use serde::{Deserialize, Serialize};
use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_HIDDEN;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirFileInfo {
    name: String,           // 目录名称
    path: String,           // 目录路径
    is_dir: bool,           // 是否是目录
    has_sub_dir: bool,      // 是否包含子目录
    sub_dirs: Vec<DirFileInfo>,  // 子目录
    file_type: Option<String>,   // 文件类型
}

impl DirFileInfo {
    pub fn get_dir_info(dir_path: &str) -> Result<DirFileInfo, Box<dyn std::error::Error + 'static>> {
        log::info!("get_dir_info, the dir path is {}", dir_path);
        let normalized_path = if dir_path.ends_with(':') {
            format!(r"{}\", dir_path)
        } else {
            dir_path.to_string()
        };
        let path = Path::new(&normalized_path);
        if !path.is_dir() {
            log::error!("get_dir_info, the dir path not found, the dir path is {}", dir_path);
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Directory not found")));
        }

        let dir_name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| path.display().to_string());

        let mut dir_info = DirFileInfo {
            name: dir_name.clone(),
            path: dir_path.to_string(),
            is_dir: true,
            has_sub_dir: false,
            sub_dirs: Vec::new(),
            file_type: None,
        };

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
            let is_entry_dir = entry_path.is_dir();
            if is_entry_dir {
                dir_info.has_sub_dir = true;
            }

            let file_type = entry_path.extension().map(|ext| ext.to_string_lossy().to_lowercase());

            let display_path = entry_path.display().to_string();
            let entry_name = entry
                .file_name()
                .to_string_lossy()
                .into_owned();

            dir_info.sub_dirs.push(DirFileInfo {
                name: entry_name,
                path: display_path,
                is_dir: is_entry_dir,
                file_type,
                // 目录节点可继续展开；是否真有子项在下次展开时再拉取
                has_sub_dir: is_entry_dir,
                sub_dirs: Vec::new(),
            });

        }

        log::info!(
            "get_dir_info, path={}, child_count={}, has_sub_dir={}",
            dir_path,
            dir_info.sub_dirs.len(),
            dir_info.has_sub_dir
        );

        Ok(dir_info)
    }
}