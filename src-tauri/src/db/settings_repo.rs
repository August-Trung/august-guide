use crate::error::{AppError, AppResult};
use rusqlite::{params, Connection, OptionalExtension};
use std::collections::HashMap;

/// Retrieve a single setting value by its key.
pub fn get_setting(conn: &Connection, key: &str) -> AppResult<Option<String>> {
    conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        params![key],
        |row| row.get(0),
    )
    .optional()
    .map_err(|e| AppError::Database(format!("Failed to query setting '{}': {}", key, e)))
}

/// Create or update a setting key-value pair.
pub fn set_setting(conn: &Connection, key: &str, value: &str) -> AppResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) 
         VALUES (?1, ?2, datetime('now'))",
        params![key, value],
    )
    .map(|_| ())
    .map_err(|e| AppError::Database(format!("Failed to insert or replace setting '{}': {}", key, e)))
}

/// Retrieve all settings as a key-value HashMap.
pub fn get_all_settings(conn: &Connection) -> AppResult<HashMap<String, String>> {
    let mut stmt = conn
        .prepare("SELECT key, value FROM settings")
        .map_err(|e| AppError::Database(format!("Failed to prepare select all settings statement: {}", e)))?;

    let rows = stmt
        .query_map([], |row| {
            let key: String = row.get(0)?;
            let value: String = row.get(1)?;
            Ok((key, value))
        })
        .map_err(|e| AppError::Database(format!("Failed to execute query all settings: {}", e)))?;

    let mut settings = HashMap::new();
    for row in rows {
        let (key, value) = row.map_err(|e| {
            AppError::Database(format!("Failed to parse setting row: {}", e))
        })?;
        settings.insert(key, value);
    }

    Ok(settings)
}
