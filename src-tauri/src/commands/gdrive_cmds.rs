use tauri::{State, AppHandle};
use std::fs;
use chrono::Local;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::services::gdrive_client;
use crate::services::backup_service;
use crate::db::settings_repo;

// Helper to get or refresh Google Drive access token without keeping Connection or MutexGuard across awaits
async fn get_valid_access_token(state: &AppState) -> AppResult<String> {
    // 1. Acquire the lock briefly to fetch the stored tokens
    let (access_token, refresh_token) = {
        let conn = state.db.lock().unwrap();
        let access = settings_repo::get_setting(&conn, "gdrive_access_token")?.unwrap_or_default();
        let refresh = settings_repo::get_setting(&conn, "gdrive_refresh_token")?.unwrap_or_default();
        (access, refresh)
    }; // Lock is dropped here

    if access_token.is_empty() {
        return Err(AppError::OAuth("Google Drive is not connected".into()));
    }

    // 2. Query user info asynchronously (no lock is held)
    match gdrive_client::get_user_info(&access_token).await {
        Ok(_) => Ok(access_token),
        Err(_) => {
            // Token is likely expired, attempt to refresh it
            if refresh_token.is_empty() {
                return Err(AppError::OAuth("Google Drive session expired, please reconnect".into()));
            }
            
            let token_resp = gdrive_client::refresh_access_token(&refresh_token).await?;
            
            // 3. Briefly acquire lock again to save the new token
            {
                let conn = state.db.lock().unwrap();
                settings_repo::set_setting(&conn, "gdrive_access_token", &token_resp.access_token)?;
                if let Some(ref new_refresh) = token_resp.refresh_token {
                    settings_repo::set_setting(&conn, "gdrive_refresh_token", new_refresh)?;
                }
            } // Lock is dropped here
            
            Ok(token_resp.access_token)
        }
    }
}

#[tauri::command]
pub async fn connect_gdrive(app_handle: AppHandle, state: State<'_, AppState>) -> AppResult<String> {
    // 1. Find port
    let port = gdrive_client::find_free_port()
        .ok_or_else(|| AppError::OAuth("No free local ports available between 45000 and 46000".into()))?;

    // 2. Redirect URI & Auth URL
    let redirect_uri = format!("http://127.0.0.1:{}", port);
    let auth_url = format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope=https://www.googleapis.com/auth/drive.file%20https://www.googleapis.com/auth/userinfo.email&state=auth&access_type=offline&prompt=consent",
        gdrive_client::AUTH_URL,
        gdrive_client::CLIENT_ID,
        redirect_uri
    );

    // 3. Open browser
    use tauri_plugin_shell::ShellExt;
    app_handle.shell().open(&auth_url, None)
        .map_err(|e| AppError::OAuth(format!("Failed to open system browser: {}", e)))?;

    // 4. Start local HTTP server to receive OAuth code (Run in spawned blocking task)
    let code = tokio::task::spawn_blocking(move || {
        gdrive_client::start_loopback_server(port)
    })
    .await
    .map_err(|e| AppError::OAuth(format!("Server thread failed: {}", e)))??;

    // 5. Exchange code for access & refresh tokens
    let token = gdrive_client::exchange_token(&code, &redirect_uri).await?;

    // 6. Fetch user email
    let user_info = gdrive_client::get_user_info(&token.access_token).await?;

    // 7. Save credentials to database
    let conn = state.db.lock().unwrap();
    settings_repo::set_setting(&conn, "gdrive_connected", "true")?;
    settings_repo::set_setting(&conn, "gdrive_access_token", &token.access_token)?;
    if let Some(ref refresh) = token.refresh_token {
        settings_repo::set_setting(&conn, "gdrive_refresh_token", refresh)?;
    }
    settings_repo::set_setting(&conn, "gdrive_user_email", &user_info.email)?;

    Ok(user_info.email)
}

