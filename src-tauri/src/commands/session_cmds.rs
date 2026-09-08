use tauri::State;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::models::{Session, CreateSessionPayload, UpdateSessionPayload};
use crate::db::{session_repo, capture_repo, issue_repo};
use crate::services::file_storage;

#[tauri::command]
pub fn get_sessions(state: State<'_, AppState>) -> AppResult<Vec<Session>> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    session_repo::get_sessions(&conn)
}

#[tauri::command]
pub fn get_sessions_by_project(
    state: State<'_, AppState>,
    project_id: String,
) -> AppResult<Vec<Session>> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    session_repo::get_sessions_by_project(&conn, &project_id)
}

#[tauri::command]
pub fn get_session(state: State<'_, AppState>, id: String) -> AppResult<Session> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    session_repo::get_session(&conn, &id)
}

#[tauri::command]
pub fn create_session(
    state: State<'_, AppState>,
    payload: CreateSessionPayload,
) -> AppResult<Session> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    session_repo::create_session(&conn, payload)
}

#[tauri::command]
pub fn update_session(
    state: State<'_, AppState>,
    id: String,
    payload: UpdateSessionPayload,
) -> AppResult<Session> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    session_repo::update_session(&conn, &id, payload)
}

#[tauri::command]
pub fn delete_session(state: State<'_, AppState>, id: String) -> AppResult<()> {
    let mut conn = state
        .db
        .lock()
        .map_err(|e| AppError::Database(e.to_string()))?;

    // 1. Retrieve all captures and issues associated with this session before deleting
    let captures = capture_repo::get_captures_by_session(&conn, &id)?;
    let issues = issue_repo::get_by_session(&conn, &id)?;

    // 2. Perform soft delete in database via a transaction
    let tx = conn
        .transaction()
        .map_err(|e| AppError::Database(format!("Failed to start transaction: {}", e)))?;

    session_repo::delete_session(&tx, &id)?;

    tx.commit()
        .map_err(|e| AppError::Database(format!("Failed to commit delete session transaction: {}", e)))?;

    // 3. Delete files from disk
    for capture in captures {
        let _ = file_storage::delete_capture_files(&state.app_data_dir, &capture.screenshot_path);
    }

    for issue in issues {
        if let Some(ref crop_path) = issue.crop_path {
            let _ = file_storage::delete_file(&state.app_data_dir, crop_path);
        }
    }

    Ok(())
}

