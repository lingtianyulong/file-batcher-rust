use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub id: u128,
    pub name: String,
    pub path: String,
    pub file_size: u64,
    pub file_type: String,
    pub file_create_time: String,
    pub file_modify_time: String,
    pub file_extension: String,
    pub is_dir: bool,
}

impl Default for FileEntry {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().as_u128(),
            name: String::new(),
            path: String::new(),
            file_size: 0,
            file_type: String::new(),
            file_create_time: String::new(),
            file_modify_time: String::new(),
            file_extension: String::new(),
            is_dir: false,
        }
    }
}

impl FileEntry {
    pub fn set_id(&mut self, id: u128) {
        self.id = id;
    }

    pub fn get_id(&self) -> u128 {
        self.id
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn set_file_size(&mut self, file_size: u64) {
        self.file_size = file_size;
    }

    pub fn get_file_size(&self) -> u64 {
        self.file_size
    }

    pub fn set_file_type(&mut self, file_type: String) {
        self.file_type = file_type;
    }

    pub fn get_file_type(&self) -> String {
        self.file_type.clone()
    }

    pub fn set_create_time(&mut self, file_create_time: String) {
        self.file_create_time = file_create_time;
    }

    pub fn get_create_time(&self) -> String {
        self.file_create_time.clone()
    }

    pub fn set_modify_time(&mut self, file_modify_time: String) {
        self.file_modify_time = file_modify_time;
    }

    pub fn get_modify_time(&self) -> String {
        self.file_modify_time.clone()
    }
}
