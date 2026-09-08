# V011_IMPLEMENTATION_PLAN.md — August Mark v0.1.1

> Kế hoạch nâng cấp v0.1.1 dành cho solo developer. Tập trung cải tiến UX, bổ sung các công cụ vẽ nâng cao và hoàn thiện trải nghiệm desktop app chuyên nghiệp.

| Trường | Giá trị |
|---|---|
| Target | Phiên bản v0.1.1 — Trải nghiệm chuyên nghiệp + Công cụ vẽ nâng cao |
| Timeline | 3 tuần phát triển + 1 tuần đánh giá & polish |
| Developer | 1 person (solo developer / coding agent) |
| Baseline | Đã hoàn thành v0.1.0 MVP |

---

## 1. Mục tiêu của v0.1.1 (Goals)

Mục tiêu chính của v0.1.1 là đưa August Mark từ trạng thái một MVP (Minimum Viable Product) thành một ứng dụng desktop thực thụ, chuyên nghiệp và tối ưu cho trải nghiệm người dùng cuối (UX).

1. **Desktop Native Polish:** Tích hợp sâu hơn vào hệ điều hành (System Tray, lưu vị trí cửa sổ, settings lưu DB).
2. **Annotation Completeness:** Bổ sung các công cụ vẽ còn thiếu nhưng cực kỳ quan trọng đối với QA/Developer (Blur thông tin nhạy cảm, Highlight, Freehand vẽ tay).
3. **Canvas UX Improvement:** Cho phép hoàn tác (Undo/Redo) các nét vẽ và xóa từng annotation trực tiếp trên canvas overlay thay vì phải hủy bỏ toàn bộ.
4. **Data Management:** Khai thác tối đa cấu trúc dữ liệu đã thiết kế sẵn ở v0.1.0 để cung cấp tính năng Tagging, Search toàn cục, Sorting và thống kê phiên.
5. **Advanced Exporting:** Bổ sung các định dạng xuất báo cáo (PDF, Markdown, CSV) và bộ lọc dữ liệu trước khi export.

---

## 2. Feature Groups & Priorities

Để đảm bảo tính khả thi cho solo developer, các tính năng được phân chia cụ thể theo mức độ ưu tiên:

### ✅ Must Have (Ưu tiên cao nhất — Bắt buộc hoàn thành)

| # | Feature | Lý do & Phân loại |
|---|---|---|
| 1 | **Blur / Pixelate Tool** | Bảo mật thông tin: che password, mã thẻ, email nhạy cảm khi báo bug. |
| 2 | **Settings Page & DB Backend** | Nơi cấu hình theme, hotkey, chất lượng ảnh, delayed capture. |
| 3 | **System Tray Icon (Basic)** | Click trái mở app, click phải Quit/Settings, chạy ngầm tiện lợi. |
| 4 | **Undo / Redo Canvas** | Tránh việc vẽ sai một nét phải bấm cancel chụp lại từ đầu. |
| 5 | **Xóa từng Annotation** | Chọn và xóa một marker/shape cụ thể trước khi lưu. |
| 6 | **Freehand & Highlight Tool** | Hoàn thiện các công cụ vẽ cơ bản cần có. |
| 7 | **Sửa lỗi Hotkey Label** | Fix nhãn phím tắt hiển thị sai trên Header (từ PrntScrn thành Ctrl+Shift+M). |
| 8 | **Window State Persistence** | Lưu kích thước, vị trí cửa sổ để khi mở lại không bị reset. |
| 9 | **Tags System UI** | Kết nối bảng `tags` và `issue_tags` sẵn có lên UI. |
| 10 | **Global Search** | Tìm nhanh issue, session bằng ô search trên AppHeader. |

### 🟡 Should Have (Ưu tiên trung bình — Thực hiện sau nhóm Must Have)

| # | Feature | Lý do |
|---|---|---|
| 11 | **Sorting & Statistics** | Sắp xếp session/issue theo thời gian, độ nghiêm trọng; hiển thị badge đếm số lượng. |
| 12 | **Delayed Capture (Hẹn giờ)** | Hẹn giờ chụp sau 3s / 5s để kịp mở dropdown, tooltip hoặc menu hover. |
| 13 | **Minimize to Tray Option** | Đóng app thì thu nhỏ xuống tray thay vì thoát hẳn (cấu hình trong Settings). |

