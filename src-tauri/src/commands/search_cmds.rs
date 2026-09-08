use tauri::State;
use serde::Serialize;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::models::session::Session;
use crate::models::issue::Issue;
use crate::db::issue_repo::get_tags_for_issue;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub sessions: Vec<Session>,
    pub issues: Vec<Issue>,
}

#[tauri::command]
pub fn search_all(
    state: State<'_, AppState>,
    query: String,
) -> AppResult<SearchResult> {
    println!("[Search Rust] search_all called with query: {:?}", query);
    let conn = state.db.lock().map_err(|e| AppError::Database(e.to_string()))?;
    
    let lowercase_query = query.to_lowercase();
    let like_query = format!("%{}%", lowercase_query);
    
    // 1. Search Sessions
    let mut session_stmt = conn.prepare(
        "SELECT id, project_id, title, description, status, created_at, updated_at, completed_at,
                (SELECT COUNT(*) FROM issues WHERE session_id = sessions.id AND is_deleted = 0) as issue_count,
                (SELECT COUNT(*) FROM captures WHERE session_id = sessions.id AND is_deleted = 0) as capture_count
         FROM sessions
         WHERE is_deleted = 0 AND (LOWER(title) LIKE ?1 OR LOWER(description) LIKE ?1)
         ORDER BY created_at DESC"
    ).map_err(|e| AppError::Database(format!("Failed to prepare search sessions statement: {}", e)))?;

    let session_iter = session_stmt
        .query_map([&like_query], |row| {
            Ok(Session {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                status: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
                completed_at: row.get(7)?,
                issue_count: row.get(8)?,
                capture_count: row.get(9)?,
            })
        })
        .map_err(|e| AppError::Database(format!("Failed to execute search sessions query: {}", e)))?;

    let mut sessions = Vec::new();
    for item in session_iter {
        sessions.push(item.map_err(|e| AppError::Database(format!("Failed to map session row: {}", e)))?);
    }
    println!("[Search Rust] Found {} matching sessions", sessions.len());

    // 2. Search Issues
    let mut issue_stmt = conn.prepare(
        "SELECT id, capture_id, session_id, project_id, marker_number, title, description,
                issue_type, severity, status, marker_x, marker_y, annotation_data,
                color, stroke_width, crop_path, created_at, updated_at
         FROM issues
         WHERE is_deleted = 0 AND (LOWER(title) LIKE ?1 OR LOWER(description) LIKE ?1)
         ORDER BY created_at DESC"
    ).map_err(|e| AppError::Database(format!("Failed to prepare search issues statement: {}", e)))?;

    let issue_iter = issue_stmt
        .query_map([&like_query], |row| {
            Ok(Issue {
                id: row.get(0)?,
                capture_id: row.get(1)?,
                session_id: row.get(2)?,
                project_id: row.get(3)?,
                marker_number: row.get(4)?,
                title: row.get(5)?,
                description: row.get(6)?,
                issue_type: row.get(7)?,
                severity: row.get(8)?,
                status: row.get(9)?,
                marker_x: row.get(10)?,
                marker_y: row.get(11)?,
                annotation_data: row.get(12)?,
                color: row.get(13)?,
                stroke_width: row.get(14)?,
                crop_path: row.get(15)?,
                created_at: row.get(16)?,
                updated_at: row.get(17)?,
                tags: None,
            })
        })
        .map_err(|e| AppError::Database(format!("Failed to execute search issues query: {}", e)))?;

    let mut issues = Vec::new();
    for item in issue_iter {
        let mut issue = item.map_err(|e| AppError::Database(format!("Failed to map issue row: {}", e)))?;
        let tags = get_tags_for_issue(&conn, &issue.id)?;
        issue.tags = Some(tags);
        issues.push(issue);
    }
    println!("[Search Rust] Found {} matching issues", issues.len());

    Ok(SearchResult { sessions, issues })
}
