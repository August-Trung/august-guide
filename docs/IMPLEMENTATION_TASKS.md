# IMPLEMENTATION_TASKS.md — August Mark v0.1

> Task list cho coding agent. Thực hiện tuần tự. Mỗi task đủ nhỏ để hoàn thành trong 1-4 giờ.

---

## Week 1 — Foundation

**Goal:** Tauri app chạy, SQLite hoạt động, Project/Session CRUD, dashboard skeleton.

---

### T1.01 — Init Tauri + Vue 3 + TypeScript project

**Description:**
Khởi tạo project Tauri 2 với Vue 3 frontend + TypeScript. Dùng `npm create tauri-app@latest`. Chọn template Vue + TypeScript. Xác nhận `npm run tauri dev` chạy được, hiện cửa sổ trống.

**Files to create:**
- `package.json`
- `tsconfig.json`
- `tsconfig.node.json`
- `vite.config.ts`
- `index.html`
- `src/main.ts`
- `src/App.vue`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`
- `src-tauri/src/main.rs`
- `src-tauri/src/lib.rs`
- `.gitignore`

**Files to modify:** None (new project)

**Dependencies:** Node.js, Rust toolchain, Tauri CLI

**Acceptance Criteria:**
- [ ] `npm run tauri dev` compiles without error
- [ ] Cửa sổ desktop mở, hiển thị nội dung Vue
- [ ] Tauri Rust backend compiles successfully
- [ ] Hot reload hoạt động (sửa App.vue → UI cập nhật)

**Manual Testing:**
1. `npm run tauri dev`
2. Thấy cửa sổ với text mặc định
3. Sửa `App.vue` → thêm `<h1>August Mark</h1>` → thấy cập nhật

**Notes:**
- Dùng `npm create tauri-app@latest ./` với flag `--template vue-ts`
- Kiểm tra `tauri.conf.json` có `identifier: "com.august.mark"`
- Set window title: `"August Mark"`

---

### T1.02 — Add Vuetify 3 + Pinia + Vue Router

**Description:**
Cài và cấu hình Vuetify 3, Pinia, Vue Router vào project. Tạo Vuetify theme `augustDark`. Xác nhận Vuetify component render đúng.

**Files to create:**
- `src/plugins/vuetify.ts`
- `src/plugins/pinia.ts`
- `src/router/index.ts`

**Files to modify:**
- `src/main.ts` — register vuetify, pinia, router
- `src/App.vue` — add `<v-app>` wrapper + `<router-view>`
- `package.json` — add dependencies

**Dependencies:** T1.01

**Acceptance Criteria:**
- [ ] `<v-btn>` render đúng Material Design style
- [ ] Dark theme `augustDark` active (background #0F1117)
- [ ] Pinia store tạo được, reactive
- [ ] Router navigate giữa 2 routes (/ và /about test)

**Manual Testing:**
1. `npm run tauri dev`
2. Thấy app với dark background (#0F1117)
3. Thấy Vuetify button render đúng
4. Console không có error

**Notes:**
- Vuetify 3 theme colors theo ARCHITECTURE.md section 2.4
- primary: `#FF6B35`, secondary: `#4ECDC4`
- Router chỉ cần route `/` cho giờ, thêm routes sau

---

### T1.03 — Create TypeScript types

**Description:**
Tạo tất cả TypeScript interfaces/types/enums dùng chung trong frontend. Dựa trên ARCHITECTURE.md section 19.11.

**Files to create:**
- `src/types/project.ts`
- `src/types/session.ts`
- `src/types/capture.ts`
- `src/types/issue.ts`
- `src/types/annotation.ts`

**Files to modify:** None

**Dependencies:** T1.01

**Acceptance Criteria:**
- [ ] `Project` interface có: id, name, description, color, isArchived, createdAt, updatedAt
- [ ] `Session` interface có: id, projectId, title, description, status, createdAt, updatedAt, completedAt, issueCount?, captureCount?
- [ ] `Capture` interface có: id, sessionId, screenshotPath, monitorName, monitorWidth, monitorHeight, scaleFactor, windowTitle, createdAt
- [ ] `Issue` interface có: id, captureId, sessionId, projectId, markerNumber, title, description, issueType, severity, status, markerX, markerY, annotationData, color, strokeWidth, cropPath, createdAt, updatedAt
- [ ] `IssueType` = `'Bug' | 'UI' | 'UX' | 'Suggestion' | 'Requirement' | 'Question'`
- [ ] `Severity` = `'Critical' | 'Major' | 'Minor' | 'Info'`
- [ ] `IssueStatus` = `'Draft' | 'Open' | 'In Progress' | 'Resolved' | 'Closed'`
- [ ] Annotation types: `MarkerAnnotation`, `RectAnnotation`, `ArrowAnnotation`, `TextAnnotation`
- [ ] `Point` = `{ x: number, y: number }`
- [ ] TypeScript compiles without error

**Manual Testing:** TypeScript compilation check — no runtime test needed.

**Notes:**
- Dùng type unions thay vì class
- Export tất cả types từ mỗi file
- Không tạo index barrel file — import trực tiếp từ file cụ thể

---

### T1.04 — Rust error types + utils

**Description:**
Tạo custom error type dùng `thiserror`, và utility functions (UUID generation, path helpers).

**Files to create:**
- `src-tauri/src/error.rs`
- `src-tauri/src/utils/mod.rs`
- `src-tauri/src/utils/paths.rs`
- `src-tauri/src/utils/id.rs`

**Files to modify:**
- `src-tauri/src/lib.rs` — declare modules
- `src-tauri/Cargo.toml` — add dependencies: `thiserror`, `uuid`, `chrono`, `serde`, `serde_json`

**Dependencies:** T1.01

**Acceptance Criteria:**
- [ ] `AppError` enum có variants: Database, FileIO, ScreenCapture, ImageProcessing, Validation, Export, Generic
- [ ] `AppError` implement `Serialize` cho Tauri IPC
- [ ] `new_uuid()` return UUID v4 string
- [ ] `screenshots_dir(base, date)` return đúng path: `{base}/screenshots/YYYY/MM/DD`
- [ ] `crops_dir(base, date)` return đúng path: `{base}/crops/YYYY/MM/DD`
- [ ] `ensure_dir(path)` tạo directory nếu chưa tồn tại
- [ ] `cargo build` thành công

**Manual Testing:** `cargo build` — no runtime test needed.

**Notes:**
- `paths.rs` — dùng `chrono::NaiveDate` cho date-based paths
- `id.rs` — wrapper đơn giản: `pub fn new_uuid() -> String { Uuid::new_v4().to_string() }`
- Thêm `anyhow` crate cho quick error handling trong services

---

### T1.05 — Rust domain models

**Description:**
Tạo Rust structs cho Project, Session, Capture, Issue. Derive `Serialize`, `Deserialize`, `Clone`, `Debug`. Tạo payload structs cho create/update operations.

**Files to create:**
- `src-tauri/src/models/mod.rs`
- `src-tauri/src/models/project.rs`
- `src-tauri/src/models/session.rs`
- `src-tauri/src/models/capture.rs`
- `src-tauri/src/models/issue.rs`

**Files to modify:**
- `src-tauri/src/lib.rs` — declare `mod models`

**Dependencies:** T1.04

**Acceptance Criteria:**
- [ ] `Project` struct match TypeScript `Project` interface (field names camelCase khi serialize)
- [ ] `CreateProjectPayload` có: name, description (optional), color (optional)
- [ ] `UpdateProjectPayload` có: name (optional), description (optional), color (optional), is_archived (optional)
- [ ] Tương tự cho Session, Capture, Issue
- [ ] Dùng `#[serde(rename_all = "camelCase")]` trên tất cả structs
- [ ] `cargo build` thành công

**Manual Testing:** `cargo build` — no runtime test needed.

**Notes:**
- Issue model cần field `annotation_data: String` (JSON string, parse ở frontend)
- Dùng `Option<T>` cho update payload fields
- Không tạo enum cho IssueType/Severity/Status ở Rust — dùng String, validate ở frontend. Giữ đơn giản.

---

### T1.06 — SQLite schema + migration

