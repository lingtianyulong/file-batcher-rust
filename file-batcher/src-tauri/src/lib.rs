// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod commands;
mod models;
use commands::button_commands::*;
use chrono::Local;
use tauri::Manager;
#[cfg(target_os = "windows")]
use window_vibrancy::apply_blur;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let log_plugin = tauri_plugin_log::Builder::new()
        .format(|out, message, record| {
        let now = Local::now();
        let formatted = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        out.finish(format_args!("{} [{}] - {}", formatted, record.level(), message));
    })
    .target(
        tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Folder {
                path: std::path::PathBuf::from("logs"),
                file_name: Some("app.log".to_string()),
            },
        )
    )
    .build();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            #[cfg(target_os = "windows")]
            {
                let _ = apply_blur(&window, Some((18, 18, 18, 125)));
            }
            window.show().unwrap();
            Ok(())
        })
        .plugin(log_plugin)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_file_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
