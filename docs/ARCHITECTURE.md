# ARCHITECTURE.md — August Mark

> Technical architecture document for August Mark — Desktop review & markup tool.

| Field | Value |
|---|---|
| App ID | `com.august.mark` |
| Stack | Tauri 2 + Vue 3 + Vuetify 3 + TypeScript + Rust + SQLite |
| Target OS | Windows 10/11 (macOS/Linux planned later) |
| Last Updated | 2026-06-15 |

---

## 1. High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Desktop OS (Windows)                    │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐    │
│  │                    August Mark App                    │    │
│  │                                                      │    │
│  │  ┌─────────────────┐    ┌─────────────────────────┐  │    │
│  │  │   Main Window   │    │   Overlay Window        │  │    │
│  │  │   (Dashboard)   │    │   (Annotation Canvas)   │  │    │
│  │  │                 │    │                         │  │    │
│  │  │  Vue 3 +        │    │  Vue 3 + Canvas 2D     │  │    │
│  │  │  Vuetify 3 +    │    │  Transparent,           │  │    │
│  │  │  TypeScript      │    │  Always-on-top,         │  │    │
│  │  │                 │    │  Fullscreen              │  │    │
│  │  └────────┬────────┘    └────────┬────────────────┘  │    │
│  │           │  Tauri IPC (invoke)  │                    │    │
│  │  ┌────────┴──────────────────────┴────────────────┐  │    │
│  │  │              Tauri Rust Backend                 │  │    │
│  │  │                                                │  │    │
│  │  │  ┌──────────┐ ┌──────────┐ ┌───────────────┐  │  │    │
│  │  │  │ Mouse    │ │ Screen   │ │ Database      │  │  │    │
│  │  │  │ Hook     │ │ Capture  │ │ Manager       │  │  │    │
│  │  │  │ Service  │ │ Service  │ │ (SQLite)      │  │  │    │
│  │  │  └──────────┘ └──────────┘ └───────────────┘  │  │    │
│  │  │  ┌──────────┐ ┌──────────┐ ┌───────────────┐  │  │    │
│  │  │  │ File     │ │ Export   │ │ Google Drive  │  │  │    │
│  │  │  │ Storage  │ │ Engine   │ │ Sync (v0.2)   │  │  │    │
│  │  │  │ Service  │ │          │ │               │  │  │    │
│  │  │  └──────────┘ └──────────┘ └───────────────┘  │  │    │
│  │  └────────────────────────────────────────────────┘  │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌──────────────┐  ┌────────────────────┐                    │
│  │ SQLite DB    │  │ Local File Storage  │                    │
│  │ august_mark  │  │ ~/AugustMark/       │                    │
│  │ .db          │  │   screenshots/      │                    │
│  └──────────────┘  │   crops/            │                    │
│                    │   exports/          │                    │
│                    └────────────────────┘                    │
│                                                              │
│  ┌──────────────────────────────────────┐                    │
│  │ Google Drive (v0.2)                  │                    │
│  │ "August Mark" root folder            │                    │
│  │   ├── backups/                       │                    │
│  │   ├── sessions/{id}/                 │                    │
│  │   └── shared/                        │                    │
│  └──────────────────────────────────────┘                    │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow

```
User holds middle mouse ≥ 1s
  → Rust MouseHookService detects hold event
  → Rust ScreenCaptureService captures current monitor
  → Screenshot saved to disk via FileStorageService
  → Tauri event emitted to frontend: "overlay:open"
  → Overlay window opens, loads screenshot from local path
  → User draws annotations on canvas
  → User fills issue forms
  → Frontend sends annotation data via Tauri IPC
  → Rust DatabaseManager persists session + issues
  → Rust FileStorageService saves annotated screenshot + crops
  → Frontend closes overlay
  → Dashboard refreshes via reactive state
```

---

## 2. Frontend Architecture — Vue 3 + Vuetify 3

### 2.1 Tech Stack

| Layer | Technology | Version | Purpose |
|---|---|---|---|
| Framework | Vue 3 | ^3.5 | Composition API, SFC |
| UI Library | Vuetify 3 | ^3.7 | Material Design components |
| Language | TypeScript | ^5.5 | Type safety |
| State | Pinia | ^2.2 | Store management |
| Router | Vue Router | ^4.4 | Multi-page navigation |
| Canvas | HTML5 Canvas 2D | native | Annotation rendering |
| Build | Vite | ^6.0 | Bundling (via Tauri) |

### 2.2 App Structure

Có **2 window contexts** trong Tauri:

1. **Main Window** — Dashboard, settings, issue management.
2. **Overlay Window** — Transparent fullscreen annotation canvas.

Cả 2 window cùng chia sẻ codebase Vue nhưng mount component khác nhau dựa trên URL path:
- `index.html` → Main Window (Dashboard)
- `overlay.html` → Overlay Window (Canvas)

### 2.3 Frontend Module Map

```
src/
├── main.ts                  # Main window entry
├── overlay.ts               # Overlay window entry
├── App.vue                  # Main window root
├── OverlayApp.vue           # Overlay window root
│
├── assets/                  # Static assets (icons, fonts)
│
├── components/
│   ├── common/              # Shared components
│   │   ├── AppHeader.vue
│   │   ├── AppSidebar.vue
│   │   ├── ConfirmDialog.vue
│   │   ├── EmptyState.vue
│   │   ├── LoadingOverlay.vue
│   │   └── StatusBadge.vue
│   │
│   ├── dashboard/           # Dashboard-specific
│   │   ├── SessionList.vue
│   │   ├── SessionCard.vue
│   │   ├── IssueList.vue
│   │   ├── IssueCard.vue
│   │   ├── IssueDetail.vue
│   │   ├── FilterBar.vue
│   │   ├── StatsOverview.vue
│   │   └── ProjectSelector.vue
│   │
│   ├── overlay/             # Overlay-specific
│   │   ├── AnnotationCanvas.vue    # Main canvas component
│   │   ├── AnnotationToolbar.vue   # Tool selection bar
│   │   ├── IssueFormPanel.vue      # Side panel for issue input
│   │   ├── MarkerLayer.vue         # Number marker rendering
│   │   └── OverlayStatusBar.vue    # Top status (session info, done btn)
│   │
│   ├── settings/            # Settings page
│   │   ├── GeneralSettings.vue
│   │   ├── HotkeySettings.vue
│   │   ├── StorageSettings.vue
│   │   └── GoogleDriveSettings.vue
│   │
│   └── export/              # Export-related
│       ├── ExportDialog.vue
│       └── ExportPreview.vue
│
├── composables/             # Vue composables (hooks)
│   ├── useAnnotation.ts     # Annotation state & logic
│   ├── useCanvas.ts         # Canvas rendering helpers
│   ├── useSession.ts        # Session CRUD
│   ├── useIssue.ts          # Issue CRUD
│   ├── useProject.ts        # Project CRUD
│   ├── useExport.ts         # Export actions
│   ├── useTauriEvents.ts    # Tauri event listeners
│   └── useKeyboard.ts       # Keyboard shortcuts
│
├── stores/                  # Pinia stores
│   ├── sessionStore.ts      # Sessions state
│   ├── issueStore.ts        # Issues state
│   ├── projectStore.ts      # Projects state
│   ├── overlayStore.ts      # Overlay state (tools, markers, current screenshot)
│   ├── settingsStore.ts     # App settings
│   └── uiStore.ts           # UI state (sidebar, dialogs, loading)
│
├── router/
│   └── index.ts             # Vue Router config
│
├── types/                   # TypeScript types
│   ├── session.ts
│   ├── issue.ts
│   ├── project.ts
│   ├── annotation.ts
│   ├── settings.ts
│   └── tauri.ts             # Tauri command types
│
├── utils/                   # Pure utility functions
│   ├── date.ts
│   ├── color.ts
│   ├── geometry.ts          # Point, rect, intersection helpers
│   ├── image.ts             # Crop, resize helpers
│   └── validators.ts
│
├── services/                # Frontend services (Tauri bridge)
│   ├── tauriCommands.ts     # All Tauri invoke() wrappers
│   └── tauriEvents.ts       # All Tauri event listeners
│
└── plugins/
    ├── vuetify.ts           # Vuetify config
    └── pinia.ts             # Pinia config
```

### 2.4 Vuetify 3 Theme

```typescript
// plugins/vuetify.ts
import { createVuetify } from 'vuetify'

export default createVuetify({
  theme: {
    defaultTheme: 'augustDark',
    themes: {
      augustDark: {
        dark: true,
        colors: {
          background: '#0F1117',
          surface: '#1A1D27',
          'surface-variant': '#252836',
          primary: '#FF6B35',        // August orange
          secondary: '#4ECDC4',      // Teal accent
          error: '#FF4757',          // Critical/Bug
          warning: '#FFA502',        // Major
          info: '#3742FA',           // Info
          success: '#2ED573',        // Resolved
          'on-background': '#E8E8E8',
          'on-surface': '#E8E8E8',
        },
      },
      augustLight: {
        dark: false,
        colors: {
          background: '#FAFBFC',
          surface: '#FFFFFF',
          'surface-variant': '#F1F3F5',
          primary: '#E85D26',
          secondary: '#3DB8AD',
          error: '#E8384F',
          warning: '#E89B26',
          info: '#2B35D8',
          success: '#27B864',
          'on-background': '#1A1D27',
          'on-surface': '#1A1D27',
        },
      },
    },
  },
})
```

