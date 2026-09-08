use crate::error::{AppError, AppResult};
use crate::models::issue::Tag;
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

/// Retrieve all tags from the database ordered alphabetically.
pub fn get_all_tags(conn: &Connection) -> AppResult<Vec<Tag>> {
    let mut stmt = conn
        .prepare("SELECT id, name, color FROM tags ORDER BY name ASC")
        .map_err(|e| AppError::Database(format!("Failed to prepare select all tags statement: {}", e)))?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
            })
        })
        .map_err(|e| AppError::Database(format!("Failed to query all tags: {}", e)))?;

    let mut tags = Vec::new();
    for row in rows {
        tags.push(row.map_err(|e| AppError::Database(format!("Failed to map tag row: {}", e)))?);
    }
    Ok(tags)
}

/// Create a new tag. If it already exists by name, returns the existing tag.
pub fn create_tag(conn: &Connection, name: &str, color: &str) -> AppResult<Tag> {
    // Check if the tag name already exists
    let existing: Option<Tag> = conn
        .query_row(
            "SELECT id, name, color FROM tags WHERE name = ?1",
            params![name],
            |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                })
            },
        )
        .optional()
        .map_err(|e| AppError::Database(format!("Failed to check existing tag by name: {}", e)))?;

    if let Some(tag) = existing {
        return Ok(tag);
    }

    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO tags (id, name, color, created_at) VALUES (?1, ?2, ?3, datetime('now'))",
        params![id, name, color],
    )
    .map_err(|e| AppError::Database(format!("Failed to insert tag '{}': {}", name, e)))?;

    Ok(Tag {
        id,
        name: name.to_string(),
        color: color.to_string(),
    })
}

/// Associate a tag with a specific issue.
pub fn associate_tag_with_issue(conn: &Connection, issue_id: &str, tag_id: &str) -> AppResult<()> {
    conn.execute(
        "INSERT OR IGNORE INTO issue_tags (issue_id, tag_id) VALUES (?1, ?2)",
        params![issue_id, tag_id],
    )
    .map(|_| ())
    .map_err(|e| AppError::Database(format!("Failed to associate tag '{}' with issue '{}': {}", tag_id, issue_id, e)))
}

/// Retrieve all tags associated with a specific issue.
pub fn get_tags_by_issue(conn: &Connection, issue_id: &str) -> AppResult<Vec<Tag>> {
    let mut stmt = conn
        .prepare(
            "SELECT t.id, t.name, t.color 
             FROM tags t
             JOIN issue_tags it ON t.id = it.tag_id
             WHERE it.issue_id = ?1
             ORDER BY t.name ASC",
        )
        .map_err(|e| AppError::Database(format!("Failed to prepare select tags by issue statement: {}", e)))?;

    let rows = stmt
        .query_map(params![issue_id], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
            })
        })
        .map_err(|e| AppError::Database(format!("Failed to query tags for issue: {}", e)))?;

    let mut tags = Vec::new();
    for row in rows {
        tags.push(row.map_err(|e| AppError::Database(format!("Failed to map issue tag row: {}", e)))?);
    }
    Ok(tags)
}

/// Clear all tag associations for a specific issue.
pub fn clear_issue_tags(conn: &Connection, issue_id: &str) -> AppResult<()> {
    conn.execute(
        "DELETE FROM issue_tags WHERE issue_id = ?1",
        params![issue_id],
    )
    .map(|_| ())
    .map_err(|e| AppError::Database(format!("Failed to clear tags for issue '{}': {}", issue_id, e)))
}