### ❌ Nice to Have / Defer (Có thể lùi lại nếu thiếu thời gian)

| # | Feature | Trạng thái |
|---|---|---|
| 14 | **Advanced Export (PDF, Markdown, CSV)** | Export HTML ở v0.1.0 đã tạm đủ. PDF/Markdown/CSV là giá trị cộng thêm. |
| 15 | **Auto-update check** | Chỉ làm ở dạng static About page kiểm tra tĩnh. |

---

## 3. Phân tích Hiện trạng (v0.1.0) & Tái sử dụng Code

Một số nền móng đã có từ v0.1.0 sẽ được tận dụng tối đa để tránh viết lại:
- **Theme:** Vuetify đã được cấu hình hai theme `augustDark` và `augustLight` trong `src/plugins/vuetify.ts`. Chúng ta chỉ cần viết UI Toggle và đồng bộ với store/DB.
- **Settings DB:** Bảng `settings` đã được định nghĩa trong file migration `v001_initial.sql` với các key mặc định. Ta chỉ cần viết Repository + Commands ở Rust và SettingsView ở Vue.
- **Tags DB:** Bảng `tags` và `issue_tags` cùng với các model quan hệ đã có sẵn ở DB. Ta cần xây dựng Vue component Combobox/Chips để gán tag khi tạo issue và lọc tag ở FilterBar.
- **Tauri Config:** Hỗ trợ plugin, tray-icon đã khai báo trong `Cargo.toml`.

---

## 4. Development Milestones

Lộ trình phát triển được chia làm 4 Milestones cụ thể trong 4 tuần:

### Milestone 1 — Desktop Experience & Settings (Tuần 1)
- Thiết lập Repository và Command cho Settings ở Rust.
- Tạo trang `SettingsView` trên Vue, cấu hình Theme Toggle, Delayed Capture logic, và lưu vào DB.
- Cấu hình System Tray Icon cho phép hiển thị menu, đóng/mở/focus cửa sổ chính.
- Tích hợp `tauri-plugin-window-state` để ghi nhớ kích thước cửa sổ.
- Sửa lỗi hiển thị phím tắt trên Header và nâng cấp trang About hiển thị version động.

### Milestone 2 — Advanced Drawing Tools & Canvas UX (Tuần 2)
- Phát triển Blur/Pixelate Tool sử dụng HTML5 Canvas pixel manipulation.
- Phát triển Freehand và Highlight tools trên Canvas.
- Triển khai cơ chế Undo/Redo stack cho annotations trên canvas overlay.
- Triển khai cơ chế click phải hoặc bấm nút X để xóa từng annotation trước khi lưu.

### Milestone 3 — Dashboard UX: Tags, Search & Filter (Tuần 3)
- Kết nối DB và xây dựng Tags UI ở `IssueFormPanel` (Combobox/Autocomplete) và hiển thị Chips.
- Thêm bộ lọc Tags vào `FilterBar`.
- Xây dựng thanh tìm kiếm toàn cục ở `AppHeader` sử dụng Tauri IPC truyền text tìm kiếm xuống SQLite.
- Thêm tính năng Sorting (sắp xếp) cho Dashboard list.
- Nâng cấp Export Dialog hỗ trợ chọn xuất định dạng PDF/Markdown/CSV và lọc Issue.

### Milestone 4 — Verification, Polish & Release (Tuần 4)
- Thực hiện kiểm thử toàn bộ hệ thống (end-to-end regression testing).
- Polish giao diện (micro-animations, transition, loading states).
- Đóng gói ứng dụng và phát hành bản v0.1.1.

---

## 5. Rủi ro & Giải pháp phòng ngừa (Risks & Mitigations)

