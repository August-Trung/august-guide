# V011_IMPLEMENTATION_TASKS.md — August Mark v0.1.1

> Danh sách Task chi tiết phục vụ cho solo developer hoặc coding agent. Các task được thiết kế để thực hiện tuần tự, hạn chế xung đột code và dễ dàng debug.

---

## MILESTONE 1 — Desktop Experience & Settings (Tuần 1)

**Mục tiêu:** Xây dựng cấu hình cài đặt hoàn chỉnh lưu trữ trong DB, tích hợp System Tray, đồng bộ Theme tự động, lưu trữ trạng thái cửa sổ và sửa đổi nhãn phím tắt.

---

### T1.01 — Rust Settings Repository (Database Layer)

**Goal:** Thao tác đọc/ghi dữ liệu cài đặt từ bảng `settings` đã có trong database SQLite.

**Files to create:**
- `src-tauri/src/db/settings_repo.rs`

**Files to modify:**
- `src-tauri/src/db/mod.rs` (Đăng ký module `settings_repo`)

**Dependencies:** Môi trường v0.1.0 đã chạy.

**Implementation notes:**
- Sử dụng bảng `settings` có các trường: `key` (TEXT PRIMARY KEY), `value` (TEXT), `updated_at` (DATETIME).
- Viết các hàm:
  - `get_setting(conn: &Connection, key: &str) -> Result<Option<String>, AppError>`
  - `set_setting(conn: &Connection, key: &str, value: &str) -> Result<(), AppError>`
  - `get_all_settings(conn: &Connection) -> Result<HashMap<String, String>, AppError>`
- Sử dụng parameterized query `INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?1, ?2, datetime('now'))` để thực hiện ghi đè.

**Acceptance criteria:**
- [ ] Hàm `get_setting` trả về đúng giá trị lưu trong SQLite.
- [ ] Hàm `set_setting` ghi đè thành công giá trị và cập nhật thời gian.
- [ ] Hàm `get_all_settings` lấy toàn bộ key-value dạng `HashMap<String, String>`.
- [ ] Code biên dịch thành công không có lỗi `cargo build`.

**Manual test steps:**
- Tạo unit test trong `settings_repo.rs` mở connection sqlite tạm thời trong memory (`Connection::open_in_memory()`), chạy migration để có bảng `settings`, thực hiện set key `"theme"` thành `"light"`, sau đó get lại xem có trả về `"light"`.

---

### T1.02 — Rust Settings Commands (IPC Layer)

**Goal:** Cung cấp Tauri Commands để Vue frontend có thể truy vấn và lưu cài đặt.

**Files to create:**
- `src-tauri/src/commands/settings_cmds.rs`

**Files to modify:**
- `src-tauri/src/commands/mod.rs` (Đăng ký `settings_cmds`)
- `src-tauri/src/main.rs` (Đăng ký commands vào tauri handler)

**Dependencies:** T1.01

**Implementation notes:**
- Tạo các commands:
  - `#[tauri::command] pub async fn get_all_settings(state: State<'_, AppState>) -> Result<HashMap<String, String>, AppError>`
  - `#[tauri::command] pub async fn get_setting(state: State<'_, AppState>, key: String) -> Result<Option<String>, AppError>`
  - `#[tauri::command] pub async fn update_setting(state: State<'_, AppState>, key: String, value: String) -> Result<(), AppError>`
- Các hàm này lấy lock connection từ `state.db.lock().unwrap()` để gọi qua `settings_repo`.

**Acceptance criteria:**
- [ ] Tauri Handler đăng ký thành công `get_all_settings`, `get_setting`, `update_setting`.
- [ ] Có thể invoke từ frontend DevTools console và trả về dữ liệu đúng định dạng JSON.

**Manual test steps:**
1. Khởi động app: `npm run tauri dev`.
2. Mở DevTools console (F12).
3. Gõ: `await window.__TAURI__.core.invoke('get_all_settings')`.
4. Xác nhận nhận được một object chứa các key: `theme`, `overlay_trigger`, v.v.

---

### T1.03 — Vue Settings Store (State Layer)

**Goal:** Quản lý state của settings ở frontend và cung cấp các API đồng bộ với backend.

**Files to create:**
- `src/stores/settingsStore.ts`

**Files to modify:**
- `src/services/tauriCommands.ts` (Thêm các wrapper gọi tới commands settings của Rust)

**Dependencies:** T1.02, T1.09 (v0.1.0)

**Implementation notes:**
- Trong `tauriCommands.ts`, thêm:
  - `export const getAllSettings = () => invoke<Record<string, string>>('get_all_settings')`
  - `export const updateSetting = (key: string, value: string) => invoke<void>('update_setting', { key, value })`
- Trong `settingsStore.ts` (Pinia), định nghĩa state:
  - `settings: Record<string, string>`
  - `loading: boolean`
- Định nghĩa các action:
  - `loadSettings()`: gọi `getAllSettings()` và gán vào state.
  - `setSetting(key: string, value: string)`: gọi `updateSetting(key, value)`, cập nhật state local.
  - `theme`: computed trả về giá trị `'dark' | 'light' | 'system'`.
