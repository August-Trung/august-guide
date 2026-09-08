# MODULE_DEPENDENCY_GRAPH.md — August Mark v0.1

> Sơ đồ phụ thuộc giữa các module. Đọc từ trên xuống = build order.

---

## 1. Rust Backend — Dependency Graph

```
                    ┌─────────────┐
                    │   main.rs   │  Tauri builder, setup, tray
                    └──────┬──────┘
                           │ registers
                           ▼
                    ┌─────────────┐
                    │  commands/  │  Tauri IPC handlers
                    │             │
                    │ project_cmds│
                    │ session_cmds│
                    │ capture_cmds│
                    │ issue_cmds  │
                    │ export_cmds │
                    └──────┬──────┘
                           │ calls
              ┌────────────┼────────────┐
              ▼            ▼            ▼
      ┌──────────┐  ┌───────────┐  ┌──────────┐
      │ services/│  │   db/     │  │  state.rs │
      │          │  │           │  │ AppState  │
      │ screen   │  │ project   │  │ (Mutex    │
      │ _capture │  │ _repo     │  │  <Conn>)  │
      │          │  │           │  └──────────┘
      │ file     │  │ session   │       │
      │ _storage │  │ _repo     │       │ owns
      │          │  │           │       ▼
      │ image    │  │ capture   │  ┌──────────┐
      │ _process │  │ _repo     │  │connection│
      │          │  │           │  │.rs       │
      │ export   │  │ issue     │  │          │
      │ _html    │  │ _repo     │  │migrations│
      └──────────┘  └─────┬─────┘  │.rs       │
              │            │        └──────────┘
              │            │             │
              ▼            ▼             ▼
      ┌──────────────────────────────────────┐
      │            models/                    │
      │                                      │
      │  project.rs  session.rs  capture.rs  │
      │  issue.rs                            │
      └──────────────┬───────────────────────┘
                     │
                     ▼
      ┌──────────────────────────────────────┐
      │            utils/                     │
      │                                      │
      │  paths.rs    id.rs                   │
      └──────────────────────────────────────┘
                     │
                     ▼
      ┌──────────────────────────────────────┐
      │            error.rs                   │
      │  AppError (thiserror)                │
      └──────────────────────────────────────┘
```

### Build Order (Rust)

```
1. error.rs          ← no deps
2. utils/            ← no deps
3. models/           ← depends on: serde, chrono, uuid
4. state.rs          ← depends on: rusqlite
5. db/connection.rs  ← depends on: state, utils/paths
6. db/migrations.rs  ← depends on: connection
7. db/*_repo.rs      ← depends on: connection, models
8. services/*        ← depends on: models, utils, db repos
9. commands/*        ← depends on: services, db repos, state
10. main.rs          ← depends on: commands, state
```

---

## 2. Vue Frontend — Dependency Graph

```
                    ┌──────────────┐    ┌──────────────┐
                    │   main.ts    │    │  overlay.ts   │
                    │ (Dashboard)  │    │ (Overlay win) │
                    └──────┬───────┘    └──────┬────────┘
                           │                   │
                           ▼                   ▼
                    ┌──────────────┐    ┌──────────────┐
                    │   App.vue    │    │OverlayApp.vue│
                    └──────┬───────┘    └──────┬────────┘
                           │                   │
              ┌────────────┤                   │
              ▼            ▼                   ▼
      ┌──────────┐  ┌───────────┐     ┌──────────────┐
      │  views/  │  │ router/   │     │ components/  │
      │          │  │ index.ts  │     │ overlay/     │
      │Dashboard │  └───────────┘     │              │
      │View      │                    │ Annotation   │
      │          │                    │ Canvas       │
      │Session   │                    │              │
      │View      │                    │ Annotation   │
      │          │                    │ Toolbar      │
      │Issue     │                    │              │
      │View      │                    │ IssueForm    │
      └────┬─────┘                    │ Panel        │
           │                          │              │
           │                          │ OverlayStatus│
           │                          │ Bar          │
           │                          └──────┬───────┘
           │                                 │
           ├──────── both use ───────────────┤
           ▼                                 ▼
    ┌─────────────────────────────────────────────┐
    │                 stores/                      │
    │                                             │
    │  projectStore   sessionStore   issueStore   │
    │                 overlayStore   uiStore      │
    └──────────────────────┬──────────────────────┘
                           │ calls
                           ▼
    ┌─────────────────────────────────────────────┐
    │               services/                      │
    │                                             │
    │  tauriCommands.ts    tauriEvents.ts         │
    └──────────────────────┬──────────────────────┘
                           │ uses
                           ▼
    ┌─────────────────────────────────────────────┐
    │                types/                        │
    │                                             │
    │  project.ts  session.ts  capture.ts         │
    │  issue.ts    annotation.ts                  │
    └──────────────────────┬──────────────────────┘
                           │
                           ▼
    ┌─────────────────────────────────────────────┐
    │                utils/                        │
    │                                             │
    │  date.ts     geometry.ts    image.ts        │
    └─────────────────────────────────────────────┘
```

### Composables Position

