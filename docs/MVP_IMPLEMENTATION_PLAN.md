# MVP_IMPLEMENTATION_PLAN.md — August Mark v0.1

> Solo developer plan. 4-6 tuần. Code được ngay.

| Field | Value |
|---|---|
| Target | MVP v0.1 — chạy end-to-end trên Windows |
| Timeline | 4 tuần core + 1 tuần polish + 1 tuần buffer |
| Developer | 1 person |
| Baseline | PROJECT_PLAN.md + ARCHITECTURE.md đã có |

---

## 1. MVP Scope Decision

### ✅ Must Have — Không có thì app vô nghĩa

| # | Feature | Lý do |
|---|---|---|
| 1 | Tauri app chạy + system tray icon | Nền tảng, không có thì không có app |
| 2 | SQLite database + schema + migration | Source of truth, mọi thứ phụ thuộc vào đây |
| 3 | Project CRUD | Cần ít nhất 1 project để gắn session vào |
| 4 | Session CRUD | Container cho review flow |
| 5 | Screenshot capture (monitor chứa cursor) | Core value — không chụp được = không review được |
| 6 | Overlay window (fullscreen, always-on-top) | Nơi user đánh dấu issue |
| 7 | Number marker tool (①②③) | Annotation tối thiểu |
| 8 | Rectangle tool | Annotation phổ biến nhất |
| 9 | Issue form (title, type, severity) | Metadata cho mỗi marker |
| 10 | Lưu session + issues + screenshot vào DB + disk | Persistence |
| 11 | Dashboard: list sessions → list issues | Xem lại kết quả review |
| 12 | Trigger overlay bằng một cách nào đó | Phải có cách mở overlay |

### 🟡 Should Have — Làm nếu kịp trong 4-6 tuần

| # | Feature | Lý do |
|---|---|---|
| 13 | Text note tool | Annotation hữu ích, canvas text đơn giản |
| 14 | Arrow tool | Chỉ hướng, hay dùng khi review |
| 15 | Issue detail view (xem screenshot + marker) | Cần cho review lại |
| 16 | Crop từng issue | Tiện cho export, nhưng có thể bỏ qua MVP |
| 17 | Export HTML (self-contained) | Chia sẻ kết quả review |
| 18 | Filter dashboard theo project/status | Usability |
| 19 | Edit issue sau khi tạo | Sửa lỗi nhập sai |
| 20 | Delete session/issue | Dọn dẹp data |

### ❌ Later — Không động vào trong MVP

| Feature | Defer đến |
|---|---|
| Google Drive sync/share | v0.2 |
| Export PDF, CSV | v0.3 |
| Free draw tool | v0.1.1 |
| Highlight tool | v0.1.1 |
| Blur tool | v0.1.1 |
| Circle tool | v0.1.1 |
| Undo/redo trên overlay | v0.1.1 |
| Tags system | v0.1.1 |
| Advanced multi-monitor (chọn monitor) | v1.0 |
| Keyboard shortcuts tùy chỉnh | v0.2 |
| Settings page đầy đủ | v0.2 |
| Auto-update | v0.2 |
| Onboarding tutorial | v0.2 |
| PostgreSQL migration | Không cần trừ khi chuyển SaaS |
| Team collaboration | Không trong desktop app |
| Installer signed | v1.0 |
| Export Markdown | v0.1.1 (đơn giản, nhưng HTML đủ cho MVP) |

---

## 2. Thứ Tự Code

```
 ① Init project (Tauri + Vue + Vuetify + TypeScript)
 │
 ② SQLite setup (connection, schema, migration)
 │
 ③ Rust models + repositories (Project, Session, Capture, Issue)
 │
 ④ Tauri commands — Project CRUD
 │
 ⑤ Vue Dashboard shell (sidebar + main area + routing)
 │
 ⑥ Vue Project management UI
 │
 ⑦ Tauri commands — Session CRUD
 │
 ⑧ Vue Session list + create session UI
 │
 ⑨ Screenshot capture service (Rust, xcap crate)
 │
 ⑩ Overlay window (Tauri dynamic window, transparent, fullscreen)
 │
 ⑪ Overlay trigger (global hotkey TRƯỚC, middle mouse hold SAU)
 │
 ⑫ Canvas rendering (screenshot layer + drawing layer)
 │
 ⑬ Number marker tool
 │
 ⑭ Rectangle tool
 │
 ⑮ Issue form panel trên overlay
 │
 ⑯ Save flow: annotations → Rust → DB + disk
 │
 ⑰ Dashboard: session detail → issue list → issue detail
 │
 ⑱ Text note tool + Arrow tool (should have)
 │
 ⑲ Crop từng issue (should have)
 │
 ⑳ Export HTML (should have)
```

