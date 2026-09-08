# V020_CODE_ORDER.md — August Mark v0.2.0

> Thứ tự lập trình tối ưu cho phiên bản v0.2.0. Đảm bảo quy trình phát triển an toàn, kiểm chứng được luồng dữ liệu ở từng chặng và giảm thiểu rủi ro kỹ thuật.

---

## Coding Sequence

| # | Bước | Rust Files | Vue Files | Lý do thứ tự này |
|---|---|---|---|---|
| **1** | Cấu hình Thư viện | `Cargo.toml` | — | Bổ sung `reqwest`, `tiny_http`, và `zip` làm nền tảng. |
| **2** | Định nghĩa Data Models | `models/gdrive.rs`, `models/aacp.rs`, `models/mod.rs` | `types/aacp.ts` | Đồng bộ cấu trúc dữ liệu JSON giữa Rust và TypeScript. |
| **3** | Google Drive API Service | `services/gdrive_client.rs`, `services/mod.rs` | — | Xây dựng lõi kết nối HTTP client với Google API. |
| **4** | OAuth2 Local Server & Cmds | `commands/gdrive_cmds.rs`, `commands/mod.rs`, `main.rs` | `services/tauriCommands.ts` | Mở HTTP server tại localhost để bắt Auth Code từ browser. |
| **5** | GDrive Settings Store & UI | — | `stores/settingsStore.ts`, `views/SettingsView.vue` | Tạo giao diện đăng nhập Google Drive trong trang Cài đặt. |
| — | **⬆ CHECKPOINT 1** | **Xác thực Google Drive thành công. Token được lưu trữ bảo mật.** | | |
| **6** | Backup & Restore Backend | `services/gdrive_client.rs` (bổ sung zip/unzip DB và media) | — | Thuật toán nén nạp dữ liệu SQLite và Screenshots. |
| **7** | Backup & Restore UI Action | `commands/gdrive_cmds.rs` (bổ sung cmd) | update `views/SettingsView.vue` | Tích hợp nút sao lưu/khôi phục và hiển thị progress. |
| **8** | Share Link Generator | `commands/gdrive_cmds.rs` (share commands) | update `components/dashboard/SessionList.vue`, `IssueDetail.vue` | Tải báo cáo HTML lên đám mây và lấy liên kết công khai. |
| — | **⬆ CHECKPOINT 2** | **Đồng bộ đám mây hoạt động. Backup/Restore và chia sẻ liên kết thành công.** | | |
| **9** | Git CLI Inspector | `services/git_inspector.rs`, `services/mod.rs` | — | Thực thi lệnh Git trên thư mục cục bộ của nhà phát triển. |
| **10** | AACP Exporter Core | `services/aacp_exporter.rs`, `services/mod.rs` | — | Sinh tệp JSON manifest, tệp markdown prompt và nén zip. |
| **11** | AACP Exporter Cmds | `commands/aacp_cmds.rs`, `commands/mod.rs`, `main.rs` | update `services/tauriCommands.ts` | IPC bridge để Vue yêu cầu xuất gói AI. |
| **12** | AACP UI Export Dialog | — | update `components/export/ExportDialog.vue` | UI tab mới trong Export Dialog để chọn thư mục và xuất. |
| — | **⬆ CHECKPOINT 3** | **Xuất gói ngữ cảnh AI Agent (AACP) hoạt động hoàn chỉnh và chính xác.** | | |
| **13** | Regression Testing | — | — | Đảm bảo các chức năng vẽ nâng cao và SQLite cũ tương thích tốt. |
| **14** | Bump version & Build | update `Cargo.toml`, `tauri.conf.json`, `package.json` | update `CHANGELOG.md` | Phát hành phiên bản release `v0.2.0` |
| — | **⬆ CHECKPOINT 4** | **Bàn giao phiên bản v0.2.0 hoàn tất.** | | |

---

## Giải Thích Sự Phụ Thuộc & Chiến Lược Giảm Thiểu Rủi Ro

### 1. Chiến lược High-Risk-First (Rủi ro cao xử lý trước)

- **Xác thực OAuth2 trên Desktop App (High Risk):** Khác với môi trường Web có Redirect URI tĩnh, app desktop Tauri cần một cơ chế hứng callback linh hoạt. Do đó, việc cấu hình cổng HTTP localhost tạm thời và thu hồi Token được viết đầu tiên. Nếu gặp sự cố với chính sách CORS hoặc Network Sandbox của Tauri, ta có thể đổi sang giải pháp hướng dẫn người dùng dán thủ công Auth Code vào app (Manual Copy-Paste Auth Code) làm phương án dự phòng.
- **Git Command Execution (Medium Risk):** Không phải môi trường chạy app nào cũng cài sẵn Git CLI. Module `git_inspector` sẽ bọc lệnh git bằng các cấu trúc `match` an toàn, nếu lệnh lỗi sẽ trả về siêu dữ liệu Git trống chứ không làm hỏng tiến trình xuất file.

### 2. Thiết kế theo chiều dọc (Vertical Slices)

- Các bước **OAuth2**, **Backup/Restore** và **AACP Export** được triển khai dọc từ Database -> Rust -> Tauri Command -> Pinia Store -> Vue UI.
- Cách tiếp cận này giúp kiểm tra ngay tính tương thích của JSON serialization/deserialization ở mỗi tính năng mà không bị nghẽn ở lớp giao diện.
