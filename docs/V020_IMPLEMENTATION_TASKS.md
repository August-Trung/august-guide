# V020_IMPLEMENTATION_TASKS.md — August Mark v0.2.0

> Danh sách Task chi tiết phục vụ cho solo developer hoặc coding agent. Các task được thiết kế để thực hiện tuần tự, hạn chế xung đột code, dễ dàng kiểm thử và đảm bảo tính bền vững của kiến trúc.

---

## MILESTONE 1 — OAuth2 Flow & Google Drive Client Backend (Tuần 1)

**Mục tiêu:** Tích hợp thư viện mới, thiết lập cơ chế đăng nhập Google OAuth2 thông qua máy chủ localhost tạm thời trên desktop app, lưu trữ Token an toàn và xây dựng API client kết nối Google Drive.

---

### T1.01 — Rust Dependencies & Configuration

**Goal:** Thêm các thư viện cần thiết phục vụ cho kết nối mạng HTTP, nén tệp zip và khởi tạo local HTTP server.

**Files to modify:**
- `src-tauri/Cargo.toml`

**Dependencies:** Không có.

**Implementation notes:**
- Thêm các crate sau vào mục `[dependencies]`:
  - `reqwest = { version = "0.12", features = ["json", "multipart"] }`
  - `tiny_http = "0.12"` (hoặc một HTTP server nhỏ tương tự)
  - `zip = "1.0"` (dùng để nén backup database và thư mục screenshot)
- Chạy thử `cargo check` ở thư mục `src-tauri` để đảm bảo các thư viện được tải xuống và biên dịch bình thường.

**Acceptance criteria:**
- [ ] Các thư viện được khai báo đúng phiên bản trong `Cargo.toml`.
- [ ] Chạy `cargo check` không gặp lỗi biên dịch.

**Manual test steps:**
- Mở terminal trong thư mục `src-tauri` và gõ `cargo check`. Đảm bảo tiến trình kết thúc thành công.

---

### T1.02 — Google Drive Models & Error handling

**Goal:** Định nghĩa cấu trúc dữ liệu cho Access/Refresh Token và tích hợp lỗi liên quan đến kết nối mạng vào hệ thống xử lý lỗi hiện có.

**Files to create:**
- `src-tauri/src/models/gdrive.rs`

**Files to modify:**
- `src-tauri/src/models/mod.rs` (Đăng ký model `gdrive`)
- `src-tauri/src/error.rs` (Thêm các biến thể lỗi mạng và API)

**Dependencies:** T1.01

**Implementation notes:**
- Trong `models/gdrive.rs`, định nghĩa các struct:
  - `#[derive(Serialize, Deserialize)] pub struct GoogleToken { access_token: String, refresh_token: Option<String>, expires_in: i64, token_type: String }`
  - `#[derive(Serialize, Deserialize)] pub struct GoogleUserInfo { email: String, picture: Option<String> }`
- Trong `error.rs`, bổ sung vào enum `AppError`:
  - `#[error("Network error: {0}")] NetworkError(#[from] reqwest::Error)`
  - `#[error("OAuth error: {0}")] OAuthError(String)`
  - `#[error("Google Drive API error: {0}")] GDriveError(String)`

**Acceptance criteria:**
- [ ] Struct `GoogleToken` và `GoogleUserInfo` biên dịch thành công.
- [ ] Lỗi `AppError` hỗ trợ chuyển đổi tự động (from) từ `reqwest::Error`.

**Manual test steps:**
- Chạy `cargo check` để xác minh việc import và định nghĩa lỗi không gặp lỗi biên dịch nào.

---

### T1.03 — Google OAuth2 Local Server

**Goal:** Thiết lập luồng OAuth2.0 Desktop Flow sử dụng máy chủ HTTP cục bộ tạm thời để lấy mã Authorization Code từ trình duyệt web.

**Files to create:**
- `src-tauri/src/services/gdrive_client.rs` (Khởi tạo khung xương dịch vụ)

**Files to modify:**
- `src-tauri/src/services/mod.rs`

**Dependencies:** T1.02