### 2.5 Annotation Canvas Architecture

Canvas overlay sử dụng **layered rendering**:

```
┌──────────────────────────────────────┐
│  Layer 4: UI Elements (HTML/Vue)     │  ← Toolbar, issue form (HTML overlay)
├──────────────────────────────────────┤
│  Layer 3: Active Drawing (Canvas)    │  ← Annotation đang vẽ (realtime)
├──────────────────────────────────────┤
│  Layer 2: Committed Markers (Canvas) │  ← Các marker đã confirm
├──────────────────────────────────────┤
│  Layer 1: Screenshot (Canvas)        │  ← Ảnh chụp màn hình (static)
└──────────────────────────────────────┘
```

Dùng **3 canvas elements** chồng lên nhau + HTML overlay cho UI:

```typescript
// composables/useCanvas.ts

interface CanvasLayers {
  screenshotCanvas: HTMLCanvasElement  // Layer 1: static screenshot
  markerCanvas: HTMLCanvasElement      // Layer 2: committed annotations
  drawingCanvas: HTMLCanvasElement     // Layer 3: active drawing (cleared on each frame)
}

interface AnnotationTool {
  type: 'marker' | 'rectangle' | 'arrow' | 'freedraw' | 'text' | 'highlight' | 'blur'
  onMouseDown(point: Point): void
  onMouseMove(point: Point): void
  onMouseUp(point: Point): void
  render(ctx: CanvasRenderingContext2D): void
}
```

---

## 3. Tauri / Rust Backend Architecture

### 3.1 Crate Dependencies

```toml
# src-tauri/Cargo.toml

[dependencies]
tauri = { version = "2", features = ["tray-icon", "image-png"] }
tauri-plugin-dialog = "2"
tauri-plugin-fs = "2"
tauri-plugin-shell = "2"
tauri-plugin-notification = "2"

# Database
rusqlite = { version = "0.32", features = ["bundled"] }

# Screenshot
xcap = "0.3"                    # Cross-platform screen capture

# Mouse hook
rdev = "0.5"                    # Global input event listener

# Image processing
image = "0.25"                  # Image manipulation (crop, resize, encode)

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Date/time
chrono = { version = "0.4", features = ["serde"] }

# UUID
uuid = { version = "1", features = ["v4", "serde"] }

# Error handling
thiserror = "2"
anyhow = "1"

# Async runtime (Tauri dùng Tokio internally)
tokio = { version = "1", features = ["full"] }

# Logging
log = "0.4"
env_logger = "0.11"

# Google Drive (v0.2 - optional)
# reqwest = { version = "0.12", features = ["json"] }
# oauth2 = "4"
```

### 3.2 Rust Module Structure

```
src-tauri/
├── Cargo.toml
├── tauri.conf.json
├── build.rs
├── icons/
│
└── src/
    ├── main.rs                    # Entry point, Tauri builder
    ├── lib.rs                     # Module declarations
    ├── error.rs                   # Custom error types
    ├── state.rs                   # AppState (shared state across commands)
    │
    ├── commands/                  # Tauri command handlers (IPC endpoints)
    │   ├── mod.rs
    │   ├── session_commands.rs    # Session CRUD
    │   ├── issue_commands.rs      # Issue CRUD
    │   ├── project_commands.rs    # Project CRUD
    │   ├── capture_commands.rs    # Screenshot + overlay control
    │   ├── export_commands.rs     # Export HTML/MD/PDF/CSV
    │   ├── settings_commands.rs   # App settings
    │   └── gdrive_commands.rs     # Google Drive (v0.2)
    │
    ├── services/                  # Business logic services
    │   ├── mod.rs
    │   ├── mouse_hook.rs          # Global mouse hook (rdev)
    │   ├── screen_capture.rs      # Screenshot capture (xcap)
    │   ├── image_processor.rs     # Crop, annotate, compress
    │   ├── file_storage.rs        # File I/O, path management
    │   ├── export_engine.rs       # HTML/MD/PDF/CSV generation
    │   └── gdrive_sync.rs         # Google Drive API (v0.2)
    │
    ├── database/                  # Database layer
    │   ├── mod.rs
    │   ├── connection.rs          # SQLite connection manager
    │   ├── migrations.rs          # Schema migrations
    │   ├── repositories/          # Data access layer
    │   │   ├── mod.rs
    │   │   ├── session_repo.rs
    │   │   ├── issue_repo.rs
    │   │   ├── project_repo.rs
    │   │   ├── tag_repo.rs
    │   │   └── settings_repo.rs
    │   └── models.rs              # Database models (struct ↔ row mapping)
    │
    ├── models/                    # Domain models (shared between layers)
    │   ├── mod.rs
    │   ├── session.rs
    │   ├── issue.rs
    │   ├── project.rs
    │   ├── capture.rs
    │   ├── annotation.rs
    │   └── settings.rs
    │
    └── utils/                     # Utility functions
        ├── mod.rs
        ├── paths.rs               # Path resolution helpers
        ├── datetime.rs            # Date formatting
        └── id.rs                  # UUID generation
```

### 3.3 AppState (Shared State)

```rust
// src/state.rs

use std::sync::Mutex;
use rusqlite::Connection;

pub struct AppState {
    pub db: Mutex<Connection>,
    pub app_data_dir: std::path::PathBuf,
    pub is_overlay_active: Mutex<bool>,
    pub active_session_id: Mutex<Option<String>>,
}

impl AppState {
    pub fn new(app_data_dir: std::path::PathBuf) -> anyhow::Result<Self> {
        let db_path = app_data_dir.join("august_mark.db");
        let conn = Connection::open(&db_path)?;

        // Enable WAL mode for better concurrent read performance
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;

        Ok(Self {
            db: Mutex::new(conn),
            app_data_dir,
            is_overlay_active: Mutex::new(false),
            active_session_id: Mutex::new(None),
        })
    }
}
```

### 3.4 Mouse Hook Service

```rust
// src/services/mouse_hook.rs

use rdev::{listen, Event, EventType, Button};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

const HOLD_DURATION: Duration = Duration::from_millis(1000);

pub struct MouseHookService {
    is_running: Arc<AtomicBool>,
}

impl MouseHookService {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Spawn mouse hook on a dedicated thread.
    /// Emits "overlay:trigger" event to frontend when middle mouse
    /// is held for ≥ 1 second.
    pub fn start(&self, app_handle: AppHandle) {
        let is_running = self.is_running.clone();
        is_running.store(true, Ordering::SeqCst);

        std::thread::spawn(move || {
            let mut middle_press_time: Option<Instant> = None;
            let mut triggered = false;

            // rdev::listen blocks the thread
            if let Err(e) = listen(move |event: Event| {
                match event.event_type {
                    EventType::ButtonPress(Button::Middle) => {
                        middle_press_time = Some(Instant::now());
                        triggered = false;

                        // Start a timer thread to check hold duration
                        let app = app_handle.clone();
                        std::thread::spawn(move || {
                            std::thread::sleep(HOLD_DURATION);
                            // The actual trigger check happens in ButtonRelease
                            // or via periodic check — see implementation note below
                            let _ = app.emit("overlay:trigger", ());
                        });
                    }
                    EventType::ButtonRelease(Button::Middle) => {
                        if let Some(press_time) = middle_press_time.take() {
                            if press_time.elapsed() < HOLD_DURATION {
                                // Released too early — cancel trigger
                                // Frontend should ignore trigger if release came first
                                let _ = app_handle.emit("overlay:cancel", ());
                            }
                        }
                        triggered = false;
                    }
                    _ => {}
                }
            }) {
                log::error!("Mouse hook error: {:?}", e);
            }
        });
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }
}
```

> **⚠️ Implementation Note:** `rdev` có thể có vấn đề trên một số Windows build. Nếu gặp sự cố:
> - **Fallback 1:** Dùng `winapi` crate trực tiếp với `SetWindowsHookExW` (low-level mouse hook).
> - **Fallback 2:** Dùng `device_query` crate (polling-based, đơn giản hơn nhưng tốn CPU hơn).
> - **Fallback 3:** Bỏ middle mouse hold, dùng global hotkey `Ctrl+Shift+M` qua Tauri's global shortcut API.

### 3.5 Screen Capture Service

