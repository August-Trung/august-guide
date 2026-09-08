# V020_MODULE_DEPENDENCY_GRAPH.md — August Mark v0.2.0

> Sơ đồ kiến trúc phụ thuộc giữa các module trong phiên bản v0.2.0. Bản vẽ thể hiện rõ các luồng dữ liệu mới đi ra ngoài Internet (Google Drive) và truy xuất thông tin hệ thống (Git CLI, Local Workspace).

---

## 1. Rust Backend — Dependency Graph (v0.2.0)

```
                    ┌─────────────┐
                    │   main.rs   │  Tauri builder, tray, window state
                    └──────┬──────┘
                           │ registers
                           ▼
                    ┌─────────────┐
                    │  commands/  │  Tauri IPC handlers
                    │             │
                    │ gdrive_cmds │  [NEW] Google Drive Sync/Backup IPC
                    │ aacp_cmds   │  [NEW] AI Agent Exporter IPC
                    │ settings    │  (Đã có) Settings CRUD IPC
                    │ _cmds       │
                    │ ...         │  (Các commands khác của v0.1.1)
                    └──────┬──────┘
                           │ calls
               ┌───────────┼────────────┐
               ▼           ▼            ▼
       ┌──────────┐ ┌───────────┐  ┌──────────┐
       │ services/│ │   db/     │  │  state.rs │
       │          │ │           │  │ AppState  │
       │ gdrive   │ │ settings  │  │ (Mutex    │
       │ _client  │ │ _repo     │  │  <Conn>)  │
       │ [NEW]    │ │           │  └──────────┘
       │          │ │ issue     │       │
       │ git      │ │ _repo     │       │ owns
       │ _inspect │ │           │       ▼
       │ [NEW]    │ │ session   │  ┌──────────┐
       │          │ │ _repo     │  │connection│
       │ aacp     │ └───────────┘  │.rs       │
       │ _exporter│                └──────────┘
       │ [NEW]    │
       └────┬─────┘
            │
            │ uses / serializes
            ▼
       ┌──────────────────────────────────────┐
       │            models/                    │
       │                                      │
       │  gdrive.rs [NEW]    aacp.rs [NEW]     │
       │  settings.rs        issue.rs          │
       └──────────────────────────────────────┘
```

**Luồng dữ liệu Cloud Sync:**
1. `SettingsView.vue` (Frontend) → Gọi `gdrive_cmds::connect_gdrive`.
2. `gdrive_cmds` kích hoạt server http cục bộ (`tiny_http`) → Mở trình duyệt xác thực Google.
3. Người dùng đăng nhập thành công → Server nhận Code → Gọi `gdrive_client::exchange_token` qua Google Endpoint → Lưu tokens vào SQLite (`settings_repo`).
4. Khi Backup: `gdrive_cmds::backup_to_gdrive` → nén `db` + `screenshots` → upload lên Drive bằng `gdrive_client` (sử dụng token từ DB).

**Luồng dữ liệu AI Agent Exporter (AACP):**
1. `ExportDialog.vue` (Frontend) → Chọn Workspace và Suspected Files → Gọi `aacp_cmds::export_aacp`.
2. `aacp_cmds` kiểm tra metadata Git dự án qua `git_inspector` (gọi Git CLI qua std command).
3. `aacp_cmds` gọi `aacp_exporter` để:
   - Truy vấn thông tin lỗi chi tiết từ SQLite (`issue_repo` / `session_repo`).
   - Đọc và copy ảnh screenshot, ảnh crops liên quan.
   - Sinh tệp JSON Manifest và Markdown Prompt.
   - Gói toàn bộ thư mục thành tệp `.aacp.zip` lưu vào nơi chỉ định.

---

## 2. Vue Frontend — Dependency Graph (v0.2.0)

```
                             ┌──────────────┐
                             │   App.vue    │
                             └──────┬───────┘
                                    │
               ┌────────────────────┼────────────────────┐
               ▼                    ▼                    ▼
       ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
       │    views/    │     │ components/  │     │ components/  │
       │              │     │ dashboard/   │     │ export/      │
       │ SettingsView │     │              │     │              │
       │ (GDrive UI)  │     │ SessionList  │     │ ExportDialog │
       │              │     │ (Share btn)  │     │ (AACP Tab)   │
       └──────┬───────┘     └──────┬───────┘     └──────┬───────┘
              │                    │                    │
              ▼                    ▼                    ▼
       ┌────────────────────────────────────────────────────────┐
       │                  stores/settingsStore.ts               │
       │             (Lưu trạng thái OAuth & Backup)            │
       └──────────────────────────┬─────────────────────────────b
                                  │ calls
                                  ▼
       ┌────────────────────────────────────────────────────────┐
       │               services/tauriCommands.ts                │
       │         (Wrappers: connect_gdrive, export_aacp, ...)   │
       └────────────────────────────────────────────────────────b
```
