use tauri::State;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::db::settings_repo;
use std::collections::HashMap;

#[tauri::command]
pub fn get_all_settings(
    state: State<'_, AppState>,
) -> AppResult<HashMap<String, String>> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    settings_repo::get_all_settings(&conn)
}

#[tauri::command]
pub fn get_setting(
    state: State<'_, AppState>,
    key: String,
) -> AppResult<Option<String>> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    settings_repo::get_setting(&conn, &key)
}

#[tauri::command]
pub fn update_setting(
    state: State<'_, AppState>,
    key: String,
    value: String,
) -> AppResult<()> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    settings_repo::set_setting(&conn, &key, &value)
}
