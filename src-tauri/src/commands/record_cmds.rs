use crate::error::{AppError, AppResult};
use std::io::Write;

/// Lưu dữ liệu video đã record (raw bytes hoặc base64) ra đường dẫn file chỉ định trên máy tính.
/// Tự động sử dụng FFmpeg (nếu có trên hệ thống) để chuẩn hóa định dạng MP4 tiêu chuẩn (H.264/AAC với +faststart moov)
/// giúp tua được timeline tức thì trên Windows Media Player và mọi trình phát video.
#[tauri::command]
pub fn save_recording_file(
    file_path: String,
    data: Option<Vec<u8>>,
    base64_data: Option<String>,
) -> AppResult<String> {
    use base64::engine::general_purpose::STANDARD as BASE64;
    use base64::Engine;

    let bytes = if let Some(raw) = data {
        raw
    } else if let Some(b64) = base64_data {
        let clean = if let Some(idx) = b64.find(',') {
            &b64[idx + 1..]
        } else {
            &b64
        };
        BASE64
            .decode(clean.trim())
            .map_err(|e| AppError::Generic(format!("Failed to decode video base64: {}", e)))?
    } else {
        return Err(AppError::Generic("No video data provided".to_string()));
    };

    let target_path = std::path::PathBuf::from(&file_path);
    if let Some(parent) = target_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::FileIO(format!("Failed to create parent directory: {}", e)))?;
        }
    }

    let is_target_mp4 = file_path.to_lowercase().ends_with(".mp4");
    let is_input_webm = bytes.starts_with(b"\x1a\x45\xdf\xa3");

    // Kiểm tra xem hệ thống có sẵn ffmpeg không
    let has_ffmpeg = {
        let mut cmd = std::process::Command::new("ffmpeg");
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }
        cmd.arg("-version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    };

    // Nếu muốn lưu MP4 và hệ thống có ffmpeg, thực hiện đóng gói/remux chuẩn hóa
    if is_target_mp4 && has_ffmpeg {
        let temp_dir = std::env::temp_dir();
        let temp_filename = format!(
            "august_record_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_millis()
        );
        let temp_input = if is_input_webm {
            temp_dir.join(format!("{}.webm", temp_filename))
        } else {
            temp_dir.join(format!("{}.tmp", temp_filename))
        };

        // Ghi dữ liệu nguồn ra file tạm
        if let Ok(mut temp_file) = std::fs::File::create(&temp_input) {
            let _ = temp_file.write_all(&bytes);
            drop(temp_file);

            let temp_input_str = temp_input.to_string_lossy().to_string();

            let mut ffmpeg_cmd = std::process::Command::new("ffmpeg");
            #[cfg(target_os = "windows")]
            {
                use std::os::windows::process::CommandExt;
                ffmpeg_cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
            }

            let ffmpeg_status = if is_input_webm {
                // Chuyển đổi WebM sang MP4 chuẩn H.264/AAC với +faststart để tua mượt mà
                ffmpeg_cmd.args([
                    "-y",
                    "-i",
                    &temp_input_str,
                    "-c:v",
                    "libx264",
                    "-pix_fmt",
                    "yuv420p",
                    "-preset",
                    "ultrafast",
                    "-crf",
                    "22",
                    "-c:a",
                    "aac",
                    "-movflags",
                    "+faststart",
                    &file_path,
                ])
                .status()
            } else {
                // Nếu nguồn đã là MP4 (fragmented MP4), remux nhanh với +faststart (chỉ mất ~50ms)
                ffmpeg_cmd.args([
                    "-y",
                    "-i",
                    &temp_input_str,
                    "-c",
                    "copy",
                    "-movflags",
                    "+faststart",
                    &file_path,
                ])
                .status()
            };

            // Dọn dẹp file tạm
            let _ = std::fs::remove_file(&temp_input);

            if let Ok(status) = ffmpeg_status {
                if status.success() && target_path.exists() {
                    return Ok(file_path);
                }
            }
        }
    }

    // Dự phòng: ghi trực tiếp dữ liệu thô nếu không có ffmpeg hoặc remux thất bại
    let mut file = std::fs::File::create(&target_path)
        .map_err(|e| AppError::FileIO(format!("Failed to create recording file: {}", e)))?;

    file.write_all(&bytes)
        .map_err(|e| AppError::FileIO(format!("Failed to write recording data: {}", e)))?;

    Ok(file_path)
}

/// Mở thư mục Explorer trên Windows và chọn sẵn (highlight) file vừa lưu
#[tauri::command]
pub fn open_in_explorer(file_path: String) -> AppResult<()> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let clean_path = file_path.replace('/', "\\");
        let _ = Command::new("explorer")
            .args(["/select,", &clean_path])
            .spawn();
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = file_path;
    }
    Ok(())
}
