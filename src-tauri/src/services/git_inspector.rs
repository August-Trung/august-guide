use serde::{Serialize, Deserialize};
use std::process::Command;
use std::path::Path;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitContext {
    pub is_repo: bool,
    pub branch: Option<String>,
    pub commit_hash: Option<String>,
    pub status: Option<String>,
}

pub fn inspect_git(workspace_path: &str) -> GitContext {
    let path = Path::new(workspace_path);
    if !path.exists() || !path.is_dir() {
        return GitContext {
            is_repo: false,
            branch: None,
            commit_hash: None,
            status: None,
        };
    }

    // Helper to execute git commands
    let run_git = |args: &[&str]| -> Option<String> {
        let mut cmd = Command::new("git");
        cmd.args(args).current_dir(path);
        
        #[cfg(target_os = "windows")]
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        let output = cmd.output().ok()?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !stdout.is_empty() {
                return Some(stdout);
            }
        }
        None
    };

    // 1. Check if inside work tree
    let is_inside = run_git(&["rev-parse", "--is-inside-work-tree"]);
    if is_inside.as_deref() != Some("true") {
        return GitContext {
            is_repo: false,
            branch: None,
            commit_hash: None,
            status: None,
        };
    }

    // 2. Get current branch
    let branch = run_git(&["rev-parse", "--abbrev-ref", "HEAD"]);

    // 3. Get current commit hash
    let commit_hash = run_git(&["rev-parse", "HEAD"]);

    // 4. Get uncommitted status
    let status = run_git(&["status", "--porcelain"]);

    GitContext {
        is_repo: true,
        branch,
        commit_hash,
        status,
    }
}