**Description:**
Tạo SQL migration file với full MVP schema (projects, sessions, captures, issues). Tạo Rust module để mở connection và chạy migrations.

**Files to create:**
- `src-tauri/migrations/v001_initial.sql`
- `src-tauri/src/db/mod.rs`
- `src-tauri/src/db/connection.rs`
- `src-tauri/src/db/migrations.rs`

**Files to modify:**
- `src-tauri/src/lib.rs` — declare `mod db`
- `src-tauri/Cargo.toml` — add `rusqlite = { version = "0.32", features = ["bundled"] }`

**Dependencies:** T1.04, T1.05

**Acceptance Criteria:**
- [ ] `v001_initial.sql` tạo 5 tables: `schema_version`, `projects`, `sessions`, `captures`, `issues`
- [ ] Default project "Default Project" được insert
- [ ] WAL mode enabled
- [ ] Foreign keys enabled
- [ ] `open_connection(app_data_dir)` return `Connection`
- [ ] `run_migrations(conn)` apply pending migrations
- [ ] Gọi `run_migrations` 2 lần → lần 2 skip (idempotent)
- [ ] Database file tạo tại `{app_data_dir}/august_mark.db`
- [ ] `cargo build` thành công

**Manual Testing:**
1. Chạy app → check `august_mark.db` file tồn tại
2. Dùng SQLite browser mở file → 5 tables hiện
3. `projects` table có 1 row "Default Project"

**Notes:**
- Schema từ ARCHITECTURE.md section 5, nhưng bỏ tables: `tags`, `issue_tags`, `settings`, `sync_log`
- Migration system đơn giản: read `schema_version` table → compare version → apply SQL
- `include_str!("../../migrations/v001_initial.sql")` để embed SQL vào binary

---

### T1.07 — AppState + Tauri setup

**Description:**
Tạo `AppState` struct chứa database connection (Mutex) và app data directory. Setup trong Tauri builder, quản lý bằng `tauri::Manager`.

**Files to create:**
- `src-tauri/src/state.rs`

**Files to modify:**
- `src-tauri/src/main.rs` — create AppState, manage() nó trong Tauri builder
- `src-tauri/src/lib.rs` — declare `mod state`

**Dependencies:** T1.06

**Acceptance Criteria:**
- [ ] `AppState` có fields: `db: Mutex<Connection>`, `app_data_dir: PathBuf`, `is_overlay_active: Mutex<bool>`
- [ ] AppState tạo thành công khi app khởi động
- [ ] Database connection mở, migrations chạy
- [ ] App data dir = Tauri `app_data_dir()` / `"AugustMark"`
- [ ] `npm run tauri dev` vẫn chạy bình thường

**Manual Testing:**
1. `npm run tauri dev` → app mở, không crash
2. Check folder `~/AppData/Roaming/com.august.mark/AugustMark/` tồn tại
3. File `august_mark.db` tồn tại trong folder đó

**Notes:**
- Dùng `app.path().app_data_dir()` để lấy platform-specific path
- Tạo subfolder `AugustMark` bên trong app data dir
- `Mutex<Connection>` đủ cho single-writer pattern, không cần connection pool

---

### T1.08 — Project repository + commands

**Description:**
Tạo Project CRUD ở database layer (repo) và Tauri IPC layer (commands). Gồm: create, get_all, get_by_id, update, delete.

**Files to create:**
- `src-tauri/src/db/project_repo.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/commands/project_cmds.rs`

**Files to modify:**
- `src-tauri/src/db/mod.rs` — add `pub mod project_repo`
- `src-tauri/src/lib.rs` — declare `mod commands`
- `src-tauri/src/main.rs` — register Tauri commands: `create_project`, `get_projects`, `get_project`, `update_project`, `delete_project`

**Dependencies:** T1.07

**Acceptance Criteria:**
- [ ] `create_project(name, description, color)` → insert row, return Project
- [ ] `get_projects(include_archived)` → return Vec\<Project\>
- [ ] `get_project(id)` → return Project
- [ ] `update_project(id, payload)` → update row, return Project
- [ ] `delete_project(id)` → delete row (CASCADE sessions)
- [ ] Default project "Default Project" vẫn có sau migration
- [ ] Tauri commands callable từ frontend console: `window.__TAURI__.invoke('get_projects')`

**Manual Testing:**
1. `npm run tauri dev`
2. Mở DevTools console (F12)
3. `await window.__TAURI__.core.invoke('get_projects')` → thấy ["Default Project"]
4. `await window.__TAURI__.core.invoke('create_project', { payload: { name: "Test" } })` → return project
5. `await window.__TAURI__.core.invoke('get_projects')` → thấy 2 projects

**Notes:**
- Repo functions nhận `&Connection`, command functions nhận `State<AppState>`
- Dùng `rusqlite::params![]` cho parameterized queries
- UUID generate bằng `utils::id::new_uuid()`
- `delete_project` phải CASCADE delete sessions → captures → issues (SQLite FK ON DELETE CASCADE)

---

### T1.09 — Tauri IPC bridge (Vue) — Project section

**Description:**
Tạo file `tauriCommands.ts` với typed wrapper functions gọi Tauri `invoke()`. Bắt đầu với project commands.

**Files to create:**
- `src/services/tauriCommands.ts`

**Files to modify:** None

**Dependencies:** T1.03, T1.08

**Acceptance Criteria:**
- [ ] `createProject(payload)` gọi `invoke('create_project', { payload })`, return `Promise<Project>`
- [ ] `getProjects(includeArchived?)` gọi `invoke('get_projects', { includeArchived })`, return `Promise<Project[]>`
- [ ] `getProject(id)` return `Promise<Project>`
- [ ] `updateProject(id, payload)` return `Promise<Project>`
- [ ] `deleteProject(id)` return `Promise<void>`
- [ ] TypeScript types match Rust response types

**Manual Testing:** Dùng trong store ở task sau.

**Notes:**
- Import `invoke` từ `@tauri-apps/api/core`
- File này sẽ lớn dần — tổ chức bằng comments: `// === Project Commands ===`, `// === Session Commands ===`, etc.
- Không tách nhiều files — 1 file duy nhất, dễ tìm, dễ grep

---

### T1.10 — Project store + sidebar UI

**Description:**
Tạo Pinia store cho projects và UI components: AppSidebar (project list), ProjectSelector (tạo project mới).

**Files to create:**
- `src/stores/projectStore.ts`
- `src/components/common/AppSidebar.vue`
- `src/components/dashboard/ProjectSelector.vue`

**Files to modify:**
- `src/App.vue` — add sidebar layout

**Dependencies:** T1.02, T1.09

**Acceptance Criteria:**
- [ ] `projectStore` có: projects list, activeProject, fetchProjects(), createProject(), deleteProject()
- [ ] AppSidebar hiển thị danh sách projects
- [ ] Click project → set activeProject
- [ ] Active project highlighted
- [ ] Button "New Project" → dialog → nhập name → tạo → hiển thị trong list
- [ ] Sidebar responsive (collapse trên màn hình nhỏ — không bắt buộc MVP)

**Manual Testing:**
1. App mở → sidebar hiện "Default Project"
2. Click "New Project" → nhập "Website Redesign" → OK → thấy trong list
3. Click "Website Redesign" → highlighted
4. Refresh → vẫn thấy 2 projects

**Notes:**
- Dùng Vuetify `v-navigation-drawer`, `v-list`, `v-dialog`
- ProjectSelector dùng `v-dialog` với `v-text-field` cho name

---

### T1.11 — Session repository + commands

**Description:**
Tạo Session CRUD ở Rust. Gồm: create, get_all, get_by_project, get_by_id, update, delete, complete.

**Files to create:**
- `src-tauri/src/db/session_repo.rs`
- `src-tauri/src/commands/session_cmds.rs`

**Files to modify:**
- `src-tauri/src/db/mod.rs` — add `pub mod session_repo`
- `src-tauri/src/commands/mod.rs` — add `pub mod session_cmds`
- `src-tauri/src/main.rs` — register session commands
- `src/services/tauriCommands.ts` — add session command wrappers

**Dependencies:** T1.08

