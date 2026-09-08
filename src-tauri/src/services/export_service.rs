use std::fs;
use std::path::Path;
use rusqlite::Connection;
use base64::Engine;
use crate::error::{AppError, AppResult};
use crate::db::{session_repo, project_repo, capture_repo, issue_repo};

fn file_to_base64(base_dir: &Path, rel_path: &str) -> String {
    if rel_path.is_empty() {
        return "".to_string();
    }
    let abs_path = base_dir.join(rel_path);
    if abs_path.exists() {
        if let Ok(bytes) = fs::read(&abs_path) {
            let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
            return format!("data:image/png;base64,{}", encoded);
        }
    }
    "".to_string()
}

fn html_escape(input: &str) -> String {
    let mut escaped = String::new();
    for c in input.chars() {
        match c {
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#x27;"),
            _ => escaped.push(c),
        }
    }
    escaped
}

fn csv_escape(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') || field.contains('\r') {
        let escaped = field.replace('"', "\"\"");
        format!("\"{}\"", escaped)
    } else {
        field.to_string()
    }
}

pub fn export_session_html(
    conn: &Connection,
    base_dir: &Path,
    session_id: &str,
    output_path: &Path,
    severities: &[String],
    statuses: &[String],
) -> AppResult<()> {
    let session = session_repo::get_session(conn, session_id)?;
    let project = project_repo::get_project(conn, &session.project_id)?;
    let captures = capture_repo::get_captures_by_session(conn, session_id)?;
    let mut issues = issue_repo::get_by_session(conn, session_id)?;

    if !severities.is_empty() {
        issues.retain(|i| severities.contains(&i.severity));
    }
    if !statuses.is_empty() {
        issues.retain(|i| statuses.contains(&i.status));
    }

    // Statistics
    let total_issues = issues.len();

    // Generate HTML template
    let mut html = String::new();
    html.push_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>"#);
    html.push_str(&html_escape(&session.title));
    html.push_str(r#" - Review Report</title>
    <style>
        :root {
            --bg-color: #121214;
            --surface-color: #1a1a1e;
            --surface-card: #222228;
            --border-color: #2e2e38;
            --text-primary: #ffffff;
            --text-secondary: #a0a0ab;
            --primary-color: #ff6b35;
            
            --critical-color: #ff5252;
            --major-color: #ffb300;
            --minor-color: #4caf50;
            --info-color: #2196f3;
        }
        
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            background-color: var(--bg-color);
            color: var(--text-primary);
            line-height: 1.6;
            padding: 2rem 1rem;
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
        }
        
        header {
            background-color: var(--surface-color);
            border: 1px solid var(--border-color);
            border-radius: 12px;
            padding: 2rem;
            margin-bottom: 2rem;
            box-shadow: 0 4px 20px rgba(0,0,0,0.3);
        }
        
        .project-badge {
            display: inline-block;
            background-color: var(--primary-color);
            color: white;
            font-size: 0.8rem;
            font-weight: 700;
            padding: 0.25rem 0.75rem;
            border-radius: 9999px;
            text-transform: uppercase;
            margin-bottom: 0.75rem;
        }
        
        h1 {
            font-size: 2.25rem;
            font-weight: 800;
            margin-bottom: 0.5rem;
            background: linear-gradient(135deg, #fff, #a0a0ab);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }
        
        .session-desc {
            color: var(--text-secondary);
            font-size: 1.1rem;
            margin-bottom: 1.5rem;
        }
        
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            margin-top: 1.5rem;
        }
        
        .stat-card {
            background-color: var(--surface-card);
            border: 1px solid var(--border-color);
            border-radius: 8px;
            padding: 1rem;
            text-align: center;
        }
        
        .stat-val {
            font-size: 1.75rem;
            font-weight: 700;
            color: var(--primary-color);
        }
        
        .stat-label {
            font-size: 0.85rem;
            color: var(--text-secondary);
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }
        
        .capture-block {
            background-color: var(--surface-color);
            border: 1px solid var(--border-color);
            border-radius: 12px;
            padding: 2rem;
            margin-bottom: 2.5rem;
            box-shadow: 0 4px 20px rgba(0,0,0,0.2);
        }
        
        .capture-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 1.5rem;
            border-bottom: 1px solid var(--border-color);
            padding-bottom: 0.75rem;
        }
        
        .capture-title {
            font-size: 1.35rem;
            font-weight: 700;
        }
        
        .capture-time {
            color: var(--text-secondary);
            font-size: 0.9rem;
        }
        
        .image-container {
            width: 100%;
            margin-bottom: 1.5rem;
            border-radius: 8px;
            overflow: hidden;
            border: 1px solid var(--border-color);
            background-color: #000;
        }
        
        .screenshot-img {
            width: 100%;
            height: auto;
            display: block;
        }
        
        .issues-title {
            font-size: 1.2rem;
            font-weight: 700;
            margin-bottom: 1rem;
            color: var(--primary-color);
        }
        
        .issues-list {
            display: flex;
            flex-direction: column;
            gap: 1.25rem;
        }
        
        .issue-card {
            background-color: var(--surface-card);
            border: 1px solid var(--border-color);
            border-radius: 8px;
            padding: 1.25rem;
            display: flex;
            gap: 1.25rem;
        }
        
        @media (max-width: 768px) {
            .issue-card {
                flex-direction: column;
            }
        }
        
        .crop-container {
            flex-shrink: 0;
            width: 120px;
            height: 120px;
            border-radius: 6px;
            overflow: hidden;
            border: 1px solid var(--border-color);
            background-color: #000;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        
        .crop-img {
            width: 100%;
            height: 100%;
            object-fit: cover;
        }
        
        .issue-details {
            flex-grow: 1;
        }
        
        .issue-top {
            display: flex;
            align-items: center;
            gap: 0.75rem;
            flex-wrap: wrap;
            margin-bottom: 0.5rem;
        }
        
        .marker-number {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            width: 24px;
            height: 24px;
            border-radius: 50%;
            background-color: var(--primary-color);
            color: white;
            font-size: 0.85rem;
            font-weight: 700;
        }
        
        .issue-title {
            font-size: 1.1rem;
            font-weight: 700;
        }
        
        .badge {
            font-size: 0.75rem;
            font-weight: 700;
            padding: 0.15rem 0.5rem;
            border-radius: 4px;
            text-transform: uppercase;
        }
        
        .badge-type {
            background-color: rgba(255, 255, 255, 0.15);
            color: #ffffff;
        }
        
        .badge-severity-critical {
            background-color: rgba(255, 82, 82, 0.2);
            color: var(--critical-color);
            border: 1px solid rgba(255, 82, 82, 0.4);
        }
        
        .badge-severity-major {
            background-color: rgba(255, 179, 0, 0.2);
            color: var(--major-color);
            border: 1px solid rgba(255, 179, 0, 0.4);
        }
        
        .badge-severity-minor {
            background-color: rgba(76, 175, 80, 0.2);
            color: var(--minor-color);
            border: 1px solid rgba(76, 175, 80, 0.4);
        }
        
        .badge-severity-info {
            background-color: rgba(33, 150, 243, 0.2);
            color: var(--info-color);
            border: 1px solid rgba(33, 150, 243, 0.4);
        }
        
        .badge-status {
            background-color: rgba(255, 255, 255, 0.08);
            color: var(--text-secondary);
        }
        
        .issue-desc {
            color: var(--text-secondary);
            font-size: 0.95rem;
            margin-bottom: 0.75rem;
            white-space: pre-wrap;
        }
        
        .tag-list {
            display: flex;
            gap: 0.4rem;
            flex-wrap: wrap;
        }
        
        .tag {
            font-size: 0.75rem;
            background-color: rgba(78, 205, 196, 0.15);
            color: #4ecdc4;
            padding: 0.1rem 0.4rem;
            border-radius: 4px;
        }
        
        .no-issues {
            color: var(--text-secondary);
            font-style: italic;
            padding: 1rem 0;
        }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <span class="project-badge">"#);
    html.push_str(&html_escape(&project.name));
    html.push_str(r#"</span>
            <h1>"#);
    html.push_str(&html_escape(&session.title));
    html.push_str(r#"</h1>
            <p class="session-desc">"#);
    html.push_str(&html_escape(&session.description));
    html.push_str(r#"</p>
            
            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-val">"#);
    html.push_str(&total_issues.to_string());
    html.push_str(r#"</div>
                    <div class="stat-label">Tổng số bước / Total Steps</div>
                </div>
                <div class="stat-card">
                    <div class="stat-val" style="color: var(--primary-color);">"#);
    html.push_str(&captures.len().to_string());
    html.push_str(r#"</div>
                    <div class="stat-label">Ảnh chụp màn hình / Screenshots</div>
                </div>
            </div>
        </header>

        <main>
"#);

    // Loop through each capture
    for (cap_index, capture) in captures.iter().enumerate() {
        html.push_str("            <section class=\"capture-block\">\n");
        html.push_str("                <div class=\"capture-header\">\n");
        html.push_str("                    <h2 class=\"capture-title\">Capture #");
        html.push_str(&(cap_index + 1).to_string());
        if let Some(ref title) = capture.window_title {
            if !title.is_empty() {
                html.push_str(" - ");
                html.push_str(&html_escape(title));
            }
        }
        html.push_str("</h2>\n");
        html.push_str("                    <span class=\"capture-time\">");
        html.push_str(&capture.created_at);
        html.push_str("</span>\n");
        html.push_str("                </div>\n");

        // Try reading annotated screenshot, if it doesn't exist try original
        let annotated_rel = capture.screenshot_path.replace(".png", "_annotated.png");
        let mut base64_image = file_to_base64(base_dir, &annotated_rel);
        if base64_image.is_empty() {
            base64_image = file_to_base64(base_dir, &capture.screenshot_path);
        }

        if !base64_image.is_empty() {
            html.push_str("                <div class=\"image-container\">\n");
            html.push_str("                    <img class=\"screenshot-img\" src=\"");
            html.push_str(&base64_image);
            html.push_str("\" alt=\"Capture Screenshot\">\n");
            html.push_str("                </div>\n");
        }

        html.push_str("                <h3 class=\"issues-title\">Annotations</h3>\n");
        html.push_str("                <div class=\"issues-list\">\n");

        // Find issues belonging to this capture
        let capture_issues: Vec<&crate::models::Issue> = issues
            .iter()
            .filter(|iss| iss.capture_id == capture.id)
            .collect();

        if capture_issues.is_empty() {
            html.push_str("                    <p class=\"no-issues\">No annotations on this capture.</p>\n");
        } else {
            for issue in capture_issues {
                html.push_str("                    <div class=\"issue-card\">\n");

                // Crop image
                let crop_base64 = issue.crop_path.as_ref()
                    .map(|cp| file_to_base64(base_dir, cp))
                    .unwrap_or_default();

                if !crop_base64.is_empty() {
                    html.push_str("                        <div class=\"crop-container\">\n");
                    html.push_str("                            <img class=\"crop-img\" src=\"");
                    html.push_str(&crop_base64);
                    html.push_str("\" alt=\"Issue Crop\">\n");
                    html.push_str("                        </div>\n");
                }

                html.push_str("                        <div class=\"issue-details\">\n");
                html.push_str("                            <div class=\"issue-top\">\n");
                html.push_str("                                <span class=\"marker-number\">");
                html.push_str(&issue.marker_number.to_string());
                html.push_str("</span>\n");
                html.push_str("                                <span class=\"issue-title\">");
                html.push_str(&html_escape(&issue.title));
                html.push_str("</span>\n");

                // Badges
                html.push_str("                                <span class=\"badge badge-type\">");
                html.push_str(&html_escape(&issue.issue_type));
                html.push_str("</span>\n");

                let severity_class = match issue.severity.as_str() {
                    "Critical" => "badge-severity-critical",
                    "Major" => "badge-severity-major",
                    "Minor" => "badge-severity-minor",
                    _ => "badge-severity-info",
                };
                html.push_str("                                <span class=\"badge ");
                html.push_str(severity_class);
                html.push_str("\">");
                html.push_str(&html_escape(&issue.severity));
                html.push_str("</span>\n");

                html.push_str("                                <span class=\"badge badge-status\">");
                html.push_str(&html_escape(&issue.status));
                html.push_str("</span>\n");
                html.push_str("                            </div>\n");

                html.push_str("                            <p class=\"issue-desc\">");
                html.push_str(&html_escape(&issue.description));
                html.push_str("</p>\n");

                // Tags
                if let Some(ref tags) = issue.tags {
                    if !tags.is_empty() {
                        html.push_str("                            <div class=\"tag-list\">\n");
                        for tag in tags {
                            html.push_str("                                <span class=\"tag\">#");
                            html.push_str(&html_escape(&tag.name));
                            html.push_str("</span>\n");
                        }
                        html.push_str("                            </div>\n");
                    }
                }

                html.push_str("                        </div>\n");
                html.push_str("                    </div>\n");
            }
        }

        html.push_str("                </div>\n");
        html.push_str("            </section>\n");
    }

    html.push_str(r#"        </main>
    </div>
</body>
</html>
"#);

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            AppError::FileIO(format!("Failed to create parent directories for export: {}", e))
        })?;
    }

    fs::write(output_path, html).map_err(|e| {
        AppError::FileIO(format!("Failed to write HTML report to {:?}: {}", output_path, e))
    })?;

    Ok(())
}

pub fn export_session_markdown(
    conn: &Connection,
    base_dir: &Path,
    session_id: &str,
    output_path: &Path,
    severities: &[String],
    statuses: &[String],
) -> AppResult<()> {
    let session = session_repo::get_session(conn, session_id)?;
    let project = project_repo::get_project(conn, &session.project_id)?;
    let mut issues = issue_repo::get_by_session(conn, session_id)?;

    if !severities.is_empty() {
        issues.retain(|i| severities.contains(&i.severity));
    }
    if !statuses.is_empty() {
        issues.retain(|i| statuses.contains(&i.status));
    }

    let report_name = output_path.file_stem().and_then(|s| s.to_str()).unwrap_or("report");
    let images_dir_name = format!("{}_images", report_name);
    
    let parent_dir = output_path.parent().ok_or_else(|| {
        AppError::FileIO("Invalid output path".to_string())
    })?;
    let target_images_dir = parent_dir.join(&images_dir_name);

    if !issues.is_empty() {
        fs::create_dir_all(&target_images_dir).map_err(|e| AppError::FileIO(e.to_string()))?;
    }

    let mut md = String::new();
    md.push_str(&format!("# {}\n\n", session.title));
    md.push_str(&format!("- **Dự án / Project:** {}\n", project.name));
    md.push_str(&format!("- **Trạng thái / Status:** {}\n", session.status));
    if !session.description.is_empty() {
        md.push_str(&format!("- **Mô tả / Description:** {}\n\n", session.description));
    } else {
        md.push_str("\n");
    }

    md.push_str("## Danh sách các bước hướng dẫn (Guide Steps)\n\n");
    md.push_str("| Bước # | Tiêu đề | Phân loại | Hướng dẫn thao tác | Hình ảnh |\n");
    md.push_str("|--------|---------|-----------|--------------------|----------|\n");

    for issue in &issues {
        let mut rel_image_path = String::new();
        if let Some(ref crop_rel_path) = issue.crop_path {
            let src_crop_path = base_dir.join(crop_rel_path);
            if src_crop_path.exists() {
                if let Some(file_name) = src_crop_path.file_name() {
                    let target_crop_path = target_images_dir.join(file_name);
                    if let Err(e) = fs::copy(&src_crop_path, &target_crop_path) {
                        log::error!("Failed to copy crop image: {}", e);
                    } else {
                        rel_image_path = format!("{}/{}", images_dir_name, file_name.to_string_lossy());
                    }
                }
            }
        }

        let img_markdown = if !rel_image_path.is_empty() {
            format!("![Crop]({})", rel_image_path)
        } else {
            "No Image".to_string()
        };

        let title_esc = issue.title.replace('|', "\\|");
        let desc_esc = issue.description.replace('|', "\\|").replace('\n', "<br>");

        md.push_str(&format!(
            "| Bước {} | {} | {} | {} | {} |\n",
            issue.marker_number,
            title_esc,
            issue.issue_type,
            desc_esc,
            img_markdown
        ));
    }

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| AppError::FileIO(e.to_string()))?;
    }
    fs::write(output_path, md).map_err(|e| AppError::FileIO(e.to_string()))?;

    Ok(())
}

pub fn export_session_csv(
    conn: &Connection,
    session_id: &str,
    output_path: &Path,
    severities: &[String],
    statuses: &[String],
) -> AppResult<()> {
    let mut issues = issue_repo::get_by_session(conn, session_id)?;
    if !severities.is_empty() {
        issues.retain(|i| severities.contains(&i.severity));
    }
    if !statuses.is_empty() {
        issues.retain(|i| statuses.contains(&i.status));
    }

    let mut csv_content = String::new();
    csv_content.push_str("ID,Title,Severity,Type,Status,Description,ScreenshotPath\n");

    for issue in issues {
        let row = format!(
            "{},{},{},{},{},{},{}\n",
            csv_escape(&issue.id),
            csv_escape(&issue.title),
            csv_escape(&issue.severity),
            csv_escape(&issue.issue_type),
            csv_escape(&issue.status),
            csv_escape(&issue.description),
            csv_escape(&issue.crop_path.unwrap_or_default())
        );
        csv_content.push_str(&row);
    }

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| AppError::FileIO(e.to_string()))?;
    }
    fs::write(output_path, csv_content).map_err(|e| AppError::FileIO(e.to_string()))?;
    Ok(())
}

