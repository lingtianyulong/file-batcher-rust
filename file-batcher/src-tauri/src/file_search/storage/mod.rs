#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct FileId(u64);
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ParentId(u64);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullPath(String);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileName(String);
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct FileSize(u64);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileType(String);
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CreateTime(i64);
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ModifyTime(i64);
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct IsDir(bool);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub file_id: FileId,
    pub parent_id: ParentId,
    pub full_path: FullPath,
    pub file_name: FileName,
    pub file_size: FileSize,
    pub file_type: FileType,
    pub create_time: CreateTime,
    pub modify_time: ModifyTime,
    pub is_dir: IsDir,
}

impl Default for FileInfo {
    fn default() -> Self {
        Self {
            file_id: FileId(0),
            parent_id: ParentId(0),
            full_path: FullPath(String::new()),
            file_name: FileName(String::new()),
            file_size: FileSize(0),
            file_type: FileType(String::new()),
            create_time: CreateTime(0),
            modify_time: ModifyTime(0),
            is_dir: IsDir(false),
        }
    }
}

impl FileInfo {
    pub fn new(
        file_id: FileId,
        parent_id: ParentId,
        full_path: FullPath,
        file_name: FileName,
        file_size: FileSize,
        file_type: FileType,
        create_time: CreateTime,
        modify_time: ModifyTime,
        is_dir: IsDir,
    ) -> Self {
        Self {
            file_id,
            parent_id,
            full_path,
            file_name,
            file_size,
            file_type,
            create_time,
            modify_time,
            is_dir,
        }
    }

    pub fn set_file_id(&mut self, file_id: FileId) {
        self.file_id = file_id;
    }

    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }

    pub fn set_parent_id(&mut self, parent_id: ParentId) {
        self.parent_id = parent_id;
    }

    pub fn get_parent_id(&self) -> ParentId {
        self.parent_id
    }

    pub fn set_full_path(&mut self, full_path: FullPath) {
        self.full_path = full_path;
    }

    pub fn get_full_path(&self) -> FullPath {
        self.full_path.clone()
    }

    pub fn set_file_name(&mut self, file_name: FileName) {
        self.file_name = file_name;
    }

    pub fn get_file_name(&self) -> FileName {
        self.file_name.clone()
    }

    pub fn set_file_size(&mut self, file_size: FileSize) {
        self.file_size = file_size;
    }

    pub fn get_file_size(&self) -> FileSize {
        self.file_size
    }

    pub fn set_file_type(&mut self, file_type: FileType) {
        self.file_type = file_type;
    }

    pub fn get_file_type(&self) -> FileType {
        self.file_type.clone()
    }

    pub fn set_create_time(&mut self, create_time: CreateTime) {
        self.create_time = create_time;
    }

    pub fn get_create_time(&self) -> CreateTime {
        self.create_time
    }

    pub fn set_modify_time(&mut self, modify_time: ModifyTime) {
        self.modify_time = modify_time;
    }

    pub fn get_modify_time(&self) -> ModifyTime {
        self.modify_time
    }

    pub fn set_is_dir(&mut self, is_dir: IsDir) {
        self.is_dir = is_dir;
    }

    pub fn get_is_dir(&self) -> IsDir {
        self.is_dir
    }
}