**Acceptance Criteria:**
- [ ] `create_session(project_id, title, description)` → insert, return Session
- [ ] `get_sessions(filter)` → filter by project_id, status; return Vec\<Session\>
- [ ] `get_session(id)` → return Session with issue_count, capture_count
- [ ] `update_session(id, payload)` → update title/description/status
- [ ] `delete_session(id)` → CASCADE delete captures + issues + files
- [ ] `complete_session(id)` → set status='completed', completed_at=now
- [ ] TypeScript wrappers in `tauriCommands.ts`

**Manual Testing:**
1. DevTools: `invoke('create_session', { payload: { projectId: 'default', title: 'Test' } })` → OK
2. `invoke('get_sessions', { filter: { projectId: 'default' } })` → thấy session
3. `invoke('delete_session', { id: '...' })` → session biến mất

**Notes:**
- Session query cần LEFT JOIN count issues + captures cho `issueCount`, `captureCount`
- Delete session phải xóa cả files trên disk (screenshots, crops) — gọi `file_storage` service

---

### T1.12 — Session store + session list UI

**Description:**
Tạo Pinia store cho sessions. Tạo SessionList + SessionCard components. Tạo DashboardView.

**Files to create:**
- `src/stores/sessionStore.ts`
- `src/components/dashboard/SessionList.vue`
- `src/components/dashboard/SessionCard.vue`
- `src/views/DashboardView.vue`
- `src/components/common/EmptyState.vue`

**Files to modify:**
- `src/router/index.ts` — add route `/` → DashboardView
- `src/App.vue` — add `<router-view>` + dashboard layout

**Dependencies:** T1.10, T1.11

**Acceptance Criteria:**
- [ ] DashboardView hiện: sidebar (projects) + main area (sessions)
- [ ] SessionList hiển thị sessions của active project
- [ ] SessionCard hiện: title, created date, issue count, status badge
- [ ] Button "New Session" → dialog → nhập title → tạo
- [ ] Empty state khi chưa có session
- [ ] Switch project → session list cập nhật

**Manual Testing:**
1. App mở → dashboard hiện
2. Select "Default Project" → thấy "No sessions yet"
3. Click "New Session" → nhập "Homepage Review" → tạo → thấy card
4. Card hiện title + date + "0 issues"

**Notes:**
- SessionCard dùng `v-card`
- Date format: relative ("2 minutes ago") hoặc "Jun 15, 2026"
- Status badge: active=green, completed=blue

---

### T1.13 — AppHeader + system tray

**Description:**
Tạo AppHeader component. Setup system tray icon với menu cơ bản (Open Dashboard, Quit).

**Files to create:**
- `src/components/common/AppHeader.vue`

**Files to modify:**
- `src-tauri/src/main.rs` — setup system tray icon + menu
- `src/App.vue` — add AppHeader
- `src-tauri/tauri.conf.json` — tray config

**Dependencies:** T1.12

**Acceptance Criteria:**
- [ ] AppHeader hiện: app name "August Mark", hotkey hint "Ctrl+Shift+M"
- [ ] System tray icon hiển thị khi app chạy
- [ ] Right-click tray → menu: "Open Dashboard", "Quit"
- [ ] Click "Open Dashboard" → main window focus
- [ ] Click "Quit" → app tắt hoàn toàn
- [ ] Close main window (X) → app minimize to tray, không tắt

**Manual Testing:**
1. App mở → header hiện ở top
2. System tray icon visible
3. Close window (X) → window ẩn, tray icon vẫn có
4. Right-click tray → "Open Dashboard" → window hiện lại
5. Right-click tray → "Quit" → app tắt

**Notes:**
- Dùng Tauri 2 tray API: `tauri::tray::TrayIconBuilder`
- Icon: dùng default Tauri icon, thay sau
- Close event: intercept `on_window_event` → hide thay vì close

---

## Week 2 — Screenshot + Overlay

**Goal:** Nhấn hotkey → chụp screenshot → overlay fullscreen hiển thị ảnh.

---

### T2.01 — Screenshot capture service

**Description:**
Tạo Rust service dùng `xcap` crate để capture monitor chứa cursor. Save ảnh dưới dạng PNG.

**Files to create:**
- `src-tauri/src/services/mod.rs`
- `src-tauri/src/services/screen_capture.rs`

**Files to modify:**
- `src-tauri/Cargo.toml` — add `xcap = "0.3"`, `image = "0.25"`
- `src-tauri/src/lib.rs` — declare `mod services`

**Dependencies:** T1.04

**Acceptance Criteria:**
- [ ] `capture_current_monitor()` → return `(DynamicImage, MonitorInfo)`
- [ ] `MonitorInfo` có: x, y, width, height, scale_factor, name
- [ ] Capture đúng monitor chứa mouse cursor
- [ ] Chạy trên Windows 10/11
- [ ] Capture time < 1 second
- [ ] `cargo build` thành công

**Manual Testing:**
1. Viết test function tạm trong `main.rs`
2. Gọi `capture_current_monitor()` → save PNG → mở ảnh → đúng nội dung màn hình

**Notes:**
- `xcap` capture trả về `RgbaImage` hoặc `image::DynamicImage`
- Nếu `xcap` không compile: fallback `screenshots` crate
- Nếu cả 2 lỗi: dùng Windows GDI API qua `windows` crate
- Cursor position: dùng `windows::Win32::UI::WindowsAndMessaging::GetCursorPos`
- Thêm `windows` crate nếu cần cho cursor position

---

### T2.02 — File storage service

**Description:**
Tạo Rust service quản lý file I/O: save screenshot, save crop, delete file, ensure directories.

**Files to create:**
- `src-tauri/src/services/file_storage.rs`

**Files to modify:**
- `src-tauri/src/services/mod.rs` — add `pub mod file_storage`

**Dependencies:** T1.04

**Acceptance Criteria:**
- [ ] `save_screenshot(image, base_dir, capture_id, date)` → save PNG, return relative path
- [ ] `save_crop(image, base_dir, issue_id, date)` → save PNG, return relative path
- [ ] `delete_file(base_dir, relative_path)` → delete file
- [ ] `delete_capture_files(base_dir, capture_id, date)` → delete screenshot + annotated screenshot
- [ ] `ensure_dirs(base_dir)` → create screenshots/, crops/, exports/ if not exist
- [ ] Paths follow convention: `screenshots/YYYY/MM/DD/{capture_id}.png`

**Manual Testing:** Dùng cùng test function ở T2.01 — save ảnh → check file trên disk.

**Notes:**
- `image` crate `save()` method cho PNG output
- Relative paths lưu trong DB (để portable)
- Absolute path = `base_dir.join(relative_path)`

---

### T2.03 — Capture repository

**Description:**
Tạo Rust repo cho Capture CRUD (tạo khi chụp screenshot, query khi mở dashboard).

**Files to create:**
- `src-tauri/src/db/capture_repo.rs`

**Files to modify:**
- `src-tauri/src/db/mod.rs` — add `pub mod capture_repo`

**Dependencies:** T1.06, T1.05

**Acceptance Criteria:**
- [ ] `create(conn, session_id, screenshot_path, monitor_info)` → insert, return Capture
- [ ] `get_by_session(conn, session_id)` → return Vec\<Capture\>
- [ ] `get_by_id(conn, id)` → return Capture
- [ ] `delete(conn, id)` → delete row

**Manual Testing:** Test via DevTools console sau khi có commands (T2.04).

**Notes:** Monitor info fields (x, y, width, height, scale_factor, name) lưu flat trong captures table.

---

### T2.04 — Capture commands (trigger + open overlay)

**Description:**
Tạo Tauri commands cho capture flow: `trigger_capture` (chụp screenshot, lưu DB, mở overlay window), `close_overlay`, `cancel_capture`.

**Files to create:**
- `src-tauri/src/commands/capture_cmds.rs`

**Files to modify:**
- `src-tauri/src/commands/mod.rs` — add `pub mod capture_cmds`
- `src-tauri/src/main.rs` — register capture commands
- `src/services/tauriCommands.ts` — add capture command wrappers

**Dependencies:** T2.01, T2.02, T2.03