use std::path::PathBuf;

fn find_edge_path() -> PathBuf {
    let paths = [
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
    ];
    for p in &paths {
        let path = Path::new(p);
        if path.exists() {
            return path.to_path_buf();
        }
    }
    PathBuf::from("msedge")
}

pub fn export_session_pdf(
    conn: &Connection,
    base_dir: &Path,
    session_id: &str,
    output_path: &Path,
    severities: &[String],
    statuses: &[String],
) -> AppResult<()> {
    let temp_html_path = output_path.with_extension("tmp.html");
    export_session_html(conn, base_dir, session_id, &temp_html_path, severities, statuses)?;
    
    let edge_exe = find_edge_path();
    let file_url = format!("file:///{}", temp_html_path.to_string_lossy().replace('\\', "/"));
    
    let status = std::process::Command::new(edge_exe)
        .arg("--headless")
        .arg("--disable-gpu")
        .arg(format!("--print-to-pdf={}", output_path.to_string_lossy()))
        .arg(&file_url)
        .status();
        
    let _ = fs::remove_file(&temp_html_path);
    
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(_) => Err(AppError::FileIO("Microsoft Edge failed to print PDF".to_string())),
        Err(e) => Err(AppError::FileIO(format!("Failed to start Microsoft Edge for PDF generation: {}", e))),
    }
}
