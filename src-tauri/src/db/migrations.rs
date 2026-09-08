use rusqlite::{Connection, OptionalExtension};
use crate::error::{AppError, AppResult};

const MIGRATION_V001: &str = include_str!("../../migrations/v001_initial.sql");
const MIGRATION_V002: &str = include_str!("../../migrations/v002_guide_schema.sql");

/// Get the current migration version of the database.
/// Returns 0 if the database is uninitialized (i.e. the schema_version table doesn't exist).
pub fn get_current_version(conn: &Connection) -> AppResult<i32> {
    // Check if the schema_version table exists
    let table_exists: bool = conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name='schema_version'",
            [],
            |_| Ok(true),
        )
        .optional()
        .map_err(|e| AppError::Database(format!("Failed to query sqlite_master: {}", e)))?
        .unwrap_or(false);

    if !table_exists {
        return Ok(0);
    }

    // Retrieve the highest applied migration version
    let version: i32 = conn
        .query_row(
            "SELECT MAX(version) FROM schema_version",
            [],
            |row| row.get(0),
        )
        .map_err(|e| AppError::Database(format!("Failed to query schema_version: {}", e)))?;

    Ok(version)
}

/// Run pending database migrations inside an atomic transaction.
pub fn run_migrations(conn: &mut Connection) -> AppResult<()> {
    let mut current_version = get_current_version(conn)?;

    if current_version < 1 {
        log::info!("Database schema not initialized. Running migration v001...");

        let tx = conn
            .transaction()
            .map_err(|e| AppError::Database(format!("Failed to start migration transaction: {}", e)))?;

        tx.execute_batch(MIGRATION_V001)
            .map_err(|e| AppError::Database(format!("Failed to execute initial migration batch: {}", e)))?;

        tx.commit()
            .map_err(|e| AppError::Database(format!("Failed to commit migration transaction: {}", e)))?;

        log::info!("Migration v001 applied successfully!");
        current_version = 1;
    }

    if current_version < 2 {
        log::info!("Upgrading database schema to v002 (GuideStep schema)...");

        // Turn off foreign keys outside transaction for table restructuring
        conn.execute_batch("PRAGMA foreign_keys = OFF;")
            .map_err(|e| AppError::Database(format!("Failed to disable foreign keys for migration: {}", e)))?;

        let tx = conn
            .transaction()
            .map_err(|e| AppError::Database(format!("Failed to start migration v002 transaction: {}", e)))?;

        tx.execute_batch(MIGRATION_V002)
            .map_err(|e| AppError::Database(format!("Failed to execute v002 migration batch: {}", e)))?;

        tx.commit()
            .map_err(|e| AppError::Database(format!("Failed to commit migration v002 transaction: {}", e)))?;

        // Re-enable foreign keys
        conn.execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(|e| AppError::Database(format!("Failed to re-enable foreign keys after migration: {}", e)))?;

        log::info!("Migration v002 applied successfully!");
    } else {
        log::info!("Database schema is up-to-date (version: {}).", current_version);
    }

    Ok(())
}
