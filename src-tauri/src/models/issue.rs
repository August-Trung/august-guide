use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: String,
    pub capture_id: String,
    pub session_id: String,
    pub project_id: String,
    pub marker_number: i32,
    pub title: String,
    pub description: String,
    pub issue_type: String,
    pub severity: String,
    pub status: String,
    pub marker_x: f64,
    pub marker_y: f64,
    pub annotation_data: String, // JSON string stored in SQLite text column
    pub color: String,
    pub stroke_width: f64,
    pub crop_path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIssuePayload {
    pub marker_number: i32,
    pub title: String,
    pub description: String,
    pub issue_type: String,
    pub severity: String,
    pub status: String,
    pub marker_x: f64,
    pub marker_y: f64,
    pub annotation_data: String, // JSON string
    pub color: String,
    pub stroke_width: f64,
    pub tags: Vec<String>,       // List of tag names/IDs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIssuePayload {
    pub title: Option<String>,
    pub description: Option<String>,
    pub issue_type: Option<String>,
    pub severity: Option<String>,
    pub status: Option<String>,
    pub tags: Option<Vec<String>>,
}
