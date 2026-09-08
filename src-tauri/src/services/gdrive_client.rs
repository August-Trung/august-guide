// Google Drive Client API Service Implementation
// Forces recompilation to load the new .env file variables.
use std::collections::HashMap;
use std::net::TcpListener;
use std::time::{Duration, Instant};
use tiny_http::{Server, Response};
use url::Url;
use reqwest::Client; // Async client

use crate::error::{AppError, AppResult};
use crate::models::gdrive::{GoogleToken, GoogleUserInfo};

include!(concat!(env!("OUT_DIR"), "/credentials.rs"));

pub const CLIENT_ID: &str = GOOGLE_CLIENT_ID;
pub const CLIENT_SECRET: &str = GOOGLE_CLIENT_SECRET;
pub const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
pub const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
pub const USERINFO_URL: &str = "https://www.googleapis.com/oauth2/v2/userinfo";
pub const DRIVE_FILES_URL: &str = "https://www.googleapis.com/drive/v3/files";
pub const DRIVE_UPLOAD_URL: &str = "https://www.googleapis.com/upload/drive/v3/files";

/// Finds a free local port between 45000 and 46000
pub fn find_free_port() -> Option<u16> {
    for port in 45000..=46000 {
        if TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok() {
            return Some(port);
        }
    }
    None
}

/// Starts a temporary loopback HTTP server to listen for Google OAuth redirect
/// Note: Keeps synchronous signature because it will be run in spawn_blocking task
pub fn start_loopback_server(port: u16) -> AppResult<String> {
    let server = Server::http(format!("127.0.0.1:{}", port))
        .map_err(|e| AppError::Generic(format!("Failed to start OAuth server: {}", e)))?;
    
    let start_time = Instant::now();
    let timeout = Duration::from_secs(180); // 3 minutes timeout

    loop {
        if start_time.elapsed() > timeout {
            return Err(AppError::OAuth("Authentication timed out after 3 minutes.".into()));
        }

        match server.recv_timeout(Duration::from_millis(1000)) {
            Ok(Some(request)) => {
                let url_str = format!("http://127.0.0.1:{}{}", port, request.url());
                if let Ok(url) = Url::parse(&url_str) {
                    let params: HashMap<_, _> = url.query_pairs().into_owned().collect();
                    if let Some(code) = params.get("code") {
                        let code_val = code.clone();
                        let html = r#"
                            <!DOCTYPE html>
                            <html>
                            <head>
                                <meta charset="utf-8">
                                <title>August Mark Authentication</title>
                                <style>
                                    body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; background-color: #121212; color: #E0E0E0; text-align: center; padding: 50px; }
                                    .container { max-width: 500px; margin: 0 auto; background: #1E1E1E; padding: 30px; border-radius: 12px; box-shadow: 0 4px 20px rgba(0,0,0,0.5); }
                                    h1 { color: #4CAF50; margin-bottom: 20px; }
                                    p { font-size: 16px; line-height: 1.6; }
                                </style>
                            </head>
                            <body>
                                <div class="container">
                                    <h1>Xác thực thành công!</h1>
                                    <p>August Mark đã nhận được mã ủy quyền từ Google.</p>
                                    <p>Bạn có thể đóng cửa sổ trình duyệt này và quay lại ứng dụng.</p>
                                </div>
                            </body>
                            </html>
                        "#;
                        let response = Response::from_string(html)
                            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap());
                        let _ = request.respond(response);
                        return Ok(code_val);
                    } else if let Some(error) = params.get("error") {
                        let err_msg = error.clone();
                        let html = format!(r#"
                            <!DOCTYPE html>
                            <html>
                            <head>
                                <meta charset="utf-8">
                                <title>August Mark Authentication Failed</title>
                                <style>
                                    body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; background-color: #121212; color: #E0E0E0; text-align: center; padding: 50px; }}
                                    .container {{ max-width: 500px; margin: 0 auto; background: #1E1E1E; padding: 30px; border-radius: 12px; box-shadow: 0 4px 20px rgba(0,0,0,0.5); }}
                                    h1 {{ color: #F44336; margin-bottom: 20px; }}
                                    p {{ font-size: 16px; line-height: 1.6; }}
                                </style>
                            </head>
                            <body>
                                <div class="container">
                                    <h1>Xác thực thất bại!</h1>
                                    <p>Lỗi: {}</p>
                                    <p>Vui lòng đóng trình duyệt và thử lại.</p>
                                </div>
                            </body>
                            </html>
                        "#, err_msg);
                        let response = Response::from_string(html)
                            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap());
                        let _ = request.respond(response);
                        return Err(AppError::OAuth(err_msg));
                    }
                }
                let response = Response::from_string("Redirecting...")
                    .with_status_code(200);
                let _ = request.respond(response);
            }
            Ok(None) => {}
            Err(e) => {
                return Err(AppError::OAuth(format!("Server read error: {}", e)));
            }
        }
    }
}

/// Exchanges Auth Code for access and refresh tokens (Async)
pub async fn exchange_token(auth_code: &str, redirect_uri: &str) -> AppResult<GoogleToken> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    
    let mut params = HashMap::new();
    params.insert("client_id", CLIENT_ID);
    params.insert("client_secret", CLIENT_SECRET);
    params.insert("code", auth_code);
    params.insert("redirect_uri", redirect_uri);
    params.insert("grant_type", "authorization_code");

    let res = client.post(TOKEN_URL)
        .form(&params)
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(AppError::OAuth(format!("Token exchange failed: {}", err_text)));
    }

    let token: GoogleToken = res.json().await?;
    Ok(token)
}

/// Refreshes the Access Token using a Refresh Token (Async)
pub async fn refresh_access_token(refresh_token: &str) -> AppResult<GoogleToken> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let mut params = HashMap::new();
    params.insert("client_id", CLIENT_ID);
    params.insert("client_secret", CLIENT_SECRET);
    params.insert("refresh_token", refresh_token);
    params.insert("grant_type", "refresh_token");

    let res = client.post(TOKEN_URL)
        .form(&params)
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(AppError::OAuth(format!("Token refresh failed: {}", err_text)));
    }

    let token: GoogleToken = res.json().await?;
    Ok(token)
}

/// Fetches user profile info (email address) from Google UserInfo endpoint (Async)
pub async fn get_user_info(access_token: &str) -> AppResult<GoogleUserInfo> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let res = client.get(USERINFO_URL)
        .bearer_auth(access_token)
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(AppError::GDrive(format!("Failed to get user info: {}", err_text)));
    }

    let info: GoogleUserInfo = res.json().await?;
    Ok(info)
}

