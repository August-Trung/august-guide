use crate::error::{AppError, AppResult};
use crate::models::{CreateIssuePayload, Issue, Tag, UpdateIssuePayload};
use crate::utils::id::new_uuid;
use chrono::Utc;
use rusqlite::{params, Connection};

/// Query tags linked to a specific issue.
pub fn get_tags_for_issue(conn: &Connection, issue_id: &str) -> AppResult<Vec<Tag>> {
    let mut stmt = conn
        .prepare(
            "SELECT t.id, t.name, t.color
             FROM tags t
             JOIN issue_tags it ON t.id = it.tag_id
             WHERE it.issue_id = ?1",
        )
        .map_err(|e| AppError::Database(format!("Failed to prepare get_tags statement: {}", e)))?;

    let tag_iter = stmt
        .query_map(params![issue_id], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
            })
        })
        .map_err(|e| AppError::Database(format!("Failed to query tags for issue: {}", e)))?;

    let mut tags = Vec::new();
    for t in tag_iter {
        tags.push(t.map_err(|e| AppError::Database(format!("Failed to map tag row: {}", e)))?);
    }
    Ok(tags)
}

/// Query a single issue by its ID.
pub fn get_issue(conn: &Connection, id: &str) -> AppResult<Issue> {
    let mut issue = conn
        .query_row(
            "SELECT id, capture_id, session_id, project_id, marker_number, title, description,
                    issue_type, severity, status, marker_x, marker_y, annotation_data,
                    color, stroke_width, crop_path, created_at, updated_at
             FROM issues WHERE id = ?1 AND is_deleted = 0",
            params![id],
            |row| {
                Ok(Issue {
                    id: row.get(0)?,
                    capture_id: row.get(1)?,
                    session_id: row.get(2)?,
                    project_id: row.get(3)?,
                    marker_number: row.get(4)?,
                    title: row.get(5)?,
                    description: row.get(6)?,
                    issue_type: row.get(7)?,
                    severity: row.get(8)?,
                    status: row.get(9)?,
                    marker_x: row.get(10)?,
                    marker_y: row.get(11)?,
                    annotation_data: row.get(12)?,
                    color: row.get(13)?,
                    stroke_width: row.get(14)?,
                    crop_path: row.get(15)?,
                    created_at: row.get(16)?,
                    updated_at: row.get(17)?,
                    tags: None,
                })
            },
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => {
                AppError::Validation(format!("Issue with ID '{}' not found", id))
            }
            _ => AppError::Database(format!("Failed to query issue by ID: {}", e)),
        })?;

    let tags = get_tags_for_issue(conn, id)?;
    issue.tags = Some(tags);
    Ok(issue)
}

/// Query all issues associated with a specific session.
pub fn get_by_session(conn: &Connection, session_id: &str) -> AppResult<Vec<Issue>> {
    let mut stmt = conn
        .prepare(
            "SELECT id, capture_id, session_id, project_id, marker_number, title, description,
                    issue_type, severity, status, marker_x, marker_y, annotation_data,
                    color, stroke_width, crop_path, created_at, updated_at
             FROM issues
             WHERE session_id = ?1 AND is_deleted = 0
             ORDER BY marker_number ASC",
        )
        .map_err(|e| AppError::Database(format!("Failed to prepare get_by_session statement: {}", e)))?;

    let issue_iter = stmt
        .query_map(params![session_id], |row| {
            Ok(Issue {
                id: row.get(0)?,
                capture_id: row.get(1)?,
                session_id: row.get(2)?,
                project_id: row.get(3)?,
                marker_number: row.get(4)?,
                title: row.get(5)?,
                description: row.get(6)?,
                issue_type: row.get(7)?,
                severity: row.get(8)?,
                status: row.get(9)?,
                marker_x: row.get(10)?,
                marker_y: row.get(11)?,
                annotation_data: row.get(12)?,
                color: row.get(13)?,
                stroke_width: row.get(14)?,
                crop_path: row.get(15)?,
                created_at: row.get(16)?,
                updated_at: row.get(17)?,
                tags: None,
            })
        })
        .map_err(|e| AppError::Database(format!("Failed to query issues by session: {}", e)))?;

    let mut issues = Vec::new();
    for item in issue_iter {
        let mut issue = item.map_err(|e| AppError::Database(format!("Failed to map issue row: {}", e)))?;
        let tags = get_tags_for_issue(conn, &issue.id)?;
        issue.tags = Some(tags);
        issues.push(issue);
    }
    Ok(issues)
}