#[tauri::command]
pub async fn disconnect_gdrive(state: State<'_, AppState>) -> AppResult<()> {
    let conn = state.db.lock().unwrap();
    settings_repo::set_setting(&conn, "gdrive_connected", "false")?;
    settings_repo::set_setting(&conn, "gdrive_access_token", "")?;
    settings_repo::set_setting(&conn, "gdrive_refresh_token", "")?;
    settings_repo::set_setting(&conn, "gdrive_user_email", "")?;
    Ok(())
}

#[tauri::command]
pub async fn check_gdrive_status(state: State<'_, AppState>) -> AppResult<Option<String>> {
    let conn = state.db.lock().unwrap();
    let connected = settings_repo::get_setting(&conn, "gdrive_connected")?
        .unwrap_or_else(|| "false".to_string());
    
    if connected == "true" {
        let email = settings_repo::get_setting(&conn, "gdrive_user_email")?
            .unwrap_or_default();
        if !email.is_empty() {
            return Ok(Some(email));
        }
    }
    
    Ok(None)
}

#[tauri::command]
pub async fn backup_to_gdrive(state: State<'_, AppState>) -> AppResult<String> {
    // 1. Get valid access token
    let access_token = get_valid_access_token(&state).await?;

    // 2. Generate local ZIP backup path
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("august_backup_{}.zip", timestamp);
    let dest_zip_path = state.app_data_dir.join(&filename);

    // 3. Compress database and media folders to ZIP
    {
        let conn = state.db.lock().unwrap();
        backup_service::create_backup_zip(&conn, &state.app_data_dir, &dest_zip_path)?;
    } // Lock is dropped here

    // 4. Find or create folder in Google Drive (async, no lock held)
    let folder_id = gdrive_client::create_folder_if_not_exists(&access_token, "August Mark Backups").await?;

    // 5. Read ZIP file data
    let file_data = fs::read(&dest_zip_path)
        .map_err(|e| AppError::FileIO(format!("Failed to read backup ZIP file: {}", e)))?;

    // 6. Upload file to Google Drive (async, no lock held)
    let _file_id = gdrive_client::upload_file(
        &access_token,
        &filename,
        "application/zip",
        file_data,
        Some(&folder_id),
    )
    .await?;

    // 7. Clean up local ZIP file
    let _ = fs::remove_file(&dest_zip_path);

    // 8. Update last backup time in settings
    let backup_time_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    {
        let conn = state.db.lock().unwrap();
        settings_repo::set_setting(&conn, "gdrive_last_backup_time", &backup_time_str)?;
    } // Lock is dropped here

    Ok(backup_time_str)
}