- Khi store init, tự động gọi `loadSettings()`.

**Acceptance criteria:**
- [ ] Pinia store lưu trữ đúng trạng thái settings.
- [ ] Hàm `setSetting` gọi API cập nhật SQLite backend đồng thời cập nhật UI store local.

**Manual test steps:**
- Kiểm tra qua Pinia DevTools hoặc log ra console khi load store để chắc chắn dữ liệu được nạp đầy đủ.

---

### T1.04 — Vue Settings View UI (UI Layer)

**Goal:** Xây dựng trang cài đặt SettingsView để người dùng thay đổi cấu hình.

**Files to create:**
- `src/views/SettingsView.vue`

**Files to modify:**
- `src/router/index.ts` (Thêm route `/settings`)
- `src/components/common/AppSidebar.vue` (Thêm icon/nút Cài đặt dẫn tới `/settings`)

**Dependencies:** T1.03

**Implementation notes:**
- Thiết kế giao diện bằng Vuetify:
  - Menu bên trái hoặc Tabs: General, Capture Settings, Info.
  - **General:**
    - Theme selector: Radio buttons hoặc Select cho Dark, Light, System.
    - Language selector: dropdown (Tiếng Việt, English).
    - Default Project dropdown: load danh sách project từ `projectStore` cho người dùng chọn mặc định.
  - **Capture Settings:**
    - Screenshot quality slider: từ 50% đến 100%.
    - Delayed Capture Select: 0s, 3s, 5s, 10s.
    - Mouse hold duration: slider từ 500ms đến 2000ms.
  - **Info:**
    - Tên ứng dụng, dynamic version, statistics (tổng số project, session, issue - lấy từ T1.08).
- Khi thay đổi bất kỳ value nào, gọi `settingsStore.setSetting(key, value)`.
- Đặc biệt đối với Theme, gọi `vuetify.theme.global.name.value` tương ứng để cập nhật giao diện ngay lập tức mà không cần reload app.

**Acceptance criteria:**
- [ ] Chuyển tới route `/settings` thông qua Sidebar hiển thị đúng UI.
- [ ] Thay đổi Theme từ Dark sang Light cập nhật giao diện lập tức.
- [ ] Cấu hình lưu trữ thành công vào DB, khi khởi động lại app cấu hình đã chọn được giữ nguyên.

**Manual test steps:**
1. Mở app, click biểu tượng Settings ở Sidebar góc dưới.
2. Chuyển theme sang "Light" -> Toàn bộ app chuyển sang giao diện sáng.
3. Tắt app và bật lại -> App khởi động trực tiếp bằng giao diện sáng (Light).
4. Thay đổi Delayed Capture thành 3s, tắt app mở lại -> Trạng thái dropdown vẫn là 3s.

---

### T1.05 — System Tray Icon (Backend Layer)

**Goal:** Hiển thị tray icon của August Mark ở thanh tác vụ hệ thống (Taskbar tray) và điều khiển cửa sổ.

**Files to modify:**
- `src-tauri/src/lib.rs` (Cấu hình TrayIcon và xử lý sự kiện)
- `src-tauri/tauri.conf.json` (Đảm bảo cấu hình window label chính xác)

**Dependencies:** T1.01

**Implementation notes:**
- Sử dụng thư viện built-in `tauri::tray` của Tauri 2.
- Trong `lib.rs`, hàm `run()`:
  - Tạo menu cho Tray gồm các items:
    - `"capture"` -> "Chụp màn hình"
    - `"open"` -> "Mở August Mark"
    - `"settings"` -> "Cài đặt"
    - `"quit"` -> "Thoát"
  - Đăng ký sự kiện Tray:
    - Click trái vào Tray Icon -> Show và Focus cửa sổ chính `"main"`.
    - Click vào menu item `"capture"` -> Gọi hàm trigger capture (giống nhấn hotkey).
    - Click `"open"` -> Show cửa sổ `"main"`.
    - Click `"settings"` -> Show cửa sổ `"main"`, gửi sự kiện Vue router navigate sang `/settings`.
    - Click `"quit"` -> Thoát app hoàn toàn.
  - Xử lý đóng cửa sổ: Nếu setting `minimize_to_tray` được bật, bắt sự kiện close cửa sổ chính và thực hiện `window.hide()` thay vì đóng app.

**Acceptance criteria:**
- [ ] Tray icon hiển thị chính xác logo August Mark ở khay hệ thống Windows.
- [ ] Click trái restore cửa sổ. Click phải hiện đúng context menu 4 items bằng Tiếng Việt hoặc English.
- [ ] Bấm đóng cửa sổ (nếu bật minimize to tray) sẽ ẩn cửa sổ xuống tray.

**Manual test steps:**
1. Chạy app -> Kiểm tra khay hệ thống có biểu tượng August Mark.
2. Click biểu tượng đóng `X` ở cửa sổ chính -> Cửa sổ biến mất nhưng app vẫn chạy ở tray.
3. Click đúp/click trái vào tray icon -> Cửa sổ chính hiện lại đúng vị trí cũ.
4. Click phải tray icon -> Chọn "Thoát" -> App tắt hoàn toàn.

