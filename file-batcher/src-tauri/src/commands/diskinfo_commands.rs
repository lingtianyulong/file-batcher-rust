use crate::disks::DiskInfo;

#[tauri::command]
pub async fn get_disk_list_command() -> Result<String, String> {
    let disk_info = match DiskInfo::get_disk_list() {
        Ok(disk_info) => disk_info,
        Err(e) => {
            log::error!("get disk info failed, the error is {}", e);
            return Err(e.to_string());
        }
    };
    let json = match serde_json::to_string(&disk_info) {
        Ok(json) => json,
        Err(e) => {
            log::error!("convert disk info to json failed, the error is {}", e);
            return Err(e.to_string());
        }
    };
    log::info!("get disk info command success, the disk info is {}", json);
    Ok(json)
}