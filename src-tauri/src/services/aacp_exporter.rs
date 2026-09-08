use std::fs;
use std::collections::{HashSet, HashMap};
use std::path::Path;
use crate::error::{AppError, AppResult};
use crate::models::{Issue, AacpManifest, IssueInfo, CodebaseInfo, CropInfo, Visuals};
use crate::services::git_inspector::inspect_git;
use rusqlite::Connection;

pub fn generate_instruction_md(
    issues: &[Issue],
    git: &crate::services::git_inspector::GitContext,
    suspected_files: &[String],
    screenshot_filenames: &[String],
    crop_filenames: &[String],
) -> String {
    let mut issues_section = String::new();
    for issue in issues {
        let tags_str = if let Some(ref tags) = issue.tags {
            tags.iter().map(|t| t.name.as_str()).collect::<Vec<_>>().join(", ")
        } else {
            "None".to_string()
        };
        issues_section.push_str(&format!(
r#"### Issue #{}: {}
- **Type**: {}
- **Severity**: {}
- **Status**: {}
- **Description**: {}
- **Tags**: {}

"#,
            issue.marker_number,
            issue.title,
            issue.issue_type,
            issue.severity,
            issue.status,
            issue.description,
            tags_str
        ));
    }

    let suspected_files_str = if suspected_files.is_empty() {
        "- *No suspected files listed. Please locate the relevant code.*".to_string()
    } else {
        suspected_files.iter().map(|f| format!("- `{}`", f)).collect::<Vec<_>>().join("\n")
    };

    let git_branch = git.branch.as_deref().unwrap_or("Unknown");
    let git_commit = git.commit_hash.as_deref().unwrap_or("Unknown");
    let git_status = git.status.as_deref().unwrap_or("No uncommitted changes / Unknown");

    let mut screenshots_list = String::new();
    for scr in screenshot_filenames {
        screenshots_list.push_str(&format!("- **Screenshot**: `visuals/{}`\n", scr));
    }

    let mut crops_list = String::new();
    for crop in crop_filenames {
        crops_list.push_str(&format!("- **Zoomed Region Crop**: `visuals/{}`\n", crop));
    }

    let mut md = format!(
r#"# August Mark — AI Agent Bug Fix Instruction

The following issue(s) have been annotated and exported from August Mark. As an AI coding agent, your goal is to locate, analyze, and fix these bugs in the codebase.

## 1. Issue Details
{}
## 2. Codebase Context
- **Target Workspace Branch**: `{}`
- **Target Workspace Commit**: `{}`
- **Git Status (Uncommitted changes)**:
```
{}
```

### Suspected Files
The developer has identified the following suspected files that may contain the bug or are highly relevant:
{}

## 3. Visual Context
Please refer to the following visuals located in this context directory:
{}
{}
"#,
        issues_section,
        git_branch,
        git_commit,
        git_status,
        suspected_files_str,
        screenshots_list,
        crops_list
    );

    md.push_str(
        r#"
## 4. Action Instructions for AI Agent

1. **Locate Code**: Look at the "Suspected Files" first. If they exist, read their contents. If not, search the workspace for code symbols, texts, or layouts that match the bug description or visual layout.
2. **Analyze Bug**:
   - Compare the description of the bugs with the visual clues in `visuals/`.
   - Inspect the code to find logic errors, style issues, wrong layouts, or missing conditions.
3. **Implement Fix**: Modify the codebase directly to fix the bugs. Make sure to follow the workspace's code styles, existing architecture, and clean coding practices.
4. **Verify**: Ensure code compiles correctly. Proactively check if there are test scripts or verification commands available.
"#
    );

    md
}