---

### T1.06 — Window State Persistence

**Goal:** Ghi nhớ trạng thái kích thước (width, height) và tọa độ (x, y) của app khi đóng để khôi phục khi mở lại.

**Files to modify:**
- `src-tauri/Cargo.toml` (Thêm dependency `tauri-plugin-window-state = "2.0.0"`)
- `src-tauri/src/lib.rs` (Đăng ký plugin trong builder)

**Dependencies:** T1.01

**Implementation notes:**
- Thêm `tauri-plugin-window-state = "2.0.0"` vào `Cargo.toml`.
- Trong `lib.rs`: `.plugin(tauri_plugin_window_state::Builder::default().build())`.
- Đảm bảo cửa sổ chính `"main"` trong `tauri.conf.json` có cấu hình cho phép resize.

**Acceptance criteria:**
- [ ] Kích thước và vị trí cửa sổ được tự động khôi phục chính xác sau khi khởi động lại app.

**Manual test steps:**
1. Kéo rộng cửa sổ app ra hết cỡ hoặc di chuyển sang góc màn hình.
2. Đóng ứng dụng.
3. Mở lại -> Cửa sổ xuất hiện ở đúng tọa độ và kích thước đã chỉnh.

---

### T1.07 — About Page Dynamic Version & App Stats

**Goal:** Đọc app version động từ metadata cấu hình và lấy các thông số cơ sở dữ liệu để hiển thị trên About page.

**Files to create:**
- `src-tauri/src/commands/app_cmds.rs`

**Files to modify:**
- `src-tauri/src/commands/mod.rs` (Đăng ký `app_cmds`)
- `src-tauri/src/main.rs` (Đăng ký command `get_app_stats`)
- `src/views/AboutView.vue` (Nâng cấp giao diện hiển thị thông số động)

**Dependencies:** T1.02

**Implementation notes:**
- Viết command Rust `get_app_stats`:
  - Trả về struct `AppStats`: `projectCount: i64`, `sessionCount: i64`, `issueCount: i64`, `dbSize: u64` (độ lớn file `august_mark.db` tính bằng byte).
  - Lấy file size: `std::fs::metadata(&app_state.db_path).map(|m| m.len()).unwrap_or(0)`.
- Ở Vue `AboutView.vue`:
  - Dùng `@tauri-apps/api/app` hàm `getVersion()` để lấy version từ `tauri.conf.json`.
  - Gọi command `get_app_stats` để nhận dữ liệu thống kê, định dạng hiển thị đẹp đẽ (dung lượng KB/MB, số lượng issue).

**Acceptance criteria:**
- [ ] Trang About không còn chứa text tĩnh `"v0.1.0"`, thay vào đó hiển thị version từ tauri config.
- [ ] Hiển thị chính xác dung lượng DB hiện tại và tổng quan lượng dữ liệu người dùng đã tạo.

**Manual test steps:**
1. Vào mục "About" trên Dashboard.
2. Xác nhận version hiển thị khớp với cấu hình trong `tauri.conf.json`.
3. Tạo thêm vài issue mới -> Kiểm tra lại About page thấy số lượng thống kê tăng lên tương ứng.

---

### T1.08 — Sửa lỗi nhãn hiển thị Hotkey

**Goal:** Chỉnh sửa nhãn phím tắt hiển thị trên header từ "PrntScrn" thành "Ctrl+Shift+M" để khớp với phím tắt thực tế hoạt động.

**Files to modify:**
- `src/components/common/AppHeader.vue`

**Dependencies:** Không.

**Implementation notes:**
- Tìm thẻ hiển thị hotkey gợi ý trên AppHeader. Sửa text/tooltip thành "Ctrl+Shift+M".
- (Tùy chọn) Ràng buộc động nhãn hiển thị này với key `overlay_trigger` từ `settingsStore` nếu người dùng đã đổi phím tắt trong Settings.

**Acceptance criteria:**
- [ ] Nhãn phím tắt hiển thị chính xác là `Ctrl + Shift + M`.

**Manual test steps:**
- Xem Header và xác nhận text hiển thị trực quan khớp với phím bấm.

---

## MILESTONE 2 — Advanced Drawing Tools & Canvas UX (Tuần 2)

**Mục tiêu:** Cải tiến tính năng vẽ của Overlay, bổ sung các công cụ vẽ nét tự do, highlight, blur làm mờ chi tiết, tích hợp Undo/Redo stack và cho phép xóa từng annotation riêng biệt.

---

### T2.01 — Blur / Pixelate HTML5 Canvas Rendering

**Goal:** Cho phép kéo vùng chữ nhật để làm mờ (mã hóa pixel/mosaic) chi tiết nhạy cảm trực tiếp trên canvas overlay.

**Files to modify:**
- `src/types/annotation.ts` (Đảm bảo Type `BlurAnnotation` thừa kế đúng cấu trúc `{ id, type: 'blur', x, y, width, height }`)
- `src/composables/useAnnotation.ts` (Thêm trạng thái `blur` cho active tool)
- `src/composables/useCanvas.ts` (Thêm hàm vẽ và render hiệu ứng blur mosaic)
- `src/components/overlay/AnnotationToolbar.vue` (Thêm nút Blur vào toolbar và gán phím tắt `5`)