```rust
// src/services/screen_capture.rs

use xcap::Monitor;
use image::RgbaImage;
use std::path::PathBuf;

pub struct ScreenCaptureService;

impl ScreenCaptureService {
    /// Capture the monitor that currently contains the mouse cursor.
    pub fn capture_current_monitor() -> anyhow::Result<(RgbaImage, MonitorInfo)> {
        let monitors = Monitor::all()?;
        let cursor_pos = Self::get_cursor_position()?;

        let target_monitor = monitors
            .into_iter()
            .find(|m| {
                let x = m.x();
                let y = m.y();
                let w = m.width() as i32;
                let h = m.height() as i32;
                cursor_pos.0 >= x && cursor_pos.0 < x + w
                    && cursor_pos.1 >= y && cursor_pos.1 < y + h
            })
            .unwrap_or_else(|| Monitor::all().unwrap().remove(0)); // Fallback: primary

        let image = target_monitor.capture_image()?;

        let info = MonitorInfo {
            x: target_monitor.x(),
            y: target_monitor.y(),
            width: target_monitor.width(),
            height: target_monitor.height(),
            scale_factor: target_monitor.scale_factor(),
            name: target_monitor.name().to_string(),
        };

        Ok((image, info))
    }

    /// Save captured image to disk as PNG.
    pub fn save_screenshot(image: &RgbaImage, path: &PathBuf) -> anyhow::Result<()> {
        image.save(path)?;
        Ok(())
    }

    /// Crop a region from the screenshot for an individual issue.
    pub fn crop_region(
        image: &RgbaImage,
        x: u32, y: u32,
        width: u32, height: u32,
        padding: u32,
    ) -> anyhow::Result<RgbaImage> {
        let px = x.saturating_sub(padding);
        let py = y.saturating_sub(padding);
        let pw = (width + padding * 2).min(image.width() - px);
        let ph = (height + padding * 2).min(image.height() - py);

        let cropped = image::imageops::crop_imm(image, px, py, pw, ph).to_image();
        Ok(cropped)
    }

    #[cfg(target_os = "windows")]
    fn get_cursor_position() -> anyhow::Result<(i32, i32)> {
        use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
        use windows::Win32::Foundation::POINT;

        let mut point = POINT::default();
        unsafe { GetCursorPos(&mut point)? };
        Ok((point.x, point.y))
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MonitorInfo {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
    pub name: String,
}
```

---

## 4. Module / Service Breakdown

| Module | Responsibility | Layer | MVP? |
|---|---|---|---|
| `MouseHookService` | Detect global middle mouse hold | Rust Service | ✅ |
| `ScreenCaptureService` | Capture monitor screenshot | Rust Service | ✅ |
| `ImageProcessor` | Crop, annotate, compress images | Rust Service | ✅ |
| `FileStorageService` | Save/read/delete files on disk | Rust Service | ✅ |
| `DatabaseManager` | SQLite connection, migrations | Rust Database | ✅ |
| `SessionRepository` | Session CRUD | Rust Database | ✅ |
| `IssueRepository` | Issue CRUD | Rust Database | ✅ |
| `ProjectRepository` | Project CRUD | Rust Database | ✅ |
| `TagRepository` | Tag CRUD + autocomplete | Rust Database | ✅ |
| `ExportEngine` | Generate HTML/MD/PDF/CSV reports | Rust Service | ✅ (HTML/MD) |
| `GoogleDriveSync` | OAuth2, upload, download, share | Rust Service | ❌ v0.2 |
| `AnnotationCanvas` | Canvas rendering + tool handling | Vue Component | ✅ |
| `OverlayStore` | Overlay state management | Vue Store | ✅ |
| `SessionStore` | Session list + active session | Vue Store | ✅ |
| `IssueStore` | Issue list + filter/search | Vue Store | ✅ |

---

## 5. SQLite Schema

### 5.1 Database: `august_mark.db`

```sql
-- ============================================================
-- Migration v001: Initial schema
-- ============================================================

-- Schema version tracking
CREATE TABLE IF NOT EXISTS schema_version (
    version     INTEGER PRIMARY KEY,
    applied_at  TEXT NOT NULL DEFAULT (datetime('now')),
    description TEXT
);

INSERT INTO schema_version (version, description)
VALUES (1, 'Initial schema');

-- ============================================================
-- Projects
-- ============================================================
CREATE TABLE projects (
    id          TEXT PRIMARY KEY,              -- UUID v4
    name        TEXT NOT NULL,
    description TEXT DEFAULT '',
    color       TEXT DEFAULT '#FF6B35',        -- Hex color for UI
    is_archived INTEGER NOT NULL DEFAULT 0,   -- 0 = active, 1 = archived
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Default project
INSERT INTO projects (id, name, description, color)
VALUES ('default', 'Default Project', 'Uncategorized reviews', '#FF6B35');

-- ============================================================
-- Sessions (a review session containing multiple captures)
-- ============================================================
CREATE TABLE sessions (
    id          TEXT PRIMARY KEY,              -- UUID v4
    project_id  TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    title       TEXT NOT NULL,
    description TEXT DEFAULT '',
    status      TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'completed', 'archived')),
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now')),
    completed_at TEXT                          -- When session was ended
);

CREATE INDEX idx_sessions_project ON sessions(project_id);
CREATE INDEX idx_sessions_status ON sessions(status);
CREATE INDEX idx_sessions_created ON sessions(created_at DESC);

-- ============================================================
-- Captures (one overlay activation = one screenshot)
-- ============================================================
CREATE TABLE captures (
    id              TEXT PRIMARY KEY,          -- UUID v4
    session_id      TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    screenshot_path TEXT NOT NULL,             -- Relative path from app data dir
    monitor_name    TEXT,
    monitor_x       INTEGER,
    monitor_y       INTEGER,
    monitor_width   INTEGER,
    monitor_height  INTEGER,
    scale_factor    REAL DEFAULT 1.0,
    window_title    TEXT,                      -- Foreground window title at capture time
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_captures_session ON captures(session_id);

-- ============================================================
-- Issues (individual markers/annotations on a capture)
-- ============================================================
CREATE TABLE issues (
    id              TEXT PRIMARY KEY,          -- UUID v4
    capture_id      TEXT NOT NULL REFERENCES captures(id) ON DELETE CASCADE,
    session_id      TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE, -- Denormalized for fast query
    project_id      TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE, -- Denormalized for fast query

    -- Issue metadata
    marker_number   INTEGER NOT NULL,          -- ①②③ display number within capture
    title           TEXT NOT NULL,
    description     TEXT DEFAULT '',
    issue_type      TEXT NOT NULL DEFAULT 'Bug'
                    CHECK (issue_type IN ('Bug', 'UI', 'UX', 'Suggestion', 'Requirement', 'Question')),
    severity        TEXT NOT NULL DEFAULT 'Minor'
                    CHECK (severity IN ('Critical', 'Major', 'Minor', 'Info')),
    status          TEXT NOT NULL DEFAULT 'Open'
                    CHECK (status IN ('Draft', 'Open', 'In Progress', 'Resolved', 'Closed')),

    -- Annotation position on screenshot (in screenshot pixels)
    marker_x        REAL NOT NULL,             -- Center X of marker
    marker_y        REAL NOT NULL,             -- Center Y of marker

    -- Annotation geometry (JSON string for flexibility)
    -- Format depends on annotation type:
    -- marker:    {"type": "marker"}
    -- rectangle: {"type": "rect", "x": 0, "y": 0, "width": 100, "height": 50}
    -- arrow:     {"type": "arrow", "x1": 0, "y1": 0, "x2": 100, "y2": 50}
    -- freedraw:  {"type": "freedraw", "points": [[0,0],[1,1],...]}
    -- text:      {"type": "text", "text": "...", "fontSize": 16}
    -- highlight: {"type": "highlight", "x": 0, "y": 0, "width": 100, "height": 50}
    -- blur:      {"type": "blur", "x": 0, "y": 0, "width": 100, "height": 50}
    annotation_data TEXT NOT NULL DEFAULT '{}',

    -- Annotation style
    color           TEXT DEFAULT '#FF6B35',
    stroke_width    REAL DEFAULT 2.0,

    -- Crop image
    crop_path       TEXT,                      -- Relative path, NULL if not yet generated

    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_issues_capture ON issues(capture_id);
CREATE INDEX idx_issues_session ON issues(session_id);
CREATE INDEX idx_issues_project ON issues(project_id);
CREATE INDEX idx_issues_type ON issues(issue_type);
CREATE INDEX idx_issues_severity ON issues(severity);
CREATE INDEX idx_issues_status ON issues(status);

-- ============================================================
-- Tags
-- ============================================================
CREATE TABLE tags (
    id      TEXT PRIMARY KEY,                  -- UUID v4
    name    TEXT NOT NULL UNIQUE,
    color   TEXT DEFAULT '#4ECDC4',
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Many-to-many: issues ↔ tags
CREATE TABLE issue_tags (
    issue_id TEXT NOT NULL REFERENCES issues(id) ON DELETE CASCADE,
    tag_id   TEXT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (issue_id, tag_id)
);

CREATE INDEX idx_issue_tags_tag ON issue_tags(tag_id);

-- ============================================================
-- Settings (key-value store)
-- ============================================================
CREATE TABLE settings (
    key         TEXT PRIMARY KEY,
    value       TEXT NOT NULL,
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Default settings
INSERT INTO settings (key, value) VALUES
    ('theme', '"dark"'),
    ('overlay_trigger', '"middle_mouse_hold"'),
    ('hold_duration_ms', '1000'),
    ('screenshot_quality', '90'),
    ('default_project_id', '"default"'),
    ('auto_backup', 'false'),
    ('gdrive_connected', 'false');

-- ============================================================
-- Sync tracking (v0.2)
-- ============================================================
CREATE TABLE sync_log (
    id              TEXT PRIMARY KEY,
    entity_type     TEXT NOT NULL,              -- 'session', 'issue', 'screenshot'
    entity_id       TEXT NOT NULL,
    action          TEXT NOT NULL,              -- 'upload', 'download', 'delete'
    gdrive_file_id  TEXT,
    status          TEXT NOT NULL DEFAULT 'pending',
    error_message   TEXT,
    synced_at       TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);
```

