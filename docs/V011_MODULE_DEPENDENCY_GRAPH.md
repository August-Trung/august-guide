# V011_MODULE_DEPENDENCY_GRAPH.md — August Mark v0.1.1

> Sơ đồ kiến trúc phụ thuộc giữa các module trong phiên bản v0.1.1. Bản vẽ giúp developer định hình rõ ràng luồng dữ liệu và biên giới giữa các tầng.

---

## 1. Rust Backend — Dependency Graph (v0.1.1)

```
                    ┌─────────────┐
                    │   main.rs   │  Tauri builder, tray, window state
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
                    │ settings    │  [NEW] Settings IPC
                    │ _cmds       │
                    │ tag_cmds    │  [NEW] Tag IPC
                    │ search_cmds │  [NEW] Search IPC
                    │ export_cmds │  [MOD] Multi-format
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
      │ _service │  │ _repo     │  │migrations│
      │          │  │           │  │.rs       │
      │          │  │ settings  │  └──────────┘
      │          │  │ _repo[NEW]│
      │          │  │           │
      │          │  │ tag_repo  │
      │          │  │ [NEW]     │
      └──────────┘  └─────┬─────┘
              │            │
              │            │
              ▼            ▼
      ┌──────────────────────────────────────┐
      │            models/                    │
      │                                      │
      │  project.rs  session.rs  capture.rs  │
      │  issue.rs    settings.rs tag.rs      │
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

---

## 2. Vue Frontend — Dependency Graph (v0.1.1)

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
      │          │                    │ Panel        │
      │Settings  │                    │              │
      │View [NEW]│                    │ OverlayStatus│
      └────┬─────┘                    │ Bar          │
           │                          └──────┬───────┘
           │                                 │
           ├──────── both use ───────────────┤
           ▼                                 ▼
    ┌─────────────────────────────────────────────┐
    │                 stores/                      │
    │                                             │
    │  projectStore   sessionStore   issueStore   │
    │  settingsStore  overlayStore   uiStore      │
    │  tagStore [NEW]                             │
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
    │  issue.ts    annotation.ts settings.ts      │
    └──────────────────────┬──────────────────────┘
                           │
                           ▼
    ┌─────────────────────────────────────────────┐
    │                utils/                        │
    │                                             │
    │  date.ts     geometry.ts    image.ts        │
    └─────────────────────────────────────────────┘
```

---

## 3. Subsystem Dependency Graphs (Chi tiết phân hệ v0.1.1)

Dưới đây là sơ đồ chi tiết biểu diễn luồng tương tác và phân rã cấu trúc cho 6 phân hệ chính nâng cấp ở bản v0.1.1:

### 3.1. Settings Subsystem (Phân hệ Cài đặt)

```
┌──────────────────────────────────────────────────────────────────┐
│ Vue UI Component: SettingsView.vue                               │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ reads/writes
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Pinia Store: settingsStore.ts                                    │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ invokes
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Frontend IPC: services/tauriCommands.ts (get/update setting)      │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ Tauri IPC Bridge
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Rust IPC: commands/settings_cmds.rs                              │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ lock connection & call
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Rust Repo: db/settings_repo.rs                                   │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ SQL Query
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Storage: SQLite settings table                                   │
└──────────────────────────────────────────────────────────────────┘
```

### 3.2. Overlay Subsystem (Phân hệ Vẽ & Canvas)

```
┌──────────────────────────────────────────────────────────────────┐
│ Vue UI: OverlayApp.vue ◄──[Shortcuts]── Mouse & Key events       │
└────────┬───────────────────────────────┬─────────────────────────┘
         │ uses                          │ renders
         ▼                               ▼
┌──────────────────┐            ┌──────────────────────────────────┐
│ Pinia Store:     │            │ Component: AnnotationCanvas.vue  │
│ overlayStore.ts  │            └────────────────┬─────────────────┘
└────────┬─────────┘                             │
         │ manages state                         │ delegates events
         ▼                                       ▼
┌──────────────────────────────────────────────────────────────────┐
│ Composable: useCanvas.ts (Load screen & draw layers)             │
│   ├─ Layer 1: Screenshot background                              │
│   ├─ Layer 2: Committed shapes (Rect, Arrow, Text, Marker...)    │
│   └─ Layer 3: Active drawing preview (Freehand, Blur Rect...)    │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ uses logic
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Composable: useAnnotation.ts (Active tool state machine)          │
└──────────────────────────────────────────────────────────────────┘
```

