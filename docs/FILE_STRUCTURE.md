# FILE_STRUCTURE.md — August Mark v0.1

> Cấu trúc thư mục cho MVP. Chỉ liệt kê file thực sự cần tạo.

```
august-mark/
│
├── index.html                          # Main window entry (Vite default)
├── package.json
├── tsconfig.json
├── tsconfig.node.json
├── vite.config.ts
├── .gitignore
├── .eslintrc.cjs
├── .prettierrc
├── README.md
├── CHANGELOG.md
│
├── public/
│   ├── favicon.ico
│   └── overlay.html                    # Overlay window entry (separate Vite entry)
│
├── src/                                # ── Vue 3 Frontend ──
│   │
│   ├── main.ts                         # Main window bootstrap (createApp, vuetify, pinia, router)
│   ├── overlay.ts                      # Overlay window bootstrap (createApp, vuetify, pinia)
│   ├── App.vue                         # Main window root component
│   ├── OverlayApp.vue                  # Overlay window root component
│   │
│   ├── plugins/
│   │   ├── vuetify.ts                  # Vuetify instance + augustDark / augustLight themes
│   │   └── pinia.ts                    # Pinia instance
│   │
│   ├── router/
│   │   └── index.ts                    # Vue Router: / (dashboard), /project/:id, /session/:id, /issue/:id
│   │
│   ├── types/                          # ── TypeScript types (shared) ──
│   │   ├── project.ts                  # Project interface
│   │   ├── session.ts                  # Session interface
│   │   ├── capture.ts                  # Capture interface + MonitorInfo
│   │   ├── issue.ts                    # Issue interface + IssueType, Severity, IssueStatus enums
│   │   └── annotation.ts              # Annotation union types (Marker, Rect, Arrow, Text) + Point
│   │
│   ├── services/                       # ── Tauri IPC bridge ──
│   │   ├── tauriCommands.ts            # All invoke() wrappers grouped by domain
│   │   └── tauriEvents.ts             # All listen()/emit() event wrappers
│   │
│   ├── stores/                         # ── Pinia stores ──
│   │   ├── projectStore.ts             # Projects list + active project
│   │   ├── sessionStore.ts             # Sessions list + active session
│   │   ├── issueStore.ts               # Issues list + filters
│   │   ├── overlayStore.ts             # Overlay state: tools, annotations, current capture
│   │   └── uiStore.ts                  # UI state: loading, dialogs, toasts
│   │
│   ├── composables/                    # ── Vue composables ──
│   │   ├── useCanvas.ts                # Canvas layer management (3 layers: screenshot, markers, drawing)
│   │   ├── useAnnotation.ts            # Tool state machine: select tool → draw → commit → issue form
│   │   └── useTauriEvents.ts           # Setup/teardown Tauri event listeners
│   │
│   ├── components/                     # ── Vue components ──
│   │   │
│   │   ├── common/                     # Shared across pages
│   │   │   ├── AppHeader.vue           # Top bar: app name, session indicator, hotkey hint
│   │   │   ├── AppSidebar.vue          # Left sidebar: project list, new project button
│   │   │   ├── ConfirmDialog.vue       # Reusable confirm dialog (delete session/issue)
│   │   │   └── EmptyState.vue          # Empty state placeholder (no sessions, no issues)
│   │   │
│   │   ├── dashboard/                  # Dashboard page components
│   │   │   ├── SessionList.vue         # List of sessions (cards)
│   │   │   ├── SessionCard.vue         # Single session card (title, date, issue count, status)
│   │   │   ├── IssueList.vue           # List of issues within a session
│   │   │   ├── IssueCard.vue           # Single issue card (thumbnail, title, type badge, severity badge)
│   │   │   ├── IssueDetail.vue         # Full issue view: screenshot + marker highlight + edit form
│   │   │   ├── FilterBar.vue           # Filter: project, type, severity, status
│   │   │   └── ProjectSelector.vue     # Project picker in sidebar + create project dialog
│   │   │
│   │   ├── overlay/                    # Overlay window components
│   │   │   ├── AnnotationCanvas.vue    # Main canvas: 3 layers + mouse event handling
│   │   │   ├── AnnotationToolbar.vue   # Bottom toolbar: tool buttons (marker, rect, arrow, text)
│   │   │   ├── IssueFormPanel.vue      # Right side panel: title, type, severity, description, save/cancel
│   │   │   └── OverlayStatusBar.vue    # Top bar: session name, issue count, Done/Cancel buttons
│   │   │
│   │   └── export/                     # Export UI
│   │       └── ExportDialog.vue        # Export dialog: choose format, save location
│   │
│   ├── views/                          # ── Page-level components (router targets) ──
│   │   ├── DashboardView.vue           # Main dashboard: sidebar + session list
│   │   ├── SessionView.vue             # Session detail: issue list + stats
│   │   └── IssueView.vue              # Issue detail: screenshot + marker + edit form
│   │
│   ├── utils/                          # ── Pure utility functions ──
│   │   ├── date.ts                     # formatDate, formatRelativeTime, toISOString
│   │   ├── geometry.ts                 # Point math: distance, rectContains, boundingBox
│   │   └── image.ts                    # dataUrlToBlob, loadImage helper
│   │
│   └── assets/                         # Static assets
│       └── logo.svg                    # App logo
│
├── src-tauri/                          # ── Tauri Rust Backend ──
│   │
│   ├── Cargo.toml                      # Rust dependencies
│   ├── tauri.conf.json                 # Tauri config: windows, permissions, app metadata
│   ├── build.rs                        # Tauri build script
│   ├── capabilities/
│   │   └── default.json                # Tauri 2 capability permissions
│   │
│   ├── icons/                          # App icons (generated by Tauri)
│   │   ├── icon.ico
│   │   ├── icon.png
│   │   └── ...
│   │
│   ├── migrations/
│   │   └── v001_initial.sql            # Full MVP schema: projects, sessions, captures, issues
│   │
│   └── src/
│       ├── main.rs                     # Entry: Tauri builder, register commands, setup AppState, system tray
│       ├── lib.rs                      # Module declarations
│       ├── error.rs                    # AppError enum (thiserror)
│       ├── state.rs                    # AppState: Mutex<Connection>, app_data_dir, is_overlay_active
│       │
│       ├── models/                     # ── Domain models (serde Serialize/Deserialize) ──
│       │   ├── mod.rs                  # Re-exports
│       │   ├── project.rs              # Project, CreateProjectPayload, UpdateProjectPayload
│       │   ├── session.rs              # Session, CreateSessionPayload, UpdateSessionPayload
│       │   ├── capture.rs              # Capture, MonitorInfo
│       │   └── issue.rs                # Issue, CreateIssuePayload, UpdateIssuePayload, IssueType, Severity, IssueStatus
│       │
│       ├── db/                         # ── Database layer ──
│       │   ├── mod.rs                  # Re-exports
│       │   ├── connection.rs           # open_connection(), ensure_app_dirs()
│       │   ├── migrations.rs           # run_migrations(), get_current_version()
│       │   ├── project_repo.rs         # create, get_all, get_by_id, update, delete
│       │   ├── session_repo.rs         # create, get_all, get_by_project, get_by_id, update, delete
│       │   ├── capture_repo.rs         # create, get_by_session, get_by_id, delete
│       │   └── issue_repo.rs           # create, get_by_session, get_by_capture, get_by_id, update, delete
│       │
│       ├── services/                   # ── Business logic ──
│       │   ├── mod.rs                  # Re-exports
│       │   ├── screen_capture.rs       # capture_current_monitor(), get_cursor_position()
│       │   ├── file_storage.rs         # save_screenshot(), save_crop(), delete_file(), ensure_dirs()
│       │   ├── image_processor.rs      # crop_for_issue(), render_annotations_on_image()
│       │   └── export_html.rs          # export_session_html() — template + base64 images
│       │
│       ├── commands/                   # ── Tauri IPC command handlers ──
│       │   ├── mod.rs                  # Re-exports
│       │   ├── project_cmds.rs         # create_project, get_projects, update_project, delete_project
│       │   ├── session_cmds.rs         # create_session, get_sessions, get_session, update_session, delete_session
│       │   ├── capture_cmds.rs         # trigger_capture, open_overlay, close_overlay, cancel_capture
│       │   ├── issue_cmds.rs           # save_capture_annotations, get_issues, get_issue, update_issue, delete_issue
│       │   └── export_cmds.rs          # export_html
│       │
│       └── utils/                      # ── Helpers ──
│           ├── mod.rs                  # Re-exports
│           ├── paths.rs                # screenshots_dir(), crops_dir(), exports_dir(), ensure_dir()
│           └── id.rs                   # new_uuid() — wrapper quanh uuid::Uuid::new_v4()
│
└── docs/                               # ── Documentation ──
    ├── PROJECT_PLAN.md
    ├── ARCHITECTURE.md
    ├── MVP_IMPLEMENTATION_PLAN.md
    ├── IMPLEMENTATION_TASKS.md
    ├── CODE_ORDER.md
    ├── MODULE_DEPENDENCY_GRAPH.md
    └── FILE_STRUCTURE.md               # (file này)
```