### 5.2 Migration Strategy

```rust
// src/database/migrations.rs

const MIGRATIONS: &[Migration] = &[
    Migration { version: 1, description: "Initial schema", sql: include_str!("../../migrations/v001_initial.sql") },
    // Future:
    // Migration { version: 2, description: "Add attachments", sql: include_str!("../../migrations/v002_attachments.sql") },
];

pub fn run_migrations(conn: &Connection) -> anyhow::Result<()> {
    let current_version = get_current_version(conn)?;

    for migration in MIGRATIONS {
        if migration.version > current_version {
            conn.execute_batch(migration.sql)?;
            log::info!("Applied migration v{}: {}", migration.version, migration.description);
        }
    }

    Ok(())
}
```

---

## 6. File Storage Structure

```
~/AugustMark/                              # User's app data root (configurable)
├── august_mark.db                         # SQLite database
├── screenshots/                           # Full screenshots
│   ├── 2026/
│   │   ├── 06/
│   │   │   ├── 15/
│   │   │   │   ├── {capture_id}.png       # Original screenshot
│   │   │   │   └── {capture_id}_annotated.png  # Annotated version
│   │   │   └── ...
│   │   └── ...
│   └── ...
├── crops/                                 # Individual issue crops
│   ├── 2026/
│   │   ├── 06/
│   │   │   ├── 15/
│   │   │   │   ├── {issue_id}.png
│   │   │   │   └── ...
│   │   │   └── ...
│   │   └── ...
│   └── ...
├── exports/                               # Generated reports
│   ├── {session_id}_report.html
│   ├── {session_id}_report.md
│   └── ...
├── backups/                               # Database backups
│   ├── august_mark_2026-06-15_100000.db
│   └── ...
└── logs/                                  # App logs
    ├── august_mark_2026-06-15.log
    └── ...
```

### Path Resolution

```rust
// src/utils/paths.rs

pub fn screenshots_dir(base: &Path, date: &NaiveDate) -> PathBuf {
    base.join("screenshots")
        .join(date.format("%Y").to_string())
        .join(date.format("%m").to_string())
        .join(date.format("%d").to_string())
}

pub fn crops_dir(base: &Path, date: &NaiveDate) -> PathBuf {
    base.join("crops")
        .join(date.format("%Y").to_string())
        .join(date.format("%m").to_string())
        .join(date.format("%d").to_string())
}

pub fn screenshot_path(base: &Path, capture_id: &str, date: &NaiveDate) -> PathBuf {
    screenshots_dir(base, date).join(format!("{}.png", capture_id))
}

pub fn crop_path(base: &Path, issue_id: &str, date: &NaiveDate) -> PathBuf {
    crops_dir(base, date).join(format!("{}.png", issue_id))
}
```

---

## 7. Google Drive Sync Design (v0.2)

### 7.1 Architecture

```
┌─────────────────┐     OAuth2     ┌─────────────────┐
│  August Mark     │ ←──────────→  │  Google OAuth    │
│  Desktop App     │               │  Server          │
└────────┬────────┘               └─────────────────┘
         │
         │  Google Drive API v3
         │
┌────────┴────────────────────────────┐
│  Google Drive                        │
│  └── August Mark/                    │  ← Root folder (auto-created)
│      ├── backups/                    │
│      │   └── august_mark_{date}.db   │
│      ├── sessions/                   │
│      │   └── {session_id}/           │
│      │       ├── metadata.json       │
│      │       ├── report.html         │
│      │       ├── screenshots/        │
│      │       │   ├── {capture_id}.png│
│      │       │   └── ...             │
│      │       └── crops/              │
│      │           ├── {issue_id}.png  │
│      │           └── ...             │
│      └── shared/                     │
│          └── {share_package_id}/     │
│              ├── index.html          │  ← Self-contained viewable report
│              └── assets/             │
└──────────────────────────────────────┘
```

### 7.2 OAuth2 Flow (Desktop App)

```
1. User clicks "Connect Google Drive" in Settings
2. App starts local HTTP server on random port (e.g., 127.0.0.1:9876)
3. App opens system browser with Google OAuth URL:
   - client_id = {from Google Cloud Console}
   - redirect_uri = http://127.0.0.1:9876/callback
   - scope = https://www.googleapis.com/auth/drive.file
   - response_type = code
4. User grants permission in browser
5. Browser redirects to local server with auth code
6. App exchanges code for access_token + refresh_token
7. Tokens stored in SQLite settings (encrypted)
8. Local HTTP server stops
```

> **Scope note:** `drive.file` chỉ cho phép truy cập file do app tạo ra, không toàn bộ Drive. An toàn hơn `drive` scope.

### 7.3 Sync Strategy

- **One-way upload** (MVP cho v0.2): Local → Drive.
- **Conflict resolution:** Local wins (local là source of truth).
- **Sync granularity:** Per-session (user chọn session để sync).
- **Không auto-sync:** User chủ động trigger sync.
- **Idempotent:** Re-sync cùng session chỉ upload file mới/thay đổi.

### 7.4 Share Package

```json
// metadata.json — included in share package
{
  "app": "August Mark",
  "version": "0.2.0",
  "session": {
    "id": "uuid",
    "title": "Homepage Review",
    "project": "Website Redesign",
    "created_at": "2026-06-15T10:00:00Z",
    "issue_count": 8
  },
  "issues": [
    {
      "id": "uuid",
      "marker_number": 1,
      "title": "Button color mismatch",
      "type": "UI",
      "severity": "Minor",
      "status": "Open",
      "screenshot": "screenshots/capture_001.png",
      "crop": "crops/issue_001.png",
      "marker_x": 450,
      "marker_y": 320
    }
  ]
}
```

---

## 8. Overlay Window Design

### 8.1 Tauri Window Configuration

```json
// tauri.conf.json (relevant overlay window config)
{
  "app": {
    "windows": [
      {
        "label": "main",
        "title": "August Mark",
        "width": 1280,
        "height": 800,
        "center": true,
        "resizable": true,
        "decorations": true
      }
    ]
  }
}
```

Overlay window được tạo **dynamically** khi trigger, không khai báo trong config:

```rust
// src/commands/capture_commands.rs

use tauri::{AppHandle, Manager, WebviewWindowBuilder, WebviewUrl};

#[tauri::command]
pub async fn open_overlay(app: AppHandle, capture_id: String) -> Result<(), String> {
    // Determine target monitor (where cursor is)
    let monitor = app.primary_monitor()
        .map_err(|e| e.to_string())?
        .ok_or("No monitor found")?;

    let position = monitor.position();
    let size = monitor.size();

    // Create overlay window
    let overlay = WebviewWindowBuilder::new(
        &app,
        "overlay",
        WebviewUrl::App("overlay.html".into()),
    )
    .title("August Mark Overlay")
    .position(position.x as f64, position.y as f64)
    .inner_size(size.width as f64, size.height as f64)
    .decorations(false)
    .transparent(true)
    .always_on_top(true)
    .skip_taskbar(true)
    .focused(true)
    .resizable(false)
    .build()
    .map_err(|e| e.to_string())?;

    // Pass capture_id to overlay via event
    overlay.emit("overlay:init", &capture_id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn close_overlay(app: AppHandle) -> Result<(), String> {
    if let Some(overlay) = app.get_webview_window("overlay") {
        overlay.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}
```

### 8.2 Overlay Lifecycle

