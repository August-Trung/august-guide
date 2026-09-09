import { invoke } from '@tauri-apps/api/core'
import { Project, CreateProjectPayload, UpdateProjectPayload } from '@/types/project'
import { Session, CreateSessionPayload, UpdateSessionPayload } from '@/types/session'
import { Capture } from '@/types/capture'

/**
 * Tauri IPC command wrappers for August Mark.
 * Maps frontend calls directly to Rust backend command handlers.
 */

// ============================================================================
// Project Commands
// ============================================================================

/**
 * Retrieve all projects, optionally including archived ones.
 */
export async function getProjects(includeArchived?: boolean): Promise<Project[]> {
  return invoke<Project[]>('get_projects', { includeArchived })
}

/**
 * Retrieve a single project by its ID.
 */
export async function getProject(id: string): Promise<Project> {
  return invoke<Project>('get_project', { id })
}

/**
 * Create a new project.
 */
export async function createProject(payload: CreateProjectPayload): Promise<Project> {
  return invoke<Project>('create_project', { payload })
}

/**
 * Update an existing project's metadata or status.
 */
export async function updateProject(id: string, payload: UpdateProjectPayload): Promise<Project> {
  return invoke<Project>('update_project', { id, payload })
}

/**
 * Delete a project and all associated sessions, captures, and issues.
 */
export async function deleteProject(id: string): Promise<void> {
  return invoke<void>('delete_project', { id })
}

// ============================================================================
// Session Commands
// ============================================================================

/**
 * Retrieve all sessions.
 */
export async function getSessions(): Promise<Session[]> {
  return invoke<Session[]>('get_sessions')
}

/**
 * Retrieve all sessions associated with a specific project ID.
 */
export async function getSessionsByProject(projectId: string): Promise<Session[]> {
  return invoke<Session[]>('get_sessions_by_project', { projectId })
}

/**
 * Retrieve a single session by its ID.
 */
export async function getSession(id: string): Promise<Session> {
  return invoke<Session>('get_session', { id })
}

/**
 * Create a new session.
 */
export async function createSession(payload: CreateSessionPayload): Promise<Session> {
  return invoke<Session>('create_session', { payload })
}

/**
 * Update an existing session's metadata or status.
 */
export async function updateSession(id: string, payload: UpdateSessionPayload): Promise<Session> {
  return invoke<Session>('update_session', { id, payload })
}

/**
 * Delete a session. Downstream captures, issues, and files will be automatically cascade cleaned.
 */
export async function deleteSession(id: string): Promise<void> {
  return invoke<void>('delete_session', { id })
}

// ============================================================================
// Capture Commands
// ============================================================================

export interface CaptureResult {
  captureId: string
  screenshotPath: string
  monitorInfo: any
}

/**
 * Chụp ảnh màn hình cho một session.
 */
export async function triggerCapture(sessionId: string): Promise<CaptureResult> {
  return invoke<CaptureResult>('trigger_capture', { sessionId })
}

/**
 * Mở cửa sổ overlay chứa ảnh chụp màn hình.
 */
export async function openOverlay(captureId: string, screenshotPath?: string): Promise<void> {
  return invoke<void>('open_overlay', { captureId, screenshotPath })
}

/**
 * Hiện cửa sổ overlay sau khi đã load xong content (tránh white flash trên Windows).
 */
export async function showOverlay(): Promise<void> {
  return invoke<void>('show_overlay')
}

/**
 * Đóng cửa sổ overlay.
 */
export async function closeOverlay(): Promise<void> {
  return invoke<void>('close_overlay')
}

/**
 * Hủy lượt chụp hiện tại (xóa file và DB).
 */
export async function cancelCapture(captureId: string): Promise<void> {
  return invoke<void>('cancel_capture', { captureId })
}

/**
 * Lấy thông tin chi tiết một Capture.
 */
export async function getCapture(id: string): Promise<Capture> {
  return invoke<Capture>('get_capture', { id })
}

/**
 * Lấy danh sách tất cả các Capture trong một Session.
 */
export async function getCapturesBySession(sessionId: string): Promise<Capture[]> {
  return invoke<Capture[]>('get_captures_by_session', { sessionId })
}

/**
 * Mở thư mục lưu ảnh của ứng dụng trong File Explorer.
 */
export async function openAppFolder(folderType?: string): Promise<void> {
  return invoke<void>('open_app_folder', { folderType })
}

