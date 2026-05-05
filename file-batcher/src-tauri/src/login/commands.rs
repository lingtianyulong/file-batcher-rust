// use crate::login::model::User;

#[tauri::command]
pub fn login_command(username: String, password: String) -> Result<String, String> {
    log::info!("login_command, username: {}, password: {}", username, password);

    Err(String::from("登录失败，账号或密码错误!"))


    // Ok(String::from("login success"))
    // let user = User {
    //     id: 1,
    //     username,
    //     password,
    // };
    // Ok(user.to_string())
}