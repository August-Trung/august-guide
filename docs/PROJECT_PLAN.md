# PROJECT_PLAN.md — August Mark

> **Mark everything, review later.**

| Field | Value |
|---|---|
| Product Name | August Mark |
| App ID | `com.august.mark` |
| Repository | `august-mark` |
| Version | v0.1 (MVP) |
| Last Updated | 2026-06-15 |

---

## 1. Product Vision

August Mark là cây bút đỏ cho thế giới số.

Trong quy trình review bất kỳ nội dung nào trên màn hình — website, app, design, tài liệu, game, video — người dùng kích hoạt overlay bằng một thao tác duy nhất (giữ chuột giữa 1 giây), đánh dấu **nhiều issue trên cùng một screenshot**, rồi quản lý/export/share toàn bộ.

**Không cần mở app riêng. Không phải chụp từng ảnh riêng lẻ. Không phải paste qua paste lại.**

Triết lý: bạn đang xem — bạn thấy lỗi — bạn mark ngay — review sau.

---

## 2. Target Users

| Persona | Mô tả | Pain Point hiện tại |
|---|---|---|
| **QA Engineer** | Test web/app, cần report bug có screenshot + annotation | Phải chụp riêng từng ảnh, paste vào Jira, viết mô tả tay |
| **UI/UX Designer** | Review design implementation, so sánh mockup vs thực tế | Comment trên Figma không capture được trạng thái runtime |
| **Product Manager** | Review feature mới, đánh dấu yêu cầu thay đổi | Email/Slack với screenshot rời rạc, khó track |
| **Frontend Developer** | Self-review UI trước khi push, mark pixel issues | Không có tool nhanh, thường dùng DevTools + note riêng |
| **Freelancer / Agency** | Review deliverable cho client, tạo feedback report | Loom + screenshot + Google Docs = rời rạc, tốn thời gian |
| **Technical Writer** | Review tài liệu, đánh dấu lỗi formatting/content | Screenshot + comment trong Word = chậm |

**Primary target (MVP):** QA Engineer, UI/UX Designer, Frontend Developer.

---

## 3. Main Use Cases

### UC-01: Quick Review Session
1. Mở August Mark (chạy system tray).
2. Duyệt website/app bình thường.
3. Giữ chuột giữa 1 giây → overlay bật lên → đánh dấu 5 issue trên màn hình hiện tại.
4. Nhấn Done → tất cả 5 issue được lưu vào session.
5. Tiếp tục duyệt → giữ chuột giữa lần nữa → mark thêm 3 issue trên trang khác.
6. Kết thúc session → có 8 issue với screenshot và metadata đầy đủ.

### UC-02: Export Bug Report
1. Mở Dashboard → chọn session.
2. Export HTML report → gửi email cho team.
3. Hoặc export CSV → import vào Jira/Linear.

### UC-03: Google Drive Sync & Share
1. Kết nối Google Drive.
2. Sync session + screenshot + report lên Drive.
3. Tạo share link → gửi cho client/stakeholder xem report trên browser.

### UC-04: Session Management
1. Mở Dashboard → filter theo project, ngày, severity.
2. Update status issue từ Open → Resolved.
3. Archive session cũ.

### UC-05: Multi-Monitor Review
1. Có 2 màn hình.
2. Giữ chuột giữa trên màn hình 2 → overlay bật trên màn hình 2.
3. Mark issue → screenshot chỉ capture màn hình 2.

---

## 4. MVP Scope (v0.1)

### ✅ Trong MVP

| # | Feature | Priority |
|---|---|---|
| 1 | System tray app, luôn chạy nền | P0 |
| 2 | Global middle mouse hold (1s) → bật overlay | P0 |
| 3 | Transparent overlay phủ toàn màn hình hiện tại | P0 |
| 4 | Screenshot capture khi overlay bật | P0 |
| 5 | Number marker (①②③) trên overlay | P0 |
| 6 | Rectangle annotation | P0 |
| 7 | Arrow annotation | P0 |
| 8 | Free draw annotation | P1 |
| 9 | Text note annotation | P1 |
| 10 | Issue form: title, description, type, severity | P0 |
| 11 | Issue status tracking (Draft → Open → Resolved → Closed) | P0 |
| 12 | Tags cho issue | P1 |
| 13 | SQLite local database | P0 |
| 14 | Dashboard: list sessions, list issues | P0 |
| 15 | Filter/search theo project, type, severity, status, date | P0 |
| 16 | Export HTML report | P0 |
| 17 | Export Markdown report | P1 |
| 18 | Local file storage cho screenshot/crops | P0 |
| 19 | Project management (CRUD) | P0 |
| 20 | Session management (CRUD) | P0 |

