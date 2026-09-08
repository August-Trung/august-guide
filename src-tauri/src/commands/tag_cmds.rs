use tauri::State;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::models::issue::Tag;
use crate::db::tag_repo;

#[tauri::command]
pub fn get_all_tags(
    state: State<'_, AppState>,
) -> AppResult<Vec<Tag>> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    tag_repo::get_all_tags(&conn)
}

#[tauri::command]
pub fn create_tag(
    state: State<'_, AppState>,
    name: String,
    color: String,
) -> AppResult<Tag> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    tag_repo::create_tag(&conn, &name, &color)
}

#[tauri::command]
pub fn associate_tag_with_issue(
    state: State<'_, AppState>,
    issue_id: String,
    tag_id: String,
) -> AppResult<()> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    tag_repo::associate_tag_with_issue(&conn, &issue_id, &tag_id)
}

#[tauri::command]
pub fn get_tags_by_issue(
    state: State<'_, AppState>,
    issue_id: String,
) -> AppResult<Vec<Tag>> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    tag_repo::get_tags_by_issue(&conn, &issue_id)
}

#[tauri::command]
pub fn clear_issue_tags(
    state: State<'_, AppState>,
    issue_id: String,
) -> AppResult<()> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    tag_repo::clear_issue_tags(&conn, &issue_id)
}