```
                    ┌─────────────┐
                    │   Inactive   │
                    └──────┬──────┘
                           │ Middle mouse hold ≥ 1s
                           ▼
                    ┌─────────────┐
                    │  Capturing   │  ← Screenshot taken
                    └──────┬──────┘
                           │ Screenshot saved
                           ▼
                    ┌─────────────┐
                    │   Opening    │  ← Window creating
                    └──────┬──────┘
                           │ Window ready
                           ▼
                    ┌─────────────┐
                    │   Active     │  ← User annotating
                    │             │
                    │ Tools:      │
                    │ - Marker    │
                    │ - Rect      │
                    │ - Arrow     │
                    │ - Draw      │
                    │ - Text      │
                    └──────┬──────┘
                           │ User clicks "Done" or Esc
                           ▼
                    ┌─────────────┐
                    │   Saving     │  ← Persist to DB + files
                    └──────┬──────┘
                           │ Data saved
                           ▼
                    ┌─────────────┐
                    │   Closing    │  ← Window destroyed
                    └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │   Inactive   │  ← Ready for next trigger
                    └─────────────┘
```

### 8.3 Overlay UI Layout

```
┌────────────────────────────────────────────────────────────────────┐
│ ┌──────────────────────────────────────────────────────────────┐  │
│ │  Status Bar: "Session: Homepage Review  |  5 issues  |  Done │  │
│ └──────────────────────────────────────────────────────────────┘  │
│                                                                    │
│                                                                    │
│         ┌────────────────────────────────────────────┐            │
│         │                                            │            │
│         │                                            │            │
│         │           Screenshot (Canvas)              │  ┌──────┐ │
│         │                                            │  │Issue │ │
│         │       ① ─────────────────────┐             │  │Form  │ │
│         │         ┌───────────────────┐│             │  │Panel │ │
│         │       ② │  Rectangle       ││             │  │      │ │
│         │         └───────────────────┘│             │  │Title │ │
│         │                   ③ ←────────┘             │  │Desc  │ │
│         │                                            │  │Type  │ │
│         │                                            │  │Sev.  │ │
│         │                                            │  │Tags  │ │
│         │                                            │  │      │ │
│         │                                            │  │[Save]│ │
│         └────────────────────────────────────────────┘  └──────┘ │
│                                                                    │
│  ┌────────────────────────────────────────┐                        │
│  │ Toolbar: [①] [▭] [→] [✎] [T] [█] [◻] │                        │
│  │         marker rect arrow draw text    │                        │
│  │               highlight blur           │                        │
│  │         [Color] [Size]    [Undo] [Redo]│                        │
│  └────────────────────────────────────────┘                        │
└────────────────────────────────────────────────────────────────────┘
```

---

## 9. Global Middle Mouse Hold Trigger Design

### 9.1 Implementation Approach

```
Approach 1 (Primary): rdev crate — global input listener
  ├── Pro: Cross-platform, pure Rust
  ├── Pro: Non-blocking listener in dedicated thread
  ├── Con: May require accessibility permissions on macOS
  └── Con: Some antivirus may flag it

Approach 2 (Fallback): Windows Raw Input API via winapi
  ├── Pro: Native Windows, no 3rd-party dependency
  ├── Pro: Most reliable on Windows
  ├── Con: Windows-only
  └── Con: More boilerplate code

Approach 3 (Last resort): Global hotkey via Tauri
  ├── Pro: Built into Tauri, zero extra dependencies
  ├── Pro: Works everywhere Tauri works
  ├── Con: Not middle mouse hold — different UX
  └── Con: User must remember keyboard shortcut
```

### 9.2 Hold Detection Logic

```
Timeline:
──────────────────────────────────────────────────────→ time

  Middle Press         1000ms             Middle Release
      │                   │                    │
      ▼                   ▼                    ▼
  ┌───────────────────────────────────────────────┐
  │ t=0    start timer    t=1000ms               │
  │  │                      │                     │
  │  │                      ├─ IF still held:     │
  │  │                      │   → TRIGGER overlay │
  │  │                      │                     │
  │  │                      ├─ IF released before: │
  │  │                      │   → CANCEL, pass     │
  │  │                      │     through as       │
  │  │                      │     normal click     │
  └───────────────────────────────────────────────┘
```

### 9.3 Conflict Avoidance

- Middle mouse click (scroll click) < 200ms → ignored, pass through.
- Middle mouse hold 200ms-999ms → no action, pass through.
- Middle mouse hold ≥ 1000ms → trigger overlay.
- Khi overlay đang active → middle mouse không trigger lại.
- Khi app đang trong settings/export dialog → không trigger.

---

## 10. Screenshot + Annotation Pipeline

### 10.1 Capture Pipeline

```
Step 1: Trigger
  Mouse hook detects hold ≥ 1s
  → Emit event "overlay:trigger"

Step 2: Capture
  Rust ScreenCaptureService.capture_current_monitor()
  → Returns RgbaImage + MonitorInfo
  → Save PNG to: screenshots/{date}/{capture_id}.png
  → Create Capture record in SQLite

Step 3: Open Overlay
  Create Tauri window (fullscreen, transparent, always-on-top)
  → Load overlay.html
  → Send capture_id + screenshot_path to frontend

Step 4: Load Screenshot
  Frontend loads screenshot via Tauri asset protocol:
    convertFileSrc(screenshot_path)
  → Draw onto Layer 1 canvas

Step 5: Annotate
  User interacts with canvas tools
  → Each tool creates AnnotationData object
  → Committed annotations rendered on Layer 2
  → Active drawing on Layer 3

Step 6: Save (on "Done")
  Frontend sends to Rust:
    - List of issues with annotation_data
    - Annotated canvas as data URL (for annotated screenshot)
  Rust:
    - Save annotated screenshot: {capture_id}_annotated.png
    - For each issue:
      - Calculate crop region from annotation bounds
      - Save crop: crops/{date}/{issue_id}.png
      - Insert Issue record in SQLite
  → Close overlay window
  → Emit "dashboard:refresh"
```

### 10.2 Annotation Data Format

```typescript
// types/annotation.ts

interface Point {
  x: number
  y: number
}

interface BaseAnnotation {
  id: string
  type: AnnotationType
  color: string
  strokeWidth: number
}

type AnnotationType = 'marker' | 'rect' | 'arrow' | 'freedraw' | 'text' | 'highlight' | 'blur'

interface MarkerAnnotation extends BaseAnnotation {
  type: 'marker'
  position: Point
  number: number
}

interface RectAnnotation extends BaseAnnotation {
  type: 'rect'
  topLeft: Point
  width: number
  height: number
}

interface ArrowAnnotation extends BaseAnnotation {
  type: 'arrow'
  start: Point
  end: Point
}

interface FreeDrawAnnotation extends BaseAnnotation {
  type: 'freedraw'
  points: Point[]
}

interface TextAnnotation extends BaseAnnotation {
  type: 'text'
  position: Point
  text: string
  fontSize: number
}

interface HighlightAnnotation extends BaseAnnotation {
  type: 'highlight'
  topLeft: Point
  width: number
  height: number
  opacity: number  // 0.3 default
}

interface BlurAnnotation extends BaseAnnotation {
  type: 'blur'
  topLeft: Point
  width: number
  height: number
  blurRadius: number  // pixels
}

type Annotation =
  | MarkerAnnotation
  | RectAnnotation
  | ArrowAnnotation
  | FreeDrawAnnotation
  | TextAnnotation
  | HighlightAnnotation
  | BlurAnnotation
```

### 10.3 Image Processing Pipeline (Rust)

```rust
// src/services/image_processor.rs

use image::{RgbaImage, Rgba, imageops};

pub struct ImageProcessor;

impl ImageProcessor {
    /// Generate annotated screenshot by drawing annotations on the original.
    /// Used for export/report (overlay annotations burned into image).
    pub fn render_annotations(
        base: &RgbaImage,
        annotations: &[AnnotationData],
    ) -> RgbaImage {
        let mut result = base.clone();

        for ann in annotations {
            match ann {
                AnnotationData::Marker { position, number, color } => {
                    Self::draw_marker(&mut result, position, *number, color);
                }
                AnnotationData::Rect { top_left, width, height, color, stroke_width } => {
                    Self::draw_rectangle(&mut result, top_left, *width, *height, color, *stroke_width);
                }
                // ... other annotation types
            }
        }

        result
    }

    /// Crop region around a marker with padding.
    /// Returns the cropped image.
    pub fn crop_for_issue(
        base: &RgbaImage,
        center: (u32, u32),
        annotation_bounds: (u32, u32, u32, u32), // x, y, w, h
        padding: u32,
    ) -> RgbaImage {
        let (ax, ay, aw, ah) = annotation_bounds;
        let x = ax.saturating_sub(padding);
        let y = ay.saturating_sub(padding);
        let w = (aw + padding * 2).min(base.width() - x);
        let h = (ah + padding * 2).min(base.height() - y);

        imageops::crop_imm(base, x, y, w, h).to_image()
    }
}
```

---

## 11. Multi-Monitor Considerations

### MVP (v0.1)
- Capture **only the monitor containing the mouse cursor** when trigger fires.
- Overlay window positioned and sized to match that specific monitor.
- Tọa độ annotation = tọa độ tương đối trên screenshot (0,0 = top-left of captured monitor).
- `MonitorInfo` lưu vào `captures` table để biết monitor nào đã capture.

