# Thiết kế Kiến trúc: Chuyển đổi August Guide thành Thư viện ảnh chụp & Quản lý Album tài liệu

## 1. Bối cảnh & Mục tiêu (Context & Objectives)

### 1.1 Vấn đề hiện tại
- Trước đây, August Guide được thiết kế xoay quanh khái niệm cứng nhắc: mỗi lần chụp ảnh phải gắn liền với một "Bài hướng dẫn" (`Session`).
- Trong thực tế, người dùng sử dụng ứng dụng như một **công cụ chụp ảnh & ghi chú màn hình hàng ngày** từ nhiều ứng dụng, trang web và thời điểm khác nhau. Việc coi 137 tấm ảnh là "1 bài hướng dẫn gồm 137 bước" gây hiểu lầm và làm trải nghiệm trở nên gò bó.

### 1.2 Mục tiêu sau chuyển đổi
1. **Giao diện chính (Main Content):** Chuyển đổi thành **"Thư viện ảnh chụp" (Screenshot Gallery)**:
   - Hiển thị toàn bộ ảnh chụp trong dự án dưới dạng lưới trực quan.
   - Sắp xếp mặc định: ảnh mới nhất lên đầu tiên.
   - Phân nhóm theo Dòng thời gian (`Hôm nay`, `Hôm qua`, `Tháng này`...) hoặc lọc theo ứng dụng/cửa sổ đã chụp (`Window Title`).
2. **Thanh bên (Sidebar):** Quản lý các **"Bộ tài liệu / Album"**:
   - Mục cố định: 📸 **Tất cả ảnh chụp (Thư viện)**.
   - Danh sách các Bộ tài liệu/Album đã tạo (ví dụ: `Guide 10-09-2026`, v.v.).
   - Nút `+ Tạo bộ tài liệu mới`.
3. **Quy trình gom tài liệu linh hoạt (Multi-select Workflow):**
   - Người dùng có thể tích chọn nhiều ảnh bằng checkbox trên từng thẻ ảnh.
   - Khi chọn ≥ 1 ảnh: Xuất hiện **Thanh tác vụ nổi (Floating Action Bar)** ở đáy màn hình với các hành động:
     - *Tạo bộ tài liệu mới từ X ảnh đã chọn*
     - *Thêm vào bộ tài liệu đã có*
     - *Xuất nhanh PDF*
     - *Xóa hàng loạt* (kèm popup xác nhận an toàn theo quy định hệ thống)
     - *Bỏ chọn*

---

## 2. Kiến trúc Dữ liệu & Backend Rust

### 2.1 Bảng Cơ sở dữ liệu SQLite
Hiện tại database có:
- `projects` (id, name, created_at, ...)
- `sessions` (id, project_id, title, description, status, created_at, ...)
- `captures` (id, session_id, screenshot_path, annotated_screenshot_path, window_title, created_at, ...)

### 2.2 Chiến lược tương thích & Mở rộng
- Để đảm bảo **100% dữ liệu cũ không bị xáo trộn** và các file xuất báo cáo cũ vẫn chạy tốt:
  - Mỗi `Project` sẽ có 1 `default_session` đóng vai trò là "Inbox / Thư viện tổng". Khi chụp nhanh qua phím tắt, ảnh được lưu vào `default_session` của dự án đó.
  - Bổ sung bảng liên kết nhiều-nhiều (hoặc cơ chế Album) giữa Album và Capture:
    - Bảng `album_captures (album_id TEXT, capture_id TEXT, sort_order INTEGER, added_at DATETIME)` hoặc tận dụng bảng `sessions` làm `Albums`.
  - Lệnh truy vấn:
    - `get_all_captures_by_project(project_id)`: Lấy toàn bộ ảnh trong toàn bộ dự án để hiển thị ở trang Thư viện ảnh chính.
    - `get_captures_by_window_title(project_id)`: Phân loại theo ứng dụng.

---

## 3. Thiết kế Giao diện Người dùng (Frontend UX / UI)

