# Kế hoạch Thực hiện: Chuyển đổi August Guide thành Thư viện ảnh chụp & Quản lý Album tài liệu

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Chuyển đổi giao diện chính của August Guide thành Thư viện ảnh chụp (Screenshot Gallery) với dòng thời gian, bộ lọc ứng dụng, chế độ chọn nhiều ảnh (multi-select) và thanh tác vụ nổi để gom thành bài tài liệu / xuất PDF.

**Architecture:** Mở rộng backend Rust với các lệnh truy vấn toàn bộ ảnh theo dự án (`get_all_captures_by_project`) và tạo tài liệu từ danh sách ảnh (`create_session_from_captures`). Xây dựng giao diện Thư viện ảnh `GalleryView.vue` với dòng thời gian theo ngày, bộ lọc cửa sổ `window_title`, thanh tác vụ nổi `FloatingSelectionBar.vue` khi chọn checkbox, và cập nhật thanh bên `AppSidebar.vue`.

**Tech Stack:** Tauri 2 (Rust, SQLite), Vue 3, TypeScript, Vuetify 3, Pinia.

**Spec:** `docs/superpowers/specs/2026-10-11-screenshot-gallery-and-albums-design.md`

## Global Constraints
- Tuân thủ quy tắc Code Quality (không check length trước vòng lặp, dùng `?.`, không so sánh `=== true`, dùng `??` thay vì `||` khi `0` hoặc `""` hợp lệ).
- Hộp thoại xác nhận an toàn bắt buộc khi xóa nhiều ảnh (Mandatory Confirmation Dialogs for Risky Actions).
- Tiếng Việt chuẩn mã hóa UTF-8 có dấu.
- Làm việc trên nhánh `feat/gallery-and-albums-transformation`, verify `npm run build` và `cargo check`, bump version 0.4.0, merge `main`, tạo tag `v0.4.0` và push lên GitHub.

---

### Task 1: Backend Rust - Lệnh lấy toàn bộ ảnh theo dự án & tạo tài liệu từ ảnh đã chọn