#[tauri::command]
pub async fn restore_from_gdrive(state: State<'_, AppState>, file_id: String) -> AppResult<()> {
    // 1. Get valid access token
    let access_token = get_valid_access_token(&state).await?;

    // 2. Download backup ZIP file from Google Drive (async, no lock held)
    let file_data = gdrive_client::download_file(&access_token, &file_id).await?;
    let zip_path = state.app_data_dir.join("august_restore_temp.zip");
    fs::write(&zip_path, &file_data)
        .map_err(|e| AppError::FileIO(format!("Failed to save downloaded backup ZIP: {}", e)))?;

    // 3. Extract ZIP and verify database integrity (blocking, no lock held)
    let temp_db_restore_path = backup_service::extract_and_verify_backup(&state.app_data_dir, &zip_path)?;

    // 4. Close database connection, swap files, and reopen connection (blocking lock block)
    {
        let mut conn = state.db.lock().unwrap();

        // Swap a temporary in-memory connection to close the active august_mark.db connection
        let temp_conn = rusqlite::Connection::open_in_memory()
            .map_err(|e| AppError::Database(format!("Failed to open temp in-memory connection: {}", e)))?;
        let old_conn = std::mem::replace(&mut *conn, temp_conn);
        drop(old_conn); // This closes the connection to august_mark.db!

        let db_path = state.app_data_dir.join("august_mark.db");
        let backup_db_path = state.app_data_dir.join("august_mark.db.bak");

        // Copy current database as backup
        if db_path.exists() {
            let _ = fs::copy(&db_path, &backup_db_path);
        }

        // Copy restored database file over the active database file
        if let Err(e) = fs::copy(&temp_db_restore_path, &db_path) {
            // Revert from backup if copy failed
            if backup_db_path.exists() {
                let _ = fs::copy(&backup_db_path, &db_path);
            }
            let _ = fs::remove_file(&temp_db_restore_path);
            let _ = fs::remove_file(&zip_path);
            return Err(AppError::FileIO(format!("Failed to overwrite active database file during restore: {}", e)));
        }

        // Reopen database connection
        let new_conn = match crate::db::open_connection(&db_path) {
            Ok(c) => c,
            Err(e) => {
                // Revert to backup and try reopening
                if backup_db_path.exists() {
                    let _ = fs::copy(&backup_db_path, &db_path);
                }
                let fallback_conn = crate::db::open_connection(&db_path)
                    .map_err(|err| AppError::Database(format!("Database restore failed and revert failed: {}", err)))?;
                *conn = fallback_conn;
                let _ = fs::remove_file(&temp_db_restore_path);
                let _ = fs::remove_file(&zip_path);
                return Err(AppError::Database(format!("Failed to reopen restored database connection: {}", e)));
            }
        };

        *conn = new_conn;
    } // Lock is dropped here

    // 5. Clean up temporary files
    let _ = fs::remove_file(&temp_db_restore_path);
    let _ = fs::remove_file(&zip_path);

    Ok(())
}

#[tauri::command]
pub async fn list_backups_on_gdrive(state: State<'_, AppState>) -> AppResult<Vec<(String, String)>> {
    // 1. Get valid access token
    let access_token = get_valid_access_token(&state).await?;

    // 2. Find or create folder in Google Drive (async, no lock held)
    let folder_id = gdrive_client::create_folder_if_not_exists(&access_token, "August Mark Backups").await?;

    // 3. List backups (async, no lock held)
    let backups = gdrive_client::list_backups(&access_token, &folder_id).await?;

    Ok(backups)
}

#[tauri::command]
pub async fn share_session_on_gdrive(state: State<'_, AppState>, session_id: String) -> AppResult<String> {
    // 1. Get valid access token
    let access_token = get_valid_access_token(&state).await?;

    // 2. Find or create folder "August Mark Shared Reports" on Google Drive
    let folder_id = gdrive_client::create_folder_if_not_exists(&access_token, "August Mark Shared Reports").await?;

    // 3. Generate PDF report file locally
    let temp_pdf_path = state.app_data_dir.join(format!("august_report_session_{}_temp.pdf", session_id));
    {
        let conn = state.db.lock().unwrap();
        crate::services::export_service::export_session_pdf(
            &conn,
            &state.app_data_dir,
            &session_id,
            &temp_pdf_path,
            &[],
            &[],
        )?;
    } // Lock is dropped here

    // 4. Read PDF file data
    let file_data = fs::read(&temp_pdf_path)
        .map_err(|e| AppError::FileIO(format!("Failed to read temporary export PDF: {}", e)))?;

    // 5. Upload PDF to Google Drive (async, no lock held)
    let filename = format!("august_report_session_{}.pdf", session_id);
    let file_id = gdrive_client::upload_file(
        &access_token,
        &filename,
        "application/pdf",
        file_data,
        Some(&folder_id),
    )
    .await?;

    // 6. Clean up local temp PDF file
    let _ = fs::remove_file(&temp_pdf_path);

    // 7. Share file publicly (async, no lock held)
    gdrive_client::share_file_publicly(&access_token, &file_id).await?;

    // 8. Get WebView Link (async, no lock held)
    let share_link = gdrive_client::get_web_view_link(&access_token, &file_id).await?;

    Ok(share_link)
}