/// Query all issues associated with a specific capture.
pub fn get_by_capture(conn: &Connection, capture_id: &str) -> AppResult<Vec<Issue>> {
    let mut stmt = conn
        .prepare(
            "SELECT id, capture_id, session_id, project_id, marker_number, title, description,
                    issue_type, severity, status, marker_x, marker_y, annotation_data,
                    color, stroke_width, crop_path, created_at, updated_at
             FROM issues
             WHERE capture_id = ?1 AND is_deleted = 0
             ORDER BY marker_number ASC",
        )
        .map_err(|e| AppError::Database(format!("Failed to prepare get_by_capture statement: {}", e)))?;

    let issue_iter = stmt
        .query_map(params![capture_id], |row| {
            Ok(Issue {
                id: row.get(0)?,
                capture_id: row.get(1)?,
                session_id: row.get(2)?,
                project_id: row.get(3)?,
                marker_number: row.get(4)?,
                title: row.get(5)?,
                description: row.get(6)?,
                issue_type: row.get(7)?,
                severity: row.get(8)?,
                status: row.get(9)?,
                marker_x: row.get(10)?,
                marker_y: row.get(11)?,
                annotation_data: row.get(12)?,
                color: row.get(13)?,
                stroke_width: row.get(14)?,
                crop_path: row.get(15)?,
                created_at: row.get(16)?,
                updated_at: row.get(17)?,
                tags: None,
            })
        })
        .map_err(|e| AppError::Database(format!("Failed to query issues by capture: {}", e)))?;

    let mut issues = Vec::new();
    for item in issue_iter {
        let mut issue = item.map_err(|e| AppError::Database(format!("Failed to map issue row: {}", e)))?;
        let tags = get_tags_for_issue(conn, &issue.id)?;
        issue.tags = Some(tags);
        issues.push(issue);
    }
    Ok(issues)
}

/// Create a single issue and link its tags.
pub fn create_issue(
    conn: &Connection,
    capture_id: &str,
    session_id: &str,
    project_id: &str,
    payload: &CreateIssuePayload,
    crop_path: Option<&str>,
    id_override: Option<&str>,
) -> AppResult<Issue> {
    let id = id_override.map(|s| s.to_string()).unwrap_or_else(new_uuid);
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO issues (
            id, capture_id, session_id, project_id, marker_number, title, description,
            issue_type, severity, status, marker_x, marker_y, annotation_data,
            color, stroke_width, crop_path, is_deleted, created_at, updated_at
         )
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, 0, ?17, ?17)",
        params![
            id,
            capture_id,
            session_id,
            project_id,
            payload.marker_number,
            payload.title,
            payload.description,
            payload.issue_type,
            payload.severity,
            payload.status,
            payload.marker_x,
            payload.marker_y,
            payload.annotation_data,
            payload.color,
            payload.stroke_width,
            crop_path,
            now
        ],
    )
    .map_err(|e| AppError::Database(format!("Failed to insert issue: {}", e)))?;

    // Handle tag links
    for tag_name in &payload.tags {
        let tag_id: String = match conn.query_row(
            "SELECT id FROM tags WHERE name = ?1",
            params![tag_name],
            |row| row.get(0),
        ) {
            Ok(existing_id) => existing_id,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                let new_tag_id = new_uuid();
                conn.execute(
                    "INSERT INTO tags (id, name, color, created_at) VALUES (?1, ?2, '#4ECDC4', ?3)",
                    params![new_tag_id, tag_name, now],
                )
                .map_err(|e| AppError::Database(format!("Failed to insert tag: {}", e)))?;
                new_tag_id
            }
            Err(e) => return Err(AppError::Database(format!("Failed to query tag: {}", e))),
        };

        conn.execute(
            "INSERT OR IGNORE INTO issue_tags (issue_id, tag_id) VALUES (?1, ?2)",
            params![id, tag_id],
        )
        .map_err(|e| AppError::Database(format!("Failed to link issue and tag: {}", e)))?;
    }

    get_issue(conn, &id)
}

/// Create a batch of issues.
pub fn create_batch(
    conn: &Connection,
    capture_id: &str,
    session_id: &str,
    project_id: &str,
    issues: &[CreateIssuePayload],
    crop_paths: &[Option<String>],
    id_overrides: &[String],
) -> AppResult<Vec<Issue>> {
    let mut created = Vec::new();
    for (i, payload) in issues.iter().enumerate() {
        let crop = crop_paths.get(i).and_then(|c| c.as_deref());
        let id_override = id_overrides.get(i).map(|s| s.as_str());
        let issue = create_issue(
            conn,
            capture_id,
            session_id,
            project_id,
            payload,
            crop,
            id_override,
        )?;
        created.push(issue);
    }
    Ok(created)
}