### ❌ Ngoài MVP (để sau)

| Feature | Lý do để sau |
|---|---|
| Circle annotation | Nice-to-have, rectangle đã đủ |
| Highlight annotation | Nice-to-have |
| Blur annotation | Cần thêm image processing, không urgent |
| Export PDF | Cần thêm PDF renderer library |
| Export CSV | Đơn giản nhưng không MVP-critical |
| Google Drive sync | Cần OAuth flow, API integration |
| Google Drive share link | Phụ thuộc sync |
| Multi-monitor: chọn monitor | MVP chỉ cần capture monitor chứa con trỏ |
| Keyboard shortcut tùy chỉnh | Dùng default trước |
| Undo/redo trên overlay | Complexity cao |
| Comparison view (before/after) | V2 feature |
| Team/collaboration features | SaaS scope |
| AI-powered analysis | Out of scope hoàn toàn giai đoạn đầu |
| Notification/reminder | V2 |

---

## 5. Feature List theo Phase

### Phase 1 — Foundation (v0.1-alpha)
> Mục tiêu: Chạy được flow cơ bản từ đầu đến cuối.

- [x] Tauri app scaffold + system tray
- [ ] Global mouse hook (middle button hold detection)
- [ ] Overlay window (transparent, fullscreen, always-on-top)
- [ ] Screenshot capture (current monitor)
- [ ] Canvas rendering screenshot trên overlay
- [ ] Number marker tool
- [ ] Rectangle tool
- [ ] Arrow tool
- [ ] Issue form (title, description, type, severity)
- [ ] SQLite schema + migration
- [ ] Save session + issues + screenshot to local
- [ ] Basic dashboard (list view)

### Phase 2 — Usability (v0.1-beta)
> Mục tiêu: Dùng được hàng ngày, export được.

- [ ] Free draw tool
- [ ] Text note tool
- [ ] Issue status management
- [ ] Tags
- [ ] Filter/search trên dashboard
- [ ] Export HTML report
- [ ] Export Markdown report
- [ ] Crop riêng cho từng issue
- [ ] Settings page (hotkey, save path, default project)
- [ ] Auto-update check

### Phase 3 — Polish (v0.1)
> Mục tiêu: Release-ready MVP.

- [ ] Circle tool
- [ ] Highlight tool
- [ ] Blur tool
- [ ] Undo/redo trên overlay
- [ ] Keyboard shortcuts (Esc thoát, 1-9 chọn marker, R rectangle, A arrow...)
- [ ] Onboarding / first-run tutorial
- [ ] Error handling toàn diện
- [ ] Performance optimization (large screenshots)
- [ ] Installer (MSI/EXE cho Windows)

### Phase 4 — Sync & Share (v0.2)
> Mục tiêu: Google Drive integration.

- [ ] Google OAuth2 flow (desktop app)
- [ ] Google Drive: upload screenshots
- [ ] Google Drive: upload database backup
- [ ] Google Drive: upload report.html
- [ ] Google Drive: upload metadata.json
- [ ] Share package generation
- [ ] Share link (Google Drive public link)
- [ ] Sync status indicator

### Phase 5 — Export & Integration (v0.3)
> Mục tiêu: Export đa dạng, integration bên ngoài.

- [ ] Export PDF report
- [ ] Export CSV (Jira/Linear compatible)
- [ ] Deep link: mở app từ URL scheme `augustmark://`
- [ ] Clipboard: copy issue as formatted text
- [ ] Bulk operations (multi-select issues, batch status change)

### Phase 6 — Pro (v1.0)
> Mục tiêu: Production-grade desktop app.

