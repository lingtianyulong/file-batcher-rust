// 窗口命令
use std::sync::Mutex;
use tauri::WindowEvent;
use tauri::{AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};

#[derive(Default)]
pub struct RenameWindowState {
    old_file_path: Mutex<Option<String>>,
}

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

    let main_window = match app.get_webview_window("main") {
        Some(window) => window,
        None => return Err("main window not found".to_string()),
    };
    main_window.set_enabled(false).map_err(|e| e.to_string())?;

    if let Some(existing) = app.get_webview_window("search-config") {
        log::info!("search-config window already exists, focusing");
        existing.show().map_err(|e| e.to_string())?;
        existing.set_focus().map_err(|e| e.to_string())?;

        existing.on_window_event(move |event| {
            if let WindowEvent::Destroyed = event {
                main_window
                    .set_enabled(true)
                    .map_err(|e| e.to_string())
                    .unwrap();
                log::info!("main window enabled");
                main_window.set_focus().map_err(|e| e.to_string()).unwrap();
                log::info!("main window focused");
            }
        });

        return Ok(());
    }

    let window = WebviewWindowBuilder::new(
        &app,
        "search-config",
        WebviewUrl::App("index.html#/search-config".into()),
    )
    .title("搜索配置")
    .inner_size(1600.0, 950.0)
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
    window.on_window_event(move |event| {
        if let WindowEvent::Destroyed = event {
            main_window
                .set_enabled(true)
                .map_err(|e| e.to_string())
                .unwrap();
            log::info!("main window enabled");
            main_window.set_focus().map_err(|e| e.to_string()).unwrap();
            log::info!("main window focused");
        }
    });

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

#[tauri::command]
pub async fn open_rename_window_command(app: AppHandle, old_file_path: &str) -> Result<(), String> {
    log::info!("open_rename_window_command invoked");
    let state = app.state::<RenameWindowState>();
    {
        let mut old_file_path_state = state.old_file_path.lock().map_err(|e| e.to_string())?;
        *old_file_path_state = Some(old_file_path.to_string());
    }

    let main_window = match app.get_webview_window("main") {
        Some(window) => window,
        None => return Err("main window not found".to_string()),
    };
    main_window.set_enabled(false).map_err(|e| e.to_string())?;

    if let Some(existing) = app.get_webview_window("rename") {
        log::info!("rename window already exists, focusing");
        existing.show().map_err(|e| e.to_string())?;
        existing.set_focus().map_err(|e| e.to_string())?;
        existing
            .emit(
                "rename_old_file_path",
                serde_json::json!({
                    "old_file_path": old_file_path,
                }),
            )
            .map_err(|e| e.to_string())?;
        existing.on_window_event(move |event| {
            if let WindowEvent::Destroyed = event {
                main_window
                    .set_enabled(true)
                    .map_err(|e| e.to_string())
                    .unwrap();
                log::info!("main window enabled");
                main_window.set_focus().map_err(|e| e.to_string()).unwrap();
                log::info!("main window focused");
            }
        });
        return Ok(());
    }

    let window =
        WebviewWindowBuilder::new(&app, "rename", WebviewUrl::App("index.html#/rename".into()))
            .title("文件重命名")
            .inner_size(360.0, 250.0)
            .min_inner_size(300.0, 225.0)
            .decorations(false)
            .transparent(false)
            .resizable(true)
            .center()
            .visible(false)
            .build()
            .map_err(|e| {
                log::error!("failed to build rename window: {}", e);
                e.to_string()
            })?;

    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;

    window
        .emit(
            "rename_old_file_path",
            serde_json::json!({
                "old_file_path": old_file_path,
            }),
        )
        .map_err(|e| e.to_string())?;

    window.on_window_event(move |event| {
        if let WindowEvent::Destroyed = event {
            main_window
                .set_enabled(true)
                .map_err(|e| e.to_string())
                .unwrap();
            log::info!("main window enabled");
            main_window.set_focus().map_err(|e| e.to_string()).unwrap();
            log::info!("main window focused");
        }
    });
    // 调试模式下自动打开 rename 窗口自己的开发者工具，方便排查该窗口的控制台日志
    // #[cfg(debug_assertions)]
    // window.open_devtools();

    log::info!("rename window created successfully");
    Ok(())
}

#[tauri::command]
pub async fn request_rename_old_file_path_command(
    state: State<'_, RenameWindowState>,
) -> Result<Option<String>, String> {
    log::info!("request_rename_old_file_path_command invoked");
    state
        .old_file_path
        .lock()
        .map(|old_file_path| old_file_path.clone())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn close_rename_window_command(app: AppHandle) -> Result<(), String> {
    log::info!("close_rename_window_command invoked");
    if let Some(window) = app.get_webview_window("rename") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}
