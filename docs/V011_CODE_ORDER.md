# V011_CODE_ORDER.md — August Mark v0.1.1

> Thứ tự code tối ưu cho v0.1.1. Đảm bảo phát triển bền vững, kiểm tra được từng bước và giảm thiểu rủi ro kỹ thuật cao nhất.

---

## Coding Sequence

| # | Bước | Rust Files | Vue Files | Lý do thứ tự này |
|---|---|---|---|---|
| **1** | SQLite Settings Repo | `db/settings_repo.rs`, `db/mod.rs` | — | Khởi đầu bằng data layer cho cài đặt |
| **2** | Settings Commands | `commands/settings_cmds.rs`, `commands/mod.rs`, `main.rs` | `services/tauriCommands.ts` | IPC bridge để trao đổi cài đặt |
| **3** | Settings Store & Page | — | `stores/settingsStore.ts`, `views/SettingsView.vue`, `router/index.ts` | Người dùng cấu hình được theme, hotkey, delayed capture |
| **4** | Theme dynamic sync | — | update `plugins/vuetify.ts`, `views/SettingsView.vue` | Kiểm chứng theme Light/Dark chuyển đổi runtime |
| **5** | System Tray Icon | update `main.rs` | — | Tích hợp Native Tray. Đảm bảo chạy ngầm ổn định |
| **6** | Window State persistence | update `Cargo.toml`, `main.rs` | — | Lưu kích thước cửa sổ để hoàn tất Native Polish |
| **7** | About page & stats | `commands/app_cmds.rs`, `commands/mod.rs` | update `views/AboutView.vue` | Hiển thị version động và stats từ DB |
| **8** | Fix hotkey label | — | update `components/common/AppHeader.vue` | Sửa UI label hiển thị sai để thống nhất trải nghiệm |
| — | **⬆ CHECKPOINT 1** | **Trải nghiệm Desktop Native hoàn tất. Cài đặt hoạt động.** | | |
| **9** | Blur Tool (Canvas side) | — | update `types/annotation.ts`, `composables/useAnnotation.ts`, `composables/useCanvas.ts` | Thêm tool Blur vẽ trên Canvas HTML5 |
| **10** | Freehand & Highlight | — | update `types/annotation.ts`, `composables/useAnnotation.ts`, `composables/useCanvas.ts` | Thêm tool Freehand và Highlight |
| **11** | Toolbar buttons & hotkeys | — | update `components/overlay/AnnotationToolbar.vue`, `OverlayApp.vue` | Phím tắt nhanh 5 (Blur), 6 (Freehand), 7 (Highlight) |
| **12** | Canvas Undo / Redo | — | update `stores/overlayStore.ts`, `components/overlay/OverlayStatusBar.vue` | Quản lý stack nét vẽ. Ctrl+Z/Ctrl+Y hoạt động |
| **13** | Xóa Annotation cụ thể | — | update `composables/useAnnotation.ts`, `composables/useCanvas.ts` | Chọn và xóa từng nét vẽ trực tiếp |
| **14** | Save Flow Backend | update `services/image_processor.rs` | — | Rust render đè các nét vẽ mới (blur, freehand, highlight) lên ảnh PNG |
| — | **⬆ CHECKPOINT 2** | **Công cụ vẽ nâng cao hoạt động hoàn chỉnh. Ảnh lưu đĩa chính xác.** | | |
| **15** | Tags System Backend | `db/tag_repo.rs`, `commands/tag_cmds.rs`, `db/mod.rs`, `commands/mod.rs` | `services/tauriCommands.ts` | Database layer & IPC cho Tags |
| **16** | Tags UI (Overlay & Detail) | — | update `components/overlay/IssueFormPanel.vue`, `components/dashboard/IssueDetail.vue`, `components/dashboard/IssueCard.vue` | Gán tag trong form và hiển thị chips |
| **17** | Filter Tag Dashboard | — | update `components/dashboard/FilterBar.vue`, `stores/issueStore.ts` | Lọc issue theo tag |
| **18** | Global Search Backend | `commands/search_cmds.rs`, `commands/mod.rs` | `services/tauriCommands.ts` | Backend search query SQL |
| **19** | Search Header UI | — | update `components/common/AppHeader.vue` | Autocomplete search input và navigate |
| **20** | Sorting & Stats | — | update `components/dashboard/SessionList.vue`, `components/dashboard/IssueList.vue`, `stores/sessionStore.ts`, `stores/issueStore.ts` | Sắp xếp và hiển thị thống kê |
| **21** | Advanced Export | update `services/export_html.rs` (thành `export_service.rs` hoặc bổ sung) | update `components/export/ExportDialog.vue` | Xuất Markdown, CSV và PDF kèm filter |
| — | **⬆ CHECKPOINT 3** | **Tính năng quản lý và xuất dữ liệu nâng cao hoàn tất.** | | |
| **22** | Regression Testing | — | — | Đảm bảo tương thích ngược dữ liệu v0.1.0 |
| **23** | Visual Polish | update `src/index.css` | update views/components | Thêm transition, loading states |
| **24** | Bump version & Build | update `Cargo.toml`, `tauri.conf.json`, `package.json` | update `CHANGELOG.md` | Đóng gói sản phẩm release `v0.1.1` |
| — | **⬆ CHECKPOINT 4** | **Phát hành phiên bản v0.1.1 thành công.** | | |

---

## Giải Thích Sự Phụ Thuộc & Chiến Lược Giảm Thiểu Rủi Ro

### 1. Chiến lược High-Risk-First (Rủi ro cao xử lý trước)

- **Rủi ro 1: Canvas Performance & Blur Rendering (Medium-High):** Thao tác xử lý điểm ảnh (pixel manipulation) để tạo hiệu ứng blur mosaic trên HTML5 canvas có thể gây đơ lag trình duyệt nếu không tối ưu. Do đó, Blur Tool được code ngay đầu Milestone 2 để kiểm tra FPS và xử lý thuật toán tối ưu sớm.
- **Rủi ro 2: System Tray & Window State (Medium):** Tương tác giữa Tauri Rust thread và hệ điều hành Windows đôi khi có thể bị tranh chấp hoặc lỗi luồng hiển thị (ví dụ minimize to tray bị lỗi render). Chúng ta đưa tính năng này vào ngay Milestone 1 để kiểm tra cơ chế lifecycle của app (Show/Hide/Minimize/Focus).

### 2. Thiết kế theo chiều dọc (Vertical Slices)

- Các tính năng như **Settings** và **Tags** không được chia nhỏ theo kiểu code hết Rust rồi mới code Vue.
- Thay vào đó, chúng đi theo vertical slice: `Repository (Rust) -> Commands (Rust) -> API Bridge (Vue service) -> Store (Vue pinia) -> View/Component (Vue UI)`.
- Lợi ích: Test được dòng chảy dữ liệu ngay lập tức. Sửa lỗi serialize/deserialize dữ liệu JSON nhanh chóng trước khi xây dựng giao diện phức tạp.

### 3. Checkpoints kiểm thử rõ ràng

- **Checkpoint 1:** Đảm bảo toàn bộ khung desktop app chạy bền bỉ như một ứng dụng nền (background app). Lưu được settings.
- **Checkpoint 2:** Đảm bảo khả năng vẽ của Overlay nâng cấp vượt trội và lưu được ảnh đĩa có độ bảo mật cao (đã làm mờ).
- **Checkpoint 3:** Đảm bảo khả năng quản lý và phân loại lỗi bug sau khi chụp (Dashboard nâng cấp).
- **Checkpoint 4:** Product hoàn thiện sẵn sàng cài đặt.