**Acceptance Criteria:**
- [ ] `trigger_capture(session_id)` → capture screenshot → save file → create DB record → return `{ captureId, screenshotPath, monitorInfo }`
- [ ] `open_overlay(capture_id)` → create Tauri WebviewWindow: fullscreen, transparent, always-on-top, decorations off
- [ ] `close_overlay()` → close overlay window
- [ ] `cancel_capture(capture_id)` → delete screenshot file + DB record → close overlay
- [ ] Overlay window mở đúng vị trí/kích thước của monitor chứa cursor
- [ ] Overlay window có label `"overlay"`

**Manual Testing:**
1. DevTools: `invoke('trigger_capture', { sessionId: '...' })` → return captureId
2. Check screenshots/ folder → file PNG tồn tại
3. (Overlay window chưa hiển thị gì — test ở T2.05)

**Notes:**
- Dùng `WebviewWindowBuilder::new()` cho Tauri 2
- Set: `.decorations(false)`, `.transparent(true)`, `.always_on_top(true)`, `.skip_taskbar(true)`
- Emit event `"overlay:init"` với `captureId` để overlay window biết load ảnh nào
- Nếu transparent không work: set background white, load screenshot fullscreen thay thế

---

### T2.05 — Overlay window (Vue side)

**Description:**
Tạo overlay entry point + root component. Overlay nhận captureId, load screenshot từ disk, hiển thị fullscreen.

**Files to create:**
- `public/overlay.html`
- `src/overlay.ts`
- `src/OverlayApp.vue`
- `src/components/overlay/OverlayStatusBar.vue`
- `src/services/tauriEvents.ts`

**Files to modify:**
- `vite.config.ts` — add overlay.html as additional entry point (multi-page)

**Dependencies:** T2.04, T1.02

**Acceptance Criteria:**
- [ ] `overlay.html` load `overlay.ts` → mount `OverlayApp.vue`
- [ ] OverlayApp listen event `"overlay:init"` → nhận captureId
- [ ] Load screenshot từ disk via `convertFileSrc()`
- [ ] Screenshot hiển thị fullscreen (fit window)
- [ ] OverlayStatusBar hiện: "Session: {name} | 0 issues | [Cancel] [Done]"
- [ ] Click Cancel → gọi `cancel_capture` → window đóng
- [ ] Click Done → (chưa save logic) → gọi `close_overlay` → window đóng
- [ ] Nhấn Esc → same as Cancel

**Manual Testing:**
1. `invoke('trigger_capture', { sessionId: '...' })` từ DevTools
2. `invoke('open_overlay', { captureId: '...' })` → overlay window mở
3. Thấy screenshot hiển thị fullscreen
4. Thấy status bar ở top
5. Click Cancel → window đóng
6. Nhấn Esc → window đóng

**Notes:**
- Vite multi-page: config `build.rollupOptions.input` thêm `overlay.html`
- `convertFileSrc` từ `@tauri-apps/api/core` convert local path → asset URL
- Overlay Vue app khởi tạo riêng (không dùng router, chỉ cần vuetify + pinia)
- Screenshot hiển thị dưới dạng `<img>` tạm, chuyển canvas ở Week 3

---

### T2.06 — Global hotkey trigger

**Description:**
Đăng ký global keyboard shortcut `Ctrl+Shift+M`. Khi nhấn: nếu có active session → trigger capture + open overlay. Nếu không có session → tạo session mới rồi trigger.

**Files to create:**
- `src/composables/useTauriEvents.ts`

**Files to modify:**
- `src-tauri/src/main.rs` — register global shortcut
- `src/services/tauriEvents.ts` — add event listener helpers
- `src/stores/sessionStore.ts` — add `ensureActiveSession()` method

**Dependencies:** T2.04, T2.05

**Acceptance Criteria:**
- [ ] Nhấn `Ctrl+Shift+M` ở bất kỳ đâu trên desktop → overlay mở
- [ ] Screenshot chụp đúng màn hình chứa cursor
- [ ] Nếu có active session → dùng session đó
- [ ] Nếu không có active session → auto-create session "Quick Review {date}"
- [ ] Hotkey không trigger khi overlay đang mở
- [ ] Hotkey hoạt động khi main window minimized/hidden

**Manual Testing:**
1. Tạo session trong dashboard
2. Minimize app to tray
3. Mở website bất kỳ
4. Nhấn Ctrl+Shift+M → overlay mở → thấy screenshot website
5. Cancel → overlay đóng
6. Nhấn Ctrl+Shift+M lại → vẫn hoạt động

**Notes:**
- Tauri 2 global shortcut: `app.global_shortcut().register("CmdOrCtrl+Shift+M", handler)`
- Handler: emit event `"capture:trigger"` → frontend listen → gọi `trigger_capture`
- Hoặc handler gọi trực tiếp Rust capture logic — cần truyền active session_id
- Nếu conflict: try `Ctrl+Alt+M` hoặc `F9` as fallback
- Thêm plugin `tauri-plugin-global-shortcut` nếu cần

---

## Week 3 — Annotation Tools + Issue Form

**Goal:** Vẽ marker, rectangle, arrow, text trên overlay. Nhập issue metadata. Lưu tất cả.

---

### T3.01 — Canvas setup (3 layers)

**Description:**
Chuyển overlay từ `<img>` sang HTML5 Canvas. Setup 3 canvas layers chồng lên nhau: screenshot (static), markers (committed), drawing (active).

**Files to create:**
- `src/composables/useCanvas.ts`

**Files to modify:**
- `src/OverlayApp.vue` — thay `<img>` bằng canvas layout
- `src/components/overlay/AnnotationCanvas.vue` (tạo mới nếu chưa có, hoặc extract từ OverlayApp)

**Dependencies:** T2.05

**Acceptance Criteria:**
- [ ] 3 canvas elements stacked đúng z-order (screenshot bottom, markers middle, drawing top)
- [ ] Screenshot render lên canvas layer 1 (drawImage)
- [ ] Canvas size = screenshot size (match pixel-to-pixel)
- [ ] Mouse events (mousedown, mousemove, mouseup) capture đúng trên drawing canvas
- [ ] Mouse coordinates chuyển đổi đúng từ page coords → canvas coords
- [ ] Canvas xử lý DPI scaling: `canvas.width` = `element.width * devicePixelRatio`
- [ ] Tối thiểu: click canvas → log tọa độ (x, y) vào console → đúng vị trí

**Manual Testing:**
1. Trigger overlay → screenshot render trên canvas (không phải `<img>`)
2. Click canvas → console log tọa độ → đúng vị trí
3. Canvas full viewport (không cuộn, không margin)

**Notes:**
- `useCanvas.ts` export composable: `setupCanvas(screenshotCanvasRef, markerCanvasRef, drawingCanvasRef)`
- DPI handling: set canvas dimension = element.clientWidth × devicePixelRatio
- Context scaling: `ctx.scale(dpr, dpr)` để draw ở logical pixels
- Screenshot load: `new Image()` → `ctx.drawImage(img, 0, 0, width, height)`

---

### T3.02 — Overlay store

**Description:**
Tạo Pinia store cho overlay state: active tool, annotations list, current capture info.

**Files to create:**
- `src/stores/overlayStore.ts`

**Files to modify:** None

**Dependencies:** T1.02, T1.03

**Acceptance Criteria:**
- [ ] `overlayStore` có state: `activeTool`, `annotations[]`, `captureId`, `screenshotPath`, `monitorInfo`, `nextMarkerNumber`
- [ ] `activeTool` type: `'marker' | 'rect' | 'arrow' | 'text' | null`
- [ ] `setTool(tool)` → set activeTool
- [ ] `addAnnotation(annotation)` → push to list, increment nextMarkerNumber
- [ ] `removeAnnotation(id)` → remove from list
- [ ] `init(captureId, screenshotPath, monitorInfo)` → reset state, set capture info
- [ ] `reset()` → clear everything

**Manual Testing:** Test indirectly via annotation tools (T3.04+).

**Notes:** Annotations stored in memory during overlay session. Persisted to DB only on "Done".

---

### T3.03 — Annotation toolbar

