use crate::error::{AppError, AppResult};
use crate::models::{Capture, MonitorInfo};
use crate::utils::id::new_uuid;
use chrono::Utc;
use rusqlite::{params, Connection};

/// Query a single capture by its ID.
pub fn get_capture_by_id(conn: &Connection, id: &str) -> AppResult<Capture> {
    conn.query_row(
        "SELECT id, session_id, screenshot_path, monitor_name, monitor_x, monitor_y, monitor_width, monitor_height, scale_factor, window_title, created_at
         FROM captures WHERE id = ?1 AND is_deleted = 0",
        params![id],
        |row| {
            Ok(Capture {
                id: row.get(0)?,
                session_id: row.get(1)?,
                screenshot_path: row.get(2)?,
                monitor_name: row.get(3)?,
                monitor_x: row.get(4)?,
                monitor_y: row.get(5)?,
                monitor_width: row.get(6)?,
                monitor_height: row.get(7)?,
                scale_factor: row.get(8)?,
                window_title: row.get(9)?,
                created_at: row.get(10)?,
            })
        },
    ).map_err(|e| {
        match e {
            rusqlite::Error::QueryReturnedNoRows => {
                AppError::Validation(format!("Capture with ID '{}' not found", id))
            }
            _ => AppError::Database(format!("Failed to query capture by ID: {}", e)),
        }
    })
}

/// Query all captures associated with a specific session ID.
pub fn get_captures_by_session(conn: &Connection, session_id: &str) -> AppResult<Vec<Capture>> {
    let mut stmt = conn.prepare(
        "SELECT id, session_id, screenshot_path, monitor_name, monitor_x, monitor_y, monitor_width, monitor_height, scale_factor, window_title, created_at
         FROM captures
         WHERE session_id = ?1 AND is_deleted = 0
         ORDER BY created_at DESC"
    ).map_err(|e| AppError::Database(format!("Failed to prepare get_captures_by_session statement: {}", e)))?;

    let iter = stmt
        .query_map(params![session_id], |row| {
            Ok(Capture {
                id: row.get(0)?,
                session_id: row.get(1)?,
                screenshot_path: row.get(2)?,
                monitor_name: row.get(3)?,
                monitor_x: row.get(4)?,
                monitor_y: row.get(5)?,
                monitor_width: row.get(6)?,
                monitor_height: row.get(7)?,
                scale_factor: row.get(8)?,
                window_title: row.get(9)?,
                created_at: row.get(10)?,
            })
        })
        .map_err(|e| AppError::Database(format!("Failed to query captures: {}", e)))?;

    let mut captures = Vec::new();
    for c in iter {
        captures.push(c.map_err(|e| AppError::Database(format!("Failed to map capture: {}", e)))?);
    }
    Ok(captures)
}

/// Create a new capture metadata record.
pub fn create_capture(
    conn: &Connection,
    session_id: &str,
    screenshot_path: &str,
    monitor_info: &MonitorInfo,
    window_title: Option<String>,
) -> AppResult<Capture> {
    let id = new_uuid();
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO captures (id, session_id, screenshot_path, monitor_name, monitor_x, monitor_y, monitor_width, monitor_height, scale_factor, window_title, is_deleted, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 0, ?11)",
        params![
            id,
            session_id,
            screenshot_path,
            monitor_info.name,
            monitor_info.x,
            monitor_info.y,
            monitor_info.width,
            monitor_info.height,
            monitor_info.scale_factor,
            window_title,
            now,
        ],
    ).map_err(|e| AppError::Database(format!("Failed to insert capture record: {}", e)))?;

    get_capture_by_id(conn, &id)
}

/// Delete a capture metadata record (soft delete).
pub fn delete_capture(conn: &Connection, id: &str) -> AppResult<()> {
    conn.execute(
        "UPDATE captures SET is_deleted = 1 WHERE id = ?1",
        params![id],
    )
    .map_err(|e| AppError::Database(format!("Failed to soft delete capture: {}", e)))?;
    Ok(())
}

/// Cleanup captures that have is_deleted = 0, no associated issues, and no annotated screenshot file on disk.
pub fn cleanup_uncommitted_captures(conn: &Connection, app_data_dir: &std::path::Path) -> AppResult<()> {
    // 1. Query all captures where is_deleted = 0
    let mut stmt = conn.prepare(
        "SELECT id, screenshot_path FROM captures WHERE is_deleted = 0"
    ).map_err(|e| AppError::Database(format!("Failed to prepare captures cleanup statement: {}", e)))?;

    let iter = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    }).map_err(|e| AppError::Database(format!("Failed to query captures for cleanup: {}", e)))?;

    let mut to_delete = Vec::new();
    for item in iter {
        if let Ok((id, screenshot_path)) = item {
            // Check count of active issues associated with this capture
            let mut count_stmt = conn.prepare(
                "SELECT COUNT(*) FROM issues WHERE capture_id = ?1 AND is_deleted = 0"
            ).map_err(|e| AppError::Database(e.to_string()))?;
            let count: i64 = count_stmt.query_row(params![id], |r| r.get(0)).unwrap_or(0);

            if count == 0 {
                // If there are no issues, verify if an annotated screenshot file exists on disk
                let annotated_path = screenshot_path.replace(".png", "_annotated.png");
                let abs_annotated_path = app_data_dir.join(&annotated_path);
                if !abs_annotated_path.exists() {
                    // It was never saved/committed with "Done"
                    to_delete.push((id, screenshot_path));
                }
            }
        }
    }

    // 2. Perform deletion of files and database records
    for (id, screenshot_path) in to_delete {
        println!("[Cleanup] Deleting uncommitted capture: {} ({})", id, screenshot_path);
        let abs_path = app_data_dir.join(&screenshot_path);
        if abs_path.exists() {
            let _ = std::fs::remove_file(abs_path);
        }
        
        // Also clean up database entry (hard delete since it's a temp/orphaned capture)
        let _ = conn.execute("DELETE FROM captures WHERE id = ?1", params![id]);
    }

    Ok(())
}
