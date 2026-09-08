# V020_IMPLEMENTATION_PLAN.md — August Mark v0.2.0

> Kế hoạch nâng cấp v0.2.0 dành cho solo developer. Tập trung vào khả năng đồng bộ đám mây (Google Drive Sync & Backup) và cầu nối tự động hóa sửa lỗi thông qua AI Agent (AI Agent Context Pack Exporter).

| Trường | Giá trị |
|---|---|
| Target | Phiên bản v0.2.0 — Đồng bộ đám mây + Xuất gói ngữ cảnh AI Agent |
| Timeline | 3 tuần phát triển + 1 tuần đánh giá & polish |
| Developer | 1 person (solo developer / coding agent) |
| Baseline | Đã hoàn thành v0.1.1 với công cụ vẽ nâng cao, tags và search |

---

## 1. Mục tiêu của v0.2.0 (Goals)

Mục tiêu chính của v0.2.0 là chuyển dịch August Mark từ một ứng dụng ghi chép lỗi cục bộ (offline-first local-only) thành một công cụ tích hợp sâu vào quy trình làm việc hiện đại của QC/QA và Developer, tận dụng sức mạnh của Cloud Sync và các AI Agent sửa lỗi tự động.

1. **Google Drive Integration (Sync & Share):**
   - **OAuth2 desktop flow:** Triển khai cơ chế xác thực tài khoản Google ngay trong ứng dụng desktop bằng trình duyệt và máy chủ local loopback.
   - **Database & Media Backup:** Tự động hoặc thủ công nén file SQLite (`august_mark.db`) và thư mục ảnh chụp (`screenshots/`) tải lên thư mục an toàn trên Google Drive cá nhân của người dùng.
   - **Share Link Generation:** Xuất báo cáo HTML trực tiếp lên Google Drive và lấy liên kết công khai (Public Link) để chia sẻ tức thì qua Slack/Email cho khách hàng hoặc đồng nghiệp.
2. **AI Agent Context Pack (AACP) Exporter:**
   - **Gói ngữ cảnh thông minh:** Xuất một thư mục hoặc tệp ZIP chứa đầy đủ thông tin lỗi ở định dạng chuẩn hóa để các AI Coding Agent (như Antigravity, Roo Code, Aider, Codex, Claude Engineer) có thể đọc hiểu và tự động định vị, sửa lỗi trong mã nguồn.
   - **Git Context Integration:** Thu thập thông tin từ thư mục dự án cục bộ của lập trình viên bao gồm: đường dẫn tuyệt đối, Git commit hash, Git branch hiện tại và danh sách các file nghi ngờ lỗi (Suspected Files).
   - **LLM-Optimized Prompting:** Tạo tệp `agent_instruction.md` chứa prompt tối ưu hóa cho mô hình đa phương tiện (multimodal LLM), trỏ trực tiếp đến ảnh chụp lỗi gốc và ảnh khoanh vùng cắt nhỏ (crops) để AI đối chiếu.

---

## 2. Feature Groups & Priorities

### ✅ Must Have (Ưu tiên cao nhất — Bắt buộc hoàn thành)

| # | Feature | Lý do & Phân loại |
|---|---|---|
| 1 | **Tauri OAuth2 Local Server** | Mở trình duyệt xác thực Google Drive và bắt mã Auth Code thông qua một HTTP server tạm thời trên localhost. |
| 2 | **Secure Token Storage** | Lưu trữ Access Token và Refresh Token mã hóa vào database SQLite để tự động làm mới (refresh) phiên làm việc. |
| 3 | **Google Drive Client (Rust)** | Xây dựng client kết nối Google Drive API v3 để tạo thư mục, upload tệp tin và chỉnh sửa quyền chia sẻ (permission). |
| 4 | **Cloud Backup & Restore** | Chức năng nén toàn bộ database + ảnh chụp thành file `.zip` và upload lên Drive. Cho phép tải xuống và khôi phục khi chuyển máy. |
| 5 | **Share to Cloud** | Xuất báo cáo HTML lên Drive, đặt quyền "Anyone with link can view" và trả về link chia sẻ ngay trên giao diện Dashboard. |
| 6 | **AACP Data Model & Manifest** | Định nghĩa cấu trúc JSON của tệp `issue_manifest.json` lưu trữ siêu dữ liệu lỗi và thông tin Git dự án. |
| 7 | **AACP Prompt Generator** | Tự động sinh tệp markdown `agent_instruction.md` đóng gói hướng dẫn chi tiết dành riêng cho AI Agent. |
| 8 | **AACP UI Export Panel** | Dialog cho phép chọn Thư mục mã nguồn cục bộ (Workspace Path), nhập các file nghi ngờ lỗi (Suspected Files) và chỉ định nơi xuất Gói Context. |
| 9 | **Rust Git Inspector** | Sử dụng lệnh Shell / Git để lấy nhánh hiện tại, commit hash mới nhất và trạng thái diff (nếu có) từ Workspace được chọn. |

### 🟡 Should Have (Ưu tiên trung bình — Thực hiện sau nhóm Must Have)

