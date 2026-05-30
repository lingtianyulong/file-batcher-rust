use crate::file_sys::file_op::FileOp;

#[tauri::command]
pub async fn paste_files_command(sources: Vec<String>, target: &str, is_cut: bool) -> Result<(), String> {
    let result = FileOp::paste_files(sources, target, is_cut);
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string().into()),
    }
}

#[tauri::command]
pub async fn delete_files_command(sources: Vec<String>) -> Result<(), String> {
    let result = FileOp::delete_files(sources);
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string().into()),
    }
}