/// Update an existing issue's metadata and update tags link.
pub fn update(conn: &Connection, id: &str, payload: &UpdateIssuePayload) -> AppResult<Issue> {
    let _existing = get_issue(conn, id)?;
    let now = Utc::now().to_rfc3339();

    let mut query = String::from("UPDATE issues SET ");
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    let mut sets = Vec::new();
    let mut param_index = 1;

    if let Some(ref title) = payload.title {
        sets.push(format!("title = ?{}", param_index));
        params_vec.push(Box::new(title.clone()));
        param_index += 1;
    }

    if let Some(ref desc) = payload.description {
        sets.push(format!("description = ?{}", param_index));
        params_vec.push(Box::new(desc.clone()));
        param_index += 1;
    }

    if let Some(ref issue_type) = payload.issue_type {
        sets.push(format!("issue_type = ?{}", param_index));
        params_vec.push(Box::new(issue_type.clone()));
        param_index += 1;
    }

    if let Some(ref severity) = payload.severity {
        sets.push(format!("severity = ?{}", param_index));
        params_vec.push(Box::new(severity.clone()));
        param_index += 1;
    }

    if let Some(ref status) = payload.status {
        sets.push(format!("status = ?{}", param_index));
        params_vec.push(Box::new(status.clone()));
        param_index += 1;
    }

    sets.push(format!("updated_at = ?{}", param_index));
    params_vec.push(Box::new(now));
    param_index += 1;

    query.push_str(&sets.join(", "));
    query.push_str(&format!(" WHERE id = ?{}", param_index));
    params_vec.push(Box::new(id.to_string()));

    let refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|b| b.as_ref()).collect();

    conn.execute(&query, refs.as_slice()).map_err(|e| {
        AppError::Database(format!("Failed to execute update issue query: {}", e))
    })?;

    // Update tags if provided
    if let Some(ref tag_names) = payload.tags {
        conn.execute("DELETE FROM issue_tags WHERE issue_id = ?1", params![id])
            .map_err(|e| AppError::Database(format!("Failed to clear existing tags: {}", e)))?;

        let now_str = Utc::now().to_rfc3339();
        for tag_name in tag_names {
            let tag_id: String = match conn.query_row(
                "SELECT id FROM tags WHERE name = ?1",
                params![tag_name],
                |row| row.get(0),
            ) {
                Ok(existing_id) => existing_id,
                Err(rusqlite::Error::QueryReturnedNoRows) => {
                    let new_tag_id = new_uuid();
                    conn.execute(
                        "INSERT INTO tags (id, name, color, created_at) VALUES (?1, ?2, '#4ECDC4', ?3)",
                        params![new_tag_id, tag_name, now_str],
                    )
                    .map_err(|e| AppError::Database(format!("Failed to insert tag: {}", e)))?;
                    new_tag_id
                }
                Err(e) => return Err(AppError::Database(format!("Failed to query tag: {}", e))),
            };

            conn.execute(
                "INSERT OR IGNORE INTO issue_tags (issue_id, tag_id) VALUES (?1, ?2)",
                params![id, tag_id],
            )
            .map_err(|e| AppError::Database(format!("Failed to link issue and tag: {}", e)))?;
        }
    }

    get_issue(conn, id)
}

/// Soft-delete an issue.
pub fn delete(conn: &Connection, id: &str) -> AppResult<()> {
    conn.execute(
        "UPDATE issues SET is_deleted = 1, updated_at = ?2 WHERE id = ?1",
        params![id, Utc::now().to_rfc3339()],
    )
    .map_err(|e| AppError::Database(format!("Failed to soft delete issue: {}", e)))?;
    Ok(())
}

/// Soft-delete all issues of a capture.
pub fn delete_by_capture(conn: &Connection, capture_id: &str) -> AppResult<()> {
    conn.execute(
        "UPDATE issues SET is_deleted = 1, updated_at = ?2 WHERE capture_id = ?1",
        params![capture_id, Utc::now().to_rfc3339()],
    )
    .map_err(|e| AppError::Database(format!("Failed to soft delete issues by capture: {}", e)))?;
    Ok(())
}
