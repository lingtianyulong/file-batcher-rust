use sysinfo::Disks;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    pub disk_name: String,
    pub file_system: String,
    pub mount_point: String,
}

impl DiskInfo {
    pub fn get_disk_list() -> Result<Vec<DiskInfo>, Box<dyn std::error::Error + 'static>> {
        let disk_list = Disks::new_with_refreshed_list();
        let mut disk_info_list: Vec<DiskInfo> = Vec::new();
        for disk in disk_list.list() {
            disk_info_list.push(DiskInfo {
                disk_name: disk.name().to_string_lossy().into_owned(),
                file_system: disk.file_system().to_string_lossy().into_owned(),
                mount_point: disk.mount_point().to_string_lossy().into_owned(),
            });
        }
        Ok(disk_info_list)
    }
}