---

## 3. Week-by-Week Plan

---

### Week 1 — Foundation

**Goal:** Tauri app chạy được, có SQLite, có dashboard skeleton, CRUD project/session.

#### Tasks

| # | Task | Est. |
|---|---|---|
| 1.1 | `npm create tauri-app@latest` — init project với Vue + TypeScript | 1h |
| 1.2 | Add Vuetify 3, Pinia, Vue Router | 1h |
| 1.3 | Setup Vuetify dark theme (augustDark từ ARCHITECTURE.md) | 1h |
| 1.4 | Tạo `src-tauri/migrations/v001_initial.sql` — full schema | 2h |
| 1.5 | Rust: `database/connection.rs` + `database/migrations.rs` | 3h |
| 1.6 | Rust: `state.rs` — AppState với Mutex\<Connection\> | 1h |
| 1.7 | Rust: `models/project.rs`, `models/session.rs` | 2h |
| 1.8 | Rust: `database/repositories/project_repo.rs` — CRUD | 3h |
| 1.9 | Rust: `database/repositories/session_repo.rs` — CRUD | 3h |
| 1.10 | Rust: `commands/project_commands.rs` — Tauri commands | 2h |
| 1.11 | Rust: `commands/session_commands.rs` — Tauri commands | 2h |
| 1.12 | Vue: `services/tauriCommands.ts` — invoke wrappers | 2h |
| 1.13 | Vue: `types/` — Project, Session, Issue TypeScript types | 1h |
| 1.14 | Vue: Dashboard layout — AppHeader, AppSidebar, router | 3h |
| 1.15 | Vue: ProjectSelector + create project dialog | 2h |
| 1.16 | Vue: SessionList + create session dialog | 3h |
| 1.17 | System tray icon (basic: Open + Quit) | 2h |

**Total: ~32h (~4 ngày làm 8h)**

#### Files tạo mới

```
# Rust
src-tauri/migrations/v001_initial.sql
src-tauri/src/main.rs                    (modify — register commands)
src-tauri/src/lib.rs                     (modify — declare modules)
src-tauri/src/state.rs
src-tauri/src/error.rs
src-tauri/src/models/mod.rs
src-tauri/src/models/project.rs
src-tauri/src/models/session.rs
src-tauri/src/models/capture.rs          (struct only, logic later)
src-tauri/src/models/issue.rs            (struct only, logic later)
src-tauri/src/database/mod.rs
src-tauri/src/database/connection.rs
src-tauri/src/database/migrations.rs
src-tauri/src/database/repositories/mod.rs
src-tauri/src/database/repositories/project_repo.rs
src-tauri/src/database/repositories/session_repo.rs
src-tauri/src/commands/mod.rs
src-tauri/src/commands/project_commands.rs
src-tauri/src/commands/session_commands.rs
src-tauri/src/utils/mod.rs
src-tauri/src/utils/paths.rs
src-tauri/src/utils/id.rs

# Vue
src/main.ts
src/App.vue
src/plugins/vuetify.ts
src/plugins/pinia.ts
src/router/index.ts
src/types/project.ts
src/types/session.ts
src/types/issue.ts
src/types/annotation.ts
src/services/tauriCommands.ts
src/stores/projectStore.ts
src/stores/sessionStore.ts
src/stores/uiStore.ts
src/components/common/AppHeader.vue
src/components/common/AppSidebar.vue
src/components/common/EmptyState.vue
src/components/common/ConfirmDialog.vue
src/components/dashboard/ProjectSelector.vue
src/components/dashboard/SessionList.vue
src/components/dashboard/SessionCard.vue
```

#### Acceptance Criteria

- [ ] `npm run tauri dev` → app mở, hiển thị dashboard.
- [ ] System tray icon hiển thị, right-click có menu.
- [ ] Tạo project mới → hiển thị trong sidebar.
- [ ] Tạo session mới → hiển thị trong main area.
- [ ] Đóng app, mở lại → data vẫn còn (SQLite persist).
- [ ] `august_mark.db` file tồn tại trong app data directory.

#### Risks