**Files:**
- Modify: `src-tauri/src/db/capture_repo.rs`
- Modify: `src-tauri/src/commands/capture_cmds.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Viết hàm DB `get_all_captures_by_project` trong `capture_repo.rs`**
  - Viết truy vấn SQL join `captures` với `sessions` theo `project_id`, lọc `is_deleted = 0`, sắp xếp `created_at DESC`.
  - Trả về `Vec<Capture>`.

- [ ] **Step 2: Viết hàm DB & command `create_session_from_captures` trong `capture_cmds.rs`**
  - Nhận `project_id: String`, `title: String`, `description: Option<String>`, `capture_ids: Vec<String>`.
  - Tạo một session mới trong DB.
  - Clone/sao chép các capture được chọn sang session mới (giữ nguyên đường dẫn ảnh và vector annotation).
  - Trả về `Session` vừa tạo.

- [ ] **Step 3: Đăng ký command trong `src-tauri/src/lib.rs`**
  - Thêm `commands::capture_cmds::get_all_captures_by_project`
  - Thêm `commands::capture_cmds::create_session_from_captures`

- [ ] **Step 4: Kiểm tra biên dịch backend Rust**
  - Chạy `cargo check --manifest-path src-tauri/Cargo.toml` và xác minh mã thoát 0.

---

### Task 2: Frontend API & Store - Tích hợp các lệnh mới

**Files:**
- Modify: `src/services/tauriCommands.ts`
- Modify: `src/stores/sessionStore.ts`

- [ ] **Step 1: Khai báo invoke commands trong `src/services/tauriCommands.ts`**
  - `getAllCapturesByProject(projectId: string): Promise<Capture[]>`
  - `createSessionFromCaptures(projectId: string, title: string, captureIds: string[], description?: string): Promise<Session>`

- [ ] **Step 2: Cập nhật `sessionStore.ts`**
  - Thêm state `allProjectCaptures = ref<Capture[]>([])`
  - Thêm method `loadAllProjectCaptures(projectId: string)`
  - Thêm getter/lọc theo ngày và theo ứng dụng (`windowTitle`).

- [ ] **Step 3: Kiểm tra frontend compile**
  - Chạy `npm run build` xác minh không có lỗi TypeScript.

---

### Task 3: Frontend Component - Thanh tác vụ nổi `FloatingSelectionBar.vue`

**Files:**
- Create: `src/components/gallery/FloatingSelectionBar.vue`

- [ ] **Step 1: Tạo component `FloatingSelectionBar.vue`**
  - Props: `selectedCount: number`, `selectedIds: string[]`.
  - Emits: `create-guide`, `export-pdf`, `delete-selected`, `clear-selection`.
  - Giao diện: Thanh nổi bo tròn (Capsule) ở đáy màn hình (`bottom: 24px; left: 50%`), nền tối với hiệu ứng kính mờ (blur), viền primary sáng, hiệu ứng trượt mượt mà khi xuất hiện (`v-slide-y-reverse-transition`).
  - Các nút:
    - Badge hiển thị: `Đã chọn: X ảnh`
    - Nút `Tạo tài liệu mới` (icon `mdi-book-plus-outline`)
    - Nút `Xuất PDF` (icon `mdi-file-pdf-box`)
    - Nút `Xóa` (icon `mdi-delete-outline`, màu error)
    - Nút `Bỏ chọn` (icon `mdi-close`)

---

### Task 4: Frontend View - Nâng cấp màn hình chính thành Thư viện ảnh (`GalleryView.vue`)

**Files:**
- Create/Modify: `src/views/DashboardView.vue` (hoặc tích hợp `GalleryView`)
- Create: `src/components/gallery/GalleryGrid.vue`
- Create: `src/components/gallery/CreateGuideFromSelectionDialog.vue`

- [ ] **Step 1: Tạo Dialog `CreateGuideFromSelectionDialog.vue`**
  - Cho phép người dùng nhập Tên tài liệu / Tên quy trình (ví dụ: *"Quy trình nghiệm thu ISO"*).
  - Mô tả tùy chọn.
  - Hiển thị số ảnh đã chọn sẽ được đưa vào tài liệu.
  - Khi bấm Xác nhận -> gọi `createSessionFromCaptures` -> chuyển hướng vào trang tài liệu vừa tạo hoặc thông báo thành công.

- [ ] **Step 2: Xây dựng `GalleryGrid.vue`**
  - Hiển thị ảnh dạng lưới phân nhóm theo dòng thời gian (Hôm nay, Hôm qua, Tuần trước, Tháng này...).
  - Mỗi thẻ ảnh có checkbox ở góc trên bên trái để chọn/bỏ chọn.
  - Hỗ trợ nút Chọn tất cả / Bỏ chọn tất cả.
  - Các nút tác vụ nhanh trên từng ảnh: Phóng to xem lớn, Ghim nổi trên màn hình (`Pin`), Sửa nét vẽ (`Re-open Overlay`), Xóa ảnh.
  - Dropdown lọc theo ứng dụng chụp (`Tất cả`, `Chrome`, `Word`, v.v.).

- [ ] **Step 3: Tích hợp vào `DashboardView.vue`**
  - Biến Dashboard thành Trung tâm Thư viện ảnh chụp của dự án.
  - Khi người dùng chụp ảnh mới bằng phím tắt, ảnh lập tức hiện ngay đầu danh sách hôm nay.

---

### Task 5: Frontend Sidebar & Navigation - Cập nhật thanh bên

**Files:**
- Modify: `src/components/common/AppSidebar.vue`

- [ ] **Step 1: Đổi mục điều hướng chính thành "Thư viện ảnh"**
  - Menu icon `mdi-image-multiple`, tiêu đề: `Thư viện ảnh` (to `/`).
- [ ] **Step 2: Thêm danh mục "Bộ tài liệu / Hướng dẫn"**
  - Hiển thị danh sách các bài tài liệu đã gom của dự án hiện tại.
  - Mỗi bài có tên, số ảnh con, click vào để mở xem chi tiết bài đó (`/session/:id`).
  - Nút bấm `+ Tạo tài liệu mới` nhanh.

---

### Task 6: Kiểm thử toàn diện, Nâng phiên bản lên 0.4.0 & Phát hành

**Files:**
- Modify: `package.json` (0.4.0)
- Modify: `src-tauri/Cargo.toml` (0.4.0)
- Modify: `src-tauri/tauri.conf.json` (0.4.0)

- [ ] **Step 1: Chạy build kiểm tra toàn diện**
  - `npm run build`
  - `cargo check --manifest-path src-tauri/Cargo.toml`
- [ ] **Step 2: Commit trên nhánh `feat/gallery-and-albums-transformation`**
  - `git commit -m "feat: transform main view to screenshot gallery with timeline, filters, multi-select, and albums sidebar"`
- [ ] **Step 3: Merge vào `main`, tạo tag `v0.4.0`, và đẩy lên GitHub**
  - `git checkout main && git merge feat/gallery-and-albums-transformation`
  - `git tag -a v0.4.0 -m "Release v0.4.0: Screenshot Gallery & Document Albums Transformation"`
  - `git push origin main && git push origin v0.4.0`
