use chrono::Local;
use uuid::{Uuid, Timestamp};

/**
 * 用户
 */
#[allow(dead_code)]
 pub struct User {
    pub id: String,                // 用户ID
    pub username: String,               // 用户名
    pub password: String,               // 用户密码
    pub create_time: Option<String>,    // 创建时间
    pub update_time: Option<String>,    // 更新时间
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        let timestamp = Timestamp::now(uuid::NoContext);
        Self {
            id: Uuid::new_v7(timestamp).to_string(),
            username,
            password,
            create_time: Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: None,
        }
    }
}
