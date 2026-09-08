use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capture {
    pub id: String,
    pub session_id: String,
    pub screenshot_path: String,
    pub monitor_name: String,
    pub monitor_x: i32,
    pub monitor_y: i32,
    pub monitor_width: i32,
    pub monitor_height: i32,
    pub scale_factor: f64,
    pub window_title: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorInfo {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub scale_factor: f64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCapturePayload {
    pub session_id: String,
    pub screenshot_path: String,
    pub monitor_name: String,
    pub monitor_x: i32,
    pub monitor_y: i32,
    pub monitor_width: i32,
    pub monitor_height: i32,
    pub scale_factor: f64,
    pub window_title: Option<String>,
}