**Implementation notes:**
- Trong `gdrive_client.rs`, viết hàm khởi chạy server loopback:
  - Hàm quét cổng tự động từ `45000` đến `46000` đến khi tìm thấy cổng trống: `tiny_http::Server::http(format!("127.0.0.1:{}", port))`.
  - Hàm sinh Google Auth URL chứa Client ID (cấu hình tĩnh) và Redirect URI dạng `http://127.0.0.1:port`.
  - Mở URL trên trình duyệt mặc định thông qua shell: `tauri::shell::open`.
  - Lắng nghe request trên server http. Khi nhận được request từ trình duyệt chứa query parameter `code=...`, trích xuất mã code, gửi phản hồi HTML "Xác thực thành công! Bạn có thể đóng tab này" cho trình duyệt, sau đó shutdown HTTP server và trả về Auth Code.

**Acceptance criteria:**
- [ ] Cổng HTTP được quét và gán tự động mà không bị cố định.
- [ ] Server tự động dừng lại ngay sau khi nhận được Auth Code hoặc quá thời gian timeout (3 phút).

**Manual test steps:**
- Viết unit test hoặc chạy thử lệnh in ra URL auth và chạy thử server tạm thời để xác minh cổng mạng hoạt động.

---

### T1.04 — Rust Google Drive Service Implementation

**Goal:** Triển khai các hàm trao đổi mã Auth Code lấy Access/Refresh Token, tự động làm mới token và upload file đơn giản lên Google Drive.

**Files to modify:**
- `src-tauri/src/services/gdrive_client.rs`

**Dependencies:** T1.03

**Implementation notes:**
- Triển khai hàm `exchange_token(auth_code: &str, redirect_uri: &str) -> Result<GoogleToken, AppError>` gọi endpoint `https://oauth2.googleapis.com/token`.
- Triển khai hàm `refresh_access_token(refresh_token: &str) -> Result<GoogleToken, AppError>` để lấy access token mới khi hết hạn (thông qua refresh token lưu trong DB).
- Triển khai hàm `upload_file(access_token: &str, file_name: &str, mime_type: &str, data: Vec<u8>, parent_id: Option<&str>) -> Result<String, AppError>` sử dụng multipart upload của Drive API `https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart`.
- Triển khai hàm `create_folder_if_not_exists(access_token: &str, name: &str) -> Result<String, AppError>`.

**Acceptance criteria:**
- [ ] Hàm `upload_file` trả về đúng file ID từ Google API.
- [ ] Triển khai đúng luồng Refresh Token khi Access Token hết hạn (401 Unauthorized).

**Manual test steps:**
- Mô phỏng cuộc gọi API với mock token hoặc chạy integration tests với tài khoản Google sandbox.

---

### T1.05 — Google Drive IPC Commands (Rust Backend)

**Goal:** Tạo Tauri Commands để Vue frontend có thể điều khiển và truy xuất trạng thái kết nối Google Drive.

**Files to create:**
- `src-tauri/src/commands/gdrive_cmds.rs`

**Files to modify:**
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/main.rs` (Đăng ký commands vào tauri handler)

**Dependencies:** T1.04

**Implementation notes:**
- Viết các command:
  - `#[tauri::command] pub async fn connect_gdrive(state: State<'_, AppState>) -> Result<String, AppError>`: Chạy server local, lấy Auth Code, exchange token, lưu refresh_token vào SQLite bảng `settings` thông qua `settings_repo`, lấy email người dùng qua `GoogleUserInfo` lưu vào setting `gdrive_user_email` và trả về email này.
  - `#[tauri::command] pub async fn disconnect_gdrive(state: State<'_, AppState>) -> Result<(), AppError>`: Xoá bỏ refresh token và email khỏi SQLite.
  - `#[tauri::command] pub async fn check_gdrive_status(state: State<'_, AppState>) -> Result<Option<String>, AppError>`: Trả về email người dùng nếu đã kết nối, ngược lại trả về `None`.

**Acceptance criteria:**
- [ ] Tauri Handler đăng ký thành công `connect_gdrive`, `disconnect_gdrive`, và `check_gdrive_status`.
- [ ] Token được lưu trữ bảo mật (không log ra console hoặc file không an toàn).