---

## File Count Summary

| Directory | Files | Purpose |
|---|---|---|
| `src/types/` | 5 | TypeScript interfaces/enums |
| `src/services/` | 2 | Tauri IPC bridge |
| `src/stores/` | 5 | Pinia state |
| `src/composables/` | 3 | Vue hooks |
| `src/components/common/` | 4 | Shared UI |
| `src/components/dashboard/` | 7 | Dashboard UI |
| `src/components/overlay/` | 4 | Overlay UI |
| `src/components/export/` | 1 | Export dialog |
| `src/views/` | 3 | Router pages |
| `src/utils/` | 3 | Helpers |
| `src/plugins/` | 2 | Vue plugins |
| `src-tauri/src/models/` | 5 | Rust domain models |
| `src-tauri/src/db/` | 7 | Database layer |
| `src-tauri/src/services/` | 5 | Business logic |
| `src-tauri/src/commands/` | 6 | IPC handlers |
| `src-tauri/src/utils/` | 3 | Rust helpers |
| **Total** | **~65 source files** | |

---

## Naming Conventions

| Item | Convention | Example |
|---|---|---|
| Vue component file | `PascalCase.vue` | `SessionCard.vue` |
| Vue view file | `PascalCase.vue` ending with `View` | `DashboardView.vue` |
| TypeScript file | `camelCase.ts` | `tauriCommands.ts` |
| Rust file | `snake_case.rs` | `project_repo.rs` |
| Pinia store | `camelCase.ts` ending with `Store` | `sessionStore.ts` |
| Composable | `camelCase.ts` starting with `use` | `useCanvas.ts` |
| SQL migration | `v{NNN}_{description}.sql` | `v001_initial.sql` |

---

## Điểm lưu ý

1. **`src/views/` vs `src/components/`**: Views là targets của router, chứa layout. Components là building blocks tái sử dụng.
2. **`src-tauri/src/db/` thay vì `database/`**: Ngắn hơn, gõ nhanh hơn, solo dev không cần tên dài.
3. **`src-tauri/src/commands/` dùng suffix `_cmds.rs`**: Phân biệt rõ với models cùng tên.
4. **Không có `src/api/`**: Không có backend server. Tất cả giao tiếp qua Tauri IPC (`services/tauriCommands.ts`).
5. **Không có `src/i18n/`**: MVP chỉ tiếng Anh. i18n thêm sau.
6. **Không có `tests/`**: MVP test thủ công. Unit test thêm sau khi flow ổn định.
