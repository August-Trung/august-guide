# CODE_ORDER.md — August Mark v0.1

> Thứ tự code tối ưu cho solo developer. Mỗi bước build trên bước trước. Không skip.

---

## Coding Sequence

| # | Step | Rust files | Vue files | Tại sao thứ tự này |
|---|---|---|---|---|
| **1** | Project scaffold | `Cargo.toml`, `main.rs`, `tauri.conf.json` | `package.json`, `main.ts`, `App.vue`, `vite.config.ts` | Phải có app chạy được trước khi code bất kỳ thứ gì |
| **2** | Vuetify + Pinia + Router | — | `plugins/vuetify.ts`, `plugins/pinia.ts`, `router/index.ts` | UI framework phải setup trước khi viết component |
| **3** | Shared TypeScript types | — | `types/project.ts`, `types/session.ts`, `types/capture.ts`, `types/issue.ts`, `types/annotation.ts` | Types định nghĩa trước, tất cả layers tham chiếu |
| **4** | Rust error types + utils | `error.rs`, `utils/paths.rs`, `utils/id.rs` | — | Foundation cho mọi Rust code phía sau |
| **5** | Rust domain models | `models/project.rs`, `models/session.rs`, `models/capture.rs`, `models/issue.rs` | — | Structs cần có trước repos và commands |
| **6** | SQLite connection + migration | `db/connection.rs`, `db/migrations.rs`, `migrations/v001_initial.sql` | — | Database phải chạy trước khi CRUD |
| **7** | AppState + Tauri setup | `state.rs`, update `main.rs` | — | State phải init trước khi register commands |
| **8** | Project repo + commands | `db/project_repo.rs`, `commands/project_cmds.rs` | — | CRUD đơn giản nhất, test IPC round-trip |
| **9** | Tauri IPC bridge (Vue) | — | `services/tauriCommands.ts` (project section) | Frontend cần gọi được backend |
| **10** | Project store + UI | — | `stores/projectStore.ts`, `components/common/AppSidebar.vue`, `components/dashboard/ProjectSelector.vue` | Có UI → test được visually |
| **11** | Session repo + commands | `db/session_repo.rs`, `commands/session_cmds.rs` | `services/tauriCommands.ts` (session section) | Session phụ thuộc Project |
| **12** | Session store + UI | — | `stores/sessionStore.ts`, `components/dashboard/SessionList.vue`, `components/dashboard/SessionCard.vue` | Dashboard bắt đầu có data |
| **13** | Dashboard layout | — | `views/DashboardView.vue`, `components/common/AppHeader.vue`, `components/common/EmptyState.vue` | Layout tổng, kết nối sidebar + session list |
| **14** | System tray | update `main.rs` | — | App chạy nền — nền tảng cho overlay trigger |
| — | **⬆ CHECKPOINT: App CRUD chạy end-to-end** | | | |
| **15** | Screenshot service | `services/screen_capture.rs`, `services/file_storage.rs` | — | Phải chụp được trước khi mở overlay |
| **16** | Capture repo + commands | `db/capture_repo.rs`, `commands/capture_cmds.rs` | — | Lưu metadata screenshot vào DB |
| **17** | Overlay window (Tauri) | update `commands/capture_cmds.rs` (open/close overlay) | `public/overlay.html`, `overlay.ts`, `OverlayApp.vue` | Window phải mở được trước khi vẽ canvas |
| **18** | Global hotkey trigger | update `main.rs` (register global shortcut) | `services/tauriEvents.ts`, `composables/useTauriEvents.ts` | Trigger → capture → overlay — full pipeline |
| — | **⬆ CHECKPOINT: Hotkey → screenshot → overlay hiển thị** | | | |
| **19** | Canvas rendering | — | `composables/useCanvas.ts`, `components/overlay/AnnotationCanvas.vue` | Screenshot render lên canvas, mouse events ready |
| **20** | Overlay toolbar | — | `components/overlay/AnnotationToolbar.vue`, `components/overlay/OverlayStatusBar.vue` | UI controls cho tools |
| **21** | Overlay store | — | `stores/overlayStore.ts` | State cho annotations, current tool, issue list |
| **22** | Annotation composable | — | `composables/useAnnotation.ts` | Tool state machine: select → draw → commit |
| **23** | Number marker tool | — | update `useAnnotation.ts`, `useCanvas.ts` | Tool đơn giản nhất — click → marker |
| **24** | Rectangle tool | — | update `useAnnotation.ts`, `useCanvas.ts` | Tool phổ biến nhất — drag → rect |
| **25** | Issue form panel | — | `components/overlay/IssueFormPanel.vue` | Nhập metadata cho marker vừa vẽ |
| **26** | Arrow tool | — | update `useAnnotation.ts`, `useCanvas.ts` | Tương tự rect, thêm arrowhead |
| **27** | Text note tool | — | update `useAnnotation.ts`, `useCanvas.ts` | Input text trên canvas |
| — | **⬆ CHECKPOINT: Mark nhiều issues trên 1 screenshot** | | | |
| **28** | Issue repo + commands | `db/issue_repo.rs`, `commands/issue_cmds.rs` | `services/tauriCommands.ts` (issue section) | Save annotation data vào DB |
| **29** | Save flow (overlay → DB) | update `commands/issue_cmds.rs` | update `overlayStore.ts` (saveAndClose) | Done → persist tất cả |
| **30** | Image processor (crop) | `services/image_processor.rs` | — | Crop vùng quanh mỗi marker |
| **31** | Annotated screenshot save | update `commands/issue_cmds.rs` | — | Save ảnh tổng có annotations burned in |
| — | **⬆ CHECKPOINT: Data persist end-to-end** | | | |
| **32** | Issue store | — | `stores/issueStore.ts` | Query issues cho dashboard |
| **33** | Issue list + card UI | — | `components/dashboard/IssueList.vue`, `components/dashboard/IssueCard.vue` | Xem issues trong session |
| **34** | Issue detail view | — | `components/dashboard/IssueDetail.vue`, `views/IssueView.vue` | Xem full screenshot + marker + metadata |
| **35** | Session detail view | — | `views/SessionView.vue` | Xem session detail + captures + issues |
| **36** | Edit issue | update `commands/issue_cmds.rs` | update `IssueDetail.vue` | Sửa title, type, severity, status |
| **37** | Delete issue + session | update `commands/issue_cmds.rs`, `session_cmds.rs` | `components/common/ConfirmDialog.vue` | Dọn dẹp data |
| **38** | Filter bar | — | `components/dashboard/FilterBar.vue` | Filter by type, severity, status |
| — | **⬆ CHECKPOINT: Dashboard fully functional** | | | |
| **39** | Export HTML engine | `services/export_html.rs` | — | Template rendering + base64 images |
| **40** | Export command + dialog | `commands/export_cmds.rs` | `components/export/ExportDialog.vue` | UI → chọn session → save file |
| — | **⬆ CHECKPOINT: Export works, MVP feature-complete** | | | |
| **41** | UI store + toasts | — | `stores/uiStore.ts` | Loading states, error toasts, success notifications |
| **42** | Error handling polish | update all `commands/` | update all stores | Graceful errors thay vì crash |
| **43** | Utils | — | `utils/date.ts`, `utils/geometry.ts`, `utils/image.ts` | Helper functions refactor |
| — | **⬆ CHECKPOINT: MVP polish done** | | | |