**Manual test steps:**
- Mở DevTools console của Tauri app, chạy: `await window.__TAURI__.core.invoke('check_gdrive_status')` và kiểm tra kết quả trả về.

---

### T1.06 — GDrive settings store & UI Sync (Frontend Layer)

**Goal:** Cập nhật frontend state và hiển thị nút đăng nhập/đăng xuất trong trang Cài đặt.

**Files to modify:**
- `src/types/settings.ts` (Thêm các thuộc tính google drive)
- `src/services/tauriCommands.ts` (Thêm các wrapper gọi tới commands gdrive của Rust)
- `src/stores/settingsStore.ts` (Quản lý state gdrive)
- `src/views/SettingsView.vue` (Tạo giao diện hiển thị trạng thái tài khoản)

**Dependencies:** T1.05

**Implementation notes:**
- Trong `types/settings.ts`, thêm `gdriveConnected: boolean` và `gdriveEmail: string | null`.
- Trong `settingsStore.ts`, thêm các actions:
  - `connectGDrive()`: gọi command `connect_gdrive`, cập nhật state local.
  - `disconnectGDrive()`: gọi command `disconnect_gdrive`, xoá state local.
- Trong `SettingsView.vue`, thiết kế thêm tab hoặc nhóm chức năng "Cloud Sync & Backup":
  - Nếu chưa kết nối: Hiển thị nút "Connect Google Drive" kèm icon Google đầy màu sắc. Khi bấm, hiển thị loading spinner và đợi trình duyệt hoàn thành.
  - Nếu đã kết nối: Hiển thị email tài khoản, hình ảnh đại diện (nếu có), trạng thái "Connected" màu xanh lục và nút "Disconnect".

**Acceptance criteria:**
- [ ] Bấm nút đăng nhập sẽ mở browser và hiển thị đúng màn hình uỷ quyền của Google.
- [ ] Khi đăng nhập hoàn tất trên browser, Vue nhận được tín hiệu thành công và chuyển sang giao diện "Connected" tức thì.
- [ ] Bấm ngắt kết nối sẽ xóa sạch session và đưa UI về trạng thái chưa đăng nhập.

**Manual test steps:**
1. Khởi động app, vào Settings -> Cloud Sync.
2. Click "Connect Google Drive". Xác nhận trình duyệt được mở ra trang Google Login.
3. Cho phép uỷ quyền. Quay lại ứng dụng, xác nhận giao diện chuyển sang hiển thị email tài khoản của bạn.
4. Tắt app bật lại, xác nhận trạng thái tài khoản vẫn được giữ nguyên.

---
---

## MILESTONE 2 — Cloud Sync, Backup & Share UI (Tuần 2)

**Mục tiêu:** Phát triển tính năng nén cơ sở dữ liệu SQLite cục bộ cùng thư mục media thành tệp ZIP, sao lưu lên Google Drive, khôi phục từ bản sao lưu và hỗ trợ xuất nhanh HTML báo cáo lên Drive để chia sẻ link công khai.

---

### T2.01 — SQLite & Media Backup Compressor

**Goal:** Viết các hàm nén/giải nén database SQLite và thư mục ảnh screenshots thành tệp ZIP trong Rust backend.

**Files to modify:**
- `src-tauri/src/services/gdrive_client.rs` (Hoặc tạo module `services/backup_service.rs`)

**Dependencies:** T1.01

**Implementation notes:**
- Sử dụng thư viện `zip` để tạo tệp ZIP:
  - File SQLite `august_mark.db` và toàn bộ các tệp ảnh `.png` trong thư mục `screenshots/` được nén lại theo cấu trúc thư mục chuẩn.
  - Cần thực hiện khoá connection hoặc gọi `VACUUM INTO` để tạo file backup SQLite sạch sẽ trước khi nén để tránh hỏng dữ liệu (database corruption) nếu ghi dữ liệu dở dang.
  - Triển khai hàm giải nén (restore) nhận tệp zip, giải nén ghi đè cơ sở dữ liệu hiện tại và giải phóng các tệp ảnh vào thư mục screenshot. Cần backup database hiện tại ra file tạm trước khi khôi phục, đề phòng file zip tải về bị lỗi.