| Risk | Mitigation |
|---|---|
| Tauri 2 CLI thay đổi API | Pin version trong `package.json` và `Cargo.toml` |
| Vuetify 3 conflict với Tauri webview | Test ngay sau khi add, trước khi code tiếp |
| SQLite bundled compile chậm | Bình thường, build lần đầu ~2-3 phút. Lần sau incremental nhanh |

#### Manual Test

```
1. npm run tauri dev
2. App mở → thấy dashboard trống
3. Click "New Project" → nhập tên → thấy project xuất hiện
4. Click project → click "New Session" → nhập tên → thấy session
5. Đóng app hoàn toàn
6. Mở lại → project và session vẫn còn
7. Right-click system tray → "Quit" → app tắt
```

---

### Week 2 — Screenshot + Overlay

**Goal:** Giữ hotkey → chụp screenshot → overlay mở hiển thị ảnh.

#### Tasks

| # | Task | Est. |
|---|---|---|
| 2.1 | Rust: add `xcap` crate, test screenshot capture cơ bản | 2h |
| 2.2 | Rust: `services/screen_capture.rs` — capture monitor chứa cursor | 4h |
| 2.3 | Rust: `services/file_storage.rs` — save screenshot PNG to disk | 2h |
| 2.4 | Rust: `database/repositories/capture_repo.rs` — CRUD | 2h |
| 2.5 | Rust: `commands/capture_commands.rs` — `trigger_capture` command | 3h |
| 2.6 | Rust: `commands/capture_commands.rs` — `open_overlay` (dynamic window) | 4h |
| 2.7 | Tạo `public/overlay.html` — entry point cho overlay window | 1h |
| 2.8 | Vue: `src/overlay.ts` — overlay entry point | 1h |
| 2.9 | Vue: `src/OverlayApp.vue` — root component | 2h |
| 2.10 | Vue: `components/overlay/AnnotationCanvas.vue` — load screenshot lên canvas | 4h |
| 2.11 | Vue: `components/overlay/OverlayStatusBar.vue` — top bar (session info + Done + Cancel) | 2h |
| 2.12 | Global hotkey trigger: `Ctrl+Shift+M` (Tauri global shortcut) | 3h |
| 2.13 | Wire up: hotkey → capture → overlay opens → screenshot displays | 4h |
| 2.14 | Close overlay: Done/Esc → window closes, return to idle | 2h |

**Total: ~34h**

#### Files tạo mới

```
# Rust
src-tauri/src/services/mod.rs
src-tauri/src/services/screen_capture.rs
src-tauri/src/services/file_storage.rs
src-tauri/src/database/repositories/capture_repo.rs
src-tauri/src/commands/capture_commands.rs

# Vue
public/overlay.html
src/overlay.ts
src/OverlayApp.vue
src/components/overlay/AnnotationCanvas.vue
src/components/overlay/OverlayStatusBar.vue
src/services/tauriEvents.ts
src/composables/useTauriEvents.ts
```

#### Acceptance Criteria

- [ ] Nhấn `Ctrl+Shift+M` → overlay mở fullscreen.
- [ ] Overlay hiển thị screenshot của monitor hiện tại (static image).
- [ ] Overlay transparent background, always-on-top.
- [ ] Screenshot lưu thành file PNG trong `~/AugustMark/screenshots/`.
- [ ] Nhấn Esc hoặc Done → overlay đóng.
- [ ] Capture record lưu vào SQLite (session_id, screenshot_path, monitor info).

#### Risks

| Risk | Impact | Fallback |
|---|---|---|
| `xcap` không compile trên Windows | Chặn hoàn toàn | Dùng `screenshots` crate thay thế. Nếu cả hai lỗi → dùng Windows GDI trực tiếp qua `winapi` |
| Overlay window không transparent | UI xấu nhưng vẫn dùng được | Set background color = screenshot image, bỏ transparency. Vẫn hoạt động. |
| Overlay window bị che bởi app khác | Annotation sai | Double check `always_on_top(true)` + `focused(true)`. Nếu vẫn lỗi → set window z-order qua `winapi` |
| Tauri global shortcut conflict với app khác | Hotkey không trigger | Cho user chọn hotkey khác trong settings (hard-code 2-3 options ban đầu) |

#### ⚠️ Quyết định: Middle Mouse Hold

**KHÔNG làm middle mouse hold trong Week 2.** Lý do:
- Cần global mouse hook (`rdev` crate) — phức tạp, có thể bị antivirus block.
- Hold detection logic dễ gây false positive với scroll click.
- Solo dev không nên chặn 1 tuần vì 1 feature UX.

