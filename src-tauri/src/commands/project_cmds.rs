use tauri::State;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::models::{Project, CreateProjectPayload, UpdateProjectPayload};
use crate::db::project_repo;

#[tauri::command]
pub fn get_projects(
    state: State<'_, AppState>,
    include_archived: Option<bool>,
) -> AppResult<Vec<Project>> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    project_repo::get_projects(&conn, include_archived.unwrap_or(false))
}

#[tauri::command]
pub fn get_project(
    state: State<'_, AppState>,
    id: String,
) -> AppResult<Project> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    project_repo::get_project(&conn, &id)
}

#[tauri::command]
pub fn create_project(
    state: State<'_, AppState>,
    payload: CreateProjectPayload,
) -> AppResult<Project> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    project_repo::create_project(&conn, payload)
}

#[tauri::command]
pub fn update_project(
    state: State<'_, AppState>,
    id: String,
    payload: UpdateProjectPayload,
) -> AppResult<Project> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    project_repo::update_project(&conn, &id, payload)
}

#[tauri::command]
pub fn delete_project(
    state: State<'_, AppState>,
    id: String,
) -> AppResult<()> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    project_repo::delete_project(&conn, &id)
}
