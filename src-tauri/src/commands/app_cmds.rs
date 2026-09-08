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
