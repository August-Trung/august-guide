use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueInfo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub issue_type: String,
    pub severity: String,
    pub status: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodebaseInfo {
    pub local_path: String,
    pub git_commit: Option<String>,
    pub git_branch: Option<String>,
    pub git_status: Option<String>,
    pub suspected_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CropInfo {
    pub filename: String,
    pub marker_number: i32,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Visuals {
    pub full_screenshots: Vec<String>,
    pub crops: Vec<CropInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AacpManifest {
    pub issues: Vec<IssueInfo>,
    pub codebase: CodebaseInfo,
    pub visuals: Visuals,
}