| Rủi ro | Mức độ | Giải pháp phòng ngừa |
|---|---|---|
| **Lỗi thư viện `tauri-plugin-window-state`** | Thấp | Nếu plugin không hoạt động ổn định trên Windows, tự lưu tọa độ/kích thước cửa sổ vào bảng `settings` thông qua event lắng nghe resize/move của Tauri. |
| **Hiệu năng Canvas bị giật lag khi Blur/Pixelate nhiều vùng** | Trung bình | Xử lý cache canvas phụ (offscreen canvas). Thay vì pixelate toàn bộ ảnh, chỉ get-put pixel data của đúng bounding box của Rect vẽ Blur. |
| **Xung đột phím tắt toàn cục (Hotkey conflicts)** | Trung bình | Trong `settings_cmds.rs`, kiểm tra nếu register hotkey thất bại (phím tắt bị app khác chiếm), thông báo lỗi lên UI và tự động revert về phím tắt mặc định. |
| **Xuất PDF không có thư viện phù hợp ở Rust** | Cao | Thay vì tạo PDF engine phức tạp từ đầu ở Rust, có thể sử dụng giải pháp in ra PDF thông qua Tauri Webview Print API hoặc convert Markdown sang HTML rồi dùng template in của hệ điều hành. |

---

## 6. Acceptance Criteria (Tiêu chí Nghiệm thu Tổng quát)

- [ ] Cửa sổ ứng dụng ghi nhớ kích thước khi đóng và mở lại khớp 100%.
- [ ] Chuyển đổi Light/Dark theme hoạt động tức thì, lưu cấu hình vào SQLite.
- [ ] System tray hiển thị đúng icon, click phải hiển thị menu chức năng, click trái restore window.
- [ ] Chế độ "Minimize to tray" hoạt động khi bấm nút đóng cửa sổ (nếu bật trong Settings).
- [ ] Người dùng có thể vẽ nét vẽ tự do, highlight màu vàng bán trong suốt và blur làm mờ chi tiết nhạy cảm.
- [ ] Bấm `Ctrl+Z` thu hồi nét vẽ cuối cùng, `Ctrl+Y` phục hồi nét vẽ vừa thu hồi.
- [ ] Click phải vào nét vẽ hoặc marker bất kỳ hiển thị tùy chọn xóa.
- [ ] Gán được Tags cho Issue trong overlay, lọc được Issue theo Tags trên Dashboard.
- [ ] Nhập từ khóa vào ô Search trên Header hiển thị danh sách kết quả phù hợp, click sẽ chuyển hướng tới đúng Session/Issue.
- [ ] Xuất báo cáo hoạt động tốt ở các định dạng HTML, PDF, Markdown, CSV.

---

## 7. Chiến lược kiểm thử (Testing Strategy)

Do dự án được phát triển bởi solo developer, kiểm thử thủ công kết hợp với các kịch bản kiểm thử tích hợp (Integration Tests) nhẹ nhàng ở Rust repository là phù hợp nhất:

1. **Rust Repo Unit Tests:** Viết test suite cho `settings_repo.rs` và `tag_repo.rs` để đảm bảo thao tác DB SQLite chính xác.
2. **Manual E2E Walkthrough:** Đi theo sơ đồ luồng dữ liệu: Chụp màn hình (Delayed Capture) -> Vẽ hỗn hợp (Marker, Rect, Blur, Freehand) -> Undo/Redo -> Xóa bớt annotation -> Gán Tags -> Điền metadata -> Done -> Xem trên Dashboard -> Tìm kiếm -> Lọc -> Export.
3. **Compatibility Testing:** Đảm bảo data tạo ra từ v0.1.0 vẫn đọc và hiển thị hoàn hảo trên giao diện v0.1.1 (Backwards compatibility).

---

## 8. Release Checklist

- [ ] Chạy `cargo clippy` và `npm run lint` để kiểm tra chất lượng code và sửa các cảnh báo.
- [ ] Tăng phiên bản ứng dụng trong `Cargo.toml` và `tauri.conf.json` lên `"0.1.1"`.
- [ ] Chạy migration script và kiểm tra database nâng cấp tự động.
- [ ] Đóng gói phiên bản release: `npm run tauri build`.
- [ ] Chạy thử file installer được build ra trên máy sạch (không cài môi trường dev) để xác nhận không lỗi runtime dependency.
- [ ] Tạo commit release và tag `v0.1.1` trên git.
