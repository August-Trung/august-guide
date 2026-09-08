use tauri::State;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use std::path::Path;

#[tauri::command]
pub fn export_session(
    state: State<'_, AppState>,
    session_id: String,
    output_path: String,
    format: String,
    severities: Vec<String>,
    statuses: Vec<String>,
) -> AppResult<()> {
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    let path = Path::new(&output_path);
    
    match format.to_lowercase().as_str() {
        "html" => {
            crate::services::export_service::export_session_html(
                &conn,
                &state.app_data_dir,
                &session_id,
                path,
                &severities,
                &statuses,
            )
        }
        "markdown" | "md" => {
            crate::services::export_service::export_session_markdown(
                &conn,
                &state.app_data_dir,
                &session_id,
                path,
                &severities,
                &statuses,
            )
        }
        "csv" => {
            crate::services::export_service::export_session_csv(
                &conn,
                &session_id,
                path,
                &severities,
                &statuses,
            )
        }
        "pdf" => {
            crate::services::export_service::export_session_pdf(
                &conn,
                &state.app_data_dir,
                &session_id,
                path,
                &severities,
                &statuses,
            )
        }
        _ => Err(AppError::FileIO(format!("Unsupported export format: {}", format))),
    }
}
