use crate::error::{AppError, AppResult};
use std::io::Write;

/// Lưu dữ liệu video đã record (base64) ra đường dẫn file chỉ định trên máy tính
#[tauri::command]
pub fn save_recording_file(file_path: String, base64_data: String) -> AppResult<String> {
    use base64::engine::general_purpose::STANDARD as BASE64;
    use base64::Engine;

    let clean_base64 = if let Some(idx) = base64_data.find(',') {
        &base64_data[idx + 1..]
    } else {
        &base64_data
    };

    let bytes = BASE64
        .decode(clean_base64.trim())
        .map_err(|e| AppError::Generic(format!("Failed to decode video base64: {}", e)))?;

    let path = std::path::PathBuf::from(&file_path);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::FileIO(format!("Failed to create parent directory: {}", e)))?;
        }
    }

    let mut file = std::fs::File::create(&path)
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
            .arg(format!("/select,{}", clean_path))
            .spawn();
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = file_path;
    }
    Ok(())
}
