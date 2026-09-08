use std::path::{Path, PathBuf};
use chrono::NaiveDate;
use crate::error::{AppError, AppResult};

/// Return the directory path for screenshots: {base}/screenshots/YYYY/MM/DD
pub fn screenshots_dir<P: AsRef<Path>>(base: P, date: NaiveDate) -> PathBuf {
    base.as_ref()
        .join("screenshots")
        .join(date.format("%Y").to_string())
        .join(date.format("%m").to_string())
        .join(date.format("%d").to_string())
}

/// Return the directory path for cropped issue images: {base}/crops/YYYY/MM/DD
pub fn crops_dir<P: AsRef<Path>>(base: P, date: NaiveDate) -> PathBuf {
    base.as_ref()
        .join("crops")
        .join(date.format("%Y").to_string())
        .join(date.format("%m").to_string())
        .join(date.format("%d").to_string())
}

/// Return the directory path for exports: {base}/exports
pub fn exports_dir<P: AsRef<Path>>(base: P) -> PathBuf {
    base.as_ref().join("exports")
}

/// Ensure a directory exists by creating it if necessary
pub fn ensure_dir<P: AsRef<Path>>(path: P) -> AppResult<()> {
    std::fs::create_dir_all(path.as_ref())
        .map_err(|e| AppError::FileIO(format!("Failed to create directory: {}", e)))
}