- [ ] Multi-monitor: chọn monitor cụ thể
- [ ] Video recording annotation (mark timestamp)
- [ ] Comparison view
- [ ] Custom templates cho report
- [ ] Plugin system (export adapters)
- [ ] Auto-backup schedule
- [ ] Usage analytics (local, opt-in)

---

## 6. User Flow Chi Tiết

### 6.1 First Run

```
App Launch
  → Welcome Screen
    → "Create your first Project"
    → Enter project name
    → Choose save directory (default: ~/AugustMark)
    → Done → Dashboard (empty state)
    → System tray icon active
```

### 6.2 Review Session Flow

```
User đang duyệt website/app/bất kỳ gì trên màn hình
  │
  ├─ Giữ chuột giữa ≥ 1 giây
  │   → Visual feedback: cursor thay đổi (hoặc subtle indicator)
  │   → Sau 1 giây: Screenshot captured (freeze frame)
  │   → Overlay window mở (fullscreen, always-on-top, transparent)
  │   → Screenshot hiển thị dưới overlay (static image)
  │   → Toolbar xuất hiện (bottom hoặc side)
  │
  ├─ User chọn tool từ toolbar
  │   ├─ Number Marker: click vị trí → marker ① xuất hiện → issue form mở
  │   ├─ Rectangle: drag để vẽ → issue form mở
  │   ├─ Arrow: drag để vẽ → issue form mở
  │   ├─ Free Draw: draw tự do → issue form mở
  │   ├─ Text Note: click vị trí → nhập text trực tiếp
  │   ├─ Highlight: drag vùng → semi-transparent highlight
  │   └─ Blur: drag vùng → blur applied
  │
  ├─ Issue Form (slide-in panel bên phải)
  │   ├─ Title (required)
  │   ├─ Description (optional, rich text)
  │   ├─ Type: [Bug | UI | UX | Suggestion | Requirement | Question]
  │   ├─ Severity: [Critical | Major | Minor | Info]
  │   ├─ Status: defaults to "Open"
  │   ├─ Tags: autocomplete từ tags đã dùng
  │   └─ Save → marker confirmed, quay lại overlay
  │
  ├─ User có thể đặt thêm marker (không giới hạn số lượng)
  │
  ├─ User nhấn "Done" (hoặc Esc)
  │   → Overlay đóng
  │   → Screenshot tổng (với tất cả annotations) được lưu
  │   → Crop riêng cho từng issue (vùng xung quanh marker)
  │   → Session entry được tạo trong database
  │   → Notification: "8 issues saved to Session #12"
  │
  └─ User tiếp tục duyệt
      → Giữ chuột giữa lần nữa → screenshot mới → overlay mới
      → Các issue mới thuộc cùng session đang active
```

### 6.3 Dashboard Flow

```
User mở app từ system tray
  → Dashboard
    ├─ Sidebar: Projects list
    ├─ Main area: Sessions list (sorted by date desc)
    │   └─ Expand session → Issues list with thumbnails
    ├─ Top bar: Filters (project, type, severity, status, date range)
    ├─ Search bar: full-text search title/description
    │
    ├─ Click issue → Issue detail view
    │   ├─ Full screenshot với marker highlighted
    │   ├─ Cropped view
    │   ├─ Edit form (all fields editable)
    │   ├─ Status change buttons
    │   └─ Delete issue
    │
    ├─ Select session → Actions
    │   ├─ Export HTML
    │   ├─ Export Markdown
    │   ├─ Export PDF (v0.3)
    │   ├─ Export CSV (v0.3)
    │   ├─ Sync to Google Drive (v0.2)
    │   └─ Delete session
    │
    └─ Settings (gear icon)
        ├─ General: language, theme (dark/light)
        ├─ Hotkey: customize trigger
        ├─ Storage: change save directory, view usage
        ├─ Google Drive: connect/disconnect
        └─ About: version, update check
```

### 6.4 Export Flow

```
Dashboard → Select session → Export HTML
  → Choose save location
  → Generate:
    ├─ report.html (self-contained, inline images)
    ├─ Includes: session info, all issues with screenshots, severity badges
    └─ Open in browser? [Yes / No]
```

---

## 7. Roadmap

### v0.1 — MVP (Target: 8-10 tuần)