### Known Issues & Mitigations

| Issue | Mitigation |
|---|---|
| HiDPI scaling khác nhau giữa monitors | Lưu `scale_factor`, scale tọa độ khi render |
| Monitor arrangement (left/right/top/bottom) | Dùng absolute coordinates từ OS, không assume layout |
| Cursor ở ranh giới 2 monitors | Pick monitor có diện tích chứa cursor point lớn hơn |
| Overlay window position trên secondary monitor | Tauri `position()` dùng physical pixels, cần convert |
| DPI-aware screenshot vs overlay coordinates | Screenshot luôn ở native resolution; overlay canvas scale theo DPI |

### Future (v1.0)
- Cho phép user chọn monitor từ minimap.
- Capture all monitors cùng lúc (option).
- Stitch multiple screenshots thành panorama view.

---

## 12. Export Pipeline

### 12.1 Architecture

```rust
// src/services/export_engine.rs

pub trait ExportFormat {
    fn export(&self, session: &SessionExportData, output_path: &Path) -> anyhow::Result<()>;
}

pub struct HtmlExporter;
pub struct MarkdownExporter;
pub struct PdfExporter;   // v0.3
pub struct CsvExporter;   // v0.3

pub struct SessionExportData {
    pub session: Session,
    pub project: Project,
    pub captures: Vec<CaptureExportData>,
}

pub struct CaptureExportData {
    pub capture: Capture,
    pub screenshot_bytes: Vec<u8>,    // For embedding in HTML
    pub issues: Vec<IssueExportData>,
}

pub struct IssueExportData {
    pub issue: Issue,
    pub tags: Vec<Tag>,
    pub crop_bytes: Option<Vec<u8>>,  // For embedding in HTML
}
```

### 12.2 HTML Export

- **Self-contained:** Single HTML file, all CSS inline, images base64-encoded.
- **Responsive:** Viewable on any device/browser.
- **Print-friendly:** CSS `@media print` styles included.
- **Sections:** Session header → Issue table → Per-capture detail (screenshot + annotation markers + issue details).

```html
<!-- Template structure -->
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Review Report: {session.title}</title>
  <style>/* Inline CSS */</style>
</head>
<body>
  <header>
    <h1>{session.title}</h1>
    <p>Project: {project.name} | Date: {session.created_at} | Issues: {count}</p>
  </header>

  <section class="summary">
    <table><!-- Summary table: count by type, severity, status --></table>
  </section>

  <section class="captures">
    <!-- For each capture -->
    <article class="capture">
      <img src="data:image/png;base64,{annotated_screenshot}" />
      <div class="issues">
        <!-- For each issue -->
        <div class="issue">
          <span class="marker">①</span>
          <h3>{issue.title}</h3>
          <span class="badge type-{type}">{type}</span>
          <span class="badge severity-{severity}">{severity}</span>
          <p>{description}</p>
          <img src="data:image/png;base64,{crop}" class="crop" />
        </div>
      </div>
    </article>
  </section>

  <footer>
    <p>Generated by August Mark v{version} on {date}</p>
  </footer>
</body>
</html>
```

### 12.3 Markdown Export

```markdown
# Review Report: {session.title}

**Project:** {project.name}
**Date:** {created_at}
**Issues:** {count}

## Summary

| Type | Count |
|------|-------|
| Bug  | 3     |
| UI   | 2     |

## Capture 1

![Screenshot](./screenshots/{capture_id}_annotated.png)

### ① {issue.title}

- **Type:** Bug
- **Severity:** Critical
- **Status:** Open
- **Tags:** login, auth

{description}

![Crop](./crops/{issue_id}.png)

---
```

> Note: Markdown export tạo folder chứa `.md` file + `screenshots/` + `crops/` subfolders.

### 12.4 CSV Export (v0.3)

```csv
session_id,session_title,capture_id,issue_number,title,description,type,severity,status,tags,marker_x,marker_y,created_at
uuid,Homepage Review,uuid,1,Button color mismatch,Color does not match design spec,UI,Minor,Open,"design,color",450,320,2026-06-15T10:30:00Z
```

### 12.5 PDF Export (v0.3)

Options to evaluate:
1. **`printpdf` crate** — Pure Rust PDF generation. Pro: no external deps. Con: low-level, manual layout.
2. **`headless-chrome` + print-to-PDF** — Render HTML → PDF. Pro: perfect fidelity. Con: heavy dependency (Chromium).
3. **`wkhtmltopdf`** — HTML → PDF converter. Pro: good quality. Con: external binary dependency.