**Description:**
Tạo toolbar component ở bottom overlay. Buttons: Marker (①), Rectangle (▭), Arrow (→), Text (T).

**Files to create:**
- `src/components/overlay/AnnotationToolbar.vue`

**Files to modify:**
- `src/OverlayApp.vue` — add AnnotationToolbar

**Dependencies:** T3.02

**Acceptance Criteria:**
- [ ] Toolbar hiện fixed ở bottom center overlay
- [ ] 4 tool buttons: Marker, Rectangle, Arrow, Text
- [ ] Click button → `overlayStore.setTool()` → button highlighted (active state)
- [ ] Click same button → deselect (tool = null)
- [ ] Toolbar có `pointer-events: auto`, phần còn lại overlay có `pointer-events: none` cho UI layer
- [ ] Toolbar style: semi-transparent dark background, rounded, compact

**Manual Testing:**
1. Trigger overlay → toolbar visible ở bottom
2. Click "Marker" → highlighted
3. Click "Rectangle" → Marker deselected, Rectangle highlighted
4. Click "Rectangle" again → deselected

**Notes:**
- Dùng Vuetify `v-btn-toggle` hoặc custom buttons
- Icons: dùng mdi icons (vuetify built-in): `mdi-map-marker`, `mdi-rectangle-outline`, `mdi-arrow-top-right`, `mdi-format-text`
- Toolbar CSS: `position: fixed; bottom: 20px; left: 50%; transform: translateX(-50%)`

---

### T3.04 — Annotation composable + Number Marker tool

**Description:**
Tạo composable quản lý tool state machine. Implement marker tool: click canvas → place numbered marker.

**Files to create:**
- `src/composables/useAnnotation.ts`

**Files to modify:**
- `src/components/overlay/AnnotationCanvas.vue` — wire up mouse events → useAnnotation
- `src/composables/useCanvas.ts` — add `renderMarker()` function

**Dependencies:** T3.01, T3.02, T3.03

**Acceptance Criteria:**
- [ ] Click canvas khi tool = 'marker' → marker ① xuất hiện tại vị trí click
- [ ] Click lần 2 → marker ② xuất hiện
- [ ] Marker render: orange circle (30px radius) + white number text inside
- [ ] Marker render trên marker canvas (layer 2)
- [ ] Marker position lưu vào `overlayStore.annotations[]`
- [ ] Mỗi annotation có: id, type, markerNumber, position (x, y), color

**Manual Testing:**
1. Trigger overlay → chọn Marker tool
2. Click 3 vị trí → thấy ① ② ③
3. Markers ở đúng vị trí click
4. Markers render trên screenshot (không bị che)

**Notes:**
- `useAnnotation` expose: `onCanvasMouseDown(e)`, `onCanvasMouseMove(e)`, `onCanvasMouseUp(e)`
- Sau khi place marker → emit event hoặc set flag để mở IssueFormPanel (T3.07)
- Marker rendering: `ctx.beginPath()`, `ctx.arc()`, `ctx.fill()`, `ctx.fillText()`

---

### T3.05 — Rectangle tool

**Description:**
Implement rectangle tool: mousedown → start, mousemove → preview, mouseup → commit.

**Files to create:** None (modify existing)

**Files to modify:**
- `src/composables/useAnnotation.ts` — add rect tool logic
- `src/composables/useCanvas.ts` — add `renderRect()`, `renderRectPreview()`

**Dependencies:** T3.04

**Acceptance Criteria:**
- [ ] Chọn Rectangle tool → drag trên canvas → thấy dashed rectangle preview (layer 3)
- [ ] Release mouse → solid rectangle committed (layer 2) + numbered marker ở top-left
- [ ] Rectangle color: orange stroke, transparent fill
- [ ] Preview clears mỗi mousemove (redraw layer 3)
- [ ] Annotation data lưu: type='rect', topLeft, width, height, markerNumber

**Manual Testing:**
1. Chọn Rectangle → drag trên canvas
2. Thấy preview rectangle khi drag
3. Release → solid rectangle + marker number
4. Vẽ 2 rectangles → mỗi cái có number riêng

**Notes:**
- Preview render trên drawing canvas (layer 3) — clear + redraw mỗi frame
- Commit render trên marker canvas (layer 2) — persist
- `ctx.setLineDash([6, 3])` cho preview dashed line
- Number marker auto-placed ở `(topLeft.x, topLeft.y - 20)`

---

### T3.06 — Arrow tool

**Description:**
Implement arrow tool: drag từ start → end, render arrow line + arrowhead.

**Files to create:** None

**Files to modify:**
- `src/composables/useAnnotation.ts` — add arrow tool logic
- `src/composables/useCanvas.ts` — add `renderArrow()`, `renderArrowPreview()`

**Dependencies:** T3.04

**Acceptance Criteria:**
- [ ] Chọn Arrow tool → drag → thấy arrow preview
- [ ] Release → solid arrow committed + marker at start point
- [ ] Arrow có arrowhead ở end point
- [ ] Annotation data lưu: type='arrow', start, end, markerNumber

**Manual Testing:**
1. Chọn Arrow → drag → thấy arrow
2. Arrowhead hiển thị đúng hướng
3. Marker number ở start point

**Notes:**
- Arrowhead: vẽ 2 lines tạo góc ~30° từ endpoint
- `Math.atan2(dy, dx)` để tính angle
- Stroke width: 2px, arrowhead length: 15px

---

### T3.07 — Text note tool

**Description:**
Implement text tool: click canvas → hiện input field → type → Enter → render text trên canvas.

**Files to create:** None

**Files to modify:**
- `src/composables/useAnnotation.ts` — add text tool logic
- `src/composables/useCanvas.ts` — add `renderText()`
- `src/components/overlay/AnnotationCanvas.vue` — add floating input element

**Dependencies:** T3.04

**Acceptance Criteria:**
- [ ] Chọn Text tool → click canvas → floating input hiện tại vị trí click
- [ ] Nhập text → Enter → text render trên canvas + marker number
- [ ] Esc → cancel text input
- [ ] Text render: white text, font 16px, dark background padding
- [ ] Annotation data lưu: type='text', position, text, fontSize, markerNumber

**Manual Testing:**
1. Chọn Text → click canvas → thấy text input
2. Type "Fix alignment" → Enter → text render trên canvas
3. Marker number bên trái text

**Notes:**
- Floating input: `<input>` absolute positioned, transparent background
- Sau khi commit: hide input, render text via `ctx.fillText()`
- Background: `ctx.fillRect()` với semi-transparent black trước khi draw text

---

### T3.08 — Issue form panel

**Description:**
Tạo slide-in panel bên phải overlay. Hiện khi user đặt annotation. Chứa form: title, type, severity, description.

**Files to create:**
- `src/components/overlay/IssueFormPanel.vue`

**Files to modify:**
- `src/OverlayApp.vue` — add IssueFormPanel, quản lý show/hide
- `src/stores/overlayStore.ts` — add `pendingAnnotation`, `showIssueForm`

**Dependencies:** T3.04

**Acceptance Criteria:**
- [ ] Panel slide-in từ phải khi annotation được đặt
- [ ] Form fields: Title (required, text), Type (dropdown), Severity (dropdown), Description (textarea, optional)
- [ ] Type options: Bug, UI, UX, Suggestion, Requirement, Question
- [ ] Severity options: Critical, Major, Minor, Info
- [ ] Default: Type=Bug, Severity=Minor
- [ ] Click "Save" → annotation confirmed, panel đóng, marker style thay đổi (solid)
- [ ] Click "Cancel" → annotation bị xóa khỏi canvas, panel đóng
- [ ] Panel width: 320px, overlay canvas co lại (hoặc panel overlay trên canvas)
- [ ] Issue form data attach vào annotation trong overlayStore

**Manual Testing:**
1. Đặt marker → panel mở
2. Nhập "Button color wrong" → Type=UI → Severity=Minor
3. Save → panel đóng → marker solid
4. Đặt rect → panel mở → Cancel → rect biến mất

