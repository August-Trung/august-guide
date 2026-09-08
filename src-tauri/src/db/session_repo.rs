use crate::error::{AppError, AppResult};
use crate::models::{CreateSessionPayload, Session, UpdateSessionPayload};
use crate::utils::id::new_uuid;
use chrono::Utc;
use rusqlite::{params, Connection};

/// Query a single session by its ID.
pub fn get_session(conn: &Connection, id: &str) -> AppResult<Session> {
    conn.query_row(
        "SELECT id, project_id, title, description, status, created_at, updated_at, completed_at,
                (SELECT COUNT(*) FROM issues WHERE session_id = sessions.id AND is_deleted = 0) as issue_count,
                (SELECT COUNT(*) FROM captures WHERE session_id = sessions.id AND is_deleted = 0) as capture_count
         FROM sessions WHERE id = ?1 AND is_deleted = 0",
        params![id],
        |row| {
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
        },
    ).map_err(|e| {
        match e {
            rusqlite::Error::QueryReturnedNoRows => {
                AppError::Validation(format!("Session with ID '{}' not found", id))
            }
            _ => AppError::Database(format!("Failed to query session by ID: {}", e)),
        }
    })
}

/// Query all sessions in the database.
pub fn get_sessions(conn: &Connection) -> AppResult<Vec<Session>> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, title, description, status, created_at, updated_at, completed_at,
                (SELECT COUNT(*) FROM issues WHERE session_id = sessions.id AND is_deleted = 0) as issue_count,
                (SELECT COUNT(*) FROM captures WHERE session_id = sessions.id AND is_deleted = 0) as capture_count
         FROM sessions
         WHERE is_deleted = 0
         ORDER BY created_at DESC"
    ).map_err(|e| AppError::Database(format!("Failed to prepare get_sessions statement: {}", e)))?;

    let session_iter = stmt
        .query_map([], |row| {
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
        .map_err(|e| AppError::Database(format!("Failed to query sessions list: {}", e)))?;

    let mut sessions = Vec::new();
    for s in session_iter {
        sessions
            .push(s.map_err(|e| AppError::Database(format!("Failed to map session row: {}", e)))?);
    }
    Ok(sessions)
}

/// Query all sessions associated with a specific project ID.
pub fn get_sessions_by_project(conn: &Connection, project_id: &str) -> AppResult<Vec<Session>> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, title, description, status, created_at, updated_at, completed_at,
                (SELECT COUNT(*) FROM issues WHERE session_id = sessions.id AND is_deleted = 0) as issue_count,
                (SELECT COUNT(*) FROM captures WHERE session_id = sessions.id AND is_deleted = 0) as capture_count
         FROM sessions
         WHERE project_id = ?1 AND is_deleted = 0
         ORDER BY created_at DESC"
    ).map_err(|e| AppError::Database(format!("Failed to prepare get_sessions_by_project statement: {}", e)))?;

    let session_iter = stmt
        .query_map(params![project_id], |row| {
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
        .map_err(|e| AppError::Database(format!("Failed to query sessions by project: {}", e)))?;

    let mut sessions = Vec::new();
    for s in session_iter {
        sessions
            .push(s.map_err(|e| AppError::Database(format!("Failed to map session row: {}", e)))?);
    }
    Ok(sessions)
}

/// Create a new session.
pub fn create_session(conn: &Connection, payload: CreateSessionPayload) -> AppResult<Session> {
    // Validate project existence
    let _proj = crate::db::project_repo::get_project(conn, &payload.project_id)?;

    let id = new_uuid();
    let now = Utc::now().to_rfc3339();
    let description = payload.description.unwrap_or_default();

    conn.execute(
        "INSERT INTO sessions (id, project_id, title, description, status, created_at, updated_at, completed_at)
         VALUES (?1, ?2, ?3, ?4, 'active', ?5, ?5, NULL)",
        params![id, payload.project_id, payload.title, description, now],
    ).map_err(|e| AppError::Database(format!("Failed to insert session: {}", e)))?;

    get_session(conn, &id)
}

/// Update an existing session.
pub fn update_session(
    conn: &Connection,
    id: &str,
    payload: UpdateSessionPayload,
) -> AppResult<Session> {
    let existing = get_session(conn, id)?;
    let now = Utc::now().to_rfc3339();

    let mut query = String::from("UPDATE sessions SET ");
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    let mut sets = Vec::new();
    let mut param_index = 1;

    if let Some(ref title) = payload.title {
        sets.push(format!("title = ?{}", param_index));
        params_vec.push(Box::new(title.clone()));
        param_index += 1;
    }

    if let Some(ref desc) = payload.description {
        sets.push(format!("description = ?{}", param_index));
        params_vec.push(Box::new(desc.clone()));
        param_index += 1;
    }

    let mut completing = false;
    let mut reopening = false;
    if let Some(ref status) = payload.status {
        sets.push(format!("status = ?{}", param_index));
        params_vec.push(Box::new(status.clone()));
        param_index += 1;

        if status == "completed" && existing.status != "completed" {
            completing = true;
        } else if status == "active" && existing.status == "completed" {
            reopening = true;
        }
    }

    if completing {
        sets.push(format!("completed_at = ?{}", param_index));
        params_vec.push(Box::new(now.clone()));
        param_index += 1;
    } else if reopening {
        sets.push(format!("completed_at = NULL"));
    }

    sets.push(format!("updated_at = ?{}", param_index));
    params_vec.push(Box::new(now));
    param_index += 1;

    query.push_str(&sets.join(", "));
    query.push_str(&format!(" WHERE id = ?{}", param_index));
    params_vec.push(Box::new(id.to_string()));

    let refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|b| b.as_ref()).collect();

    conn.execute(&query, refs.as_slice()).map_err(|e| {
        AppError::Database(format!("Failed to execute update session query: {}", e))
    })?;

    get_session(conn, id)
}

/// Delete a session. Downstream records will be cascade deleted.
pub fn delete_session(conn: &Connection, id: &str) -> AppResult<()> {
    conn.execute(
        "UPDATE sessions SET is_deleted = 1 WHERE id = ?1",
        params![id],
    )
    .map_err(|e| AppError::Database(format!("Failed to soft delete session: {}", e)))?;
    Ok(())
}