**Recommendation:** Dùng approach 2 (render HTML → PDF via Tauri's built-in webview print), giảm dependency.

---

## 13. Error Handling Strategy

### 13.1 Error Types

```rust
// src/error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // Database errors
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    // File I/O errors
    #[error("File I/O error: {0}")]
    FileIO(#[from] std::io::Error),

    // Screenshot errors
    #[error("Screenshot capture failed: {0}")]
    ScreenCapture(String),

    // Image processing errors
    #[error("Image processing error: {0}")]
    ImageProcessing(#[from] image::ImageError),

    // Mouse hook errors
    #[error("Mouse hook error: {0}")]
    MouseHook(String),

    // Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    // Export errors
    #[error("Export error: {0}")]
    Export(String),

    // Google Drive errors (v0.2)
    #[error("Google Drive error: {0}")]
    GoogleDrive(String),

    // Generic errors
    #[error("{0}")]
    Generic(String),
}

// Convert AppError to serializable format for Tauri IPC
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
```

### 13.2 Error Handling Rules

| Layer | Strategy |
|---|---|
| **Rust Service** | Return `Result<T, AppError>`. Never panic. |
| **Tauri Command** | Map errors to user-friendly messages. Return `Result<T, String>`. |
| **Vue Frontend** | Catch all Tauri invoke errors. Show toast notification for recoverable errors. Show error dialog for critical errors. |
| **SQLite** | Use transactions. Rollback on any failure within a batch operation. |
| **File I/O** | Check disk space before large writes. Retry once on transient failures. |
| **Screenshot** | If capture fails, show notification instead of crashing. Allow retry. |
| **Mouse Hook** | If hook fails to initialize, fall back to global hotkey. Log warning. |

### 13.3 Logging

```rust
// Uses `log` crate + `env_logger`
// Log levels:
// - ERROR: Unrecoverable errors, data loss risk
// - WARN:  Recoverable errors, fallback used
// - INFO:  Important operations (session created, export done)
// - DEBUG: Detailed flow (IPC calls, DB queries)
// - TRACE: Very verbose (mouse events, canvas coordinates)

// Log files: ~/AugustMark/logs/august_mark_{date}.log
// Rotation: Daily, keep 30 days
```

---

## 14. Security & Privacy Considerations

### 14.1 Data Privacy

| Concern | Policy |
|---|---|
| Screenshots may contain sensitive data | All data stored locally by default. User controls what to sync/share. |
| Blur tool for sensitive areas | Blur is applied at pixel level, not reversible in exported images. |
| Database contains issue descriptions | SQLite file accessible only to current OS user (file permissions). |
| Google Drive tokens | Stored in SQLite `settings` table. Consider encrypting with OS keychain in v1.0. |
| No telemetry in MVP | Zero data sent to any server. Pure local app. |
| No AI processing | Screenshots never sent to any AI/ML service. |

### 14.2 Security Measures

| Measure | Implementation |
|---|---|
| SQLite injection | Use parameterized queries exclusively (rusqlite does this by default). |
| Path traversal | Validate all file paths are within app data directory. |
| IPC validation | Validate all Tauri command inputs on Rust side. |
| Tauri CSP | Strict Content-Security-Policy in `tauri.conf.json`. |
| Overlay input isolation | Overlay window consumes all input events while active. |
| Google Drive scope | Use `drive.file` scope (only app-created files). |

### 14.3 Tauri Security Config

```json
// tauri.conf.json — security section
{
  "app": {
    "security": {
      "csp": "default-src 'self'; img-src 'self' asset: https://asset.localhost; style-src 'self' 'unsafe-inline'; script-src 'self'",
      "dangerousDisableAssetCspModification": false
    }
  }
}
```

---

## 15. Backup / Restore Strategy

### 15.1 Local Backup

```rust
// Backup = copy of august_mark.db
// Triggered: manually from Settings, or before risky operations

pub fn create_backup(app_data_dir: &Path) -> anyhow::Result<PathBuf> {
    let db_path = app_data_dir.join("august_mark.db");
    let backup_dir = app_data_dir.join("backups");
    std::fs::create_dir_all(&backup_dir)?;

    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H%M%S");
    let backup_path = backup_dir.join(format!("august_mark_{}.db", timestamp));

    // Use SQLite backup API for consistency
    let src = rusqlite::Connection::open(&db_path)?;
    let mut dst = rusqlite::Connection::open(&backup_path)?;
    let backup = rusqlite::backup::Backup::new(&src, &mut dst)?;
    backup.run_to_completion(5, std::time::Duration::from_millis(250), None)?;

    Ok(backup_path)
}
```

### 15.2 Restore

```
1. User selects backup file from Settings → Backup/Restore.
2. App creates backup of current database first (safety net).
3. App closes database connection.
4. App copies selected backup file over august_mark.db.
5. App re-opens database connection.
6. App runs any pending migrations (in case restoring older version).
7. Dashboard refreshes.
```

### 15.3 Backup Retention

- Keep last 10 manual backups.
- Auto-cleanup backups older than 90 days.
- Total backup size warning at 1GB.

---

## 16. Future Migration Path

### SQLite → PostgreSQL (nếu chuyển sang SaaS)

| Aspect | Strategy |
|---|---|
| **Schema** | SQLite schema đã dùng standard SQL. Migration sang PostgreSQL chỉ cần: thay `TEXT` datetime → `TIMESTAMPTZ`, thay `INTEGER` boolean → `BOOLEAN`. |
| **UUID** | Đã dùng UUID text từ đầu. PostgreSQL có native UUID type. |
| **Queries** | Dùng parameterized queries. Hầu hết compatible. |
| **JSON** | `annotation_data` dùng TEXT JSON. PostgreSQL có `JSONB` — migrate dễ. |
| **Migration tool** | Xem xét dùng `sqlx` hoặc `diesel` khi cần PostgreSQL. MVP dùng raw `rusqlite` cho simplicity. |
| **Data export** | Viết script Rust: read SQLite → insert PostgreSQL. |
| **File storage** | Local files → S3/GCS. Paths trong database chuyển từ relative local → object storage URLs. |

### Desktop → Web (nếu cần)

| Aspect | Strategy |
|---|---|
| **Frontend** | Vue 3 code tái sử dụng 80%+. Chỉ thay Tauri IPC bằng HTTP API calls. |
| **Backend** | Rust Tauri commands → Rust Axum/Actix web API. Logic giữ nguyên. |
| **Database** | SQLite → PostgreSQL (xem trên). |
| **Auth** | Thêm user authentication layer. |
| **Overlay** | Desktop overlay không có trên web. Thay bằng browser extension hoặc embed widget. |
| **Screenshot** | Desktop capture không có trên web. Thay bằng browser extension capture hoặc user upload. |

---

## 17. Folder Structure Đề Xuất (Project Source)

```
august-mark/
├── .github/
│   └── workflows/
│       └── build.yml              # CI/CD
│
├── src/                           # Vue frontend source
│   ├── main.ts
│   ├── overlay.ts
│   ├── App.vue
│   ├── OverlayApp.vue
│   ├── assets/
│   ├── components/
│   │   ├── common/
│   │   ├── dashboard/
│   │   ├── overlay/
│   │   ├── settings/
│   │   └── export/
│   ├── composables/
│   ├── stores/
│   ├── router/
│   ├── types/
│   ├── utils/
│   ├── services/
│   └── plugins/
│
├── src-tauri/                     # Tauri Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/
│   ├── build.rs
│   ├── icons/
│   ├── migrations/
│   │   └── v001_initial.sql
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── error.rs
│       ├── state.rs
│       ├── commands/
│       ├── services/
│       ├── database/
│       ├── models/
│       └── utils/
│
├── public/                        # Static assets served by Vite
│   ├── favicon.ico
│   └── overlay.html               # Overlay window HTML entry
│
├── tests/                         # Frontend tests
│   ├── unit/
│   └── e2e/
│
├── docs/                          # Documentation
│   ├── PROJECT_PLAN.md
│   ├── ARCHITECTURE.md
│   └── CONTRIBUTING.md
│
├── scripts/                       # Build/dev scripts
│   └── setup.sh
│
├── .gitignore
├── .eslintrc.cjs
├── .prettierrc
├── tsconfig.json
├── tsconfig.node.json
├── vite.config.ts
├── package.json
├── index.html                     # Main window HTML entry
├── README.md
├── LICENSE
└── CHANGELOG.md
```

---

## 18. Coding Conventions

### 18.1 Rust

| Rule | Convention |
|---|---|
| Style | Follow `rustfmt` defaults |
| Linting | `clippy` with default warnings |
| Naming | `snake_case` for functions/variables, `PascalCase` for types/traits |
| Error handling | Use `thiserror` for custom errors, `anyhow` for quick prototyping in services |
| Comments | Doc comments (`///`) for public functions. Inline comments for non-obvious logic. |
| Modules | One file per module. `mod.rs` for re-exports only. |
| Tests | Unit tests in same file (`#[cfg(test)]`), integration tests in `tests/` |
| Dependencies | Justify each new crate. Prefer well-maintained crates with >1M downloads. |

### 18.2 TypeScript / Vue

| Rule | Convention |
|---|---|
| Style | ESLint + Prettier |
| Vue | `<script setup lang="ts">` for all components |
| State | Pinia with Composition API (`defineStore` with setup syntax) |
| Naming | `PascalCase` for components, `camelCase` for functions/variables, `UPPER_SNAKE` for constants |
| Types | Explicit types for function params/returns. Avoid `any`. |
| Composables | Prefix with `use` (e.g., `useSession`) |
| File naming | `PascalCase.vue` for components, `camelCase.ts` for non-component files |
| CSS | Scoped `<style scoped>` by default. Vuetify utility classes preferred over custom CSS. |
| Imports | Absolute imports via `@/` alias. Group: vue → vuetify → third-party → local. |

### 18.3 Git

| Rule | Convention |
|---|---|
| Branch naming | `feature/overlay-canvas`, `fix/screenshot-dpi`, `chore/update-deps` |
| Commit messages | Conventional Commits: `feat:`, `fix:`, `chore:`, `docs:`, `refactor:`, `test:` |
| PR size | Ideally < 400 lines changed. Split large features into multiple PRs. |
| Main branch | `main` — always deployable |
| Tags | Semver: `v0.1.0`, `v0.2.0`, etc. |

---

## 19. API Contract — Vue Frontend ↔ Tauri Commands

### 19.1 Convention

- Tất cả Tauri commands return `Result<T, String>`.
- Frontend gọi qua `invoke<T>(command_name, args)`.
- Command names: `snake_case`.
- Request/response types: defined in both Rust (`models/`) và TypeScript (`types/`).

### 19.2 Project Commands

```typescript
// ─── CREATE PROJECT ───
invoke<Project>('create_project', {
  payload: { name: string, description?: string, color?: string }
})
// Returns: Project

// ─── GET ALL PROJECTS ───
invoke<Project[]>('get_projects', {
  includeArchived?: boolean
})
// Returns: Project[]

// ─── GET PROJECT BY ID ───
invoke<Project>('get_project', { id: string })
// Returns: Project

// ─── UPDATE PROJECT ───
invoke<Project>('update_project', {
  id: string,
  payload: { name?: string, description?: string, color?: string, isArchived?: boolean }
})
// Returns: Project (updated)

// ─── DELETE PROJECT ───
invoke<void>('delete_project', { id: string })
// Returns: void
// Behavior: Cascade delete all sessions, captures, issues, files
```

### 19.3 Session Commands

```typescript
// ─── CREATE SESSION ───
invoke<Session>('create_session', {
  payload: { projectId: string, title: string, description?: string }
})
// Returns: Session

// ─── GET SESSIONS ───
invoke<PaginatedResult<Session>>('get_sessions', {
  filter: {
    projectId?: string,
    status?: 'active' | 'completed' | 'archived',
    dateFrom?: string,     // ISO 8601
    dateTo?: string,
    search?: string,       // Full-text search on title/description
    page?: number,         // Default 1
    pageSize?: number,     // Default 20
    sortBy?: 'created_at' | 'updated_at' | 'title',
    sortDir?: 'asc' | 'desc'
  }
})
// Returns: { items: Session[], total: number, page: number, pageSize: number }

// ─── GET SESSION BY ID ───
invoke<SessionDetail>('get_session', { id: string })
// Returns: SessionDetail (includes captures + issues)

// ─── UPDATE SESSION ───
invoke<Session>('update_session', {
  id: string,
  payload: { title?: string, description?: string, status?: string }
})

// ─── COMPLETE SESSION ───
invoke<Session>('complete_session', { id: string })
// Sets status = 'completed', completed_at = now

// ─── DELETE SESSION ───
invoke<void>('delete_session', { id: string })
// Cascade: delete captures, issues, screenshot files, crop files
```

### 19.4 Capture Commands (Overlay)

```typescript
// ─── TRIGGER CAPTURE ───
// Called when middle mouse hold is detected
invoke<CaptureResult>('trigger_capture', {
  sessionId: string
})
// Behavior:
//   1. Screenshot current monitor
//   2. Save to disk
//   3. Create Capture record
//   4. Open overlay window
// Returns: { captureId: string, screenshotPath: string, monitorInfo: MonitorInfo }

// ─── SAVE CAPTURE ANNOTATIONS ───
// Called when user clicks "Done" on overlay
invoke<void>('save_capture_annotations', {
  captureId: string,
  issues: Array<{
    markerNumber: number,
    title: string,
    description: string,
    issueType: IssueType,
    severity: Severity,
    tags: string[],
    markerX: number,
    markerY: number,
    annotationData: AnnotationData,  // JSON
    color: string,
    strokeWidth: number
  }>,
  annotatedScreenshotDataUrl: string   // Canvas toDataURL('image/png')
})
// Behavior:
//   1. Save annotated screenshot
//   2. Generate crops for each issue
//   3. Insert Issue records
//   4. Insert/link Tags
//   5. Close overlay window

// ─── CANCEL CAPTURE ───
invoke<void>('cancel_capture', { captureId: string })
// Behavior: Delete screenshot file, delete Capture record, close overlay
```

### 19.5 Issue Commands

```typescript
// ─── GET ISSUES ───
invoke<PaginatedResult<Issue>>('get_issues', {
  filter: {
    sessionId?: string,
    projectId?: string,
    captureId?: string,
    issueType?: IssueType,
    severity?: Severity,
    status?: IssueStatus,
    tags?: string[],
    search?: string,
    page?: number,
    pageSize?: number,
    sortBy?: 'created_at' | 'marker_number' | 'severity' | 'status',
    sortDir?: 'asc' | 'desc'
  }
})

// ─── GET ISSUE BY ID ───
invoke<IssueDetail>('get_issue', { id: string })
// Returns: Issue with tags, screenshot path, crop path

// ─── UPDATE ISSUE ───
invoke<Issue>('update_issue', {
  id: string,
  payload: {
    title?: string,
    description?: string,
    issueType?: IssueType,
    severity?: Severity,
    status?: IssueStatus,
    tags?: string[]
  }
})

// ─── DELETE ISSUE ───
invoke<void>('delete_issue', { id: string })
// Deletes crop file too

// ─── BULK UPDATE STATUS ───
invoke<void>('bulk_update_issue_status', {
  issueIds: string[],
  status: IssueStatus
})
```

### 19.6 Export Commands

```typescript
// ─── EXPORT HTML ───
invoke<string>('export_html', {
  sessionId: string,
  outputPath?: string   // If not provided, open save dialog
})
// Returns: path to generated file

// ─── EXPORT MARKDOWN ───
invoke<string>('export_markdown', {
  sessionId: string,
  outputPath?: string
})
// Returns: path to generated directory (contains .md + images)

// ─── EXPORT PDF ─── (v0.3)
invoke<string>('export_pdf', {
  sessionId: string,
  outputPath?: string
})

// ─── EXPORT CSV ─── (v0.3)
invoke<string>('export_csv', {
  sessionId: string,
  outputPath?: string
})
```

### 19.7 Settings Commands

```typescript
// ─── GET ALL SETTINGS ───
invoke<Record<string, any>>('get_settings')

// ─── GET SETTING ───
invoke<any>('get_setting', { key: string })

// ─── UPDATE SETTING ───
invoke<void>('update_setting', { key: string, value: any })

// ─── GET STORAGE INFO ───
invoke<StorageInfo>('get_storage_info')
// Returns: { dbSizeBytes, screenshotsSizeBytes, cropsSizeBytes, totalSizeBytes, backupCount }
```

### 19.8 Tag Commands

```typescript
// ─── GET ALL TAGS ───
invoke<Tag[]>('get_tags')

// ─── SEARCH TAGS (autocomplete) ───
invoke<Tag[]>('search_tags', { query: string, limit?: number })

// ─── CREATE TAG ───
invoke<Tag>('create_tag', { name: string, color?: string })

// ─── DELETE TAG ───
invoke<void>('delete_tag', { id: string })
```

### 19.9 Backup Commands

```typescript
// ─── CREATE BACKUP ───
invoke<string>('create_backup')
// Returns: path to backup file

// ─── LIST BACKUPS ───
invoke<BackupInfo[]>('list_backups')
// Returns: [{ path, sizeBytes, createdAt }]

// ─── RESTORE BACKUP ───
invoke<void>('restore_backup', { backupPath: string })
// WARNING: This replaces current database!

// ─── DELETE BACKUP ───
invoke<void>('delete_backup', { backupPath: string })
```

### 19.10 Google Drive Commands (v0.2)

```typescript
// ─── CONNECT GOOGLE DRIVE ───
invoke<void>('gdrive_connect')
// Opens browser for OAuth2 flow

// ─── DISCONNECT ───
invoke<void>('gdrive_disconnect')
// Revokes tokens

// ─── SYNC SESSION ───
invoke<SyncResult>('gdrive_sync_session', { sessionId: string })
// Uploads: screenshots, crops, annotated screenshots, metadata.json, report.html

// ─── GET SYNC STATUS ───
invoke<SyncStatus>('gdrive_get_sync_status', { sessionId: string })
// Returns: { isSynced, lastSyncedAt, filesUploaded, errors }

// ─── CREATE SHARE LINK ───
invoke<string>('gdrive_create_share_link', { sessionId: string })
// Returns: Google Drive share URL
```

### 19.11 Shared TypeScript Types

```typescript
// types/tauri.ts

interface Project {
  id: string
  name: string
  description: string
  color: string
  isArchived: boolean
  createdAt: string
  updatedAt: string
}

interface Session {
  id: string
  projectId: string
  title: string
  description: string
  status: 'active' | 'completed' | 'archived'
  createdAt: string
  updatedAt: string
  completedAt: string | null
  // Computed (from JOIN):
  issueCount?: number
  captureCount?: number
}

interface Capture {
  id: string
  sessionId: string
  screenshotPath: string
  monitorName: string
  monitorWidth: number
  monitorHeight: number
  scaleFactor: number
  windowTitle: string | null
  createdAt: string
}

interface Issue {
  id: string
  captureId: string
  sessionId: string
  projectId: string
  markerNumber: number
  title: string
  description: string
  issueType: IssueType
  severity: Severity
  status: IssueStatus
  markerX: number
  markerY: number
  annotationData: AnnotationData
  color: string
  strokeWidth: number
  cropPath: string | null
  createdAt: string
  updatedAt: string
  tags?: Tag[]
}

type IssueType = 'Bug' | 'UI' | 'UX' | 'Suggestion' | 'Requirement' | 'Question'
type Severity = 'Critical' | 'Major' | 'Minor' | 'Info'
type IssueStatus = 'Draft' | 'Open' | 'In Progress' | 'Resolved' | 'Closed'

interface Tag {
  id: string
  name: string
  color: string
}

interface MonitorInfo {
  x: number
  y: number
  width: number
  height: number
  scaleFactor: number
  name: string
}

interface PaginatedResult<T> {
  items: T[]
  total: number
  page: number
  pageSize: number
}

interface StorageInfo {
  dbSizeBytes: number
  screenshotsSizeBytes: number
  cropsSizeBytes: number
  totalSizeBytes: number
  backupCount: number
}
```

---

## Appendix A: Tauri IPC Performance Notes

| Scenario | Data Size | Strategy |
|---|---|---|
| Small data (issue metadata) | < 1KB | Direct `invoke()` return — fast |
| Medium data (session list with 100 items) | ~50KB | Direct `invoke()` return — acceptable |
| Large data (screenshot bytes) | 2-10MB | **DO NOT** pass through IPC. Save to disk, pass `file://` path. Use Tauri asset protocol. |
| Annotated screenshot from canvas | 2-10MB | Pass `dataURL` string via IPC → Rust decodes base64 → save to disk. One-time cost, acceptable. |
| Real-time events (mouse position) | Continuous | Use Tauri events (`emit`/`listen`), not `invoke`. |

### Asset Protocol Usage

```typescript
// Frontend: load local image for display
import { convertFileSrc } from '@tauri-apps/api/core'

const imageSrc = convertFileSrc(screenshotAbsolutePath)
// Returns: https://asset.localhost/{path} (served by Tauri)
```

## Appendix B: Key Technical Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Database | SQLite (rusqlite, bundled) | Zero-setup, single file, embedded. Perfect for desktop app. |
| ORM | No ORM, raw SQL | Full control, no abstraction overhead, easy to debug. Queries are simple enough. |
| Image format | PNG | Lossless, widely supported. Compression reasonable for screenshots. |
| UUID version | v4 (random) | Simple, no coordination needed. Text storage in SQLite. |
| Canvas tech | HTML5 Canvas 2D | Built into browser. No extra library. Good enough for annotation tools. |
| State management | Pinia | Official Vue state management. Composition API support. |
| Overlay approach | Separate Tauri window | Isolated from main window. Can be fullscreen + transparent + always-on-top. |
| Mouse hook | rdev → winapi fallback → global hotkey fallback | Progressive fallback strategy for reliability. |
| Export HTML | Template-based, inline everything | Single file, opens anywhere, no server needed. |
| Google Drive API | REST via reqwest | No SDK needed. Simple CRUD operations. |