**Notes:**
- Dùng Vuetify: `v-navigation-drawer` (right, temporary), `v-text-field`, `v-select`, `v-textarea`
- `overlayStore.pendingAnnotation` = annotation chờ form input
- Save: copy pending → annotations list với form data attached
- Data structure mỗi annotation entry: `{ annotation: AnnotationData, issue: { title, type, severity, description } }`

---

## Week 4 — Save + Dashboard + Export

**Goal:** Data persist end-to-end. Dashboard hiển thị issues. Export HTML.

---

### T4.01 — Issue repository

**Description:**
Tạo Rust repo cho Issue CRUD: create (batch), query, update, delete.

**Files to create:**
- `src-tauri/src/db/issue_repo.rs`

**Files to modify:**
- `src-tauri/src/db/mod.rs` — add `pub mod issue_repo`

**Dependencies:** T1.06

**Acceptance Criteria:**
- [ ] `create(conn, payload)` → insert single issue, return Issue
- [ ] `create_batch(conn, issues)` → insert multiple issues in transaction
- [ ] `get_by_session(conn, session_id)` → return Vec\<Issue\>
- [ ] `get_by_capture(conn, capture_id)` → return Vec\<Issue\>
- [ ] `get_by_id(conn, id)` → return Issue
- [ ] `update(conn, id, payload)` → update fields, return Issue
- [ ] `delete(conn, id)` → delete row
- [ ] `delete_by_capture(conn, capture_id)` → delete all issues of a capture

**Manual Testing:** Test via commands (T4.03).

**Notes:**
- `create_batch` dùng `BEGIN TRANSACTION ... COMMIT` cho performance
- `annotation_data` lưu dưới dạng JSON string
- `marker_x, marker_y` = center position của marker

---

### T4.02 — Image processor (crop + annotated screenshot)

**Description:**
Tạo Rust service crop vùng quanh marker và render annotations lên screenshot (burned-in version cho export).

**Files to create:**
- `src-tauri/src/services/image_processor.rs`

**Files to modify:**
- `src-tauri/src/services/mod.rs` — add `pub mod image_processor`

**Dependencies:** T2.01

**Acceptance Criteria:**
- [ ] `crop_for_issue(image, marker_x, marker_y, padding)` → return cropped RgbaImage (400×400px centered at marker)
- [ ] Crop clamp tại image boundaries (không panic nếu marker ở edge)
- [ ] `save_annotated_screenshot(base_image, annotations_json, output_path)` → render markers/rects/arrows trên ảnh → save
- [ ] Annotated screenshot chứa: markers (numbered circles), rectangles, arrows

**Manual Testing:**
1. Sau T4.03: capture + save annotations → check crops/ folder → file PNG tồn tại, crop đúng vùng
2. Check screenshots/ folder → `{id}_annotated.png` tồn tại, annotations hiển thị

**Notes:**
- Crop đơn giản: 400×400 centered tại (marker_x, marker_y), padding 50px
- Nếu crop logic phức tạp quá: fallback = bỏ crop, chỉ lưu full screenshot
- Annotated screenshot: dùng `image` crate vẽ circles/lines/text lên RgbaImage
- Text rendering trên image khó (cần font). Fallback: chỉ vẽ markers (circles + numbers) + rectangles + arrows, bỏ text notes trên annotated image

---

### T4.03 — Issue commands + save flow

**Description:**
Tạo Tauri command `save_capture_annotations`. Frontend gọi khi user click "Done": gửi list annotations + issue metadata → Rust lưu DB + generate crops + save annotated screenshot.

**Files to create:**
- `src-tauri/src/commands/issue_cmds.rs`

**Files to modify:**
- `src-tauri/src/commands/mod.rs` — add `pub mod issue_cmds`
- `src-tauri/src/main.rs` — register issue commands
- `src/services/tauriCommands.ts` — add issue command wrappers
- `src/stores/overlayStore.ts` — add `saveAndClose()` action
- `src/components/overlay/OverlayStatusBar.vue` — wire "Done" button → saveAndClose

**Dependencies:** T4.01, T4.02, T3.08

**Acceptance Criteria:**
- [ ] Click "Done" trên overlay → gọi `save_capture_annotations` với: captureId, list issues (markerNumber, title, description, type, severity, markerX, markerY, annotationData, color, strokeWidth)
- [ ] Rust: insert issues vào DB trong transaction
- [ ] Rust: generate crop cho mỗi issue → save to crops/ folder
- [ ] Rust: save annotated screenshot → `{captureId}_annotated.png`
- [ ] Overlay window đóng sau khi save thành công
- [ ] Nếu save fail → error hiện trên overlay, không đóng

**Manual Testing:**
1. Trigger overlay → đặt 3 markers → fill form → Save each
2. Click Done
3. Check DB: 3 issues với đúng data
4. Check screenshots/: annotated screenshot tồn tại
5. Check crops/: 3 crop files tồn tại
6. Overlay đóng, quay về desktop

**Notes:**
- Frontend gửi annotated screenshot dưới dạng `canvas.toDataURL('image/png')` — base64 string
- Rust decode base64 → save PNG
- Hoặc: Rust tự render annotations lên original screenshot (dùng image_processor)
- Chọn cách nào đơn giản hơn. Canvas toDataURL đơn giản hơn ở frontend nhưng base64 string rất lớn.
- Recommend: Rust nhận annotation_data JSON → tự render, tránh truyền base64 qua IPC

---

### T4.04 — Issue store + query commands

**Description:**
Tạo Pinia store cho issues. Thêm Tauri commands: get_issues, get_issue, update_issue, delete_issue.

**Files to create:**
- `src/stores/issueStore.ts`

**Files to modify:**
- `src-tauri/src/commands/issue_cmds.rs` — add query/update/delete commands
- `src-tauri/src/main.rs` — register new commands
- `src/services/tauriCommands.ts` — add issue query/update/delete wrappers

**Dependencies:** T4.03

**Acceptance Criteria:**
- [ ] `issueStore` có: issues list, activeIssue, filters, fetchIssuesBySession(), fetchIssue(), updateIssue(), deleteIssue()
- [ ] `get_issues(filter)` command: filter by session_id, capture_id, issue_type, severity, status
- [ ] `get_issue(id)` command: return single issue with full data
- [ ] `update_issue(id, payload)` command: update title, description, type, severity, status
- [ ] `delete_issue(id)` command: delete issue + crop file

**Manual Testing:**
1. DevTools: `invoke('get_issues', { filter: { sessionId: '...' } })` → thấy issues
2. `invoke('update_issue', { id: '...', payload: { status: 'Resolved' } })` → OK
3. `invoke('delete_issue', { id: '...' })` → issue biến mất, crop file xóa

**Notes:** Filter implementation đơn giản: WHERE clause build dynamic dựa trên filter fields có giá trị.

---

### T4.05 — Issue list + card UI (dashboard)

**Description:**
Tạo components hiển thị issues trong dashboard: IssueList, IssueCard.

**Files to create:**
- `src/components/dashboard/IssueList.vue`
- `src/components/dashboard/IssueCard.vue`

**Files to modify:**
- `src/views/DashboardView.vue` — hoặc `SessionView.vue` — add issue list khi chọn session

**Dependencies:** T4.04, T1.12

**Acceptance Criteria:**
- [ ] Click session card → expand/navigate → thấy list issues
- [ ] IssueCard hiện: marker number, title, type badge (color-coded), severity badge, status badge
- [ ] Thumbnail: crop image nếu có, fallback full screenshot
- [ ] Badge colors: Bug=red, UI=orange, UX=purple, Suggestion=blue, Requirement=green, Question=teal
- [ ] Severity colors: Critical=red, Major=orange, Minor=yellow, Info=gray

**Manual Testing:**
1. Đã capture + mark issues → mở dashboard
2. Click session → thấy issues list
3. Mỗi issue card hiện đúng info

**Notes:**
- Thumbnail: load ảnh crop via `convertFileSrc(cropPath)`
- Dùng Vuetify `v-chip` cho type/severity badges
- Layout: grid hoặc list view

---

### T4.06 — Issue detail view

**Description:**
Tạo view chi tiết issue: hiển thị full screenshot với marker highlight, crop image, editable metadata.

**Files to create:**
- `src/components/dashboard/IssueDetail.vue`
- `src/views/IssueView.vue`
- `src/views/SessionView.vue`

