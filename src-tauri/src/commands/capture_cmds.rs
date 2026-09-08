use crate::db::capture_repo;
use crate::error::{AppError, AppResult};
use crate::models::{Capture, MonitorInfo};
use crate::services::{file_storage, screen_capture};
use crate::state::AppState;
use chrono::Utc;
use tauri::{window::Color, AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureResult {
    pub capture_id: String,
    pub screenshot_path: String,
    pub monitor_info: MonitorInfo,
}

#[cfg(target_os = "windows")]
fn get_foreground_window_title() -> Option<String> {
    use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() {
            return None;
        }
        let mut buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut buf);
        if len > 0 {
            Some(String::from_utf16_lossy(&buf[..len as usize]))
        } else {
            None
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn get_foreground_window_title() -> Option<String> {
    None
}

/// Chụp ảnh màn hình chứa con trỏ chuột, lưu vào đĩa, lưu thông tin vào DB.
#[tauri::command]
pub fn trigger_capture(
    _app: AppHandle,
    state: State<'_, AppState>,
    session_id: String,
) -> AppResult<CaptureResult> {
    // 1. Chụp màn hình hiện tại
    let (image, monitor_info) = screen_capture::capture_current_monitor()?;

    // 2. Tạo ID và lấy ngày hiện tại
    let capture_id = crate::utils::id::new_uuid();
    let date_str = Utc::now().format("%Y-%m-%d").to_string();

    // 3. Đảm bảo các thư mục tồn tại và lưu screenshot
    file_storage::ensure_dirs(&state.app_data_dir)?;
    let rel_path =
        file_storage::save_screenshot(image, &state.app_data_dir, &capture_id, &date_str)?;

    // 4. Lấy tiêu đề cửa sổ đang hoạt động (nếu có)
    let window_title = get_foreground_window_title();

    // 5. Lưu vào database
    let conn = state
        .db
        .lock()
        .map_err(|e| AppError::Database(e.to_string()))?;
    let capture =
        capture_repo::create_capture(&conn, &session_id, &rel_path, &monitor_info, window_title)?;

    // 6. Trả về đường dẫn tuyệt đối cho frontend nạp qua convertFileSrc
    let abs_path = state.app_data_dir.join(&rel_path);
    let screenshot_path = abs_path.to_string_lossy().to_string();

    Ok(CaptureResult {
        capture_id: capture.id,
        screenshot_path,
        monitor_info,
    })
}

/// Mở cửa sổ overlay hiển thị ảnh chụp màn hình để vẽ ghi chú.
///
/// Strategy để tránh flash trắng trên Windows/WebView2:
/// - Tạo window với visible=false và background_color transparent
/// - Frontend sẽ gọi lệnh show_overlay khi đã load xong content
#[tauri::command]
pub async fn open_overlay(
    app: AppHandle,
    state: State<'_, AppState>,
    capture_id: String,
    screenshot_path: Option<String>,
) -> AppResult<()> {
    let conn = state
        .db
        .lock()
        .map_err(|e| AppError::Database(e.to_string()))?;
    let capture = capture_repo::get_capture_by_id(&conn, &capture_id)?;
    // Release DB lock immediately - no longer needed
    drop(conn);

    let overlay_label = "overlay";

    // Đóng overlay cũ nếu đang mở
    if let Some(existing) = app.get_webview_window(overlay_label) {
        let _ = existing.close();
        // Small delay to let old window fully close
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    let scale = if capture.scale_factor > 0.0 {
        capture.scale_factor
    } else {
        1.0
    };
    let logical_x = capture.monitor_x as f64 / scale;
    let logical_y = capture.monitor_y as f64 / scale;
    let logical_width = capture.monitor_width as f64 / scale;
    let logical_height = capture.monitor_height as f64 / scale;

    // URL for the overlay window. Tauri v2 WebviewUrl::App does not support query parameters
    // because it resolves to a file path. Instead, we inject variables using initialization_script.
    let absolute_screenshot_path = screenshot_path.unwrap_or_else(|| {
        state
            .app_data_dir
            .join(&capture.screenshot_path)
            .to_string_lossy()
            .to_string()
    });
    // Replace backslashes with forward slashes to prevent escape characters issues in JS
    let js_path = absolute_screenshot_path.replace('\\', "/").replace('\'', "\\'");
    let init_script = format!(
        "window.__INITIAL_DATA__ = {{ captureId: '{}', screenshotPath: '{}' }};",
        capture_id, js_path
    );

    #[cfg(debug_assertions)]
    let url = WebviewUrl::External(tauri::Url::parse("http://localhost:1420/overlay.html").unwrap());
    #[cfg(not(debug_assertions))]
    let url = WebviewUrl::App("overlay.html".into());

    // Build opaque overlay window to ensure IME (Vietnamese typing) works properly
    let builder = WebviewWindowBuilder::new(&app, overlay_label, url)
        .title("August Guide Overlay")
        .decorations(false)
        .transparent(false)
        .background_color(Color(18, 18, 18, 255))
        .initialization_script(&init_script)
        .always_on_top(true)
        .skip_taskbar(false)
        .focused(true)
        .visible(true)
        .position(logical_x, logical_y)
        .inner_size(logical_width, logical_height);

    let overlay_window = builder
        .build()
        .map_err(|e| AppError::Generic(format!("Failed to build overlay window: {}", e)))?;

    let _ = overlay_window.set_focus();

    // Update overlay active state
    if let Ok(mut active) = state.is_overlay_active.lock() {
        *active = true;
    }

    // Emit overlay:init with captureId (backup for URL-param approach)
    let _ = overlay_window.emit("overlay:init", capture_id);

    Ok(())
}

/// Được gọi bởi frontend overlay khi đã load xong content và sẵn sàng hiển thị.
/// Lúc này mới show window và lấy focus để tránh white flash.
#[tauri::command]
pub fn show_overlay(app: AppHandle, _state: State<'_, AppState>) -> AppResult<()> {
    if let Some(window) = app.get_webview_window("overlay") {
        let _ = window.show();
        let _ = window.set_focus();
    }
    Ok(())
}

/// Đóng cửa sổ overlay.
#[tauri::command]
pub async fn close_overlay(app: AppHandle, state: State<'_, AppState>) -> AppResult<()> {
    if let Some(window) = app.get_webview_window("overlay") {
        let _ = window.close();
    }

    // Cập nhật trạng thái overlay hoạt động
    if let Ok(mut active) = state.is_overlay_active.lock() {
        *active = false;
    }

    // Clean up any uncommitted captures (screenshot taken but never saved with Done)
    if let Ok(conn) = state.db.lock() {
        let _ = capture_repo::cleanup_uncommitted_captures(&conn, &state.app_data_dir);
    }

    Ok(())
}

/// Hủy lượt chụp màn hình hiện tại (xóa file và DB record).
#[tauri::command]
pub async fn cancel_capture(
    app: AppHandle,
    state: State<'_, AppState>,
    capture_id: String,
) -> AppResult<()> {
    {
        let conn = state
            .db
            .lock()
            .map_err(|e| AppError::Database(e.to_string()))?;
        let capture = capture_repo::get_capture_by_id(&conn, &capture_id)?;

        // Xóa file trên đĩa
        let _ = file_storage::delete_capture_files(&state.app_data_dir, &capture.screenshot_path);

        // Xóa trong database
        capture_repo::delete_capture(&conn, &capture_id)?;
    } // MutexGuard conn is explicitly dropped here

    // Đóng cửa sổ overlay
    close_overlay(app, state).await?;

    Ok(())
}

/// Lấy thông tin chi tiết một Capture (chuyển đường dẫn screenshot sang tuyệt đối).
#[tauri::command]
pub fn get_capture(state: State<'_, AppState>, id: String) -> AppResult<Capture> {
    let conn = state
        .db
        .lock()
        .map_err(|e| AppError::Database(e.to_string()))?;
    let mut capture = capture_repo::get_capture_by_id(&conn, &id)?;

    // Đổi screenshot_path thành đường dẫn tuyệt đối để frontend nạp được qua asset protocol
    let abs_path = state.app_data_dir.join(&capture.screenshot_path);
    capture.screenshot_path = abs_path.to_string_lossy().to_string();

    Ok(capture)
}

/// In log từ frontend overlay lên terminal backend.
#[tauri::command]
pub fn log_from_js(msg: String) {
    println!("[Overlay JS] {}", msg);
}