**Dependencies:** Môi trường vẽ v0.1.0 hoạt động bình thường.

**Implementation notes:**
- Để tạo mosaic effect trên Canvas HTML5 không bị lag:
  - Lấy phần ảnh screenshot nguyên bản tương ứng với bounding box của Blur annotation.
  - Tạo một offscreen canvas tạm thời kích thước rất nhỏ (ví dụ chia 8 kích thước ban đầu).
  - Vẽ ảnh cắt đó lên canvas phụ với chế độ `ctx.drawImage(originalImage, x, y, w, h, 0, 0, w/8, h/8)`.
  - Tắt chế độ smooth ảnh của canvas chính: `ctx.imageSmoothingEnabled = false`.
  - Vẽ phóng to ngược lại canvas chính: `ctx.drawImage(offscreenCanvas, 0, 0, w/8, h/8, x, y, w, h)`.
  - Bật lại `ctx.imageSmoothingEnabled = true`.
- Cơ chế kéo thả tương tự Rectangle tool.

**Acceptance criteria:**
- [ ] Chọn công cụ Blur kéo một vùng chữ nhật trên screenshot -> vùng đó bị vỡ hạt (pixelated) rõ rệt, không nhìn rõ chữ hay chi tiết bên dưới.
- [ ] Thao tác mượt mà, không bị sụt giảm FPS khi kéo kéo thả vẽ trên màn hình lớn.

**Manual test steps:**
1. Bấm `Ctrl+Shift+M` để chụp màn hình mở overlay.
2. Bấm phím `5` hoặc click icon Blur trên toolbar.
3. Kéo chuột qua vùng chữ chứa thông tin nhạy cảm -> Xác nhận vùng chữ bị làm mờ vỡ hạt.
4. Di chuyển chuột vẽ thêm vài vùng blur khác -> Kiểm tra không lag.

---

### T2.02 — Freehand Drawing Tool

**Goal:** Cung cấp công cụ vẽ tay tự do (bút vẽ tự do) trên canvas.

**Files to modify:**
- `src/types/annotation.ts` (Định nghĩa `FreehandAnnotation` chứa: `id, type: 'freedraw', points: Point[], color, strokeWidth`)
- `src/composables/useAnnotation.ts` (Thêm trạng thái vẽ freedraw)
- `src/composables/useCanvas.ts` (Xử lý render mảng điểm `points` bằng nét vẽ liên tục)
- `src/components/overlay/AnnotationToolbar.vue` (Thêm nút Freehand, phím tắt `6`)

**Dependencies:** T2.01

**Implementation notes:**
- Khi người dùng MouseDown ở chế độ Freedraw: Khởi tạo mảng `points` chứa điểm đầu tiên `{x, y}`.
- MouseMove: push tọa độ mới `{x, y}` vào `points`, kích hoạt redraw canvas.
- MouseUp: commit annotation vào list.
- Hàm render: Dùng `ctx.beginPath()`, `ctx.moveTo(points[0].x, points[0].y)`. Duyệt qua mảng điểm, gọi `ctx.lineTo(p.x, p.y)`. Đặt `ctx.strokeStyle`, `ctx.lineWidth = 3`, gọi `ctx.stroke()`.

**Acceptance criteria:**
- [ ] Chọn công cụ Freehand và di chuột vẽ bất kỳ hình thù gì (vòng tròn tự do, ký tự) nét vẽ hiện mượt mà theo đường di chuột.
- [ ] Dữ liệu được lưu trữ dưới dạng mảng các tọa độ điểm trong JSON.

**Manual test steps:**
1. Nhấn hotkey mở overlay.
2. Bấm phím `6` hoặc click Bút vẽ.
3. Vẽ nguệch ngoạc lên màn hình -> Xác nhận nét vẽ hiển thị đúng vị trí di chuột.

---

### T2.03 — Highlight Tool

**Goal:** Cho phép bôi màu vàng bán trong suốt lên vùng chọn trên screenshot để làm nổi bật.

**Files to modify:**
- `src/types/annotation.ts` (Định nghĩa `HighlightAnnotation` giống `RectAnnotation` nhưng có type `'highlight'`)
- `src/composables/useAnnotation.ts` (Thêm state vẽ highlight)
- `src/composables/useCanvas.ts` (Thêm hàm render highlight)
- `src/components/overlay/AnnotationToolbar.vue` (Thêm nút Highlight, phím tắt `7`)

**Dependencies:** T2.02

**Implementation notes:**
- Cơ chế kéo thả tương tự Rectangle Tool.
- Khi render trên canvas:
  - Đặt `ctx.fillStyle = 'rgba(255, 235, 59, 0.35)'` (màu vàng bán trong suốt).
  - Sử dụng `ctx.fillRect(x, y, width, height)` để phủ màu.
  - (Tùy chọn) Vẽ thêm đường viền nét đứt mỏng màu vàng đậm bao quanh.

**Acceptance criteria:**
- [ ] Kéo vẽ highlight tô đúng vùng màu vàng bán trong suốt, chi tiết bên dưới highlight vẫn nhìn rõ bình thường.