**Files to modify:**
- `src/router/index.ts` — add routes: `/session/:id`, `/issue/:id`

**Dependencies:** T4.05

**Acceptance Criteria:**
- [ ] Route `/issue/:id` → IssueView → IssueDetail
- [ ] IssueDetail hiển thị: full annotated screenshot, crop image (nếu có), title, description, type, severity, status
- [ ] Edit mode: click title → editable, change type/severity/status → auto-save
- [ ] Back button → quay lại session view
- [ ] Screenshot hiển thị đúng (load từ disk via convertFileSrc)

**Manual Testing:**
1. Dashboard → click session → click issue → thấy detail
2. Thấy screenshot + crop + metadata
3. Edit title → save → reload → title đã đổi
4. Change status Open → Resolved → badge đổi màu

**Notes:**
- Dùng Vuetify `v-img` cho ảnh, `v-text-field` cho editable title
- Auto-save: debounce 500ms sau khi user ngừng type, hoặc on blur

---

### T4.07 — Delete session/issue + confirm dialog

**Description:**
Implement delete functionality với confirm dialog.

**Files to create:**
- `src/components/common/ConfirmDialog.vue`

**Files to modify:**
- `src/components/dashboard/SessionCard.vue` — add delete button
- `src/components/dashboard/IssueCard.vue` — add delete button
- `src/stores/sessionStore.ts` — add deleteSession()
- `src/stores/issueStore.ts` — add deleteIssue()
- `src-tauri/src/commands/session_cmds.rs` — implement file cleanup in delete

**Dependencies:** T4.06

**Acceptance Criteria:**
- [ ] Delete session button → confirm dialog "Delete session and all issues?" → Yes → session + issues + files deleted
- [ ] Delete issue button → confirm dialog → Yes → issue + crop deleted
- [ ] After delete → list auto-refresh
- [ ] ConfirmDialog reusable: title, message, confirm action

**Manual Testing:**
1. Delete issue → confirm → issue gone, crop file deleted
2. Delete session → confirm → session gone, all issues gone, all files gone
3. Cancel confirm → nothing happens

**Notes:**
- Vuetify `v-dialog` với 2 buttons: Cancel, Delete (red)
- Session delete: Rust side cascade delete captures + issues, file_storage service delete screenshots + crops

---

### T4.08 — Filter bar

**Description:**
Tạo filter bar trên dashboard: filter by type, severity, status.

**Files to create:**
- `src/components/dashboard/FilterBar.vue`

**Files to modify:**
- `src/views/DashboardView.vue` hoặc `SessionView.vue` — add FilterBar above issue list
- `src/stores/issueStore.ts` — add filter state + filtered computed

**Dependencies:** T4.05

**Acceptance Criteria:**
- [ ] Filter by issue type: dropdown, multi-select → chỉ hiện issues matching
- [ ] Filter by severity: dropdown, multi-select
- [ ] Filter by status: dropdown, multi-select
- [ ] Clear all filters button
- [ ] Filters reactive: thay đổi filter → list cập nhật ngay
- [ ] Issue count hiển thị: "Showing 5 of 12 issues"

**Manual Testing:**
1. Có 10 issues mixed types → filter Type=Bug → chỉ thấy Bugs
2. Filter Severity=Critical → chỉ thấy Critical
3. Clear filters → thấy tất cả

**Notes:**
- Dùng Vuetify `v-select` với `multiple` prop
- Filter logic: client-side (filter từ issueStore.issues), không cần query lại DB cho MVP
- Nếu data lớn sau này → chuyển sang server-side filter

---

### T4.09 — Export HTML engine

**Description:**
Tạo Rust service generate self-contained HTML report cho session. All CSS inline, images base64.

**Files to create:**
- `src-tauri/src/services/export_html.rs`

**Files to modify:**
- `src-tauri/src/services/mod.rs` — add `pub mod export_html`

**Dependencies:** T4.01, T2.02

**Acceptance Criteria:**
- [ ] `export_session_html(conn, base_dir, session_id, output_path)` → generate HTML file
- [ ] HTML file self-contained: inline CSS, base64 images
- [ ] Content: session header (title, project, date, issue count) → per-capture section (annotated screenshot + issue details)
- [ ] Issue details: marker number, title, type badge, severity badge, description, crop image
- [ ] File opens correctly in Chrome/Edge
- [ ] File size reasonable (< 20MB for typical session)

**Manual Testing:**
1. Gọi export → mở file .html → layout đẹp, ảnh hiển thị, badges đúng màu
2. Forward .html qua email → mở trên máy khác → vẫn hiển thị đúng

**Notes:**
- Template: string formatting hoặc simple Rust template (không cần template engine)
- Format: `format!()` macro với HTML template string
- Base64: `use base64::Engine; base64::engine::general_purpose::STANDARD.encode(&bytes)`
- Add `base64` crate to Cargo.toml
- CSS dark theme, responsive, print-friendly

---

### T4.10 — Export command + dialog (Vue)

**Description:**
Tạo Tauri command `export_html` và Vue dialog cho user chọn session + save location.

**Files to create:**
- `src-tauri/src/commands/export_cmds.rs`
- `src/components/export/ExportDialog.vue`

**Files to modify:**
- `src-tauri/src/commands/mod.rs` — add `pub mod export_cmds`
- `src-tauri/src/main.rs` — register export commands
- `src/services/tauriCommands.ts` — add export wrapper
- `src/components/dashboard/SessionCard.vue` — add "Export" button

**Dependencies:** T4.09

**Acceptance Criteria:**
- [ ] Session card → "Export" button → ExportDialog opens
- [ ] ExportDialog: shows session name, "Export HTML" button
- [ ] Click export → native save dialog → choose location → file saved
- [ ] Success notification after export
- [ ] Error notification if export fails

**Manual Testing:**
1. Dashboard → session card → Export
2. Choose Desktop → save
3. report.html appears on Desktop
4. Open in browser → correct content

**Notes:**
- Dùng `tauri-plugin-dialog` cho native save file dialog
- Filename default: `{session_title}_report.html`
- Vuetify `v-dialog` cho ExportDialog

---

## Week 5 — Polish (Optional)

**Goal:** Cải thiện UX, error handling, edge cases. Middle mouse hold nếu kịp.

---

### T5.01 — UI store + toast notifications

**Description:**
Tạo UI store quản lý loading states, toast notifications (success, error, info).

**Files to create:**
- `src/stores/uiStore.ts`

**Files to modify:**
- `src/App.vue` — add toast container (Vuetify `v-snackbar`)
- Tất cả stores — add loading/error states

**Dependencies:** T1.02

**Acceptance Criteria:**
- [ ] `uiStore.showToast({ message, type, duration })` → toast hiện
- [ ] Toast types: success (green), error (red), info (blue)
- [ ] Auto-dismiss after duration (default 3s)
- [ ] Loading states: `uiStore.isLoading` → loading overlay
- [ ] Tất cả operations (create, save, export, delete) show toast on success/error

**Manual Testing:**
1. Tạo session → "Session created" toast (green)
2. Export thành công → "Report exported" toast
3. Delete → "Session deleted" toast
4. Force error (disconnect DB) → error toast (red)

**Notes:** Vuetify `v-snackbar` component, position bottom-right.

---

### T5.02 — Error handling polish

**Description:**
Review tất cả Tauri commands và Vue stores. Ensure: no unhandled panics, no silent failures, all errors show user-friendly message.

**Files to create:** None

**Files to modify:**
- Tất cả `src-tauri/src/commands/*.rs` — add proper error mapping
- Tất cả `src/stores/*.ts` — add try/catch, show toast

**Dependencies:** T5.01

**Acceptance Criteria:**
- [ ] Không có `unwrap()` trong command handlers (chỉ trong main.rs setup OK)
- [ ] Mỗi command return `Result<T, String>` với human-readable error
- [ ] Frontend: mỗi `invoke()` call wrapped trong try/catch
- [ ] Error toast hiện khi: DB error, file not found, screenshot fail, export fail
- [ ] App không crash khi: disk full, DB locked, screenshot permission denied