**Acceptance criteria:**
- [ ] File zip tạo ra có thể mở được bằng các công cụ giải nén tiêu chuẩn (như WinRAR, 7-Zip).
- [ ] Giải nén thành công phục hồi 100% dữ liệu database và ảnh session cũ.

**Manual test steps:**
- Viết unit test nén một thư mục giả, giải nén ở một vị trí khác và so sánh kích thước cũng như MD5 checksum của các file nguồn và đích.

---

### T2.02 — Database Backup & Restore Commands

**Goal:** Cung cấp Tauri commands để thực hiện sao lưu/khôi phục dữ liệu lên Google Drive.

**Files to modify:**
- `src-tauri/src/commands/gdrive_cmds.rs`

**Dependencies:** T2.01

**Implementation notes:**
- Tạo Tauri commands:
  - `#[tauri::command] pub async fn backup_to_gdrive(state: State<'_, AppState>) -> Result<String, AppError>`:
    - Lấy access token (refresh nếu cần).
    - Gọi hàm nén zip tạo file `august_backup_[timestamp].zip`.
    - Tìm hoặc tạo thư mục `August Mark Backups` trên Drive.
    - Upload file zip lên thư mục này. Trả về ID file đã upload.
  - `#[tauri::command] pub async fn restore_from_gdrive(state: State<'_, AppState>, file_id: String) -> Result<(), AppError>`:
    - Tải file zip tương ứng từ Drive về bộ nhớ tạm.
    - Thực hiện khôi phục ghi đè database cục bộ.
- Xử lý timeout kết nối mạng tối đa 5 phút cho các file dung lượng lớn.

**Acceptance criteria:**
- [ ] Command `backup_to_gdrive` trả về thông tin thời gian backup thành công.
- [ ] Phục hồi thành công thay đổi giao diện dữ liệu cục bộ tức thì mà không cần cài đặt lại app.

**Manual test steps:**
- Gọi invoke command từ dev console, kiểm tra trên giao diện Google Drive Web xem có xuất hiện thư mục `August Mark Backups` và tệp tin zip hay chưa.

---

### T2.03 — Settings Backup & Restore UI

**Goal:** Thêm các nút điều khiển sao lưu và khôi phục vào SettingsView.

**Files to modify:**
- `src/views/SettingsView.vue`
- `src/stores/settingsStore.ts`

**Dependencies:** T2.02

**Implementation notes:**
- Trên giao diện SettingsView (mục Google Drive):
  - Hiển thị thông tin: "Last Backup: [Thời gian]" (đọc từ settings DB).
  - Nút "Backup Now": Khi bấm, hiển thị progress bar/circular loading và dòng chữ "Backing up database and screenshots to Google Drive...".
  - Nút "Restore Data": Cho phép người dùng dán File ID Google Drive hoặc chọn từ danh sách file backup (nếu ta viết thêm lệnh list file, hoặc đơn giản cho nhập File ID ở v0.2.0 để tinh gọn). Hiển thị cảnh báo: "Hành động này sẽ ghi đè toàn bộ dữ liệu hiện tại. Bạn có chắc chắn?".

**Acceptance criteria:**
- [ ] Giao diện khóa (disable) các nút tương tác khác khi đang backup để tránh xung đột dữ liệu.
- [ ] Hiển thị thông báo thành công (Toast/Snackbar) màu xanh sau khi hoàn tất.

**Manual test steps:**
1. Tạo một vài session giả lập trên máy.
2. Vào Settings, bấm "Backup Now". Đợi tiến trình hoàn tất.
3. Xoá app, cài lại (hoặc xoá thủ công file database sqlite cục bộ).
4. Vào lại Settings, kết nối Google Drive và bấm "Restore Data" sử dụng File ID vừa tạo. Xác nhận các session cũ xuất hiện lại đầy đủ trên Dashboard.

---

### T2.04 — HTML Cloud Share Service (Rust Backend)

**Goal:** Upload báo cáo HTML lên Drive và cấu hình quyền truy cập công khai để chia sẻ liên kết.