| # | Feature | Lý do |
|---|---|---|
| 10 | **Auto-backup Schedule** | Tự động đồng bộ/sao lưu dữ liệu lên Google Drive sau mỗi 24 giờ hoặc mỗi khi đóng ứng dụng. |
| 11 | **OAuth Revocation & Disconnect** | Nút ngắt kết nối tài khoản Google và thu hồi Token trên Google Server để đảm bảo an toàn bảo mật. |
| 12 | **AACP Workspace Autocomplete** | Đọc danh sách file trong thư mục dự án đã chọn để gợi ý (autocomplete) khi người dùng nhập "Suspected Files". |

### ❌ Nice to Have / Defer (Có thể lùi lại nếu thiếu thời gian)

| # | Feature | Trạng thái |
|---|---|---|
| 13 | **Google Drive Explorer UI** | Giao diện duyệt file trực tiếp trên Drive ngay trong Dashboard. Tạm hoãn, chỉ cần link tải/sao lưu. |
| 14 | **Direct Integration with Github CLI/Agent** | Tự động tạo Branch mới và chạy Agent trực tiếp từ ứng dụng. Tạm hoãn, để lập trình viên tự chạy Agent bằng gói AACP được xuất ra. |

---

## 3. Phân tích Hiện trạng (v0.1.1) & Tái sử dụng Code

- **Settings DB:** Chúng ta sẽ mở rộng bảng `settings` để lưu thêm các thông tin:
  - `gdrive_connected` (true/false)
  - `gdrive_refresh_token` (TEXT)
  - `gdrive_backup_enabled` (true/false)
  - `gdrive_auto_backup_interval` (TEXT - ví dụ: "daily")
- **Export Dialog:** Component `src/components/export/ExportDialog.vue` đã được thiết kế hoàn thiện ở v0.1.1. Chúng ta sẽ mở rộng nó bằng cách thêm một tab mới hoặc tuỳ chọn xuất "AI Agent Context Pack (AACP)" bên cạnh HTML/PDF/Markdown/CSV.
- **Tauri Shell Plugin:** Ứng dụng đã có `tauri-plugin-shell` khai báo trong `Cargo.toml`. Chúng ta có thể dùng plugin này để chạy các lệnh git hoặc quét danh sách file phục vụ tính năng autocomplete và git metadata.
- **SQLite Database:** Giữ nguyên cấu trúc các bảng lỗi `sessions`, `issues`, `tags` để đảm bảo tính ổn định và tương thích dữ liệu cũ.

---

## 4. Development Milestones

### Milestone 1 — OAuth2 Flow & Google Drive Client Backend (Tuần 1)
- Cấu hình thư viện HTTP client `reqwest` và `zip` trong `Cargo.toml` của Rust backend.
- Xây dựng module OAuth2: Tạo server HTTP cục bộ (`tiny_http` hoặc `hyper` phiên bản nhẹ) để hứng redirect URI `http://localhost:port` từ Google Auth page.
- Tạo Tauri Command để kích hoạt luồng đăng nhập, mở trình duyệt mặc định thông qua shell.
- Viết Repository lưu trữ Token vào cơ sở dữ liệu (mã hóa cơ bản).
- Xây dựng Google Drive API Client hỗ trợ:
  - `upload_file` (tải lên file đơn lẻ).
  - `create_directory` (tạo thư mục dự án).
  - `get_share_link` (thay đổi quyền truy cập thành public và lấy shareable webContentLink).

### Milestone 2 — Cloud Sync, Backup & Share UI (Tuần 2)
- Tạo giao diện kết nối tài khoản Google trong `SettingsView.vue` (Hiển thị trạng thái Connected, Email tài khoản, nút Disconnect).
- Triển khai chức năng Sao lưu cơ sở dữ liệu: Nén database và ảnh chụp thành zip, upload lên thư mục riêng trên Drive, hiển thị progress bar trên UI.
- Triển khai chức năng Khôi phục dữ liệu từ bản sao lưu Drive.
- Tích hợp nút "Tạo link chia sẻ đám mây" (Share to Google Drive) vào Dashboard Session và Issue. Khi bấm, Rust nén HTML report, đẩy lên Drive và hiển thị link công khai cho người dùng copy.

### Milestone 3 — AI Agent Context Pack Exporter Engine (Tuần 3)
- Triển khai module Rust `git_inspector.rs` truy vấn thông tin Git từ thư mục chỉ định.
- Viết logic xuất AACP ở Rust backend:
  - Viết file `issue_manifest.json` chứa cấu trúc siêu dữ liệu.
  - Viết công cụ tạo prompt `agent_instruction.md` dựa trên template markdown tối ưu cho AI.
  - Sao chép screenshot chính và các crop khoanh vùng liên quan vào thư mục xuất.
  - Nén toàn bộ thư mục thành file `.aacp.zip` nếu người dùng yêu cầu dạng nén.
- Thiết kế UI xuất AACP trong `ExportDialog.vue`:
  - Ô chọn thư mục Workspace dự án đích (gọi Tauri Open Dialog).
  - Ô nhập danh sách các tệp tin nghi ngờ (suspected files).
  - Nút xuất file và mở thư mục đích sau khi hoàn thành.

