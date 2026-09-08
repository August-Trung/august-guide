use crate::db::{capture_repo, issue_repo};
use crate::error::{AppError, AppResult};
use crate::models::{CreateIssuePayload, Issue, UpdateIssuePayload};
use crate::services::{file_storage, image_processor};
use crate::state::AppState;
use chrono::Utc;
use tauri::{AppHandle, Emitter, Manager, State};

/// Save annotations to the database, generate crops, save annotated screenshot, and close the overlay.
#[tauri::command]
pub async fn save_capture_annotations(
    app: AppHandle,
    state: State<'_, AppState>,
    capture_id: String,
    issues: Vec<CreateIssuePayload>,
    annotated_image_base64: Option<String>,
) -> AppResult<()> {
    println!("[Backend] save_capture_annotations called for capture_id: {}, issues: {}, has_base64: {}", capture_id, issues.len(), annotated_image_base64.is_some());
    // 1. Lock DB and retrieve capture / session info
    let mut conn = state
        .db
        .lock()
        .map_err(|e| AppError::Database(e.to_string()))?;

    let capture = capture_repo::get_capture_by_id(&conn, &capture_id)?;
    let session = crate::db::session_repo::get_session(&conn, &capture.session_id)?;

    // Extract date_str from capture creation date or fallback to current date
    let date_str = if capture.created_at.len() >= 10 {
        capture.created_at[..10].to_string()
    } else {
        Utc::now().format("%Y-%m-%d").to_string()
    };

    // 2. Load original screenshot from disk
    let screenshot_abs_path = state.app_data_dir.join(&capture.screenshot_path);
    if !screenshot_abs_path.exists() {
        return Err(AppError::FileIO(format!(
            "Screenshot file not found at: {:?}",
            screenshot_abs_path
        )));
    }
    let base_image = image::open(&screenshot_abs_path)
        .map_err(|e| AppError::FileIO(format!("Failed to open base screenshot: {}", e)))?;

    // 3. Process annotated image (either decode base64 or fallback to Rust drawing logic)
    let annotated_image = if let Some(base64_str) = annotated_image_base64 {
        let clean_base64 = if base64_str.starts_with("data:") {
            if let Some(comma_idx) = base64_str.find(',') {
                &base64_str[comma_idx + 1..]
            } else {
                &base64_str
            }
        } else {
            &base64_str
        };

        use base64::{Engine as _, engine::general_purpose};
        let decoded_bytes = general_purpose::STANDARD
            .decode(clean_base64)
            .map_err(|e| AppError::Generic(format!("Failed to decode base64 annotated image: {}", e)))?;

        image::load_from_memory(&decoded_bytes)
            .map_err(|e| AppError::FileIO(format!("Failed to load decoded image from memory: {}", e)))?
    } else {
        // Fallback: draw annotations in Rust on base_image
        let annotation_strings: Vec<String> = issues.iter().map(|i| i.annotation_data.clone()).collect();
        let annotations_json = format!("[{}]", annotation_strings.join(","));
        let rgba = image_processor::draw_annotations_to_rgba(&base_image, &annotations_json)?;
        image::DynamicImage::ImageRgba8(rgba)
    };

    // 4. Save annotated version relative to screenshot path to prevent midnight mismatch bug
    let rel_annotated_path = capture.screenshot_path.replace(".png", "_annotated.png");
    let abs_annotated_path = state.app_data_dir.join(&rel_annotated_path);

    annotated_image.save(&abs_annotated_path)
        .map_err(|e| AppError::FileIO(format!("Failed to save annotated screenshot: {}", e)))?;

    // 5. Generate crop images for each issue (crop from the annotated image!)
    let mut crop_paths = Vec::new();
    let mut id_overrides = Vec::new();

    for payload in &issues {
        let issue_id = crate::utils::id::new_uuid();
        id_overrides.push(issue_id.clone());

        // Perform centered 400x400 crop around coordinates
        let cropped = image_processor::crop_for_issue(&annotated_image, payload.marker_x, payload.marker_y);

        // Save crop
        let rel_crop_path = file_storage::save_crop(cropped, &state.app_data_dir, &issue_id, &date_str)?;
        crop_paths.push(Some(rel_crop_path));
    }

    // 6. Commit issues batch to SQLite DB in a transaction
    let tx = conn
        .transaction()
        .map_err(|e| AppError::Database(format!("Failed to start transaction: {}", e)))?;

    issue_repo::create_batch(
        &tx,
        &capture_id,
        &capture.session_id,
        &session.project_id,
        &issues,
        &crop_paths,
        &id_overrides,
    )?;

    tx.commit()
        .map_err(|e| AppError::Database(format!("Failed to commit database changes: {}", e)))?;

    // Release database lock to prevent deadlock
    drop(conn);

    // 7. Emit session-updated event to notify frontend (e.g. Dashboard, SessionView) to refresh
    let _ = app.emit("session-updated", &capture.session_id);

    // 8. Close the overlay window
    if let Some(window) = app.get_webview_window("overlay") {
        let _ = window.close();
    }

    // Reset overlay active state
    if let Ok(mut active) = state.is_overlay_active.lock() {
        *active = false;
    }

    Ok(())
}