**Dùng `Ctrl+Shift+M` hotkey trước.** Middle mouse hold defer sang Week 5 (optional).

#### Manual Test

```
1. npm run tauri dev
2. Mở một website/app bất kỳ trên màn hình
3. Nhấn Ctrl+Shift+M
4. Overlay mở → thấy screenshot của màn hình
5. Ảnh screenshot khớp với nội dung đang hiển thị
6. Nhấn Esc → overlay đóng
7. Mở ~/AugustMark/screenshots/ → thấy file PNG
8. Kiểm tra DB → capture record có đúng session_id
9. Thử trên monitor 2 nếu có (di cursor sang monitor 2 trước khi nhấn hotkey)
```

---

### Week 3 — Annotation Tools + Issue Form

**Goal:** Đặt marker, vẽ rectangle, nhập issue form, lưu vào DB.

#### Tasks

| # | Task | Est. |
|---|---|---|
| 3.1 | Vue: `composables/useCanvas.ts` — canvas layer management (screenshot + markers + active drawing) | 4h |
| 3.2 | Vue: `composables/useAnnotation.ts` — tool state machine (select tool, draw, commit) | 4h |
| 3.3 | Vue: `components/overlay/AnnotationToolbar.vue` — tool buttons (marker, rect, text, arrow) | 3h |
| 3.4 | Implement Number Marker tool — click → draw ① at position | 3h |
| 3.5 | Implement Rectangle tool — drag → draw rectangle | 4h |
| 3.6 | Implement Text Note tool — click → input text → render on canvas | 3h |
| 3.7 | Implement Arrow tool — drag → draw arrow | 3h |
| 3.8 | Vue: `components/overlay/IssueFormPanel.vue` — side panel (title, description, type, severity) | 4h |
| 3.9 | Vue: `stores/overlayStore.ts` — annotations state, current tool, issue list | 3h |
| 3.10 | Rust: `database/repositories/issue_repo.rs` — CRUD | 3h |
| 3.11 | Rust: `commands/issue_commands.rs` — save_capture_annotations command | 3h |
| 3.12 | Save flow: Done → collect annotations → invoke Rust → persist | 4h |

**Total: ~41h (~5 ngày)**

#### Files tạo mới

```
# Vue
src/composables/useCanvas.ts
src/composables/useAnnotation.ts
src/components/overlay/AnnotationToolbar.vue
src/components/overlay/IssueFormPanel.vue
src/stores/overlayStore.ts

# Rust
src-tauri/src/database/repositories/issue_repo.rs
src-tauri/src/commands/issue_commands.rs
src-tauri/src/services/image_processor.rs      (basic: save annotated screenshot)
```

#### Annotation Tool Specs (giữ đơn giản)

```
Number Marker:
  - Click canvas → marker ① tại vị trí click
  - Số tự tăng (1, 2, 3...)
  - Render: circle (30px) + number inside
  - Color: primary orange (#FF6B35)
  - Sau khi đặt → IssueFormPanel mở

Rectangle:
  - MouseDown → start point
  - MouseMove → draw preview rectangle (dashed)
  - MouseUp → commit rectangle (solid)
  - Color: primary orange, stroke 2px
  - Number marker auto-placed ở top-left corner
  - Sau khi commit → IssueFormPanel mở

Arrow:
  - MouseDown → start point
  - MouseMove → draw preview line + arrowhead
  - MouseUp → commit arrow
  - Number marker ở start point
  - Sau khi commit → IssueFormPanel mở

Text Note:
  - Click canvas → input field hiện tại vị trí
  - User nhập text → Enter → render text trên canvas
  - Number marker ở bên trái text
  - Sau khi Enter → IssueFormPanel mở
```

#### Issue Form Fields (MVP tối giản)

```
┌─────────────────────────────┐
│ Issue #3                    │
├─────────────────────────────┤
│ Title: [________________]   │  ← required
│                             │
│ Type: [Bug ▼]               │  ← dropdown: Bug, UI, UX,
│                             │     Suggestion, Requirement, Question
│ Severity: [Minor ▼]        │  ← dropdown: Critical, Major,
│                             │     Minor, Info
│ Description:                │
│ [________________________]  │  ← textarea, optional
│ [________________________]  │
│                             │
│ [Cancel]           [Save]   │
└─────────────────────────────┘
```

