use crate::error::{AppError, AppResult};
use image::DynamicImage;
use std::fs;
use std::path::Path;

/// Ensure directory structure exists under application data directory.
pub fn ensure_dirs(base_dir: &Path) -> AppResult<()> {
    fs::create_dir_all(base_dir.join("screenshots"))
        .map_err(|e| AppError::FileIO(format!("Failed to create screenshots dir: {}", e)))?;
    fs::create_dir_all(base_dir.join("crops"))
        .map_err(|e| AppError::FileIO(format!("Failed to create crops dir: {}", e)))?;
    fs::create_dir_all(base_dir.join("exports"))
        .map_err(|e| AppError::FileIO(format!("Failed to create exports dir: {}", e)))?;
    Ok(())
}

/// Save screenshot as PNG under screenshots/YYYY/MM/DD/{capture_id}.png.
pub fn save_screenshot(
    image: DynamicImage,
    base_dir: &Path,
    capture_id: &str,
    date: &str,
) -> AppResult<String> {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return Err(AppError::Validation(format!(
            "Invalid date format, expected YYYY-MM-DD: {}",
            date
        )));
    }
    let rel_path = format!(
        "screenshots/{}/{}/{}/{}.png",
        parts[0], parts[1], parts[2], capture_id
    );
    let abs_path = base_dir.join(&rel_path);

    if let Some(parent) = abs_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| AppError::FileIO(format!("Failed to create directory tree: {}", e)))?;
    }

    image
        .save(&abs_path)
        .map_err(|e| AppError::FileIO(format!("Failed to save screenshot image: {}", e)))?;

    Ok(rel_path)
}

/// Save crop as PNG under crops/YYYY/MM/DD/{issue_id}.png.
pub fn save_crop(
    image: DynamicImage,
    base_dir: &Path,
    issue_id: &str,
    date: &str,
) -> AppResult<String> {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return Err(AppError::Validation(format!(
            "Invalid date format, expected YYYY-MM-DD: {}",
            date
        )));
    }
    let rel_path = format!(
        "crops/{}/{}/{}/{}.png",
        parts[0], parts[1], parts[2], issue_id
    );
    let abs_path = base_dir.join(&rel_path);

    if let Some(parent) = abs_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| AppError::FileIO(format!("Failed to create directory tree: {}", e)))?;
    }

    image
        .save(&abs_path)
        .map_err(|e| AppError::FileIO(format!("Failed to save crop image: {}", e)))?;

    Ok(rel_path)
}

/// Delete a file inside base_dir if it exists.
pub fn delete_file(base_dir: &Path, relative_path: &str) -> AppResult<()> {
    if relative_path.is_empty() {
        return Ok(());
    }
    let abs_path = base_dir.join(relative_path);
    if abs_path.exists() {
        fs::remove_file(abs_path).map_err(|e| {
            AppError::FileIO(format!("Failed to delete file '{}': {}", relative_path, e))
        })?;
    }
    Ok(())
}

/// Delete screenshot and annotated screenshot files for a capture.
pub fn delete_capture_files(base_dir: &Path, screenshot_path: &str) -> AppResult<()> {
    if screenshot_path.is_empty() {
        return Ok(());
    }
    let rel_annotated_path = screenshot_path.replace(".png", "_annotated.png");

    delete_file(base_dir, screenshot_path)?;
    delete_file(base_dir, &rel_annotated_path)?;

    Ok(())
}
