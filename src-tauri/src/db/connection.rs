use std::path::Path;
use rusqlite::Connection;
use crate::error::{AppError, AppResult};
use crate::utils::paths::ensure_dir;

/// Open a connection to the SQLite database and enable foreign keys.
pub fn open_connection<P: AsRef<Path>>(db_path: P) -> AppResult<Connection> {
    let conn = Connection::open(db_path.as_ref())
        .map_err(|e| AppError::Database(format!("Failed to open database connection: {}", e)))?;
        
    // Enable foreign key support in SQLite
    conn.execute("PRAGMA foreign_keys = ON;", [])
        .map_err(|e| AppError::Database(format!("Failed to enable foreign keys pragma: {}", e)))?;
        
    Ok(conn)
}

/// Ensure the base app data directory and asset subdirectories exist.
pub fn ensure_app_dirs<P: AsRef<Path>>(base_dir: P) -> AppResult<()> {
    let base = base_dir.as_ref();
    ensure_dir(base)?;
    ensure_dir(base.join("screenshots"))?;
    ensure_dir(base.join("crops"))?;
    ensure_dir(base.join("exports"))?;
    Ok(())
}
