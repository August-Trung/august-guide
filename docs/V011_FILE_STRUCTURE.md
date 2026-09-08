# V011_FILE_STRUCTURE.md — August Mark v0.1.1

> Cấu trúc thư mục cho bản nâng cấp v0.1.1. Tài liệu làm nổi bật các file thêm mới [NEW] và các file cần chỉnh sửa [MODIFY] để hoàn thành các tính năng đề ra.

```
august-mark/
│
├── package.json                        # [MODIFY] Cập nhật version "0.1.1"
├── vite.config.ts
├── CHANGELOG.md                        # [MODIFY] Thêm nhật ký thay đổi v0.1.1
├── README.md
│
├── public/
│   └── overlay.html
│
├── src/                                # ── Vue 3 Frontend ──
│   │
│   ├── main.ts
│   ├── overlay.ts
│   ├── App.vue
│   ├── OverlayApp.vue                  # [MODIFY] Thêm phím nóng (Ctrl+Z, Ctrl+Y, Delete)
│   │
│   ├── plugins/
│   │   └── vuetify.ts                  # [MODIFY] Đảm bảo cấu hình Light/Dark hoạt động động
│   │
│   ├── router/
│   │   └── index.ts                    # [MODIFY] Thêm route "/settings"
│   │
│   ├── types/
│   │   ├── issue.ts                    # [MODIFY] Thêm trường "tags" vào interface Issue
│   │   ├── settings.ts                 # [NEW] Định nghĩa kiểu Settings key-value
│   │   └── annotation.ts               # [MODIFY] Khai báo chi tiết Blur, Freehand, Highlight
│   │
│   ├── services/
│   │   ├── tauriCommands.ts            # [MODIFY] Thêm wrappers cho Settings, Tags, Search API
│   │   └── tauriEvents.ts
│   │
│   ├── stores/
│   │   ├── settingsStore.ts            # [NEW] Store quản lý settings cục bộ
│   │   ├── tagStore.ts                 # [NEW] Store quản lý nhãn dán (tags)
│   │   ├── sessionStore.ts             # [MODIFY] Thêm logic sorting
│   │   ├── issueStore.ts               # [MODIFY] Thêm logic filtering theo tags & sorting
│   │   └── overlayStore.ts             # [MODIFY] Thêm Undo/Redo stack
│   │
│   ├── composables/
│   │   ├── useCanvas.ts                # [MODIFY] Render Blur (mosaic), Freehand, Highlight
│   │   └── useAnnotation.ts            # [MODIFY] Thêm hit-test select và delete annotation
│   │
│   ├── components/
│   │   │
│   │   ├── common/
│   │   │   ├── AppHeader.vue           # [MODIFY] Thêm autocomplete Search & fix hotkey label
│   │   │   └── AppSidebar.vue          # [MODIFY] Thêm nút điều hướng Settings
│   │   │
│   │   ├── dashboard/
│   │   │   ├── SessionList.vue         # [MODIFY] Thêm dropdown sắp xếp danh sách session
│   │   │   ├── IssueList.vue           # [MODIFY] Thêm dropdown sắp xếp danh sách issue
│   │   │   ├── FilterBar.vue           # [MODIFY] Thêm combo select tags để lọc
│   │   │   ├── IssueCard.vue           # [MODIFY] Hiển thị các tag chips của issue
│   │   │   └── IssueDetail.vue         # [MODIFY] Cho phép xem và chỉnh sửa tags của issue
│   │   │
│   │   ├── overlay/
│   │   │   ├── AnnotationToolbar.vue   # [MODIFY] Bổ sung các icon tool Blur, Freehand, Highlight
│   │   │   ├── OverlayStatusBar.vue    # [MODIFY] Thêm nút Undo/Redo trên thanh trạng thái
│   │   │   └── IssueFormPanel.vue      # [MODIFY] Thêm combobox chọn/gõ tags mới cho issue
│   │   │
│   │   └── export/
│   │       └── ExportDialog.vue        # [MODIFY] Thêm format MD/CSV/PDF & checkbox filter
│   │
│   ├── views/
│   │   ├── AboutView.vue               # [MODIFY] Đọc version động & hiển thị stats database
│   │   └── SettingsView.vue            # [NEW] Giao diện cấu hình cài đặt
│   │
│   └── utils/
│
└── src-tauri/                          # ── Tauri Rust Backend ──
    │
    ├── Cargo.toml                      # [MODIFY] Thêm window-state plugin & csv crate
    ├── tauri.conf.json                 # [MODIFY] Cấu hình tray-icon, window-state permissions
    │
    ├── capabilities/
    │   └── default.json                # [MODIFY] Cấp quyền cho các commands và plugin mới
    │
    └── src/
        ├── main.rs                     # [MODIFY] Đăng ký commands mới, khởi tạo Tray Icon
        ├── lib.rs                      # [MODIFY] Quản lý window-state, đọc settings khởi động
        │
        ├── models/
        │   ├── mod.rs                  # [MODIFY] Khai báo export
        │   ├── settings.rs             # [NEW] Struct Settings model
        │   └── tag.rs                  # [NEW] Struct Tag model
        │
        ├── db/
        │   ├── mod.rs                  # [MODIFY] Khai báo export repo mới
        │   ├── settings_repo.rs        # [NEW] CRUD cho bảng settings
        │   └── tag_repo.rs             # [NEW] CRUD cho bảng tags và issue_tags
        │
        ├── services/
        │   ├── image_processor.rs      # [MODIFY] Vẽ đè nét freehand, highlight, và làm mờ (blur)
        │   └── export_html.rs          # [MODIFY] Đổi tên thành export_service.rs hoặc thêm engine MD/CSV
        │
        └── commands/
            ├── mod.rs                  # [MODIFY] Khai báo các commands module mới
            ├── settings_cmds.rs        # [NEW] Tauri command get/update settings
            ├── tag_cmds.rs             # [NEW] Tauri command get/create tags
            ├── search_cmds.rs          # [NEW] Tauri command search toàn cục
            ├── app_cmds.rs             # [NEW] Tauri command get_app_stats
            └── export_cmds.rs          # [MODIFY] Nhận thêm định dạng export và params filter
```

---

## Tóm tắt số lượng file thay đổi

| Loại | Số lượng | Mô tả chi tiết |
|---|:---:|---|
| **File tạo mới [NEW]** | **7** | 1 Vue View, 2 Pinia Stores, 2 Rust Repos, 2 Rust Commands/Models. |
| **File chỉnh sửa [MODIFY]** | **25+** | Chủ yếu là tích hợp UI, bổ sung logic vẽ cho canvas và kết nối API. |

---

## Quy ước đặt tên (Naming Conventions)

Vẫn duy trì các quy ước nhất quán từ phiên bản v0.1.0:
- **Vue Components:** `PascalCase.vue` (ví dụ: `SettingsView.vue`, `ExportDialog.vue`).
- **TypeScript files:** `camelCase.ts` (ví dụ: `settingsStore.ts`, `tauriCommands.ts`).
- **Rust files:** `snake_case.rs` (ví dụ: `settings_repo.rs`, `tag_cmds.rs`).
- **SQL Migration files:** `v{NNN}_{description}.sql` (trong trường hợp có update schema DB). Tuy nhiên ở v0.1.1, do bảng `settings` và `tags` **đã được định nghĩa sẵn ở v0.1.0 (v001_initial.sql)** nên chúng ta không cần tạo thêm file SQL migration mới, tránh gây phiền phức khi nâng cấp dữ liệu.
