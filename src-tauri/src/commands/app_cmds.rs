use tauri::{Manager, State};
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStats {
    pub project_count: i64,
    pub session_count: i64,
    pub issue_count: i64,
    pub db_size: u64,
    pub db_location: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageInfo {
    pub current_path: String,
    pub default_path: String,
    pub is_custom: bool,
    pub total_size_bytes: u64,
    pub db_size_bytes: u64,
    pub captures_size_bytes: u64,
    pub screenshot_count: usize,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageConfig {
    pub custom_data_dir: Option<String>,
}

fn get_dir_size_and_count(path: &Path) -> (u64, usize) {
    let mut total_size = 0;
    let mut file_count = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    let (sub_size, sub_count) = get_dir_size_and_count(&entry.path());
                    total_size += sub_size;
                    file_count += sub_count;
                } else {
                    total_size += meta.len();
                    file_count += 1;
                }
            }
        }
    }
    (total_size, file_count)
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst_path)?;
        } else {
            fs::copy(entry.path(), &dst_path)?;
        }
    }
    Ok(())
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

#[tauri::command]
pub fn get_storage_info(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> AppResult<StorageInfo> {
    let default_path = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Generic(e.to_string()))?;

    let current_path = state.app_data_dir.clone();
    let is_custom = current_path != default_path;

    let db_path = current_path.join("august_guide.db");
    let db_size_bytes = fs::metadata(&db_path).map(|m| m.len()).unwrap_or(0);

    let (total_size_bytes, screenshot_count) = get_dir_size_and_count(&current_path);
    let captures_size_bytes = total_size_bytes.saturating_sub(db_size_bytes);

    Ok(StorageInfo {
        current_path: current_path.to_string_lossy().to_string(),
        default_path: default_path.to_string_lossy().to_string(),
        is_custom,
        total_size_bytes,
        db_size_bytes,
        captures_size_bytes,
        screenshot_count,
    })
}

#[tauri::command]
pub fn migrate_storage_location(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    target_path: String,
    copy_data: bool,
) -> AppResult<()> {
    let clean_path = target_path.trim();
    if clean_path.is_empty() {
        return Err(AppError::Validation("Đường dẫn thư mục không được để trống.".to_string()));
    }

    let target_dir = PathBuf::from(clean_path);
    crate::db::ensure_app_dirs(&target_dir)?;

    let current_dir = &state.app_data_dir;

    if copy_data && current_dir != &target_dir {
        // Copy SQLite database files
        let db_names = ["august_guide.db", "august_guide.db-wal", "august_guide.db-shm"];
        for name in &db_names {
            let src_file = current_dir.join(name);
            if src_file.exists() {
                let dst_file = target_dir.join(name);
                let _ = fs::copy(&src_file, &dst_file);
            }
        }

        // Copy asset folders
        let subfolders = ["screenshots", "crops", "exports"];
        for sub in &subfolders {
            let src_sub = current_dir.join(sub);
            if src_sub.exists() {
                let dst_sub = target_dir.join(sub);
                let _ = copy_dir_all(&src_sub, &dst_sub);
            }
        }
    }

    // Save bootstrap config to default app data directory
    let default_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Generic(e.to_string()))?;
    fs::create_dir_all(&default_dir)
        .map_err(|e| AppError::FileIO(format!("Failed to create default app dir: {}", e)))?;

    let config = StorageConfig {
        custom_data_dir: Some(clean_path.to_string()),
    };
    let config_json = serde_json::to_string_pretty(&config)
        .map_err(|e| AppError::Generic(e.to_string()))?;
    let config_path = default_dir.join("storage_config.json");
    fs::write(&config_path, config_json)
        .map_err(|e| AppError::FileIO(format!("Failed to write storage_config.json: {}", e)))?;

    Ok(())
}

#[tauri::command]
pub fn reset_storage_location(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    copy_data: bool,
) -> AppResult<()> {
    let default_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Generic(e.to_string()))?;
    let current_dir = &state.app_data_dir;

    if copy_data && current_dir != &default_dir {
        crate::db::ensure_app_dirs(&default_dir)?;
        let db_names = ["august_guide.db", "august_guide.db-wal", "august_guide.db-shm"];
        for name in &db_names {
            let src_file = current_dir.join(name);
            if src_file.exists() {
                let dst_file = default_dir.join(name);
                let _ = fs::copy(&src_file, &dst_file);
            }
        }

        let subfolders = ["screenshots", "crops", "exports"];
        for sub in &subfolders {
            let src_sub = current_dir.join(sub);
            if src_sub.exists() {
                let dst_sub = default_dir.join(sub);
                let _ = copy_dir_all(&src_sub, &dst_sub);
            }
        }
    }

    let config_path = default_dir.join("storage_config.json");
    if config_path.exists() {
        let _ = fs::remove_file(config_path);
    }

    Ok(())
}

#[tauri::command]
pub fn restart_app(app: tauri::AppHandle) -> AppResult<()> {
    app.restart();
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

/// Mở thư mục lưu trữ của ứng dụng trong File Explorer.
#[tauri::command]
pub fn open_app_folder(
    state: State<'_, AppState>,
    folder_type: Option<String>,
    custom_path: Option<String>,
) -> AppResult<()> {
    let target_path = if let Some(custom) = custom_path {
        PathBuf::from(custom)
    } else {
        match folder_type.as_deref() {
            Some("crops") => state.app_data_dir.join("crops"),
            Some("exports") => state.app_data_dir.join("exports"),
            Some("root") => state.app_data_dir.clone(),
            _ => state.app_data_dir.join("screenshots"),
        }
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