---

## Tại Sao Thứ Tự Này Giảm Rủi Ro

### Nguyên tắc 1: Vertical slice trước, horizontal polish sau

Không code toàn bộ Rust rồi mới code Vue. Thay vào đó, code **từng tính năng xuyên suốt các layer** (Rust repo → Rust command → Vue service → Vue store → Vue component). Lý do:
- Phát hiện IPC issues sớm (serialization, type mismatch).
- Có UI test được ngay, không phải đợi "backend xong hết".
- Motivation: thấy app hoạt động mỗi 2-3 ngày.

### Nguyên tắc 2: Rủi ro kỹ thuật cao → code trước

| Thứ tự | Rủi ro | Vì sao code trước |
|---|---|---|
| #15 Screenshot | HIGH — xcap crate có thể không compile | Nếu lỗi, phát hiện ở Week 2 còn kịp tìm fallback |
| #17 Overlay window | HIGH — transparency/always-on-top có thể lỗi | Nếu lỗi, biết ngay, không code annotation rồi mới thấy |
| #18 Global hotkey | MEDIUM — conflict với app khác | Nếu lỗi, đổi hotkey combo, không mất nhiều thời gian |
| #23-27 Annotation | MEDIUM — canvas DPI, coordinate issues | Lỗi ở đây thì biết ngay khi test visual |

### Nguyên tắc 3: Data layer trước, UI layer sau

Mỗi feature: `models → repo → commands → tauriCommands → store → component`. Lý do:
- Data layer ổn định, ít thay đổi.
- UI có thể refactor nhiều lần mà không ảnh hưởng data.
- Bug data layer khó fix hơn, nên test sớm.

### Nguyên tắc 4: Checkpoint mỗi 5-8 steps

5 checkpoints xuyên suốt 43 steps. Mỗi checkpoint là một **app chạy được end-to-end** ở scope nhỏ hơn. Nếu bị trễ tiến độ, có thể ship tại bất kỳ checkpoint nào (trừ checkpoint đầu tiên).

### Nguyên tắc 5: Polish cuối cùng

Error handling, toasts, loading states, utils refactor — tất cả để cuối. Lý do:
- Chưa biết error pattern cho đến khi code xong feature.
- Over-engineer error handling sớm = waste time.
- "Make it work → Make it right" — đúng thứ tự.

---

## Quick Reference: Step → Week Mapping

| Step | Week | Milestone |
|---|---|---|
| 1-14 | Week 1 | CRUD + Dashboard skeleton |
| 15-18 | Week 2 | Screenshot + Overlay |
| 19-27 | Week 3 | Annotation tools |
| 28-38 | Week 4 | Persist + Dashboard full |
| 39-40 | Week 4-5 | Export HTML |
| 41-43 | Week 5 | Polish |
