use crate::common::user::User;
use rusqlite::{Connection as RawSqliteConnection, Result as SqlResult};
use std::env;
use std::fs::File;

struct SqliteConnection;
impl SqliteConnection {
    fn connect(path: &str) -> SqlResult<RawSqliteConnection> {
        RawSqliteConnection::open(path)
    }
}

#[tauri::command]
pub fn register_command(username: String, password: String) -> Result<String, String> {
    log::info!("register_command, username: {}, password: {}", username, password);
    let user = User::new(username, password);
    let db_path = env::current_exe().unwrap().parent().unwrap().join("file-batcher.db");
    if !db_path.exists() {
        File::create(&db_path).expect("failed to create sqlite database file");
    }
    let db_file = db_path.to_string_lossy().replace('\\', "/");
    log::info!("db_file: {}", db_file);

    match SqliteConnection::connect(db_file.as_str()) {
        Ok(db) => {
            let result = db.execute("INSERT INTO users (id, username, password, create_time, update_time) VALUES (?, ?, ?, ?, ?)", (user.id, user.username, user.password, user.create_time, user.update_time)).unwrap();
            if result == 0 {
                return Err(String::from("register failed"));
            }
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
    Ok(String::from("register success"))
}