use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub status: String, // 'active' | 'completed' | 'archived'
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
    
    // Computed fields from database JOIN queries
    pub issue_count: Option<i64>,
    pub capture_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionPayload {
    pub project_id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSessionPayload {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}
