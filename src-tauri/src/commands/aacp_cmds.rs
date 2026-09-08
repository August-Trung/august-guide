use tauri::State;
use crate::error::{AppError, AppResult};
use crate::state::AppState;

#[tauri::command]
pub async fn export_aacp_pack(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    session_id: String,
    issue_id: String,
    workspace_path: String,
    suspected_files: Vec<String>,
    output_dir: String,
    compress_zip: bool,
) -> AppResult<String> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    
    let path = crate::services::aacp_exporter::export_aacp_pack(
        &conn,
        &state.app_data_dir,
        &session_id,
        &issue_id,
        &workspace_path,
        suspected_files,
        &output_dir,
        compress_zip,
    )?;

    // Open target folder on completion
    use tauri_plugin_shell::ShellExt;
    let _ = app_handle.shell().open(&output_dir, None);

    Ok(path)
}

#[tauri::command]
pub fn get_download_dir(app_handle: tauri::AppHandle) -> AppResult<String> {
    use tauri::Manager;
    let path = app_handle.path().download_dir()
        .map_err(|e| AppError::Generic(e.to_string()))?;
    Ok(path.to_string_lossy().to_string())
}