pub fn export_aacp_pack(
    conn: &Connection,
    app_data_dir: &Path,
    session_id: &str,
    issue_id: &str,
    workspace_path: &str,
    suspected_files: Vec<String>,
    output_dir: &str,
    compress_zip: bool,
) -> AppResult<String> {
    // 1. Fetch issues list from db
    let mut raw_issues = if issue_id == "all" {
        crate::db::issue_repo::get_by_session(conn, session_id)?
    } else {
        let issue = crate::db::issue_repo::get_issue(conn, issue_id)?;
        vec![issue]
    };

    if raw_issues.is_empty() {
        return Err(AppError::Validation("No issues found to export".to_string()));
    }

    // Load tags for each issue
    for issue in &mut raw_issues {
        let tags = crate::db::issue_repo::get_tags_for_issue(conn, &issue.id)?;
        issue.tags = Some(tags);
    }

    // 2. Fetch git context
    let git_ctx = inspect_git(workspace_path);

    // 3. Determine output directory paths
    let safe_title = if raw_issues.len() == 1 {
        raw_issues[0].title.chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect::<String>()
            .to_lowercase()
    } else {
        "session_all_issues".to_string()
    };
    
    let export_dir_name = if raw_issues.len() == 1 {
        format!("august_aacp_issue_{}_{}", raw_issues[0].marker_number, safe_title)
    } else {
        format!("august_aacp_session_{}", safe_title)
    };
    
    let out_dir_path = Path::new(output_dir);
    if !out_dir_path.exists() {
        fs::create_dir_all(out_dir_path)?;
    }

    // Temp folder path inside app_data_dir/temp
    let temp_parent = app_data_dir.join("temp");
    if !temp_parent.exists() {
        fs::create_dir_all(&temp_parent)?;
    }
    let temp_dir = temp_parent.join(format!("aacp_temp_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir)?;

    let visuals_dir = temp_dir.join("visuals");
    fs::create_dir_all(&visuals_dir)?;

    // 4. Copy screenshots and crops
    let mut copied_screenshots = HashSet::new();
    let mut screenshot_filenames = Vec::new();
    let mut crop_filenames = Vec::new();
    let mut manifest_crops = Vec::new();
    
    // Map of CaptureID -> DestFileName
    let mut capture_file_mapping = HashMap::new();

    for issue in &raw_issues {
        // Fetch capture for screenshot_path
        let capture = crate::db::capture_repo::get_capture_by_id(conn, &issue.capture_id)?;
        
        // Copy unique screenshot
        if !copied_screenshots.contains(&capture.id) {
            let src_screenshot = app_data_dir.join(&capture.screenshot_path);
            if src_screenshot.exists() {
                let screenshot_ext = src_screenshot.extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("png");
                
                let dest_screenshot_name = if raw_issues.len() == 1 {
                    format!("full_screenshot.{}", screenshot_ext)
                } else {
                    format!("screenshot_{}.{}", capture.id, screenshot_ext)
                };
                
                let dest_screenshot = visuals_dir.join(&dest_screenshot_name);
                fs::copy(&src_screenshot, &dest_screenshot)?;
                
                copied_screenshots.insert(capture.id.clone());
                screenshot_filenames.push(dest_screenshot_name.clone());
                capture_file_mapping.insert(capture.id.clone(), dest_screenshot_name);
            } else {
                return Err(AppError::FileIO(format!("Screenshot file not found: {:?}", src_screenshot)));
            }
        }

        // Copy crop
        if let Some(ref crop_path) = issue.crop_path {
            let src_crop = app_data_dir.join(crop_path);
            if src_crop.exists() {
                let crop_ext = src_crop.extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("png");
                
                let dest_crop_name = format!("crop_{}.{}", issue.marker_number, crop_ext);
                let dest_crop = visuals_dir.join(&dest_crop_name);
                fs::copy(&src_crop, &dest_crop)?;

                crop_filenames.push(dest_crop_name.clone());
                manifest_crops.push(CropInfo {
                    filename: dest_crop_name,
                    marker_number: issue.marker_number,
                    x: issue.marker_x,
                    y: issue.marker_y,
                });
            }
        }
    }

    // 5. Write issue_manifest.json
    let mut manifest_issues = Vec::new();
    for issue in &raw_issues {
        let tags_list = issue.tags.as_ref()
            .map(|list| list.iter().map(|t| t.name.clone()).collect())
            .unwrap_or_else(Vec::new);

        manifest_issues.push(IssueInfo {
            id: issue.id.clone(),
            title: issue.title.clone(),
            description: issue.description.clone(),
            issue_type: issue.issue_type.clone(),
            severity: issue.severity.clone(),
            status: issue.status.clone(),
            tags: tags_list,
        });
    }

    let manifest = AacpManifest {
        issues: manifest_issues,
        codebase: CodebaseInfo {
            local_path: workspace_path.to_string(),
            git_commit: git_ctx.commit_hash.clone(),
            git_branch: git_ctx.branch.clone(),
            git_status: git_ctx.status.clone(),
            suspected_files: suspected_files.clone(),
        },
        visuals: Visuals {
            full_screenshots: screenshot_filenames.clone(),
            crops: manifest_crops,
        },
    };

    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| AppError::FileIO(e.to_string()))?;
    fs::write(temp_dir.join("issue_manifest.json"), manifest_json)?;

    // 6. Write agent_instruction.md
    let instruction_md = generate_instruction_md(
        &raw_issues,
        &git_ctx,
        &suspected_files,
        &screenshot_filenames,
        &crop_filenames,
    );
    fs::write(temp_dir.join("agent_instruction.md"), instruction_md)?;

    // 7. Output
    let final_dest_path = if compress_zip {
        // Zip the temp_dir to output_dir/export_dir_name.zip
        let zip_file_name = format!("{}.zip", export_dir_name);
        let zip_path = out_dir_path.join(&zip_file_name);
        
        let file = fs::File::create(&zip_path)?;
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        // Recursively add temp_dir files to zip
        let walk_dir = |zip: &mut zip::ZipWriter<fs::File>, root_path: &Path, temp_path: &Path| -> AppResult<()> {
            let mut stack = vec![temp_path.to_path_buf()];
            while let Some(current_path) = stack.pop() {
                for entry in fs::read_dir(current_path)? {
                    let entry = entry?;
                    let path = entry.path();
                    let relative_path = path.strip_prefix(root_path)
                        .map_err(|e| AppError::FileIO(e.to_string()))?;
                    let relative_path_str = relative_path.to_string_lossy().replace('\\', "/");

                    if path.is_dir() {
                        zip.add_directory(relative_path_str.clone(), options)
                            .map_err(|e| AppError::FileIO(e.to_string()))?;
                        stack.push(path);
                    } else {
                        zip.start_file(relative_path_str.clone(), options)
                            .map_err(|e| AppError::FileIO(e.to_string()))?;
                        let mut file_content = fs::File::open(path)?;
                        std::io::copy(&mut file_content, zip)?;
                    }
                }
            }
            Ok(())
        };

        walk_dir(&mut zip, &temp_dir, &temp_dir)?;
        zip.finish().map_err(|e| AppError::FileIO(e.to_string()))?;
        
        // Remove temp dir
        let _ = fs::remove_dir_all(&temp_dir);

        zip_path.to_string_lossy().to_string()
    } else {
        // Copy/Move temp_dir to output_dir/export_dir_name/
        let target_dir = out_dir_path.join(&export_dir_name);
        if target_dir.exists() {
            fs::remove_dir_all(&target_dir)?;
        }
        
        // Copy directory helper
        let copy_dir = |src: &Path, dst: &Path| -> AppResult<()> {
            fs::create_dir_all(dst)?;
            let mut stack = vec![(src.to_path_buf(), dst.to_path_buf())];
            while let Some((curr_src, curr_dst)) = stack.pop() {
                for entry in fs::read_dir(curr_src)? {
                    let entry = entry?;
                    let s_path = entry.path();
                    let d_path = curr_dst.join(entry.file_name());
                    if s_path.is_dir() {
                        fs::create_dir_all(&d_path)?;
                        stack.push((s_path, d_path));
                    } else {
                        fs::copy(&s_path, &d_path)?;
                    }
                }
            }
            Ok(())
        };

        copy_dir(&temp_dir, &target_dir)?;
        let _ = fs::remove_dir_all(&temp_dir);

        target_dir.to_string_lossy().to_string()
    };

    Ok(final_dest_path)
}