Không cần trong MVP: tags, status (default Open), color picker.

#### Acceptance Criteria

- [ ] Chọn Marker tool → click canvas → ① xuất hiện → issue form mở.
- [ ] Chọn Rectangle tool → drag → rectangle xuất hiện → issue form mở.
- [ ] Chọn Arrow tool → drag → arrow xuất hiện → issue form mở.
- [ ] Chọn Text tool → click → nhập text → issue form mở.
- [ ] Nhập title + type + severity → Save → marker confirmed (solid color).
- [ ] Đặt được 5+ markers trên cùng 1 screenshot.
- [ ] Click Done → tất cả issues lưu vào SQLite.
- [ ] Screenshot file + annotated screenshot file lưu trên disk.

#### Risks

| Risk | Fallback |
|---|---|
| Canvas mouse events bị overlay HTML elements chặn | Dùng `pointer-events: none` cho UI layer, chỉ toolbar + form có `pointer-events: auto` |
| Canvas coordinate bị sai do DPI scaling | Lấy `devicePixelRatio`, scale tọa độ mouse event |
| Text rendering trên canvas bị mờ | Dùng `ctx.font` size × devicePixelRatio, scale canvas |
| Issue form panel che canvas | Panel slide-in từ phải, chiếm 320px, canvas resize |

#### Manual Test

```
1. Tạo session → nhấn Ctrl+Shift+M → overlay mở
2. Click toolbar "Marker" → click 3 vị trí khác nhau → thấy ①②③
3. Với mỗi marker, nhập title + chọn type + severity → Save
4. Click toolbar "Rectangle" → drag vẽ 2 rectangles
5. Click toolbar "Arrow" → drag vẽ 1 arrow
6. Click Done
7. Kiểm tra DB: session có 6 issues, mỗi issue có marker_x, marker_y, annotation_data
8. Kiểm tra disk: screenshot gốc + annotated screenshot tồn tại
9. Repeat: Ctrl+Shift+M lần nữa → overlay mới → mark thêm issue → Done
10. Session giờ có issues từ 2 captures khác nhau
```

---

### Week 4 — Dashboard + Issue Detail + Export

**Goal:** Dashboard hiển thị đầy đủ data, xem issue detail, export HTML.

#### Tasks

| # | Task | Est. |
|---|---|---|
| 4.1 | Vue: `components/dashboard/IssueList.vue` — list issues trong session | 3h |
| 4.2 | Vue: `components/dashboard/IssueCard.vue` — card với thumbnail + metadata | 3h |
| 4.3 | Vue: `components/dashboard/IssueDetail.vue` — full view (screenshot + marker highlight + metadata) | 4h |
| 4.4 | Vue: `components/dashboard/FilterBar.vue` — filter project, type, severity, status | 3h |
| 4.5 | Vue: `stores/issueStore.ts` — issues state + filter logic | 2h |
| 4.6 | Rust: issue queries — get by session, get by project, filter/search | 3h |
| 4.7 | Vue: edit issue inline (update title, type, severity, status, description) | 3h |
| 4.8 | Vue: delete session, delete issue (with confirm dialog) | 2h |
| 4.9 | Rust: `services/image_processor.rs` — crop vùng xung quanh marker | 3h |
| 4.10 | Wire up crop: sau khi save annotations → generate crops | 2h |
| 4.11 | Rust: `services/export_engine.rs` — HTML export | 5h |
| 4.12 | Rust: `commands/export_commands.rs` — export_html command | 2h |
| 4.13 | Vue: `components/export/ExportDialog.vue` — chọn session → export | 2h |
| 4.14 | HTML template: session header + issue table + screenshots | 3h |
| 4.15 | Overall UI polish: loading states, empty states, error toasts | 3h |

**Total: ~40h**

#### Files tạo mới

```
# Vue
src/components/dashboard/IssueList.vue
src/components/dashboard/IssueCard.vue
src/components/dashboard/IssueDetail.vue
src/components/dashboard/FilterBar.vue
src/components/dashboard/StatsOverview.vue
src/components/export/ExportDialog.vue
src/stores/issueStore.ts
src/composables/useIssue.ts
src/composables/useExport.ts

# Rust
src-tauri/src/services/export_engine.rs
src-tauri/src/commands/export_commands.rs
# Modify:
src-tauri/src/services/image_processor.rs  (add crop)
src-tauri/src/commands/issue_commands.rs    (add queries + update + delete)
```