**Manual test steps:**
1. Mở overlay -> Bấm phím `7` hoặc click Highlight.
2. Kéo đè lên một dòng text -> Dòng text chuyển sang nền vàng nhưng chữ vẫn đọc được rõ ràng.

---

### T2.04 — Undo / Redo Annotation Stack

**Goal:** Cho phép người dùng thu hồi (Undo) hoặc khôi phục (Redo) các thao tác vẽ annotations trên overlay canvas.

**Files to modify:**
- `src/stores/overlayStore.ts` (Tạo stack quản lý lịch sử vẽ)
- `src/OverlayApp.vue` (Bắt sự kiện phím nóng `Ctrl+Z`, `Ctrl+Y`)
- `src/components/overlay/OverlayStatusBar.vue` (Hiển thị 2 nút Undo và Redo)

**Dependencies:** T2.03

**Implementation notes:**
- Trong `overlayStore`:
  - Thêm state: `annotations: Annotation[]`, `redoStack: Annotation[]`.
  - Thay đổi cơ chế add annotation: Khi commit vẽ xong một hình mới, đẩy vào `annotations` và **clear sạch `redoStack`**.
  - Viết action `undo()`:
    - Lấy phần tử cuối cùng ra khỏi `annotations`.
    - Nếu đó là Marker annotation, giảm bộ đếm số lượng Marker đi 1 đơn vị.
    - Đẩy phần tử này vào `redoStack`.
    - Kích hoạt redraw canvas.
  - Viết action `redo()`:
    - Nếu `redoStack` không trống, pop phần tử ra.
    - Nếu là Marker, tăng lại bộ đếm Marker.
    - Đẩy vào `annotations`.
    - Kích hoạt redraw canvas.
- Ràng buộc nút Undo/Redo trên OverlayStatusBar disable/enable dựa trên độ dài của 2 stack.

**Acceptance criteria:**
- [ ] Bấm nút Undo hoặc tổ hợp `Ctrl+Z` ẩn nét vẽ vừa vẽ thành công.
- [ ] Bấm Redo hoặc `Ctrl+Y` khôi phục chính xác nét vẽ đó ở đúng tọa độ ban đầu.
- [ ] Marker counter tăng/giảm chính xác tương ứng.

**Manual test steps:**
1. Chụp màn hình mở overlay.
2. Vẽ lần lượt: Marker ① -> Rectangle -> Marker ②.
3. Nhấn `Ctrl+Z` -> Marker ② biến mất.
4. Nhấn `Ctrl+Z` -> Rectangle biến mất.
5. Nhấn `Ctrl+Y` -> Rectangle xuất hiện trở lại.
6. Nhấn `Ctrl+Y` -> Marker ② xuất hiện trở lại đúng vị trí ban đầu.

---

### T2.05 — Xóa Annotation cụ thể

**Goal:** Cho phép click phải vào một nét vẽ/marker cụ thể hoặc hover hiện nút X để xóa nhanh, thay vì phải undo từng bước.

**Files to modify:**
- `src/composables/useAnnotation.ts` (Bổ sung hàm hit-test xác định annotation nằm dưới cursor)
- `src/composables/useCanvas.ts` (Xử lý render trạng thái hover/active và nút X xóa)

**Dependencies:** T2.04

**Implementation notes:**
- Viết hàm hit-testing trong `useAnnotation`:
  - Duyệt ngược danh sách `annotations` từ cuối lên đầu.
  - Đối với Marker/Highlight/Rect/Blur: kiểm tra xem điểm click `{x, y}` có nằm trong bounding box không.
  - Đối với Arrow: tính khoảng cách từ điểm click đến đoạn thẳng của Arrow.
  - Đối với Freehand: tính khoảng cách ngắn nhất từ điểm click tới các điểm trong mảng `points` (khoảng cách nhỏ hơn 10px coi như trúng nét).
- Nếu trúng nét vẽ:
  - Khi MouseMove ở chế độ Select: highlight nét vẽ đó (ví dụ vẽ bounding box nét đứt màu xanh mỏng bao quanh).
  - Khi RightClick: Hiển thị context menu nhỏ của trình duyệt tại tọa độ chuột có dòng "Delete annotation". Click vào sẽ xóa annotation này khỏi danh sách.
  - Hoặc đơn giản: Nhấn phím `Delete` trên bàn phím khi đang select nét vẽ đó để xóa nó đi.

**Acceptance criteria:**
- [ ] Người dùng có thể click chọn nét vẽ cũ và nhấn `Delete` để xóa.
- [ ] Menu chuột phải xóa hoạt động chính xác. Không bị lỗi lệch tọa độ click.

**Manual test steps:**
1. Vẽ 3 hình khác nhau trên canvas.
2. Di chuột hover qua hình Rectangle -> Hình có viền nét đứt bao quanh.
3. Click chuột phải lên hình chữ nhật -> Chọn "Xóa nét vẽ" -> Hình biến mất.

---

### T2.06 — Mở rộng Save Flow backend cho Annotation mới