**Files to modify:**
- `src-tauri/src/services/gdrive_client.rs`
- `src-tauri/src/commands/gdrive_cmds.rs`

**Dependencies:** T1.04, T2.22 (HTML Exporter của v0.1.1)

**Implementation notes:**
- Viết command `#[tauri::command] pub async fn share_session_on_gdrive(state: State<'_, AppState>, session_id: i64) -> Result<String, AppError>`:
  - Sinh báo cáo HTML của session dưới dạng chuỗi string trong RAM (tái sử dụng module export html có sẵn).
  - Tải chuỗi HTML này lên Google Drive với tên file `august_report_session_{id}.html`.
  - Gọi Google Drive API `Permissions: create` để gán quyền `{ "role": "reader", "type": "anyone" }` cho file vừa tạo.
  - Lấy thông tin `webViewLink` từ API trả về để cung cấp đường dẫn xem trực tuyến.

**Acceptance criteria:**
- [ ] Link Drive trả về có quyền public đọc công khai.
- [ ] File HTML upload được mã hóa UTF-8 chuẩn xác, không lỗi font tiếng Việt.

**Manual test steps:**
- Chạy thử command, copy link trả về và paste vào một cửa sổ trình duyệt ẩn danh (Incognito). Đảm bảo báo cáo hiển thị chính xác toàn bộ nội dung.

---

### T2.05 — Dashboard Cloud Share Integration

**Goal:** Thêm nút Share lên giao diện Dashboard danh sách Session và chi tiết Issue.

**Files to modify:**
- `src/components/dashboard/SessionList.vue`
- `src/components/dashboard/IssueList.vue` (Hoặc `IssueDetail.vue`)
- `src/services/tauriCommands.ts`

**Dependencies:** T2.04

**Implementation notes:**
- Thêm biểu tượng nút Cloud Share (đám mây kèm mũi tên / share icon) bên cạnh nút Export cục bộ.
- Khi click:
  - Gọi wrapper `shareSessionOnGDrive(sessionId)`.
  - Hiển thị dialog nhỏ chứa link liên kết: `https://drive.google.com/file/d/[id]/view?usp=drivesdk`.
  - Nút "Copy Link" để người dùng lưu nhanh vào Clipboard.

**Acceptance criteria:**
- [ ] Trạng thái nút hiển thị spinner "Uploading..." trong lúc API đang xử lý.
- [ ] Liên kết copy vào Clipboard hoạt động chính xác.

**Manual test steps:**
1. Chọn một session có nhiều issue, bấm biểu tượng chia sẻ đám mây.
2. Đợi hộp thoại xuất hiện, click "Copy Link".
3. Gửi link này cho người khác hoặc mở ở trình duyệt khác để xác minh họ xem được đầy đủ báo cáo.

---
---

## MILESTONE 3 — AI Agent Context Pack Exporter Engine (Tuần 3)

**Mục tiêu:** Triển khai module phân tích Git dự án, thiết lập bộ sinh cấu trúc JSON manifest và Markdown prompt chuyên biệt, kết nối UI Export để xuất gói AACP phục vụ tự động vá lỗi bằng AI.

---

### T3.01 — Git CLI Inspector (Rust Backend)

**Goal:** Chạy các lệnh Git để trích xuất trạng thái commit, nhánh hiện hành và các thay đổi chưa commit từ thư mục dự án cục bộ.

**Files to create:**
- `src-tauri/src/services/git_inspector.rs`

**Files to modify:**
- `src-tauri/src/services/mod.rs`

**Dependencies:** T1.01

**Implementation notes:**
- Sử dụng thư viện chuẩn `std::process::Command` để thực thi Git CLI:
  - Xác nhận thư mục có phải là Git repo bằng lệnh `git rev-parse --is-inside-work-tree`.
  - Lấy commit hash hiện tại: `git rev-parse HEAD`.
  - Lấy nhánh hiện tại: `git rev-parse --abbrev-ref HEAD`.
  - Kiểm tra trạng thái thay đổi chưa commit (git status/diff ngắn): `git status --porcelain`.