#### HTML Export Template (đơn giản, self-contained)

```
report.html (single file)
├── Inline CSS (dark theme, responsive)
├── Session Header
│   ├── Project name
│   ├── Session title
│   ├── Date
│   └── Issue count summary (by type, by severity)
├── Per-Capture Section
│   ├── Annotated screenshot (base64 inline)
│   ├── Issue list for this capture
│   │   ├── ① Title — Type badge — Severity badge
│   │   │   ├── Description
│   │   │   └── Crop image (base64 inline, nếu có)
│   │   ├── ② ...
│   │   └── ③ ...
│   └── Horizontal separator
└── Footer: "Generated by August Mark v0.1"
```

Không cần: print CSS, interactive elements, JavaScript trong report.

#### Acceptance Criteria

- [ ] Dashboard → click session → thấy danh sách issues với thumbnail.
- [ ] Click issue → thấy screenshot gốc với marker highlighted + metadata.
- [ ] Edit issue: đổi title, type, severity, status → lưu thành công.
- [ ] Delete issue → biến mất khỏi list, crop file bị xóa.
- [ ] Delete session → tất cả issues + captures + files bị xóa.
- [ ] Filter by type (chỉ hiện Bug) → hoạt động.
- [ ] Filter by severity (chỉ hiện Critical) → hoạt động.
- [ ] Export HTML → chọn save location → file .html tạo thành công.
- [ ] Mở file .html trong Chrome → hiển thị đúng, ảnh hiện, badges đúng màu.
- [ ] Crop images hiển thị trong issue detail (nếu đã implement).

#### Risks

| Risk | Fallback |
|---|---|
| Base64 encode ảnh lớn (4K) làm HTML file quá nặng | Giới hạn ảnh inline max 2MB, scale down nếu cần. Hoặc tạo folder `report_assets/` bên cạnh .html |
| Crop bounding box tính sai | MVP: crop vuông 400×400px centered tại marker position. Đơn giản, chắc chắn đúng. |
| Dashboard chậm với nhiều issues | MVP không cần pagination. Nếu chậm → lazy load hoặc virtual scroll (Week 5) |

#### Manual Test

```
1. Đã có data từ Week 3 (sessions + issues)
2. Mở Dashboard → thấy sessions
3. Click session → thấy issues (có thumbnail nhỏ)
4. Click issue → thấy full screenshot + marker highlighted
5. Edit issue title → save → title cập nhật
6. Đổi status từ Open → Resolved → badge đổi màu
7. Click "Export HTML" → chọn Desktop → save
8. Mở report.html trong browser → hiển thị đẹp
9. Delete 1 issue → biến mất
10. Delete session → biến mất + files bị xóa
```

---

### Week 5 — Optional Polish

**Goal:** Cải thiện UX. Thêm middle mouse hold nếu có thời gian.

#### Tasks (chọn theo priority)

| # | Task | Priority | Est. |
|---|---|---|---|
| 5.1 | Middle mouse hold trigger (rdev crate) | HIGH | 6h |
| 5.2 | Visual feedback khi hold (cursor change hoặc progress ring) | MEDIUM | 2h |
| 5.3 | Annotation color picker (chọn màu cho marker/rect) | LOW | 2h |
| 5.4 | Annotation stroke width slider | LOW | 1h |
| 5.5 | Keyboard shortcuts trên overlay (1-4 chọn tool, Esc cancel) | MEDIUM | 2h |
| 5.6 | Stats overview trên dashboard (count by type/severity chart) | LOW | 3h |
| 5.7 | Settings page cơ bản (theme toggle, save path, hotkey display) | MEDIUM | 3h |
| 5.8 | Backup database thủ công (button trong settings) | MEDIUM | 2h |
| 5.9 | Session status (active/completed) management | MEDIUM | 2h |
| 5.10 | Toast notifications (issue saved, export done, error) | MEDIUM | 2h |

#### Middle Mouse Hold Implementation (5.1)

Nếu attempt:

```rust
// Thêm rdev vào Cargo.toml
// Tạo src/services/mouse_hook.rs

// Logic:
// 1. Spawn thread chạy rdev::listen()
// 2. Khi ButtonPress(Middle) → ghi timestamp
// 3. Spawn timer thread sleep(1000ms)
// 4. Sau 1s, kiểm tra middle vẫn held → emit "overlay:trigger"
// 5. Khi ButtonRelease(Middle) trước 1s → cancel

// Rủi ro:
// - rdev cần quyền admin trên một số Windows build
// - Antivirus có thể block
// - Conflict với middle-click scroll

// Nếu lỗi → giữ Ctrl+Shift+M, đánh dấu middle mouse hold cho v0.2
```