/// Retrieve all active issues associated with a session ID.
#[tauri::command]
pub fn get_issues(state: State<'_, AppState>, session_id: String) -> AppResult<Vec<Issue>> {
    let conn = state
        .db
        .lock()
        .map_err(|e| AppError::Database(e.to_string()))?;
    let mut issues = issue_repo::get_by_session(&conn, &session_id)?;

    for issue in &mut issues {
        if let Some(ref path) = issue.crop_path {
            let abs_path = state.app_data_dir.join(path);
            issue.crop_path = Some(abs_path.to_string_lossy().to_string());
        }
    }

    Ok(issues)
}

/// Retrieve a single issue by its ID.
#[tauri::command]
pub fn get_issue(state: State<'_, AppState>, id: String) -> AppResult<Issue> {
    let conn = state
        .db
        .lock()
        .map_err(|e| AppError::Database(e.to_string()))?;
    let mut issue = issue_repo::get_issue(&conn, &id)?;

    if let Some(ref path) = issue.crop_path {
        let abs_path = state.app_data_dir.join(path);
        issue.crop_path = Some(abs_path.to_string_lossy().to_string());
    }

    Ok(issue)
}

/// Update an issue's details.
#[tauri::command]
pub fn update_issue(
    state: State<'_, AppState>,
    id: String,
    payload: UpdateIssuePayload,
) -> AppResult<Issue> {
    let conn = state
        .db
        .lock()
        .map_err(|e| AppError::Database(e.to_string()))?;
    let mut issue = issue_repo::update(&conn, &id, &payload)?;

    if let Some(ref path) = issue.crop_path {
        let abs_path = state.app_data_dir.join(path);
        issue.crop_path = Some(abs_path.to_string_lossy().to_string());
    }

    Ok(issue)
}

/// Soft-delete an issue and clean up its crop file.
#[tauri::command]
pub fn delete_issue(state: State<'_, AppState>, id: String) -> AppResult<()> {
    let mut conn = state
        .db
        .lock()
        .map_err(|e| AppError::Database(e.to_string()))?;

    let issue = issue_repo::get_issue(&conn, &id)?;

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Database(format!("Failed to start transaction: {}", e)))?;

    // Delete issue record
    issue_repo::delete(&tx, &id)?;

    tx.commit()
        .map_err(|e| AppError::Database(format!("Failed to commit deletion transaction: {}", e)))?;

    // Clean up crop file from disk
    if let Some(ref crop_path) = issue.crop_path {
        let _ = file_storage::delete_file(&state.app_data_dir, crop_path);
    }

    Ok(())
}
