use std::path::PathBuf;
use std::sync::Mutex;
use rusqlite::Connection;

/// Global application state shared across Tauri commands.
pub struct AppState {
    /// Mutex-guarded connection to the local SQLite database
    pub db: Mutex<Connection>,
    /// Base application directory path for screenshots, crops, and database storage
    pub app_data_dir: PathBuf,
    /// Mutex-guarded flag tracking if the screenshot markup overlay is currently open
    pub is_overlay_active: Mutex<bool>,
}

impl AppState {
    /// Create a new AppState instance
    pub fn new(db: Connection, app_data_dir: PathBuf) -> Self {
        Self {
            db: Mutex::new(db),
            app_data_dir,
            is_overlay_active: Mutex::new(false),
        }
    }
}