#### Manual Test (Middle Mouse Hold)

```
1. Bật app
2. Di chuột đến bất kỳ đâu
3. Giữ chuột giữa > 1 giây → overlay mở
4. Giữ chuột giữa < 0.5 giây (click thường) → không có gì xảy ra
5. Giữ chuột giữa khi overlay đang mở → không trigger lại
6. Scroll bằng chuột giữa (lăn bánh xe) → không conflict
```

---

### Week 6 — Buffer & Release

**Goal:** Fix bugs, test trên máy khác, đóng gói installer.

#### Tasks

| # | Task | Est. |
|---|---|---|
| 6.1 | Fix bugs tồn đọng từ Week 1-5 | 8h |
| 6.2 | Test trên Windows 10 (nếu dev trên Win 11) | 2h |
| 6.3 | Test với DPI scaling 125%, 150% | 2h |
| 6.4 | Test với dual monitor | 2h |
| 6.5 | `npm run tauri build` → tạo installer | 2h |
| 6.6 | Test installer: cài trên máy mới (clean) | 2h |
| 6.7 | README.md | 2h |
| 6.8 | CHANGELOG.md | 1h |
| 6.9 | Tạo GitHub repo + push code | 1h |
| 6.10 | Tạo GitHub Release + upload installer | 1h |

#### Release Checklist (rút gọn cho MVP)

```
- [ ] App chạy được sau khi cài từ installer
- [ ] Full flow: tạo project → tạo session → capture → mark → save → view → export
- [ ] Không crash khi không có project/session (empty state)
- [ ] Không crash khi capture 4K screenshot
- [ ] Không crash khi đặt 10+ markers
- [ ] Export HTML mở được trên Chrome/Edge
- [ ] System tray hoạt động (open/quit)
- [ ] Data persist qua restart
- [ ] Installer < 30MB
- [ ] README có usage instructions
```

---

## 4. Fallback Decision Matrix

| Tình huống | Phát hiện khi | Fallback | Impact |
|---|---|---|---|
| **Middle mouse hold không hoạt động** (rdev fail, antivirus block, conflict) | Week 5 | Giữ `Ctrl+Shift+M` global hotkey. Nó đã work từ Week 2. User vẫn trigger được overlay. | Thấp — UX khác ý tưởng gốc nhưng vẫn functional |
| **Overlay transparent bị lỗi** (Tauri webview không hỗ trợ transparency) | Week 2 | Set overlay background = screenshot image (fit to window). Trông giống nhau, chỉ mất hiệu ứng semi-transparent edges. | Rất thấp — user không nhận ra |
| **Screenshot multi-monitor sai** (capture sai monitor, tọa độ lệch) | Week 2 | Chỉ capture primary monitor. Ghi rõ limitation trong README. Fix ở v0.1.1. | Trung bình — user có 2+ monitor bị giới hạn |
| **Crop issue lỗi** (bounding box sai, ảnh crop bị đen) | Week 4 | Bỏ crop, chỉ lưu full screenshot. Dashboard show full ảnh với marker highlight. Export cũng dùng full ảnh. | Thấp — full ảnh vẫn hữu ích |
| **Canvas DPI scaling sai** (annotation vẽ sai vị trí trên HiDPI) | Week 3 | Ép canvas render ở 1x scale, bỏ qua DPI. Ảnh nhỏ hơn native nhưng annotation đúng vị trí. | Trung bình — ảnh không sắc nét trên 4K |
| **xcap crate không compile** | Week 2 | Dùng `screenshots` crate. Nếu cũng lỗi → dùng `winapi` + `BitBlt` trực tiếp (boilerplate nhiều nhưng chắc chắn work trên Windows). | Thấp — chỉ mất thời gian implement alternative |
| **Tauri global shortcut conflict** | Week 2 | Hard-code 3 hotkey options (`Ctrl+Shift+M`, `Ctrl+Alt+M`, `F9`). User chọn trong settings. | Rất thấp |

---

## 5. Database Schema (Rút gọn cho MVP)

Dùng schema từ ARCHITECTURE.md nhưng **bỏ các table chưa cần**:

```sql
-- MVP v0.1: Chỉ dùng 5 tables
-- ✅ schema_version
-- ✅ projects
-- ✅ sessions
-- ✅ captures
-- ✅ issues
-- ❌ tags              → defer v0.1.1
-- ❌ issue_tags        → defer v0.1.1
-- ❌ settings          → hard-code defaults, defer v0.2
-- ❌ sync_log          → defer v0.2
```

Issue table MVP — bỏ `tags` join, giữ status default `Open`:

```sql
CREATE TABLE issues (
    id              TEXT PRIMARY KEY,
    capture_id      TEXT NOT NULL REFERENCES captures(id) ON DELETE CASCADE,
    session_id      TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    project_id      TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    marker_number   INTEGER NOT NULL,
    title           TEXT NOT NULL,
    description     TEXT DEFAULT '',
    issue_type      TEXT NOT NULL DEFAULT 'Bug',
    severity        TEXT NOT NULL DEFAULT 'Minor',
    status          TEXT NOT NULL DEFAULT 'Open',
    marker_x        REAL NOT NULL,
    marker_y        REAL NOT NULL,
    annotation_data TEXT NOT NULL DEFAULT '{}',
    color           TEXT DEFAULT '#FF6B35',
    stroke_width    REAL DEFAULT 2.0,
    crop_path       TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);
```

Tags table + issue_tags table + settings table + sync_log table → tạo trong migration v002 khi cần.

---

## 6. Key Technical Decisions (Giản lược cho MVP)

| Decision | MVP Choice | Lý do |
|---|---|---|
| Overlay trigger | Global hotkey `Ctrl+Shift+M` | Reliable, built-in Tauri, zero risk |
| Canvas library | Raw HTML5 Canvas 2D | Không cần thêm dependency. 4 tools cơ bản đủ tự implement |
| Image format | PNG only | Lossless, `image` crate hỗ trợ sẵn |
| Export | HTML only | Single file, no dependency. Markdown thêm sau dễ |
| State management | Pinia (setup syntax) | Official, simple, đủ cho MVP |
| Error handling | `thiserror` cho Rust, `try/catch` + toast cho Vue | Đủ cho MVP, refactor sau |
| Annotation storage | JSON string trong SQLite TEXT column | Flexible, không cần thêm table. Parse ở frontend |
| IPC large data | Screenshot save to disk, pass path qua IPC | Tránh base64 encode ảnh lớn qua IPC |
| Multi-window | 2 windows (main + overlay), overlay tạo dynamic | Tauri 2 hỗ trợ tốt |

---

## 7. Definition of Done — MVP v0.1

MVP hoàn thành khi user có thể chạy flow sau **mà không crash**:

```
1. Mở August Mark
2. Tạo project "Website Redesign"
3. Tạo session "Homepage Review"
4. Nhấn Ctrl+Shift+M → overlay mở → thấy screenshot
5. Đặt marker ① "Logo bị mờ" — Bug — Major
6. Vẽ rectangle ② "Button sai màu" — UI — Minor
7. Vẽ arrow ③ "Khoảng cách quá lớn" — UX — Minor
8. Nhấn Done → quay về desktop
9. Nhấn Ctrl+Shift+M lần nữa → mark thêm 2 issues
10. Nhấn Done
11. Mở Dashboard → thấy session với 5 issues
12. Click issue → thấy screenshot + marker
13. Đổi status issue ① từ Open → Resolved
14. Export HTML → mở browser → thấy report
15. Đóng app → mở lại → data vẫn còn
```

**Nếu flow trên chạy được end-to-end = MVP done. Ship it.**

---

## 8. Không Làm — Checklist nhắc nhở

Khi code, nếu muốn thêm feature ngoài danh sách Must Have / Should Have:

- ❌ ĐỪNG thêm tag system
- ❌ ĐỪNG thêm free draw tool
- ❌ ĐỪNG thêm blur/highlight
- ❌ ĐỪNG thêm undo/redo
- ❌ ĐỪNG thêm settings page phức tạp
- ❌ ĐỪNG thêm Google Drive
- ❌ ĐỪNG thêm PDF export
- ❌ ĐỪNG thêm search full-text
- ❌ ĐỪNG optimize performance sớm
- ❌ ĐỪNG viết unit test trước khi flow chạy được
- ❌ ĐỪNG refactor code trước khi flow chạy được

**Rule: Make it work → Make it right → Make it fast.**

Week 1-4: Make it work.
Week 5: Make it right.
Week 6: Ship.