### 3.1 Cấu trúc bố cục tổng thể
```
+---------------------------------------------------------------------------------------+
|  LOGO August Guide   [Dự án: Default Project v]    [Tìm kiếm...]    [Phím tắt: F9...] |
+------------------+--------------------------------------------------------------------+
| SIDEBAR          | MAIN GALLERY VIEW                                                  |
|                  |                                                                    |
| [Dự án đang mở]  | [📸 Tất cả ảnh chụp (137)]                [Sắp xếp: Mới nhất v]    |
|                  | [Lọc theo app: Tất cả v]                                            |
| 📸 Thư viện ảnh  |                                                                    |
|    (Tất cả)      | ── Hôm nay (11/10/2026) ────────────────────────────────────────── |
|                  | [ ☑ Ảnh 137 ]  [ ☑ Ảnh 136 ]  [ ☐ Ảnh 135 ]  [ ☐ Ảnh 134 ]          |
| 📁 Bộ tài liệu   |                                                                    |
|  • Guide 10-09   | ── Hôm qua (10/10/2026) ────────────────────────────────────────── |
|  • Quy trình LMS | [ ☐ Ảnh 133 ]  [ ☐ Ảnh 132 ]  [ ☐ Ảnh 131 ]  [ ☐ Ảnh 130 ]          |
|                  |                                                                    |
| + Tạo tài liệu   |                                                                    |
|                  | [ Floating Action Bar: Đã chọn 2 ảnh | Tạo tài liệu | Xuất PDF | X ]|
+------------------+--------------------------------------------------------------------+
```

### 3.2 Các thành phần chính (Components)
1. **`GalleryView.vue` (Màn hình chính thay thế Dashboard cũ):**
   - Header: Thống kê tổng số ảnh, công cụ lọc theo ngày / theo ứng dụng (`window_title`), dropdown sắp xếp thời gian.
   - Danh sách ảnh theo nhóm ngày (Timeline grouping).
   - Chế độ chọn (Selection mode): Checkbox góc trên bên trái từng ảnh.
   - Thẻ ảnh (`GalleryCard.vue`):
     - Thumbnail phóng to, badge thứ tự, icon app, ngày giờ chụp.
     - Nút ghim nhanh (Pin), nút sửa nét vẽ (Annotation Editor), nút xem lớn, nút xóa.
2. **`FloatingSelectionBar.vue` (Thanh thao tác nổi khi chọn nhiều):**
   - Cố định ở đáy màn hình (`bottom: 24px`, căn giữa), bo tròn hiện đại, đổ bóng nổi.
   - Hiển thị số lượng ảnh đã chọn.
   - Nút: *"Tạo bộ tài liệu mới"*, *"Xuất tài liệu (PDF)"*, *"Xóa"*, *"Hủy"*.
3. **`Sidebar.vue` (Cập nhật):**
   - Nhánh 1: **"Thư viện ảnh (Tất cả)"** với số lượng tổng.
   - Nhánh 2: **"Bộ tài liệu / Album"** hiển thị danh sách các tài liệu đã tạo kèm số ảnh con.
   - Nhấp vào tài liệu sẽ mở xem riêng tài liệu đó.

---

## 4. Kế hoạch Triển khai từng bước (Milestones)

1. **Bước 1 - Backend & Store:**
   - Bổ sung lệnh Tauri lấy toàn bộ ảnh theo dự án `get_all_captures_by_project`.
   - Cập nhật `captureStore` / `sessionStore` để hỗ trợ đa lựa chọn (multi-selection) và lọc theo ngày/cửa sổ.
2. **Bước 2 - Xây dựng Giao diện Thư viện ảnh (`GalleryView.vue`):**
   - Hiển thị danh sách ảnh toàn dự án theo dòng thời gian.
   - Tích hợp ô checkbox chọn ảnh trên từng thẻ.
3. **Bước 3 - Xây dựng Thanh tác vụ nổi (`FloatingSelectionBar.vue`):**
   - Xử lý hành động gom ảnh đã chọn thành tài liệu mới (`create_session_from_captures`).
   - Xử lý xóa nhiều ảnh (với dialog xác nhận an toàn bắt buộc).
4. **Bước 4 - Cập nhật Sidebar & Điều hướng:**
   - Thêm mục "Thư viện ảnh" và danh sách "Bộ tài liệu" ở thanh bên.
   - Cho phép chuyển đổi mượt mà giữa xem toàn bộ thư viện và xem từng bộ tài liệu.
5. **Bước 5 - Kiểm thử & Hoàn thiện:**
   - Chạy `npm run build` và `cargo check`.
   - Kiểm tra tương thích với dữ liệu 137 ảnh hiện có.