### Milestone 4 — Integration, End-to-end Testing & Release (Tuần 4)
- Kiểm thử tích hợp toàn bộ luồng đồng bộ: Đăng nhập -> Sync -> Lấy link share -> Logout.
- Kiểm thử khả năng đọc hiểu của AI Agent: Dùng thử file AACP xuất ra để nạp cho các AI Agent (như Antigravity hoặc Gemini) trên các project thử nghiệm thực tế xem chúng có đọc hiểu và sửa đúng file được định vị hay không.
- Hoàn thiện các hiệu ứng chuyển động (progress loading, sync status badges).
- Tăng version manifest lên `0.2.0`, phát hành bản cập nhật chính thức.

---

## 5. Rủi ro & Giải pháp phòng ngừa (Risks & Mitigations)

| Rủi ro | Mức độ | Giải pháp phòng ngừa |
|---|---|---|
| **Google OAuth Client ID bị lộ hoặc cần xác minh ứng dụng (App Verification)** | Cao | Sử dụng OAuth2 với Client ID cục bộ (Installed App flow) có giới hạn quyền `drive.file` (chỉ thao tác trên các file do August Mark tạo ra), giúp giảm thiểu rủi ro bảo mật và tránh thủ tục xác minh phức tạp của Google. |
| **Xung đột Port khi chạy local loopback server để nhận OAuth code** | Trung bình | Thay vì cố định port, Rust backend sẽ tự quét port rảnh trong dải từ `45000` đến `46000` để bind server. Đăng ký dải Redirect URIs tương ứng trong Google Cloud Console. |
| **Lệnh Git bị lỗi nếu máy lập trình viên chưa cài Git CLI** | Trung bình | Kiểm tra sự tồn tại của lệnh `git` trước khi chạy. Nếu không có git, tự động bỏ qua phần Git metadata trong file JSON và ghi nhận vào log, vẫn cho phép xuất gói AACP bình thường. |
| **Bản sao lưu quá lớn gây timeout kết nối mạng** | Trung bình | Chia nhỏ quá trình upload hoặc áp dụng chunked upload (Resumable Uploads) của Google Drive API đối với các tệp tin nén có dung lượng lớn hơn 5MB. |

---

## 6. Acceptance Criteria (Tiêu chí Nghiệm thu Tổng quát)

- [ ] Người dùng có thể nhấn "Connect Google Drive" từ cài đặt, hoàn thành đăng nhập trên browser và quay lại app thấy trạng thái "Connected".
- [ ] Bấm nút "Backup Now" tải thành công file dữ liệu lên Drive và hiển thị trong thư mục "August Mark Backups".
- [ ] Bấm "Restore" tải đúng tệp sao lưu và nạp lại cơ sở dữ liệu thành công (không bị crash app).
- [ ] Bấm "Share Session" xuất ra liên kết dạng `https://drive.google.com/...` có thể mở được trên trình duyệt ẩn danh không cần login.
- [ ] Dialog xuất AACP cho phép chọn đường dẫn thư mục code, nhập file nghi ngờ.
- [ ] File `issue_manifest.json` và `agent_instruction.md` sinh ra chứa thông tin chính xác về commit hash, branch và đường dẫn tương đối của các suspected files.
- [ ] Gói AACP xuất ra định dạng Zip chứa ảnh screenshot, ảnh crops sắc nét và siêu dữ liệu chính xác.
- [ ] Hệ thống hoạt động offline bình thường đối với các tính năng không liên quan đến đám mây.

---

## 7. Chiến lược kiểm thử (Testing Strategy)

1. **OAuth Loopback Tests:** Test kịch bản cổng mạng bị chiếm, kịch bản người dùng đóng tab xác thực giữa chừng (timeout).
2. **Sync Integrity Tests:** So sánh MD5 checksum của database local và database sau khi backup-restore để đảm bảo dữ liệu không bị hỏng hóc trong quá trình nén và truyền tải.
3. **AI Agent E2E Fix Test:**
   - Tạo một repository test nhỏ có chứa một bug CSS đơn giản.
   - Dùng August Mark chụp màn hình và khoanh vùng lỗi.
   - Chọn export AACP cho issue này, trỏ workspace vào test repo và nhập suspected file CSS.
   - Nạp tệp `agent_instruction.md` và các assets cho AI Agent để kiểm chứng AI có viết đúng patch sửa lỗi mà không cần giải thích thêm từ con người hay không.

---

## 8. Release Checklist

- [ ] Cập nhật phiên bản lên `"0.2.0"` trong `Cargo.toml`, `tauri.conf.json` và `package.json`.
- [ ] Cấu hình Google Client Credentials môi trường production.
- [ ] Biên dịch release build trên Windows: `npm run tauri build`.
- [ ] Cập nhật tài liệu hướng dẫn cấu hình Google Drive và hướng dẫn sử dụng AACP cho Lập trình viên vào `README.md`.
- [ ] Tạo tag Git `v0.2.0` trên nhánh `main`.
