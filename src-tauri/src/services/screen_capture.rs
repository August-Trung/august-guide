use crate::error::{AppError, AppResult};
use crate::models::capture::MonitorInfo;
use image::DynamicImage;
use xcap::Monitor;

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::POINT;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

#[cfg(target_os = "windows")]
fn get_cursor_position() -> Option<(i32, i32)> {
    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        if GetCursorPos(&mut point).is_ok() {
            Some((point.x, point.y))
        } else {
            None
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn get_cursor_position() -> Option<(i32, i32)> {
    None
}

pub fn capture_current_monitor() -> AppResult<(DynamicImage, MonitorInfo)> {
    // 1. Get all monitors
    let monitors = Monitor::all()
        .map_err(|e| AppError::ScreenCapture(format!("Failed to retrieve monitors: {}", e)))?;

    if monitors.is_empty() {
        return Err(AppError::ScreenCapture("No monitors found".to_string()));
    }

    // 2. Determine target monitor based on cursor position
    let cursor_pos = get_cursor_position();
    let target_monitor = if let Some((mx, my)) = cursor_pos {
        // Find monitor enclosing the cursor coordinates
        monitors.into_iter().find(|m| {
            let x = m.x();
            let y = m.y();
            let w = m.width() as i32;
            let h = m.height() as i32;
            mx >= x && mx < x + w && my >= y && my < y + h
        })
    } else {
        None
    };

    // Fallback if no specific monitor was identified by cursor
    let monitor = target_monitor.unwrap_or_else(|| {
        // Fallback to primary monitor
        let monitors = Monitor::all().unwrap_or_default();
        monitors
            .into_iter()
            .find(|m| m.is_primary())
            // If no primary monitor, take the first one
            .or_else(|| {
                Monitor::all().ok().and_then(|mut ms| {
                    if ms.is_empty() {
                        None
                    } else {
                        Some(ms.remove(0))
                    }
                })
            })
            .expect("At least one monitor must exist")
    });

    // 3. Capture the screen
    let image_buf = monitor
        .capture_image()
        .map_err(|e| AppError::ScreenCapture(format!("Failed to capture image: {}", e)))?;

    // 4. Convert RgbaImage to DynamicImage
    let dynamic_image = DynamicImage::ImageRgba8(image_buf);

    // 5. Gather monitor info
    let info = MonitorInfo {
        x: monitor.x(),
        y: monitor.y(),
        width: monitor.width() as i32,
        height: monitor.height() as i32,
        scale_factor: monitor.scale_factor() as f64,
        name: monitor.name().to_string(),
    };

    Ok((dynamic_image, info))
}
