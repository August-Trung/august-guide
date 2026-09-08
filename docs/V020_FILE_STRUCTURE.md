# V020_FILE_STRUCTURE.md — August Mark v0.2.0

> Cấu trúc thư mục cho bản nâng cấp v0.2.0. Tài liệu mô tả trực quan các file thêm mới [NEW] và các file cần chỉnh sửa [MODIFY] ở cả frontend và backend.

```
august-mark/
│
├── package.json                        # [MODIFY] Cập nhật version "0.2.0"
├── vite.config.ts
├── CHANGELOG.md                        # [MODIFY] Chuẩn bị nhật ký thay đổi v0.2.0
├── README.md                           # [MODIFY] Tài liệu hướng dẫn sử dụng GDrive & AACP
│
├── src/                                # ── Vue 3 Frontend ──
│   │
│   ├── main.ts
│   ├── overlay.ts
│   ├── App.vue
│   │
│   ├── types/
│   │   ├── settings.ts                 # [MODIFY] Thêm các cấu hình liên quan đến Google Drive Sync
│   │   └── aacp.ts                     # [NEW] Định nghĩa kiểu AACP Export Payload (workspace, suspected files)
│   │
│   ├── services/
│   │   └── tauriCommands.ts            # [MODIFY] Thêm API wrapper cho Google Drive & AACP commands
│   │
│   ├── stores/
│   │   └── settingsStore.ts            # [MODIFY] Thêm logic quản lý trạng thái GDrive và Trigger Sync
│   │
│   ├── components/
│   │   │
│   │   ├── dashboard/
│   │   │   ├── SessionList.vue         # [MODIFY] Thêm nút "Share to Drive" cho Session
│   │   │   └── IssueDetail.vue         # [MODIFY] Thêm nút "Share to Drive" cho Issue đơn lẻ
│   │   │
│   │   └── export/
│   │       └── ExportDialog.vue        # [MODIFY] Tích hợp tab xuất "AI Agent Context Pack (AACP)"
│   │
│   └── views/
│       └── SettingsView.vue            # [MODIFY] Thêm khu vực "Cloud Sync & Backup" (Google Login/Logout)
│
└── src-tauri/                          # ── Tauri Rust Backend ──
    │
    ├── Cargo.toml                      # [MODIFY] Thêm reqwest (HTTP calls), tiny_http (OAuth loopback), zip crate
    ├── tauri.conf.json                 # [MODIFY] Cập nhật version "0.2.0"
    │
    ├── capabilities/
    │   └── default.json                # [MODIFY] Cấp quyền IPC cho các commands OAuth, Backup và AACP
    │
    └── src/
        ├── main.rs                     # [MODIFY] Đăng ký các commands mới của gdrive_cmds và aacp_cmds
        ├── lib.rs
        │
        ├── models/
        │   ├── mod.rs                  # [MODIFY] Khai báo export gdrive và aacp models
        │   ├── gdrive.rs               # [NEW] Struct Token, AuthState và FileMetadata
        │   └── aacp.rs                 # [NEW] Struct AACP Manifest data model
        │
        ├── db/
        │   # SQLite backend sử dụng settings_repo có sẵn để lưu token
        │
        ├── services/
        │   ├── mod.rs                  # [MODIFY] Đăng ký export
        │   ├── gdrive_client.rs        # [NEW] HTTP client gọi Google Drive v3 APIs
        │   ├── git_inspector.rs        # [NEW] Chạy lệnh git CLI thu thập metadata dự án
        │   └── aacp_exporter.rs        # [NEW] Tạo JSON manifest, prompt MD, copy visual assets và nén zip
        │
        └── commands/
            ├── mod.rs                  # [MODIFY] Khai báo export commands mới
            ├── gdrive_cmds.rs          # [NEW] IPC handlers cho Đăng nhập, Backup, Restore, Share
            └── aacp_cmds.rs            # [NEW] IPC handler cho tính năng xuất Gói AI Agent
```