/// Helper to build a multipart/related body for Google Drive file uploads
fn build_multipart_body(filename: &str, parent_id: Option<&str>, mime_type: &str, file_data: &[u8]) -> (String, Vec<u8>) {
    let boundary = "august_mark_boundary_12345";
    let mut body = Vec::new();
    
    // Part 1: Metadata
    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"Content-Type: application/json; charset=UTF-8\r\n\r\n");
    
    let metadata = if let Some(parent) = parent_id {
        format!(r#"{{"name":"{}","parents":["{}"]}}"#, filename, parent)
    } else {
        format!(r#"{{"name":"{}"}}"#, filename)
    };
    body.extend_from_slice(metadata.as_bytes());
    body.extend_from_slice(b"\r\n");
    
    // Part 2: Media
    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(format!("Content-Type: {}\r\n\r\n", mime_type).as_bytes());
    body.extend_from_slice(file_data);
    body.extend_from_slice(b"\r\n");
    
    // End
    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());
    
    (boundary.to_string(), body)
}

/// Creates a folder in Google Drive if it doesn't already exist (Async)
pub async fn create_folder_if_not_exists(access_token: &str, folder_name: &str) -> AppResult<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()?;

    // 1. Search for existing folder
    let query = format!("name = '{}' and mimeType = 'application/vnd.google-apps.folder' and trashed = false", folder_name);
    let res = client.get(DRIVE_FILES_URL)
        .bearer_auth(access_token)
        .query(&[("q", query.as_str()), ("fields", "files(id)")])
        .send()
        .await?;

    if res.status().is_success() {
        if let Ok(list_json) = res.json::<serde_json::Value>().await {
            if let Some(files) = list_json.get("files").and_then(|f| f.as_array()) {
                if !files.is_empty() {
                    if let Some(id) = files[0].get("id").and_then(|id| id.as_str()) {
                        return Ok(id.to_string());
                    }
                }
            }
        }
    }

    // 2. Create it if not found
    let mut body = HashMap::new();
    body.insert("name", folder_name);
    body.insert("mimeType", "application/vnd.google-apps.folder");

    let res = client.post(DRIVE_FILES_URL)
        .bearer_auth(access_token)
        .json(&body)
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(AppError::GDrive(format!("Failed to create Drive folder: {}", err_text)));
    }

    let folder_json: serde_json::Value = res.json().await?;
    let id = folder_json.get("id")
        .and_then(|id| id.as_str())
        .ok_or_else(|| AppError::GDrive("Invalid response from folder creation".into()))?;

    Ok(id.to_string())
}