| Tuần | Milestone | Deliverable |
|---|---|---|
| 1-2 | **Foundation** | Tauri scaffold, system tray, SQLite schema, basic Vue app |
| 3-4 | **Core Capture** | Global mouse hook, screenshot capture, overlay window |
| 5-6 | **Annotation** | Number marker, rectangle, arrow, canvas rendering |
| 7-8 | **Data & Dashboard** | Issue form, save to DB, dashboard CRUD, filter/search |
| 9 | **Export** | HTML export, Markdown export |
| 10 | **Polish & Release** | Error handling, installer, README, release |

### v0.2 — Sync (Target: 4-6 tuần sau v0.1)
- Google OAuth2 integration
- Google Drive upload/download
- Share package
- Sync indicator

### v0.3 — Export++ (Target: 3-4 tuần sau v0.2)
- PDF export
- CSV export
- URL scheme `augustmark://`
- Clipboard integration
- Bulk operations

### v1.0 — Production (Target: 4-6 tuần sau v0.3)
- Multi-monitor full support
- Custom templates
- Plugin system
- Auto-backup
- Stability & performance

---

## 8. Risks & Technical Challenges

### 🔴 High Risk

| Risk | Impact | Mitigation | Fallback |
|---|---|---|---|
| **Global mouse hook trên Windows** — Cần intercept middle mouse hold ở system level | Nếu không hook được, user không trigger được overlay | Dùng `rdev` crate (Rust) hoặc Windows raw input API qua `winapi` crate | Fallback: dùng global hotkey (Ctrl+Shift+M) thay vì middle mouse hold |
| **Overlay window always-on-top** — Cần tạo transparent fullscreen window phủ lên tất cả app khác | Nếu overlay bị che, annotation sai vị trí | Tauri hỗ trợ `always_on_top`, `transparent`, `decorations: false` | Fallback: dùng Tauri secondary window với `fullscreen: true` |
| **Screenshot capture** — Cần capture chính xác monitor chứa con trỏ | Sai monitor = sai screenshot | Dùng `xcap` crate (Rust), hỗ trợ multi-monitor trên Windows/macOS/Linux | Fallback: dùng `screenshots` crate hoặc Windows GDI API trực tiếp |

### 🟡 Medium Risk

| Risk | Impact | Mitigation | Fallback |
|---|---|---|---|
| **Canvas performance** — Render screenshot lớn (4K, multi-monitor) + annotations | Lag khi vẽ trên ảnh 4K | Dùng HTML Canvas 2D, lazy rendering, chỉ render viewport | Fallback: scale ảnh xuống 2K cho annotation, giữ original cho export |
| **SQLite concurrent access** — Nếu có nhiều overlay window mở cùng lúc | Data corruption | Dùng WAL mode, single writer pattern | MVP: chỉ cho phép 1 overlay tại 1 thời điểm |
| **Tauri IPC performance** — Truyền screenshot (base64) từ Rust sang Vue | Chậm với ảnh lớn | Lưu ảnh xuống disk, chỉ truyền path qua IPC | Dùng Tauri asset protocol để serve local file |
| **Google OAuth2 cho desktop app** — Flow phức tạp hơn web app | User phải mở browser để auth | Dùng loopback redirect (`http://localhost:PORT`) | Fallback: manual token paste (không lý tưởng nhưng hoạt động) |

### 🟢 Low Risk

| Risk | Impact | Mitigation |
|---|---|---|
| **Vuetify 3 component coverage** | Thiếu component cụ thể | Vuetify 3 đã stable, đủ components cho dashboard |
| **SQLite migration** | Schema thay đổi giữa versions | Dùng migration system đơn giản (version number + SQL scripts) |
| **File size management** | Screenshots chiếm nhiều disk | Compress PNG, cleanup tool, configurable retention |

---

## 9. Acceptance Criteria cho MVP

### AC-01: System Tray
- [ ] App chạy ở system tray khi minimize.
- [ ] Icon hiển thị trạng thái: idle / recording session.
- [ ] Right-click tray icon → menu: Open Dashboard, New Session, Settings, Quit.
- [ ] Double-click tray icon → mở Dashboard.

