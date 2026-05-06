// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod common;
mod commands;
mod file_sys;
mod login;
mod register;

use chrono::Local;
use commands::button_commands::*;
use commands::contextmenu_commands::*;
use commands::window_commands::*;
use register::*;
use login::commands::login_command;
use std::fs::File;
use tauri::Manager;
use tauri_plugin_sql::{Migration, MigrationKind};
use std::env;


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

        let migration = Migration{
            version: 1,
            description: "Initial migration",
            sql: "CREATE TABLE users (id INTEGER PRIMARY KEY NOT NULL, username TEXT NOT NULL, password TEXT NOT NULL)",
            kind: MigrationKind::Up,
        };

        let db_path = env::current_exe().unwrap().parent().unwrap().join("file-batcher.db");
        if !db_path.exists() {
            File::create(&db_path).expect("failed to create sqlite database file");
        }
        let db_url = format!("sqlite:{}", db_path.to_string_lossy().replace('\\', "/"));

        tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.show().unwrap();
            Ok(())
        })
        .plugin(log_plugin)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_sql::Builder::default().add_migrations(db_url.as_str(), vec![migration]).build())
        .invoke_handler(tauri::generate_handler![
            get_file_info_command,
            get_file_list_command,
            rename_file_command,
            open_batch_window_command,
            close_batch_window_command,
            show_contextmenu_command,
            login_command,
            register_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