**Goal:** Cập nhật API lưu trữ của Rust để ghi nhận các loại annotation mới (Blur, Freehand, Highlight) vào database SQLite và render đè ảnh.

**Files to modify:**
- `src-tauri/src/services/image_processor.rs` (Cập nhật logic vẽ đè các annotation lên ảnh gốc trước khi lưu)

**Dependencies:** T2.05

**Implementation notes:**
- Khi user bấm "Done", dữ liệu annotations được chuyển xuống Rust dưới dạng JSON String.
- Cập nhật hàm `render_annotations_on_image` ở Rust:
  - Sử dụng thư viện `image` và `imageproc` vẽ đè.
  - **Highlight:** vẽ hình chữ nhật phủ màu vàng mờ (alpha blending).
  - **Freehand:** duyệt mảng điểm vẽ các đường nối nhau (line drawing).
  - **Blur:** lấy vùng pixel, thực hiện gom cụm pixel (ví dụ block size 8x8) gán màu trung bình để tạo hiệu ứng pixelate, rồi chèn đè lại ảnh.

**Acceptance criteria:**
- [ ] Ảnh toàn cục lưu trên ổ đĩa có đầy đủ các chi tiết vẽ tay, highlight vàng và đặc biệt các vùng bị blur thực sự bị vỡ pixel trên file ảnh vật lý (an toàn tuyệt đối, không thể undo từ file ảnh).

**Manual test steps:**
1. Chụp màn hình, vẽ Freehand + Blur + Highlight.
2. Nhấn "Done" để hoàn thành session.
3. Vào thư mục lưu ảnh của August Mark -> mở file ảnh lên xem -> các nét vẽ và vùng mờ đã bị ghi đè chết lên ảnh.

---

## MILESTONE 3 — Dashboard UX: Tags, Search & Export (Tuần 3)

**Mục tiêu:** Xây dựng hệ thống Tagging hoàn chỉnh, công cụ Tìm kiếm toàn cục, bộ lọc cải tiến, sắp xếp Session/Issue và bổ sung tính năng Export đa định dạng.

---

### T3.01 — Rust Tag System (Repository & Commands)

**Goal:** Triển khai API đọc, tạo và liên kết tag với Issue trong SQLite.

**Files to create:**
- `src-tauri/src/db/tag_repo.rs`
- `src-tauri/src/commands/tag_cmds.rs`

**Files to modify:**
- `src-tauri/src/db/mod.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/main.rs` (Đăng ký commands)

**Dependencies:** Milestone 1 đã xong.

**Implementation notes:**
- Sử dụng các bảng `tags` và `issue_tags` từ SQL migration v0.1.0.
- Viết các hàm repo trong `tag_repo.rs`:
  - `get_all_tags(conn) -> Result<Vec<Tag>, AppError>`
  - `create_tag(conn, name, color) -> Result<Tag, AppError>`
  - `associate_tag_with_issue(conn, issue_id, tag_id) -> Result<(), AppError>`
  - `get_tags_by_issue(conn, issue_id) -> Result<Vec<Tag>, AppError>`
  - `clear_issue_tags(conn, issue_id) -> Result<(), AppError>`
- Viết tauri commands tương ứng trong `tag_cmds.rs` và đăng ký trong `main.rs`.

**Acceptance criteria:**
- [ ] Lưu tag mới vào DB thành công.
- [ ] Liên kết issue và tag thành công thông qua bảng trung gian `issue_tags`.
- [ ] Biên dịch thành công.

**Manual test steps:**
- Sử dụng DevTools console invoke thử hàm tạo tag `"v0.1.1-testing"` và xác nhận tag được thêm vào DB.

---

### T3.02 — Vue Tags UI (Overlay & Detail)

**Goal:** Thêm ô nhập và quản lý Tags vào IssueFormPanel khi chụp màn hình và IssueDetail trên Dashboard.

**Files to modify:**
- `src/components/overlay/IssueFormPanel.vue` (Thêm input tags combobox)
- `src/components/dashboard/IssueDetail.vue` (Hiển thị và cập nhật tags)
- `src/components/dashboard/IssueCard.vue` (Render tags dạng chips mỏng trên card)

**Dependencies:** T3.01

**Implementation notes:**
- Sử dụng component `v-combobox` hoặc `v-autocomplete` của Vuetify trong form:
  - Cho phép chọn từ danh sách tag có sẵn (load từ tagStore).
  - Cho phép gõ chữ mới rồi nhấn Enter để tạo tag mới ngay lập tức.
  - Sử dụng chips có màu ngẫu nhiên hoặc màu tự chọn.
- Đồng bộ danh sách tag khi nhấn lưu Issue.

**Acceptance criteria:**
- [ ] Người dùng có thể gắn tag khi đang annotating trên overlay.
- [ ] Tag hiển thị dạng chips đẹp mắt dưới tiêu đề Issue ở Dashboard.
- [ ] Có thể thêm/xóa tag khi đang ở trang chi tiết Issue.

**Manual test steps:**
1. Chụp màn hình -> Đặt marker -> Issue panel hiện ra -> Gõ vào ô Tags: "UI", "Lỗi hiển thị" -> Done.
2. Mở dashboard -> Session vừa chụp -> Xem danh sách issue -> Thấy chip "UI" và "Lỗi hiển thị" hiện trên issue card.

