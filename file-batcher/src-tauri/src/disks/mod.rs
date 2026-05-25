use serde::{Deserialize, Serialize};
use sysinfo::Disks;

const BYTES_PER_GB: f64 = 1024.0 * 1024.0 * 1024.0;

fn bytes_to_gb(bytes: u64) -> u64 {
    f64::floor((bytes as f64 / BYTES_PER_GB * 100.0).round() / 100.0) as u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    pub disk_name: String,
    pub file_system: String,
    pub mount_point: String,
    pub total_space: u64,
    pub used_space: u64,
    pub free_space: u64,
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
                total_space: bytes_to_gb(disk.total_space()),
                used_space: bytes_to_gb(disk.total_space() - disk.available_space()),
                free_space: bytes_to_gb(disk.available_space()),
            });
        }
        Ok(disk_info_list)
    }

    pub fn get_disk_info(
        disk_name: &str,
    ) -> Result<DiskInfo, Box<dyn std::error::Error + 'static>> {
        let disk_list = Disks::new_with_refreshed_list();
        let target = String::from(disk_name) + "\\";

        for disk in disk_list.list() {
            let mount = disk.mount_point().to_string_lossy().into_owned();
            if mount.eq_ignore_ascii_case(target.as_str()) {
                return Ok(DiskInfo {
                    disk_name: disk.name().to_string_lossy().into_owned(),
                    file_system: disk.file_system().to_string_lossy().into_owned(),
                    mount_point: disk.mount_point().to_string_lossy().into_owned(),
                    total_space: bytes_to_gb(disk.total_space()),
                    used_space: bytes_to_gb(disk.total_space() - disk.available_space()),
                    free_space: bytes_to_gb(disk.available_space()),
                });
            }
        }

        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Disk not found: {}", disk_name),
        )))
    }
}