### AC-02: Overlay Trigger
- [ ] Giữ chuột giữa ≥ 1 giây ở bất kỳ đâu trên desktop → overlay bật.
- [ ] Giữ < 1 giây → không kích hoạt (tránh conflict với middle-click scroll).
- [ ] Visual feedback trong lúc giữ (progress indicator hoặc cursor change).
- [ ] Overlay tự tắt khi nhấn Esc hoặc Done.

### AC-03: Screenshot
- [ ] Screenshot capture chính xác monitor chứa con trỏ.
- [ ] Ảnh lưu dưới dạng PNG.
- [ ] Resolution = native resolution của monitor.
- [ ] Thời gian capture < 500ms.

### AC-04: Annotation Tools
- [ ] Number marker: click → marker ① với số tự tăng.
- [ ] Rectangle: drag → vẽ rectangle, có thể chọn color.
- [ ] Arrow: drag → vẽ arrow từ điểm A đến B.
- [ ] Mỗi annotation tạo ra 1 issue entry.

### AC-05: Issue Form
- [ ] Title bắt buộc.
- [ ] Type dropdown: Bug, UI, UX, Suggestion, Requirement, Question.
- [ ] Severity dropdown: Critical, Major, Minor, Info.
- [ ] Status default = Open.
- [ ] Tags: nhập tự do, autocomplete từ tags cũ.
- [ ] Save thành công → marker xác nhận trên overlay.

### AC-06: Data Persistence
- [ ] Tất cả sessions, issues, screenshots lưu local.
- [ ] App restart → data vẫn còn nguyên.
- [ ] Delete issue → file crop cũng bị xóa.
- [ ] Delete session → tất cả issues + screenshots của session bị xóa.

### AC-07: Dashboard
- [ ] Hiển thị danh sách sessions, sorted by date.
- [ ] Expand session → xem issues.
- [ ] Click issue → xem detail + screenshot + annotations.
- [ ] Edit issue inline.
- [ ] Filter theo: project, type, severity, status, date range.
- [ ] Search full-text theo title/description.

### AC-08: Export HTML
- [ ] Chọn session → Export HTML.
- [ ] File HTML self-contained (inline CSS + images base64).
- [ ] Mở được trên bất kỳ browser nào.
- [ ] Hiển thị: session info, danh sách issues với screenshot, severity badge, annotations.

### AC-09: Performance
- [ ] Overlay mở trong < 2 giây (bao gồm screenshot).
- [ ] Annotation rendering smooth (≥ 30fps khi vẽ).
- [ ] Dashboard load < 1 giây với 100 sessions.
- [ ] App memory < 200MB idle, < 500MB khi overlay active.

### AC-10: Stability
- [ ] Không crash khi capture 4K screenshot.
- [ ] Không crash khi có > 20 markers trên 1 screenshot.
- [ ] Graceful handling khi disk full.
- [ ] Graceful handling khi SQLite locked.

---

## 10. Development Milestones

```
M1: "It Runs"
  - Tauri app khởi động
  - System tray icon hiển thị
  - Vue + Vuetify dashboard skeleton
  - SQLite database tạo thành công
  → Checkpoint: demo app chạy được

M2: "It Captures"
  - Global mouse hook hoạt động
  - Middle mouse hold 1s detected
  - Screenshot capture thành công
  - Overlay window hiển thị screenshot
  → Checkpoint: giữ chuột giữa → thấy overlay

M3: "It Marks"
  - Number marker tool hoạt động
  - Rectangle tool hoạt động
  - Arrow tool hoạt động
  - Issue form hiển thị và save được
  → Checkpoint: mark 3 issues trên 1 screenshot

M4: "It Remembers"
  - Data lưu vào SQLite
  - Dashboard hiển thị sessions/issues
  - Issue detail view
  - Filter/search hoạt động
  → Checkpoint: review session hôm qua trên dashboard

M5: "It Reports"
  - Export HTML hoạt động
  - Export Markdown hoạt động
  - Report format đẹp, professional
  → Checkpoint: gửi report HTML qua email

M6: "It Ships"
  - Installer tạo thành công
  - Error handling toàn diện
  - Settings page
  - README + docs
  → Checkpoint: cài trên máy mới, chạy full flow
```

---

## 11. Testing Strategy