---

### T3.03 — Vue Filter Bar Tag Filter

**Goal:** Tích hợp bộ lọc Tag vào thanh FilterBar ở Dashboard để tìm các issue chứa tag tương ứng.

**Files to modify:**
- `src/components/dashboard/FilterBar.vue` (Bổ sung bộ chọn Tag)
- `src/stores/issueStore.ts` (Mở rộng logic filter local ở getter)

**Dependencies:** T3.02

**Implementation notes:**
- Thêm select box "Tags" hỗ trợ multiselect vào FilterBar.
- Trong `issueStore`:
  - Thêm state `selectedTags: string[]`.
  - Cập nhật getter `filteredIssues`: kiểm tra nếu `selectedTags` không rỗng, chỉ giữ lại những issue có mảng tags chứa ít nhất một phần tử trong `selectedTags`.

**Acceptance criteria:**
- [ ] Lựa chọn tag trong bộ lọc -> Danh sách issue trên dashboard lập tức thay đổi tương ứng.

**Manual test steps:**
1. Tạo 3 issue: 2 issue có tag "Frontend", 1 issue có tag "Backend".
2. Trên dashboard, mở FilterBar -> Chọn tag "Frontend".
3. Xác nhận chỉ có 2 issue hiển thị, issue có tag "Backend" bị ẩn đi.

---

### T3.04 — Rust Global Search Command

**Goal:** Tạo API tìm kiếm toàn cục ở backend để tìm kiếm nhanh session, issue bằng SQLite query.

**Files to create:**
- `src-tauri/src/commands/search_cmds.rs`

