use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use rusqlite::Connection;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};
use crate::error::{AppError, AppResult};

/// Creates a backup ZIP containing the vacuumed database and screenshots/crops folders.
pub fn create_backup_zip(
    conn: &Connection,
    app_data_dir: &Path,
    dest_zip_path: &Path,
) -> AppResult<()> {
    // 1. Create a clean backup of the database using SQLite VACUUM INTO
    let temp_db_path = app_data_dir.join("august_mark_backup_temp.db");
    if temp_db_path.exists() {
        let _ = fs::remove_file(&temp_db_path);
    }
    
    // SQLite VACUUM INTO command
    conn.execute(
        &format!("VACUUM INTO {:?}", temp_db_path.to_string_lossy()),
        [],
    )
    .map_err(|e| AppError::Database(format!("Failed to vacuum SQLite database: {}", e)))?;

    // 2. Open the ZIP file for writing
    let zip_file = File::create(dest_zip_path)
        .map_err(|e| AppError::FileIO(format!("Failed to create backup ZIP file: {}", e)))?;
    let mut zip = ZipWriter::new(zip_file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    // 3. Add the database to the ZIP as "august_mark.db"
    zip.start_file("august_mark.db", options)
        .map_err(|e| AppError::FileIO(format!("Failed to add database file to ZIP: {}", e)))?;
    
    let mut temp_db_file = File::open(&temp_db_path)
        .map_err(|e| AppError::FileIO(format!("Failed to open temp database file: {}", e)))?;
    let mut buffer = Vec::new();
    temp_db_file.read_to_end(&mut buffer)
        .map_err(|e| AppError::FileIO(format!("Failed to read temp database file: {}", e)))?;
    zip.write_all(&buffer)
        .map_err(|e| AppError::FileIO(format!("Failed to write database to ZIP: {}", e)))?;

    // Drop temp db file connection to allow deletion
    drop(temp_db_file);
    let _ = fs::remove_file(&temp_db_path);

    // 4. Add screenshots and crops directories to the ZIP
    let folders = ["screenshots", "crops"];
    for folder in &folders {
        let folder_path = app_data_dir.join(folder);
        if folder_path.exists() && folder_path.is_dir() {
            add_directory_to_zip(&mut zip, &folder_path, app_data_dir, options)?;
        }
    }

    // Finish writing the ZIP
    zip.finish()
        .map_err(|e| AppError::FileIO(format!("Failed to finalize backup ZIP file: {}", e)))?;

    Ok(())
}

/// Helper to recursively add a directory's contents to a ZIP archive, retaining relative paths.
fn add_directory_to_zip<W: Write + io::Seek>(
    zip: &mut ZipWriter<W>,
    dir_path: &Path,
    base_dir: &Path,
    options: SimpleFileOptions,
) -> AppResult<()> {
    let walk_dir = |dir: &Path| -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        let mut dirs = vec![dir.to_path_buf()];
        while let Some(current_dir) = dirs.pop() {
            for entry in fs::read_dir(current_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                } else if path.is_file() {
                    files.push(path);
                }
            }
        }
        Ok(files)
    };

    let files = walk_dir(dir_path)
        .map_err(|e| AppError::FileIO(format!("Failed to read directory for zipping: {}", e)))?;

    for file_path in files {
        let rel_path = file_path.strip_prefix(base_dir)
            .map_err(|e| AppError::FileIO(format!("Failed to strip path prefix: {}", e)))?;
        
        let rel_path_str = rel_path.to_string_lossy().replace('\\', "/");
        
        zip.start_file(rel_path_str, options)
            .map_err(|e| AppError::FileIO(format!("Failed to start ZIP entry for file: {}", e)))?;

        let mut file = File::open(&file_path)
            .map_err(|e| AppError::FileIO(format!("Failed to open file for zipping: {}", e)))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| AppError::FileIO(format!("Failed to read file for zipping: {}", e)))?;
        zip.write_all(&buffer)
            .map_err(|e| AppError::FileIO(format!("Failed to write file to ZIP: {}", e)))?;
    }

    Ok(())
}

/// Extracts the backup ZIP. Verifies the database inside the ZIP, restores the media folders,
/// and returns the path to the extracted temporary database file.
pub fn extract_and_verify_backup(
    app_data_dir: &Path,
    zip_path: &Path,
) -> AppResult<PathBuf> {
    let file = File::open(zip_path)
        .map_err(|e| AppError::FileIO(format!("Failed to open backup ZIP file for restoring: {}", e)))?;
    let mut archive = ZipArchive::new(file)
        .map_err(|e| AppError::FileIO(format!("Failed to parse backup ZIP file: {}", e)))?;

    let temp_db_restore_path = app_data_dir.join("august_mark_restore_temp.db");
    if temp_db_restore_path.exists() {
        let _ = fs::remove_file(&temp_db_restore_path);
    }

    let mut db_found = false;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| AppError::FileIO(format!("Failed to read ZIP entry: {}", e)))?;
        
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        let outpath_str = outpath.to_string_lossy().replace('\\', "/");

        if outpath_str == "august_mark.db" {
            // Write to the temporary database file
            let mut outfile = File::create(&temp_db_restore_path)
                .map_err(|e| AppError::FileIO(format!("Failed to create temporary restore database file: {}", e)))?;
            io::copy(&mut file, &mut outfile)
                .map_err(|e| AppError::FileIO(format!("Failed to extract database from ZIP: {}", e)))?;
            db_found = true;
        } else if outpath_str.starts_with("screenshots/") || outpath_str.starts_with("crops/") {
            // Extract media files directly to the app data directory
            let dest_path = app_data_dir.join(&outpath);
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| AppError::FileIO(format!("Failed to create directory structure for media file: {}", e)))?;
            }
            let mut outfile = File::create(&dest_path)
                .map_err(|e| AppError::FileIO(format!("Failed to restore media file: {}", e)))?;
            io::copy(&mut file, &mut outfile)
                .map_err(|e| AppError::FileIO(format!("Failed to write media file from ZIP: {}", e)))?;
        }
    }

    if !db_found {
        return Err(AppError::GDrive("Backup ZIP file does not contain august_mark.db".into()));
    }

    // Verify database integrity by attempting to open it and run a quick query
    {
        let conn = Connection::open(&temp_db_restore_path)
            .map_err(|e| AppError::Database(format!("Failed to open extracted database connection: {}", e)))?;
        
        // Run a simple integrity check query
        let mut stmt = conn.prepare("PRAGMA integrity_check")
            .map_err(|e| AppError::Database(format!("Integrity check SQL preparation failed: {}", e)))?;
        let mut rows = stmt.query([])
            .map_err(|e| AppError::Database(format!("Integrity check execution failed: {}", e)))?;
        
        if let Some(row) = rows.next().map_err(|e| AppError::Database(format!("Failed to fetch integrity check row: {}", e)))? {
            let status: String = row.get(0).unwrap_or_default();
            if status != "ok" {
                return Err(AppError::GDrive(format!("Database integrity check failed: {}", status)));
            }
        } else {
            return Err(AppError::GDrive("Database integrity check returned no rows".into()));
        }
    }

    Ok(temp_db_restore_path)
}