**Manual Testing:**
1. Delete DB file → restart app → graceful error hoặc recreate DB
2. Nhấn Ctrl+Shift+M khi overlay đang mở → không crash, không mở 2 overlays

**Notes:** Đây là review + fix pass, không tạo file mới.

---

### T5.03 — Keyboard shortcuts (overlay)

**Description:**
Thêm keyboard shortcuts cho overlay: 1=Marker, 2=Rect, 3=Arrow, 4=Text, Esc=Cancel/Close.

**Files to create:** None

**Files to modify:**
- `src/OverlayApp.vue` — add keydown listener
- `src/composables/useAnnotation.ts` — expose tool selection functions

**Dependencies:** T3.08

**Acceptance Criteria:**
- [ ] Key `1` → select Marker tool
- [ ] Key `2` → select Rectangle tool
- [ ] Key `3` → select Arrow tool
- [ ] Key `4` → select Text tool
- [ ] `Esc` khi không có form mở → cancel/close overlay
- [ ] `Esc` khi form mở → close form, remove pending annotation
- [ ] Shortcuts không trigger khi typing trong issue form

**Manual Testing:**
1. Overlay mở → nhấn 1 → Marker selected
2. Nhấn 2 → Rectangle selected
3. Nhấn Esc → overlay đóng
4. Mở issue form → type trong title → nhấn 1 → không đổi tool (form focused)

**Notes:**
- `document.addEventListener('keydown', handler)` trong `onMounted`, cleanup trong `onUnmounted`
- Check `event.target` — nếu input/textarea → ignore shortcut

---

### T5.04 — Middle mouse hold trigger (optional)

**Description:**
Thêm alternative trigger: giữ chuột giữa ≥ 1 giây → overlay mở. Dùng `rdev` crate.

**Files to create:**
- `src-tauri/src/services/mouse_hook.rs`

**Files to modify:**
- `src-tauri/Cargo.toml` — add `rdev = "0.5"`
- `src-tauri/src/services/mod.rs` — add `pub mod mouse_hook`
- `src-tauri/src/main.rs` — start mouse hook on app setup

**Dependencies:** T2.06

**Acceptance Criteria:**
- [ ] Giữ chuột giữa ≥ 1 giây → overlay trigger (same as Ctrl+Shift+M)
- [ ] Giữ < 1 giây → không trigger (normal middle click passthrough)
- [ ] Scroll wheel → không trigger
- [ ] Khi overlay đang mở → middle mouse không trigger lại
- [ ] `Ctrl+Shift+M` hotkey vẫn hoạt động song song

**Manual Testing:**
1. Giữ chuột giữa 2 giây → overlay mở
2. Click chuột giữa nhanh (<0.5s) → không gì xảy ra
3. Scroll wheel → không trigger
4. Overlay đang mở → giữ chuột giữa → không trigger lần 2

**Notes:**
- ⚠️ HIGH RISK: `rdev` có thể bị antivirus block hoặc không compile
- Nếu lỗi → BỎ QUA, giữ Ctrl+Shift+M. Đây là optional enhancement.
- `rdev::listen()` block thread → chạy trong `std::thread::spawn`
- Hold detection: ghi timestamp khi ButtonPress(Middle), check elapsed khi timer fires

---

### T5.05 — Session status management

**Description:**
Implement session lifecycle: active → completed. UI cho complete session.

**Files to create:** None

**Files to modify:**
- `src/components/dashboard/SessionCard.vue` — add "Complete" button
- `src/stores/sessionStore.ts` — add completeSession()
- `src-tauri/src/commands/session_cmds.rs` — ensure complete_session works

**Dependencies:** T1.11

**Acceptance Criteria:**
- [ ] Active session → "Complete" button → status = completed, completed_at = now
- [ ] Completed sessions show different style (dimmed, or different badge)
- [ ] Overlay trigger only uses active sessions (không capture vào completed session)
- [ ] Session card show duration: "Started 2h ago" or "Completed Jun 15"

**Manual Testing:**
1. Active session → Complete → badge changes → green "completed" badge
2. Nhấn Ctrl+Shift+M → auto-create new session (không dùng completed session)

**Notes:** Vuetify `v-chip` color change based on status.

---

### T5.06 — Frontend utils

**Description:**
Tạo utility functions dùng chung trong frontend.

**Files to create:**
- `src/utils/date.ts`
- `src/utils/geometry.ts`
- `src/utils/image.ts`

**Files to modify:** Refactor existing code để dùng utils

**Dependencies:** None

**Acceptance Criteria:**
- [ ] `formatDate(isoString)` → "Jun 15, 2026"
- [ ] `formatRelativeTime(isoString)` → "2 hours ago"
- [ ] `formatDateTime(isoString)` → "Jun 15, 2026 10:30 AM"
- [ ] `rectContainsPoint(rect, point)` → boolean
- [ ] `calculateBoundingBox(points)` → { x, y, width, height }
- [ ] `loadImage(src): Promise<HTMLImageElement>`

**Manual Testing:** Indirect — dates render correctly in UI.

**Notes:** Extract from existing inline code. Small pure functions, easy to test.

---

## Week 6 — Buffer & Release

**Goal:** Fix bugs, test, build installer, release.

---

### T6.01 — Bug fix pass

**Description:**
Chạy full flow 10 lần, ghi lại tất cả bugs, fix.

**Files to create:** None
**Files to modify:** Various (bug-dependent)
**Dependencies:** All previous

**Acceptance Criteria:**
- [ ] Full flow chạy 10 lần liên tiếp không crash
- [ ] Edge cases tested: 0 issues, 20+ issues, 4K screenshot, empty title attempt

**Manual Testing:** Full flow × 10

**Notes:** Keep a bug list, prioritize P0 (crash) trước, P1 (data loss), P2 (UI glitch).

---

### T6.02 — DPI / scaling test

**Description:**
Test app với Windows display scaling 100%, 125%, 150%, 200%. Fix coordinate issues.

**Files to create:** None
**Files to modify:** Canvas composables, overlay positioning

**Dependencies:** T3.01

**Acceptance Criteria:**
- [ ] 125% scaling: marker position đúng, screenshot đúng size
- [ ] 150% scaling: same
- [ ] Canvas coordinates match mouse position at all DPI levels

**Manual Testing:**
1. Windows Settings → Display → Scale → 150%
2. Full flow → markers at correct positions
3. Export → screenshot correct in HTML

**Notes:** Key: `window.devicePixelRatio`, canvas size adjustment, coordinate transform.

---

### T6.03 — Build installer

**Description:**
Run `npm run tauri build`. Test installer. Fix build issues.

**Files to create:**
- `README.md` (nếu chưa có)

**Files to modify:**
- `src-tauri/tauri.conf.json` — bundle config, version, app name
- `package.json` — version

**Dependencies:** All previous

**Acceptance Criteria:**
- [ ] `npm run tauri build` thành công
- [ ] Installer file generated (`.msi` hoặc `.exe`)
- [ ] Installer size < 30MB
- [ ] Install trên máy mới → app chạy
- [ ] Full flow hoạt động trên máy mới (không cần dev tools)
- [ ] Uninstall sạch

**Manual Testing:**
1. Build → installer xuất hiện
2. Copy installer sang máy khác (hoặc VM)
3. Install → chạy → full flow OK

**Notes:**
- Tauri bundle config: set `identifier`, `version`, `shortDescription`
- Windows: Tauri tạo `.msi` + `.exe` installer
- Test trên clean machine quan trọng — đảm bảo không thiếu runtime dependencies

---

### T6.04 — Release

**Description:**
Tạo Git tag, GitHub release, upload installer, viết changelog.

**Files to create:**
- `CHANGELOG.md`

**Files to modify:**
- `README.md` — usage instructions, screenshots

**Dependencies:** T6.03

**Acceptance Criteria:**
- [ ] Git tag `v0.1.0`
- [ ] CHANGELOG.md liệt kê features
- [ ] README.md có: description, screenshot, install instructions, usage, hotkey
- [ ] (Optional) GitHub Release với installer attached

**Manual Testing:** Download installer from release → install → works.

**Notes:** 🎉 MVP shipped!