**Files to modify:**
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/main.rs`

**Dependencies:** T1.02

**Implementation notes:**
- Tạo command `search_all(query: String) -> Result<SearchResult, AppError>`:
  - Sử dụng SQLite query kết nối các bảng để tìm kiếm:
    - Tìm session có `title` hoặc `description` chứa từ khóa (sử dụng toán tử `LIKE %query%`).
    - Tìm issue có `title` hoặc `description` chứa từ khóa.
  - Trả về cấu trúc gồm 2 danh sách kết quả: `sessions: Vec<Session>`, `issues: Vec<Issue>`.

**Acceptance criteria:**
- [ ] Biên dịch thành công, query tối ưu tránh làm khóa database.

**Manual test steps:**
- Kiểm tra bằng DevTools console invoke lệnh search với từ khóa tĩnh.

---

### T3.05 — Vue Search Input UI

**Goal:** Đặt ô tìm kiếm trên AppHeader, hiển thị dropdown kết quả nhanh khi gõ chữ và click để điều hướng nhanh.

**Files to modify:**
- `src/components/common/AppHeader.vue` (Thêm ô input search)
- `src/services/tauriCommands.ts` (Thêm invoke wrapper `searchAll`)

**Dependencies:** T3.04

**Implementation notes:**
- Sử dụng component `v-autocomplete` hoặc `v-text-field` kết hợp `v-menu` của Vuetify để làm ô Search.
- Gõ chữ -> Trigger debounce (300ms) -> Gọi hàm `searchAll(query)` -> Hiển thị kết quả chia nhóm "Sessions" và "Issues" trong dropdown menu.
- Khi người dùng click vào một item:
  - Nếu click Session -> Router chuyển hướng tới `/session/:id`.
  - Nếu click Issue -> Router chuyển hướng tới `/session/:id?issueId=:id` hoặc trang chi tiết Issue.

**Acceptance criteria:**
- [ ] Gõ từ khóa tìm kiếm -> menu hiển thị kết quả lập tức.
- [ ] Click kết quả -> chuyển hướng chính xác đến đúng Session/Issue tương ứng.

**Manual test steps:**
1. Tạo session tên "Kiểm thử Login". Tạo issue tên "Lỗi sai font chữ".
2. Ra trang chủ dashboard -> Nhập "Login" vào ô search ở top bar -> click kết quả "Kiểm thử Login" -> trang Session hiển thị.
3. Nhập "font" vào ô search -> click kết quả -> trang chi tiết Issue mở ra.

---

### T3.06 — Sorting & Statistics UI

**Goal:** Bổ sung các tùy chọn sắp xếp danh sách và hiển thị các con số thống kê phiên làm việc trực quan hơn.

**Files to modify:**
- `src/components/dashboard/SessionList.vue` (Thêm dropdown sắp xếp)
- `src/components/dashboard/IssueList.vue` (Thêm dropdown sắp xếp)
- `src/stores/sessionStore.ts` (Thêm logic sort)
- `src/stores/issueStore.ts` (Thêm logic sort)

**Dependencies:** T1.10, T1.12 (v0.1.0)

**Implementation notes:**
- Cho phép sắp xếp Sessions theo: Ngày tạo mới nhất, Ngày tạo cũ nhất, Số lượng issue giảm dần, Trạng thái (Active/Completed).
- Cho phép sắp xếp Issues theo: Độ nghiêm trọng (Severity: Critical -> Major -> Minor -> Info), Trạng thái, Ngày tạo.
- Cập nhật state store để lưu trữ tùy chọn sort hiện tại và xử lý ở computed getters.

**Acceptance criteria:**
- [ ] Thay đổi tiêu chí sort -> Thứ tự hiển thị thay đổi mượt mà.

**Manual test steps:**
- Tạo nhiều session với số lượng issue khác nhau, chọn sort "Issue count desc" và xác nhận session nhiều issue nhất nhảy lên đầu.

---

### T3.07 — Multi-format Export (PDF, Markdown, CSV)

**Goal:** Nâng cấp ExportDialog hỗ trợ xuất báo cáo sang PDF, Markdown và CSV thay vì chỉ HTML.

**Files to modify:**
- `src-tauri/src/services/export_html.rs` (Đổi tên thành `export_service.rs` hoặc viết thêm các exporter mới)
- `src-tauri/src/commands/export_cmds.rs` (Bổ sung tham số format và filter vào command)
- `src/components/export/ExportDialog.vue` (Cải tiến UI có checkbox lọc và chọn định dạng)

**Dependencies:** Đã có export HTML cơ bản.

**Implementation notes:**
- **Markdown Export:** Generate file `.md` chứa thông tin session, các issue dạng bảng kèm đường dẫn tương đối tới ảnh crop.
- **CSV Export:** Tạo file `.csv` chứa cột: ID, Title, Severity, Type, Status, Description, ScreenshotPath. Dùng crate `csv` ở Rust.
- **PDF Export:** Sử dụng Tauri WebView print API hoặc convert Markdown qua PDF thông qua thư viện xử lý gọn nhẹ. Hoặc hướng dẫn người dùng "In trang HTML sang file PDF" thông qua trình duyệt mở file HTML. Để đơn giản và chất lượng tốt nhất không cần dependency nặng, viết engine export PDF bằng cách render file HTML report rồi invoke trình duyệt mặc định in ra file PDF.
- Thêm filter trong ExportDialog: cho phép người dùng chỉ export các issue có trạng thái "Open" hoặc độ nghiêm trọng là "Critical".

**Acceptance criteria:**
- [ ] Xuất file `.md` và `.csv` chính xác nội dung, mở được bằng Excel / VS Code.
- [ ] Bộ lọc hoạt động chuẩn xác (ví dụ chọn chỉ xuất Critical thì session export không chứa issue Minor).

**Manual test steps:**
1. Nhấn nút Export Session trên dashboard.
2. Chọn định dạng "Markdown", tick chọn "Only export Critical & Major".
3. Nhấp "Export" -> Lưu file.
4. Mở file `.md` -> Xác nhận danh sách issue chỉ có Critical/Major và liên kết ảnh hoạt động tốt.

---

## MILESTONE 4 — Verification, Polish & Release (Tuần 4)

**Mục tiêu:** Kiểm thử hồi quy, polish UI/UX, tối ưu hóa database, dọn dẹp các branch thừa và đóng gói bản release v0.1.1.

---

### T4.01 — Regression & Backwards Compatibility Testing

**Goal:** Xác nhận các tính năng của bản v0.1.0 cũ không bị lỗi sau khi cập nhật v0.1.1, và dữ liệu cũ tương thích tốt.

**Files to modify:** Không.

**Dependencies:** Toàn bộ Milestone 1, 2, 3 đã hoàn tất.

**Acceptance criteria:**
- [ ] Chạy app với DB của v0.1.0 -> Mọi session và issue hiển thị đầy đủ, không crash.
- [ ] Tính năng chụp màn hình bằng phím nóng mặc định và lưu trữ hoạt động hoàn hảo.

---

### T4.02 — Micro-animations & Visual Polish

**Goal:** Tăng cường tính chuyên nghiệp của UI bằng các hiệu ứng transition và loading mượt mà.

**Files to modify:**
- `src/index.css` (Thêm các transition classes)
- Các file view/component cần thiết (Thêm `v-fade-transition` của Vuetify, loading overlay)

**Dependencies:** T4.01

**Implementation notes:**
- Thêm hiệu ứng transition khi đóng/mở sidebar, click chọn item, mở rộng IssueDetail.
- Thêm loading spinner khi đang xử lý lưu ảnh overlay hoặc đang export file nặng.

**Acceptance criteria:**
- [ ] Giao diện mang lại cảm giác mượt mà, tốc độ phản hồi nhanh.

---

### T4.03 — Final Build & Release Packaging

**Goal:** Nâng version app lên v0.1.1, dọn dẹp dự án và build bản cài đặt phân phối.

**Files to modify:**
- `src-tauri/Cargo.toml` (Đặt version = "0.1.1")
- `src-tauri/tauri.conf.json` (Đặt version = "0.1.1")
- `package.json` (Đặt version = "0.1.1")
- `CHANGELOG.md` (Ghi nhận các thay đổi của v0.1.1)

**Dependencies:** T4.02

**Manual test steps:**
1. Chạy `npm run tauri build` để tạo file installer.
2. Cài đặt thử trên máy ảo hoặc máy tính Windows khác.
3. Xác nhận app chạy bình thường, database được tự động nâng cấp nếu có thay đổi schema.