- Xử lý lỗi cẩn thận: Nếu lệnh `git` không tìm thấy (lập trình viên chưa cài git hoặc thư mục không phải git repo), trả về các trường thông tin dạng `None` hoặc `"Not a Git Repository"` thay vì quăng lỗi crash tiến trình.

**Acceptance criteria:**
- [ ] Hàm trả về chính xác tên branch và hash commit trên máy cài sẵn Git.
- [ ] Ứng dụng không bị treo nếu thư mục được chọn là thư mục trống không có Git.

**Manual test steps:**
- Viết unit test trỏ trực tiếp vào thư mục hiện tại của dự án August Mark, kiểm tra xem có lấy ra đúng nhánh `main` và commit hiện tại.

---

### T3.02 — AACP Manifest Serializer

**Goal:** Thiết lập tệp JSON manifest mô tả siêu dữ liệu lỗi và thông tin môi trường code của lập trình viên.

**Files to create:**
- `src-tauri/src/models/aacp.rs` (Định nghĩa struct manifest)

**Files to modify:**
- `src-tauri/src/models/mod.rs`

**Dependencies:** T3.01

**Implementation notes:**
- Định nghĩa cấu trúc JSON khớp với đặc tả kỹ thuật:
  - `IssueInfo`: id, title, description, severity, status, tags.
  - `CodebaseInfo`: local_path, git_commit, git_branch, suspected_files (danh sách file nghi ngờ).
  - `Visuals`: full_screenshot (tên file ảnh), crops (danh sách tên file ảnh khoanh vùng và toạ độ vùng cắt).
- Sử dụng `serde_json` để chuyển đối đối tượng Rust thành chuỗi JSON đẹp mắt (pretty print): `serde_json::to_string_pretty(&manifest)`.

**Acceptance criteria:**
- [ ] File JSON sinh ra có định dạng hợp lệ (valid JSON) chứa đầy đủ các trường thông tin.

**Manual test steps:**
- Chạy một hàm test serialize struct, ghi thử ra đĩa dạng `manifest.json` và kiểm tra cấu trúc file.

---

### T3.03 — AACP Prompt Generator (Markdown Generator)

**Goal:** Sinh tệp markdown hướng dẫn AI Agent đọc hiểu lỗi và định vị tệp cần sửa đổi.

**Files to modify:**
- `src-tauri/src/services/aacp_exporter.rs` (Khởi tạo dịch vụ xuất AACP)

**Dependencies:** T3.02

**Implementation notes:**
- Sử dụng chuỗi định dạng (formatted string) trong Rust để lắp ghép tệp `agent_instruction.md`:
  - Tiêu đề: "August Mark — AI Agent Bug Fix Instruction".
  - Phần 1: Chi tiết Lỗi (Title, Type, Severity, Description, Tags).
  - Phần 2: Ngữ cảnh Mã nguồn (Workspace path, Commit hash, Branch, Suspected files).
  - Phần 3: Minh hoạ trực quan (Liên kết đến ảnh screenshot gốc và ảnh crops tương ứng).
  - Phần 4: Chỉ dẫn hành động (Step-by-step instructions cho AI Agent: đọc suspected files, phân tích sự khác biệt visual, viết code sửa lỗi, xác minh).
- Đảm bảo prompt được viết bằng tiếng Anh chuẩn vì hầu hết AI Agent xử lý tiếng Anh tốt nhất.

**Acceptance criteria:**
- [ ] Tệp `agent_instruction.md` sinh ra có cấu trúc markdown rõ ràng, liên kết ảnh hoạt động tốt.

**Manual test steps:**
- Kiểm tra file markdown tạo ra bằng trình xem markdown của VS Code hoặc trình soạn thảo xem hiển thị đúng cấu trúc tiêu đề.

---

### T3.04 — AACP Packaging Engine (Rust Command)

**Goal:** Đóng gói toàn bộ thông tin: JSON manifest, Markdown prompt, sao chép screenshot và các tệp ảnh crops cắt nhỏ vào thư mục đích hoặc nén thành ZIP.