### 3.3. Tags Subsystem (Phân hệ Nhãn)

```
┌───────────────────────────┐      ┌───────────────────────────────┐
│ Overlay: IssueFormPanel   │      │ Dashboard: FilterBar / Detail │
└─────────────┬─────────────┘      └───────────────┬───────────────┘
              │ select/create tag                  │ select tags to filter
              ▼                                    ▼
┌──────────────────────────────────────────────────────────────────┐
│ Pinia Store: tagStore.ts ◄───[reacts]─── issueStore.ts           │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ invokes
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Frontend IPC: services/tauriCommands.ts (get/create tags)        │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ Tauri IPC
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Rust IPC: commands/tag_cmds.rs                                   │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ queries
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Rust Repo: db/tag_repo.rs                                        │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ JOIN Query
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Storage: SQLite tags & issue_tags tables                         │
└──────────────────────────────────────────────────────────────────┘
```

### 3.4. Search Subsystem (Phân hệ Tìm kiếm)

```
┌──────────────────────────────────────────────────────────────────┐
│ Vue UI: AppHeader.vue (Search input autocomplete)                │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ invokes (debounced 300ms)
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Frontend IPC: services/tauriCommands.ts (searchAll)              │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ Tauri IPC
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Rust IPC: commands/search_cmds.rs                                │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ queries
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Storage: SQLite (LIKE query on sessions + issues tables)          │
└──────────────────────────────────────────────────────────────────┘
```

### 3.5. Export Subsystem (Phân hệ Xuất Báo cáo)

```
┌──────────────────────────────────────────────────────────────────┐
│ Vue UI: ExportDialog.vue (Format: HTML, PDF, MD, CSV)            │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ invokes with format & filters
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Frontend IPC: services/tauriCommands.ts (exportSession)          │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ Tauri IPC
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Rust IPC: commands/export_cmds.rs                                │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ delegates format
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Rust Service: services/export_service.rs                         │
│   ├─ export_html() -> Self-contained report                      │
│   ├─ export_markdown() -> Table + relative crop images           │
│   └─ export_csv() -> Flat bug list for import                    │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ writes files
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│ Storage: File System (~/AugustMark/exports/)                     │
└──────────────────────────────────────────────────────────────────┘
```

---

## 4. Quy tắc cô lập Module (Module Isolation Rules)

Để tránh hiện tượng phụ thuộc vòng (circular dependency) hoặc cấu trúc "spaghetti", dự án v0.1.1 áp dụng nghiêm ngặt các quy tắc sau:

1. **Database Repositories (`db/*.rs`):** Chỉ thực hiện truy vấn SQL thô và ánh xạ (mapping) sang struct Model. **Không** chứa logic nghiệp vụ (business logic) và tuyệt đối **không** gọi tới các Service khác hay gọi chéo các Repo khác.
2. **Business Services (`services/*.rs`):** Chứa thuật toán và logic xử lý (ví dụ: vẽ đè ảnh, crop ảnh, export báo cáo, capture). Được phép gọi tới `db/` repositories để lấy dữ liệu.
3. **IPC Handlers (`commands/*.rs`):** Đóng vai trò là API controllers. Chỉ nhận request từ frontend, giải nén tham số, khóa connection DB từ `AppState`, gọi tới `services` hoặc `db` và trả về kết quả. **Không** gọi lẫn nhau (ví dụ: `issue_cmds.rs` không được gọi hàm trong `project_cmds.rs`).
4. **Pinia Stores (`stores/*.ts`):** Là nguồn lưu trữ trạng thái duy nhất ở client. Mọi giao tiếp với Rust backend bắt buộc phải thông qua API bridge `services/tauriCommands.ts`. Component **không** được tự ý gọi `invoke()` của Tauri.
5. **Vue Composables (`composables/*.ts`):** Chỉ chứa logic xử lý state UI (như drag-and-drop chuột, điều phối canvas). Composables có thể đọc/ghi Pinia Store nhưng **không** được trực tiếp thao tác DB hay gọi Tauri commands.