// ============================================================================
// Issue Commands
// ============================================================================
import { Issue, CreateIssuePayload, UpdateIssuePayload, Tag } from '@/types/issue'

export async function saveCaptureAnnotations(
  captureId: string,
  issues: CreateIssuePayload[],
  annotatedImageBase64?: string
): Promise<void> {
  return invoke<void>('save_capture_annotations', {
    captureId,
    capture_id: captureId,
    issues,
    annotatedImageBase64,
    annotated_image_base64: annotatedImageBase64
  })
}

export async function getIssues(sessionId: string): Promise<Issue[]> {
  return invoke<Issue[]>('get_issues', { sessionId })
}

export async function getIssue(id: string): Promise<Issue> {
  return invoke<Issue>('get_issue', { id })
}

export async function updateIssue(id: string, payload: UpdateIssuePayload): Promise<Issue> {
  return invoke<Issue>('update_issue', { id, payload })
}

export async function deleteIssue(id: string): Promise<void> {
  return invoke<void>('delete_issue', { id })
}

// ============================================================================
// Export Commands
// ============================================================================
export async function exportSession(
  sessionId: string,
  outputPath: string,
  format: string,
  severities: string[],
  statuses: string[]
): Promise<void> {
  return invoke<void>('export_session', { sessionId, outputPath, format, severities, statuses })
}

// ============================================================================
// Settings Commands
// ============================================================================

export async function getAllSettings(): Promise<Record<string, string>> {
  return invoke<Record<string, string>>('get_all_settings')
}

export async function getSetting(key: string): Promise<string | null> {
  return invoke<string | null>('get_setting', { key })
}

export async function updateSetting(key: string, value: string): Promise<void> {
  return invoke<void>('update_setting', { key, value })
}

// ============================================================================
// Tag Commands
// ============================================================================

export async function getAllTags(): Promise<Tag[]> {
  return invoke<Tag[]>('get_all_tags')
}

export async function createTag(name: string, color: string): Promise<Tag> {
  return invoke<Tag>('create_tag', { name, color })
}

export async function associateTagWithIssue(issueId: string, tagId: string): Promise<void> {
  return invoke<void>('associate_tag_with_issue', { issueId, tagId })
}

export async function getTagsByIssue(issueId: string): Promise<Tag[]> {
  return invoke<Tag[]>('get_tags_by_issue', { issueId })
}

export async function clearIssueTags(issueId: string): Promise<void> {
  return invoke<void>('clear_issue_tags', { issueId })
}

// ============================================================================
// Search Commands
// ============================================================================

export interface SearchResult {
  sessions: Session[]
  issues: Issue[]
}

export async function searchAll(query: string): Promise<SearchResult> {
  return invoke<SearchResult>('search_all', { query })
}

// ============================================================================
// Google Drive Commands
// ============================================================================

export async function connectGdrive(): Promise<string> {
  return invoke<string>('connect_gdrive')
}

export async function disconnectGdrive(): Promise<void> {
  return invoke<void>('disconnect_gdrive')
}

export async function checkGdriveStatus(): Promise<string | null> {
  return invoke<string | null>('check_gdrive_status')
}

export async function backupToGdrive(): Promise<string> {
  return invoke<string>('backup_to_gdrive')
}

export async function restoreFromGdrive(fileId: string): Promise<void> {
  return invoke<void>('restore_from_gdrive', { fileId })
}

export async function listBackupsOnGdrive(): Promise<[string, string][]> {
  return invoke<[string, string][]>('list_backups_on_gdrive')
}

export async function shareSessionOnGdrive(sessionId: string): Promise<string> {
  return invoke<string>('share_session_on_gdrive', { sessionId })
}

// ============================================================================
// AI Agent Context Pack Exporter Commands
// ============================================================================

export async function exportAacpPack(
  sessionId: string,
  issueId: string,
  workspacePath: string,
  suspectedFiles: string[],
  outputDir: string,
  compressZip: boolean
): Promise<string> {
  return invoke<string>('export_aacp_pack', {
    sessionId,
    issueId,
    workspacePath,
    suspectedFiles,
    outputDir,
    compressZip
  })
}

export async function getDownloadDir(): Promise<string> {
  return invoke<string>('get_download_dir')
}

export async function uninstallApp(): Promise<void> {
  return invoke<void>('uninstall_app')
}