/// Uploads a file to Google Drive under a specific folder (Async)
pub async fn upload_file(
    access_token: &str,
    filename: &str,
    mime_type: &str,
    file_data: Vec<u8>,
    parent_id: Option<&str>
) -> AppResult<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(60))
        .build()?;

    let (boundary, multipart_body) = build_multipart_body(filename, parent_id, mime_type, &file_data);

    let res = client.post(DRIVE_UPLOAD_URL)
        .query(&[("uploadType", "multipart")])
        .bearer_auth(access_token)
        .header("Content-Type", format!("multipart/related; boundary={}", boundary))
        .body(multipart_body)
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(AppError::GDrive(format!("Upload failed: {}", err_text)));
    }

    let res_json: serde_json::Value = res.json().await?;
    let id = res_json.get("id")
        .and_then(|id| id.as_str())
        .ok_or_else(|| AppError::GDrive("Invalid upload response".into()))?;

    Ok(id.to_string())
}

/// Shares a Google Drive file publicly so anyone with the link can view it (Async)
pub async fn share_file_publicly(access_token: &str, file_id: &str) -> AppResult<()> {
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()?;

    let url = format!("{}/{}/permissions", DRIVE_FILES_URL, file_id);
    let mut body = HashMap::new();
    body.insert("role", "reader");
    body.insert("type", "anyone");

    let res = client.post(&url)
        .bearer_auth(access_token)
        .json(&body)
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(AppError::GDrive(format!("Failed to share file: {}", err_text)));
    }

    Ok(())
}

/// Gets the web view link (public view link) of a Google Drive file (Async)
pub async fn get_web_view_link(access_token: &str, file_id: &str) -> AppResult<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let url = format!("{}/{}", DRIVE_FILES_URL, file_id);
    let res = client.get(&url)
        .bearer_auth(access_token)
        .query(&[("fields", "webViewLink")])
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(AppError::GDrive(format!("Failed to get file metadata: {}", err_text)));
    }

    let file_json: serde_json::Value = res.json().await?;
    let link = file_json.get("webViewLink")
        .and_then(|l| l.as_str())
        .ok_or_else(|| AppError::GDrive("WebViewLink not found in response".into()))?;

    Ok(link.to_string())
}

/// Lists all ZIP backup files in the backup folder (Async)
pub async fn list_backups(access_token: &str, folder_id: &str) -> AppResult<Vec<(String, String)>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()?;

    let query = format!("'{}' in parents and mimeType = 'application/zip' and trashed = false", folder_id);
    let res = client.get(DRIVE_FILES_URL)
        .bearer_auth(access_token)
        .query(&[("q", query.as_str()), ("fields", "files(id,name)"), ("orderBy", "createdTime desc")])
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(AppError::GDrive(format!("Failed to list backups: {}", err_text)));
    }

    let res_json: serde_json::Value = res.json().await?;
    let mut backups = Vec::new();
    if let Some(files) = res_json.get("files").and_then(|f| f.as_array()) {
        for file in files {
            if let (Some(id), Some(name)) = (file.get("id").and_then(|id| id.as_str()), file.get("name").and_then(|n| n.as_str())) {
                backups.push((id.to_string(), name.to_string()));
            }
        }
    }
    Ok(backups)
}

/// Downloads a file from Google Drive as raw bytes (Async)
pub async fn download_file(access_token: &str, file_id: &str) -> AppResult<Vec<u8>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(120))
        .build()?;

    let url = format!("{}/{}", DRIVE_FILES_URL, file_id);
    let res = client.get(&url)
        .bearer_auth(access_token)
        .query(&[("alt", "media")])
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(AppError::GDrive(format!("Failed to download file: {}", err_text)));
    }

    let data = res.bytes().await?.to_vec();
    Ok(data)
}
