use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub file_name: Option<String>,
    pub file_path: Option<String>,
    pub file_type: Option<String>,
    pub file_size: Option<String>,
    pub file_create_time: Option<String>,
    pub file_modify_time: Option<String>,
}
