use crate::error::{AppError, AppResult};
use crate::models::{CreateProjectPayload, Project, UpdateProjectPayload};
use crate::utils::id::new_uuid;
use chrono::Utc;
use rusqlite::{params, Connection};

/// Query a single project by its ID.
pub fn get_project(conn: &Connection, id: &str) -> AppResult<Project> {
    conn.query_row(
        "SELECT id, name, description, color, is_archived, created_at, updated_at 
         FROM projects WHERE id = ?1 AND is_deleted = 0",
        params![id],
        |row| {
            let is_archived_int: i32 = row.get(4)?;
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                color: row.get(3)?,
                is_archived: is_archived_int != 0,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => {
            AppError::Validation(format!("Project with ID '{}' not found", id))
        }
        _ => AppError::Database(format!("Failed to query project by ID: {}", e)),
    })
}

/// Query all projects. Filter by archived status if specified.
pub fn get_projects(conn: &Connection, include_archived: bool) -> AppResult<Vec<Project>> {
    let query = if include_archived {
        "SELECT id, name, description, color, is_archived, created_at, updated_at 
         FROM projects WHERE is_deleted = 0 ORDER BY name ASC"
    } else {
        "SELECT id, name, description, color, is_archived, created_at, updated_at 
         FROM projects WHERE is_archived = 0 AND is_deleted = 0 ORDER BY name ASC"
    };

    let mut stmt = conn.prepare(query).map_err(|e| {
        AppError::Database(format!("Failed to prepare get_projects statement: {}", e))
    })?;

    let project_iter = stmt
        .query_map([], |row| {
            let is_archived_int: i32 = row.get(4)?;
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                color: row.get(3)?,
                is_archived: is_archived_int != 0,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })
        .map_err(|e| AppError::Database(format!("Failed to query projects: {}", e)))?;

    let mut projects = Vec::new();
    for p in project_iter {
        projects
            .push(p.map_err(|e| AppError::Database(format!("Failed to map project row: {}", e)))?);
    }
    Ok(projects)
}

/// Create a new project.
pub fn create_project(conn: &Connection, payload: CreateProjectPayload) -> AppResult<Project> {
    let id = new_uuid();
    let now = Utc::now().to_rfc3339();
    let description = payload.description.unwrap_or_default();
    let color = payload.color.unwrap_or_else(|| "#FF6B35".to_string());

    conn.execute(
        "INSERT INTO projects (id, name, description, color, is_archived, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, 0, ?5, ?5)",
        params![id, payload.name, description, color, now],
    )
    .map_err(|e| AppError::Database(format!("Failed to insert project: {}", e)))?;

    get_project(conn, &id)
}

/// Update an existing project's fields.
pub fn update_project(
    conn: &Connection,
    id: &str,
    payload: UpdateProjectPayload,
) -> AppResult<Project> {
    // Validate that project exists
    let _existing = get_project(conn, id)?;

    let now = Utc::now().to_rfc3339();

    // Construct dynamic UPDATE statement based on provided payload fields
    let mut query = String::from("UPDATE projects SET ");
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    let mut sets = Vec::new();
    let mut param_index = 1;

    if let Some(ref name) = payload.name {
        sets.push(format!("name = ?{}", param_index));
        params_vec.push(Box::new(name.clone()));
        param_index += 1;
    }

    if let Some(ref desc) = payload.description {
        sets.push(format!("description = ?{}", param_index));
        params_vec.push(Box::new(desc.clone()));
        param_index += 1;
    }

    if let Some(ref color) = payload.color {
        sets.push(format!("color = ?{}", param_index));
        params_vec.push(Box::new(color.clone()));
        param_index += 1;
    }

    if let Some(is_archived) = payload.is_archived {
        sets.push(format!("is_archived = ?{}", param_index));
        let val = if is_archived { 1 } else { 0 };
        params_vec.push(Box::new(val));
        param_index += 1;
    }

    // Always update updated_at timestamp
    sets.push(format!("updated_at = ?{}", param_index));
    params_vec.push(Box::new(now));
    param_index += 1;

    query.push_str(&sets.join(", "));
    query.push_str(&format!(" WHERE id = ?{}", param_index));
    params_vec.push(Box::new(id.to_string()));

    let refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|b| b.as_ref()).collect();

    conn.execute(&query, refs.as_slice()).map_err(|e| {
        AppError::Database(format!("Failed to execute update project query: {}", e))
    })?;

    get_project(conn, id)
}

/// Delete a project. Cascading constraints will automatically handle downstream rows.
pub fn delete_project(conn: &Connection, id: &str) -> AppResult<()> {
    conn.execute(
        "UPDATE projects SET is_deleted = 1 WHERE id = ?1",
        params![id],
    )
    .map_err(|e| AppError::Database(format!("Failed to soft delete project: {}", e)))?;
    Ok(())
}
