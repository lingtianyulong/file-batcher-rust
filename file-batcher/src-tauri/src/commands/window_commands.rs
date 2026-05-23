// 窗口命令
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

/// 打开批量操作子窗口
///
/// 如果窗口已存在则聚焦并返回；否则创建一个新的独立窗口并加载 `/batch` 路由。
#[tauri::command]
pub async fn open_batch_window_command(app: AppHandle) -> Result<(), String> {
    log::info!("open_batch_window_command invoked");

    if let Some(existing) = app.get_webview_window("batch") {
        log::info!("batch window already exists, focusing");
        existing.show().map_err(|e| e.to_string())?;
        existing.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    let window =
        WebviewWindowBuilder::new(&app, "batch", WebviewUrl::App("index.html#/batch".into()))
            .title("批量操作")
            .inner_size(900.0, 700.0)
            .min_inner_size(640.0, 480.0)
            .decorations(false)
            .transparent(false)
            .resizable(true)
            .center()
            .visible(false)
            .build()
            .map_err(|e| {
                log::error!("failed to build batch window: {}", e);
                e.to_string()
            })?;

    // 已根据需求移除毛玻璃 / 半透明效果，保留下方代码以备恢复
    // #[cfg(target_os = "windows")]
    // {
    //     use window_vibrancy::apply_mica;
    //     let _ = apply_mica(&window, Some(false));
    // }

    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;

    log::info!("batch window created successfully");
    Ok(())
}

/// 关闭批量操作子窗口
#[tauri::command]
pub async fn close_batch_window_command(app: AppHandle) -> Result<(), String> {
    log::info!("close_batch_window_command invoked");
    if let Some(window) = app.get_webview_window("batch") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn open_search_config_window_command(app: AppHandle) -> Result<(), String> {
    log::info!("open_search_config_window_command invoked");
    if let Some(existing) = app.get_webview_window("search-config") {
        log::info!("search-config window already exists, focusing");
        existing.show().map_err(|e| e.to_string())?;
        existing.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    let window =
        WebviewWindowBuilder::new(&app, "search-config", WebviewUrl::App("index.html#/search-config".into()))
            .title("搜索配置")
            .inner_size(1000.0, 800.0)
            .min_inner_size(640.0, 480.0)
            .decorations(false)
            .transparent(false)
            .resizable(true)
            .center()
            .visible(false)
            .build()
            .map_err(|e| {
                log::error!("failed to build search-config window: {}", e);
                e.to_string()
            })?;

    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;

    log::info!("search-config window created successfully");
    Ok(())
}

#[tauri::command]
pub async fn close_search_config_window_command(app: AppHandle) -> Result<(), String> {
    log::info!("close_search_config_window_command invoked");
    if let Some(window) = app.get_webview_window("search-config") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}