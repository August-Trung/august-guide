use tauri::State;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStats {
    pub project_count: i64,
    pub session_count: i64,
    pub issue_count: i64,
    pub db_size: u64,
    pub db_location: String,
}

#[tauri::command]
pub fn get_app_stats(
    state: State<'_, AppState>,
) -> AppResult<AppStats> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    
    let project_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM projects WHERE is_deleted = 0",
        [],
        |row| row.get(0),
    )
    .map_err(|e| AppError::Database(format!("Failed to count projects: {}", e)))?;

    let session_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions WHERE is_deleted = 0",
        [],
        |row| row.get(0),
    )
    .map_err(|e| AppError::Database(format!("Failed to count sessions: {}", e)))?;

    let issue_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM issues WHERE is_deleted = 0",
        [],
        |row| row.get(0),
    )
    .map_err(|e| AppError::Database(format!("Failed to count issues: {}", e)))?;

    let db_path = state.app_data_dir.join("august_guide.db");
    let db_size = fs::metadata(&db_path)
        .map(|meta| meta.len())
        .unwrap_or(0);
        
    let db_location = state.app_data_dir.to_string_lossy().to_string();

    Ok(AppStats {
        project_count,
        session_count,
        issue_count,
        db_size,
        db_location,
    })
}

/// Triggers Windows uninstaller for August Guide or opens Windows App Settings.
#[tauri::command]
pub fn uninstall_app(app: tauri::AppHandle) -> AppResult<()> {
    #[cfg(target_os = "windows")]
    {
        // 1. Look for uninstaller executable in the current application directory
        if let Ok(current_exe) = std::env::current_exe() {
            if let Some(exe_dir) = current_exe.parent() {
                let candidates = [
                    exe_dir.join("Uninstall August Guide.exe"),
                    exe_dir.join("uninstall.exe"),
                    exe_dir.join("uninst.exe"),
                ];

                for candidate in candidates {
                    if candidate.exists() {
                        let _ = std::process::Command::new(&candidate).spawn();
                        app.exit(0);
                        return Ok(());
                    }
                }
            }
        }

        // 2. Fallback: open Windows Installed Apps / Features settings
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", "ms-settings:appsfeatures"])
            .spawn();
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = app;
    }

    Ok(())
}

/// Mở thư mục lưu trữ của ứng dụng (screenshots, crops, exports hoặc gốc) trong File Explorer.
#[tauri::command]
pub fn open_app_folder(state: State<'_, AppState>, folder_type: Option<String>) -> AppResult<()> {
    let target_path = match folder_type.as_deref() {
        Some("crops") => state.app_data_dir.join("crops"),
        Some("exports") => state.app_data_dir.join("exports"),
        Some("root") => state.app_data_dir.clone(),
        _ => state.app_data_dir.join("screenshots"),
    };

    if !target_path.exists() {
        let _ = std::fs::create_dir_all(&target_path);
    }

    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("explorer")
            .arg(target_path.as_os_str())
            .spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open")
            .arg(target_path.as_os_str())
            .spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open")
            .arg(target_path.as_os_str())
            .spawn();
    }

    Ok(())
}