**Files to modify:**
- `src-tauri/src/services/aacp_exporter.rs` (Triển khai logic đóng gói)
- `src-tauri/src/commands/aacp_cmds.rs` (Đăng ký command)
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/main.rs`

**Dependencies:** T3.03

**Implementation notes:**
- Viết command:
  ```rust
  #[tauri::command]
  pub async fn export_aacp_pack(
      state: State<'_, AppState>,
      issue_id: i64,
      workspace_path: String,
      suspected_files: Vec<String>,
      output_dir: String,
      compress_zip: bool
  ) -> Result<String, AppError>
  ```
- Tiến trình thực hiện:
  1. Tạo thư mục tạm `aacp_export_temp_[id]`.
  2. Lấy dữ liệu issue, screenshot và tọa độ annotation từ SQLite.
  3. Lấy Git context từ `workspace_path` thông qua `git_inspector`.
  4. Ghi file `issue_manifest.json` và `agent_instruction.md` vào thư mục tạm.
  5. Sao chép ảnh screenshot gốc của issue vào thư mục tạm.
  6. Tiến hành crop vùng ảnh khoanh vùng lỗi dựa vào tọa độ annotation của marker và lưu thành ảnh crop (ví dụ `crop_1.png`).
  7. Nếu `compress_zip` bằng `true`, nén thư mục tạm thành file `august_aacp_issue_[id].zip` lưu tại `output_dir` và xóa thư mục tạm. Ngược lại di chuyển toàn bộ thư mục tạm vào `output_dir`.

**Acceptance criteria:**
- [ ] Tạo file zip thành công chứa đầy đủ JSON, MD, screenshot chính và các file ảnh crops.
- [ ] Hàm cắt ảnh (crop) hoạt động chính xác dựa trên bounding box của nét vẽ trên canvas.

**Manual test steps:**
- Chạy command thử nghiệm từ DevTools với một issue mẫu, kiểm tra thư mục đầu ra xem có đầy đủ 4 thành phần (JSON, MD, screenshot.png, crop.png).

---

### T3.05 — ExportDialog AACP Tab (Frontend UI)

**Goal:** Tích hợp tab xuất gói ngữ cảnh AI Agent vào hộp thoại Export Dialog trong Dashboard.

**Files to create:**
- `src/types/aacp.ts`

**Files to modify:**
- `src/components/export/ExportDialog.vue`
- `src/services/tauriCommands.ts`

**Dependencies:** T3.04

**Implementation notes:**
- Thêm Tab mới "AI Agent (AACP)" bên cạnh các tab HTML/PDF.
- Trong Tab AACP:
  - Trường nhập "Target Workspace Path": Cho phép gõ hoặc click nút "Browse" (gọi Tauri dialog open directory) để chọn thư mục dự án cục bộ.
  - Trường nhập "Suspected Files": Một danh sách dạng tags/chips hoặc textarea cho phép lập trình viên nhập các tệp tin nghi ngờ (mỗi file một dòng hoặc phân tách bằng dấu phẩy, ví dụ: `src/components/Navbar.vue`).
  - Checkbox "Compress to ZIP": Mặc định bật.
  - Nút "Export Context Pack": Khi click, gọi `export_aacp_pack` với các thông số tương ứng. Hiển thị thông báo thành công và mở thư mục đích bằng Tauri Shell.

**Acceptance criteria:**
- [ ] Giao diện trực quan, dễ nhập liệu. Lưu lại "Target Workspace Path" gần nhất vào SQLite settings để người dùng không phải chọn lại nhiều lần.
- [ ] Kiểm tra tính hợp lệ dữ liệu: không cho phép bấm export nếu chưa nhập Workspace Path.

**Manual test steps:**
1. Vào Dashboard -> Click một issue -> Click Export.
2. Chọn Tab "AI Agent (AACP)".
3. Click "Browse" chọn một thư mục code trên máy bạn.
4. Nhập tên file nghi ngờ: `src/App.vue`.
5. Bấm "Export". Xác nhận hộp thoại báo thành công xuất hiện và thư mục chứa file ZIP được mở ra.

---
---

## MILESTONE 4 — Integration, End-to-end Testing & Release (Tuần 4)

**Mục tiêu:** Kiểm thử tích hợp toàn bộ luồng đồng bộ đám mây và tính năng xuất gói AACP với các AI Agent thực tế, tối ưu hóa trải nghiệm và đóng gói sản phẩm phiên bản v0.2.0.

---

### T4.01 — Google Drive Sync & Backup E2E Verification

**Goal:** Kiểm thử toàn bộ hệ thống lưu trữ đồng bộ đám mây bao gồm các trường hợp mạng yếu, token hết hạn, và khôi phục dữ liệu trên môi trường sạch.

**Dependencies:** Mở rộng từ Milestone 1 & 2.

**Manual test steps:**
1. Kết nối Google Drive và click "Backup Now". Kiểm tra dung lượng file backup trên Drive Web (thư mục `August Mark Backups`).
2. Ghi thêm 2 session mới trên app local.
3. Bấm "Restore" chọn file backup cũ. Xác nhận 2 session mới biến mất, dữ liệu khôi phục về trạng thái cũ thành công.
4. Đợi access token hết hạn (mô phỏng bằng cách đổi giờ hệ thống hoặc chỉnh sửa thời gian lưu trong SQLite) -> thực hiện share session. Xác nhận app tự động refresh access token ngầm mà không đòi hỏi user login lại.

**Acceptance criteria:**
- [ ] Không xảy ra lỗi rò rỉ bộ nhớ hoặc xung đột luồng ghi file SQLite trong quá trình backup/restore.
- [ ] Khôi phục dữ liệu chạy mượt mà không làm treo UI ứng dụng.

---

### T4.02 — AI Agent AACP Compatibility Test

**Goal:** Nạp gói AACP xuất ra cho một AI Coding Agent thực tế và kiểm chứng khả năng tự động sửa lỗi của AI.

**Dependencies:** Mở rộng từ Milestone 3.

**Manual test steps:**
1. Chọn một dự án code thực tế nhỏ của bạn có lỗi UI nhỏ (ví dụ: nút bấm lệch màu hoặc thẻ div bị tràn viền).
2. Chụp màn hình lỗi bằng August Mark, khoanh vùng rectangle đỏ quanh vị trí lệch. Nhập mô tả lỗi.
3. Mở Export Dialog -> Tab AACP -> Chọn workspace dự án -> nhập file chứa class CSS lỗi -> Bấm Export.
4. Mở trình duyệt AI chat (Gemini / Claude) hoặc dùng một CLI Agent (như Roo Code / Antigravity).
5. Drag-drop tệp zip AACP hoặc trích xuất tệp `agent_instruction.md` kèm tệp ảnh gửi cho Agent.
6. Xác nhận AI Agent đọc hiểu tệp MD và ảnh minh hoạ, tìm ra đúng dòng code trong workspace dự án của bạn và viết lệnh sửa lỗi chính xác.

**Acceptance criteria:**
- [ ] Gói AACP cung cấp đầy đủ thông tin định vị để AI Agent không cần hỏi thêm thông tin từ người dùng.
- [ ] Tọa độ vùng crop chính xác giúp AI nhìn rõ chi tiết bị lỗi.

---

### T4.03 — Version Bump & Production Packaging

**Goal:** Cập nhật thông số phiên bản lên v0.2.0, biên dịch mã nguồn và đóng gói bộ cài đặt.

**Files to modify:**
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`
- `package.json`
- `CHANGELOG.md`

**Dependencies:** Hoàn thành tất cả các tasks trước đó.

**Implementation notes:**
- Đổi version từ `0.1.1` thành `0.2.0` ở cả 3 file manifest cấu hình.
- Viết nhật ký thay đổi chi tiết các tính năng mới trong `CHANGELOG.md`.
- Chạy lệnh biên dịch phát hành chính thức: `npm run tauri build`.
- Chạy cài đặt thử nghiệm tệp EXE/MSI được sinh ra trên máy ảo sạch để chắc chắn không thiếu dll hoặc runtime dependencies.

**Acceptance criteria:**
- [ ] Bộ cài đặt EXE và MSI được đóng gói thành công.
- [ ] App khởi chạy bình thường, hiển thị phiên bản `"0.2.0"` trong màn hình Settings/About.
`