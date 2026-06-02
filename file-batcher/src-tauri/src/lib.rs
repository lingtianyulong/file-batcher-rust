// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod commands;
mod common;
mod disks;
mod file_sys;
mod login;
mod register;

use chrono::Local;
use commands::contextmenu_commands::*;
use commands::directory_commands::*;
use commands::diskinfo_commands::*;
use commands::file_info_commands::*;
use commands::file_op_commands::*;
use commands::window_commands::*;
use login::commands::login_command;
use register::{DB_URL, register_command};
use tauri::Emitter;
use tauri::Manager;
use tauri_plugin_sql::{Migration, MigrationKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_plugin = tauri_plugin_log::Builder::new()
        .format(|out, message, record| {
            let now = Local::now();
            let formatted = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
            out.finish(format_args!(
                "{} [{}] - {}",
                formatted,
                record.level(),
                message
            ));
        })
        .clear_targets()
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Folder {
                path: std::path::PathBuf::from("logs"),
                file_name: Some("app.log".to_string()),
            },
        ))
        .build();

    let migration = Migration {
        version: 1,
        description: "Initial migration",
        sql: concat!(
            "CREATE TABLE users (",
            "id TEXT PRIMARY KEY NOT NULL, ",
            "username TEXT NOT NULL, ",
            "password TEXT NOT NULL, ",
            "create_time TEXT, ",
            "update_time TEXT",
            ")"
        ),
        kind: MigrationKind::Up,
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.show()?;
            // 监听菜单事件, 向前端发送菜单事件
            app.on_menu_event(move |handle, event| {
                if let Err(error) = handle.emit("menu_event", event.id()) {
                    log::error!("failed to emit menu event: {error}");
                }
            });

            Ok(())
        })
        .plugin(log_plugin)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(DB_URL, vec![migration])
                .build(),
        )
        .manage(RenameWindowState::default())
        .invoke_handler(tauri::generate_handler![
            get_file_info_command,
            get_file_list_command,
            rename_file_command,
            open_batch_window_command,
            close_batch_window_command,
            show_contextmenu_command,
            login_command,
            register_command,
            get_disk_list_command,
            get_disk_info_command,
            get_dir_info_command,
            open_search_config_window_command,
            close_search_config_window_command,
            paste_files_command,
            delete_files_command,
            open_rename_window_command,
            request_rename_old_file_path_command,
            close_rename_window_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