### Unit Tests
- **Rust backend:**
  - SQLite CRUD operations
  - Screenshot capture (mock)
  - File management (save/delete/cleanup)
  - Mouse hook event processing
  - Data serialization/deserialization
- **Vue frontend:**
  - Component rendering (Vitest + Vue Test Utils)
  - Store logic (Pinia)
  - Utility functions (date formatting, filter logic)

### Integration Tests
- Tauri IPC: gọi command từ Vue → Rust xử lý → trả kết quả.
- Database: tạo session → thêm issues → query → delete → verify.
- File system: capture screenshot → save → read → delete.

### E2E Tests (Manual cho MVP)
- Full flow: trigger overlay → mark issues → save → view dashboard → export.
- Các edge cases:
  - 0 issues trong session
  - 50 issues trong 1 screenshot
  - 4K monitor
  - Dual monitor
  - Disk gần full
  - App restart giữa session

### Performance Tests
- Screenshot capture time trên các resolution.
- Canvas rendering FPS với nhiều annotations.
- Dashboard load time với 1000 issues.
- Memory usage profile.

### Compatibility Tests
- Windows 10
- Windows 11
- HiDPI / scaling 125%, 150%, 200%
- Multi-monitor: same resolution, different resolution

---

## 12. Release Checklist

### Pre-Release
- [ ] Tất cả acceptance criteria passed
- [ ] Không có known critical/major bugs
- [ ] Performance benchmarks đạt target
- [ ] Error handling tested (no unhandled panics/exceptions)
- [ ] README.md viết xong
- [ ] CHANGELOG.md cho version này
- [ ] LICENSE file
- [ ] App icon finalized
- [ ] Installer tested (clean install + upgrade)
- [ ] Uninstaller tested (clean removal)

### Build & Package
- [ ] `cargo build --release` thành công
- [ ] `npm run tauri build` thành công
- [ ] Installer file (`.msi` hoặc `.exe`) generated
- [ ] File size hợp lý (< 50MB installer)
- [ ] Code signing (nếu có certificate — có thể bỏ qua cho MVP)

### Release
- [ ] Git tag `v0.1.0`
- [ ] GitHub Release created với changelog
- [ ] Installer uploaded lên GitHub Releases
- [ ] README badges updated (version, build status)

### Post-Release
- [ ] Cài thử trên máy mới (clean Windows)
- [ ] Full flow test trên máy mới
- [ ] Monitor crash reports (nếu có telemetry)
- [ ] Collect user feedback

---

## Appendix A: Glossary

| Term | Definition |
|---|---|
| **Session** | Một phiên review, chứa nhiều captures. Bắt đầu khi user tạo mới, kết thúc khi user đóng. |
| **Capture** | Một lần bật overlay = 1 screenshot + N issues. Một session có nhiều captures. |
| **Issue** | Một vấn đề được đánh dấu trên screenshot. Có marker, metadata, crop riêng. |
| **Marker** | Ký hiệu visual trên overlay (number, rectangle, arrow...). Mỗi marker = 1 issue. |
| **Overlay** | Cửa sổ transparent fullscreen phủ lên desktop, dùng để annotation. |
| **Report** | File export (HTML/MD/PDF/CSV) chứa thông tin session + issues. |
| **Share Package** | Bộ file (report + screenshots + metadata) để chia sẻ qua Google Drive. |

## Appendix B: Competitive Landscape

| Tool | Strengths | Weaknesses vs August Mark |
|---|---|---|
| **Snagit** | Powerful capture, GIF, video | Chỉ capture 1 ảnh/lần, không có session/issue management |
| **Greenshot** | Free, nhanh | Chỉ capture, không annotation workflow |
| **Figma Comments** | Collaborative, design-focused | Chỉ hoạt động trong Figma, không capture desktop |
| **Loom** | Video review | Không đánh dấu chính xác vị trí, khó track issue |
| **BugHerd** | Web bug tracking | Chỉ web, cần browser extension, SaaS pricing |
| **Marker.io** | Web annotation | Chỉ web, SaaS, không desktop |
| **UserSnap** | Feedback widget | Embed vào web, không desktop-wide |

**August Mark unique value:** Desktop-wide overlay + multi-issue per screenshot + session management + local-first + free.
