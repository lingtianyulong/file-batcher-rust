// use crate::common::user::User;
// // use sqlx::Executor;
// use tauri::{AppHandle, Manager};
// use sqlx::{Pool, Sqlite};

// /// 与 `lib.rs` 中 `tauri_plugin_sql` 迁移/预加载使用的连接串一致
// pub const DB_URL: &str = "sqlite:file-batcher.db";

// #[tauri::command]
// pub async fn register_command(
//     app: AppHandle,
//     username: String,
//     password: String,
// ) -> Result<String, String> {
//     log::info!(
//         "register_command, username: {}, password: {}",
//         username,
//         password
//     );
//     let user = User::new(username, password);

//     let instances = app.state::<DbInstances>();
//     let guard = instances.0.read().await;
//     let pool = guard
//         .get(DB_URL)
//         .ok_or_else(|| "数据库未加载，请重启应用后重试".to_string())?;

//     let result = match pool {
//         DbPool::Sqlite(sqlite_pool) => {
//             sqlx::query(
//                 "INSERT INTO users (id, username, password, create_time, update_time) VALUES ($1, $2, $3, $4, $5)",
//             )
//             .bind(&user.id)
//             .bind(&user.username)
//             .bind(&user.password)
//             .bind(&user.create_time)
//             .bind(&user.update_time)
//             .execute(sqlite_pool)
//             .await
//             .map_err(|e| e.to_string())?
//         }
//         #[allow(unreachable_patterns)]
//         _ => return Err("不支持的数据库类型".to_string()),
//     };

//     if result.rows_affected() == 0 {
//         return Err("register failed".to_string());
//     }
//     Ok("register success".to_string())
// }