```
    components/overlay/*
           │
           │ uses
           ▼
    ┌─────────────────┐
    │  composables/   │
    │                 │
    │  useCanvas.ts   │ ← manages 3 canvas layers
    │  useAnnotation  │ ← tool state machine
    │  .ts            │
    │  useTauriEvents │ ← event setup/teardown
    │  .ts            │
    └────────┬────────┘
             │ uses
             ▼
    stores/ + services/ + types/
```

---

## 3. Cross-Layer Dependency (Full Picture)

```
┌─────────────────────────────────────────────────────────────┐
│                        USER                                  │
└──────────────────────────┬──────────────────────────────────┘
                           │ interacts with
                           ▼
┌──────────────────────────────────────────────────────────────┐
│  FRONTEND (Vue 3 + Vuetify 3)                                │
│                                                              │
│  views/ ──→ components/ ──→ composables/                     │
│                    │              │                           │
│                    ▼              ▼                           │
│               stores/ ◄─────────────                         │
│                    │                                         │
│                    ▼                                         │
│             services/tauriCommands.ts                        │
│             services/tauriEvents.ts                          │
└──────────────────────┬──────────────────┬────────────────────┘
                       │ invoke()         │ listen()/emit()
                       ▼                  ▼
┌──────────────────────────────────────────────────────────────┐
│  TAURI IPC BOUNDARY                                          │
└──────────────────────┬──────────────────┬────────────────────┘
                       │                  │
                       ▼                  ▼
┌──────────────────────────────────────────────────────────────┐
│  BACKEND (Rust)                                              │
│                                                              │
│  commands/ ──→ services/ ──→ db/repos/                       │
│       │              │            │                          │
│       ▼              ▼            ▼                          │
│    state.rs     models/     connection.rs                    │
│       │              │            │                          │
│       ▼              ▼            ▼                          │
│    utils/        error.rs   migrations.rs                    │
└──────────────────────┬──────────────────┬────────────────────┘
                       │                  │
                       ▼                  ▼
┌──────────────────────────────────────────────────────────────┐
│  STORAGE                                                     │
│                                                              │
│  SQLite (august_mark.db)       File System (~/AugustMark/)   │
│                                  ├── screenshots/            │
│                                  ├── crops/                  │
│                                  └── exports/                │
└──────────────────────────────────────────────────────────────┘
```

---

## 4. Data Flow: Capture → Annotate → Save

```
Hotkey Ctrl+Shift+M pressed
         │
         ▼
┌─ RUST ──────────────────────────────────────────────┐
│  commands/capture_cmds::trigger_capture()            │
│         │                                            │
│         ├──→ services/screen_capture::capture()      │
│         │         │                                  │
│         │         └──→ xcap crate → RgbaImage        │
│         │                                            │
│         ├──→ services/file_storage::save_screenshot() │
│         │         │                                  │
│         │         └──→ ~/AugustMark/screenshots/     │
│         │                                            │
│         ├──→ db/capture_repo::create()               │
│         │         │                                  │
│         │         └──→ SQLite INSERT                 │
│         │                                            │
│         └──→ open_overlay() → Tauri WebviewWindow    │
└──────────────────────────────────────────────────────┘
         │ emit("overlay:init", captureId)
         ▼
┌─ VUE ───────────────────────────────────────────────┐
│  OverlayApp.vue                                      │
│         │                                            │
│         ├──→ overlayStore.init(captureId)             │
│         │                                            │
│         ├──→ AnnotationCanvas.vue                    │
│         │         │                                  │
│         │         ├──→ useCanvas.ts (load screenshot) │
│         │         └──→ useAnnotation.ts (tool mgmt)  │
│         │                                            │
│         ├──→ User draws markers, fills forms         │
│         │                                            │
│         └──→ User clicks "Done"                      │
│                   │                                  │
│                   └──→ overlayStore.saveAndClose()    │
│                             │                        │
│                             └──→ invoke("save_capture│
│                                  _annotations", ...) │
└──────────────────────────────────────────────────────┘
         │
         ▼
┌─ RUST ──────────────────────────────────────────────┐
│  commands/issue_cmds::save_capture_annotations()     │
│         │                                            │
│         ├──→ FOR each issue:                         │
│         │     ├──→ db/issue_repo::create()           │
│         │     └──→ services/image_processor::crop()  │
│         │              └──→ file_storage::save_crop() │
│         │                                            │
│         ├──→ Save annotated screenshot               │
│         │                                            │
│         └──→ close_overlay()                         │
└──────────────────────────────────────────────────────┘
```

---

## 5. Module Isolation Rules

| Rule | Rationale |
|---|---|
| `commands/` KHÔNG import lẫn nhau | Mỗi command handler độc lập |
| `db/*_repo.rs` KHÔNG gọi service | Repo chỉ biết SQL, không biết business logic |
| `services/` có thể gọi `db/` repos | Service chứa business logic |
| `models/` KHÔNG import bất kỳ layer nào khác | Pure data structs |
| `stores/` chỉ gọi `services/tauriCommands` | Store không gọi trực tiếp Tauri invoke |
| `composables/` gọi `stores/` | Composables wrap store logic cho component |
| `components/` không gọi `tauriCommands` trực tiếp | Luôn qua store hoặc composable |
