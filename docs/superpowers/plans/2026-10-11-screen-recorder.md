# Screen Recorder (Full Screen + Mic + Hotkeys) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement full-screen video recording with synchronized microphone audio, dedicated floating recording widget, comprehensive keyboard shortcuts (F9/F10/F8), and preview/save directly to computer.

**Architecture:** Frontend uses WebRTC `getDisplayMedia` + `getUserMedia(audio: true)` and `MediaRecorder` to capture full screen and mix microphone audio into a single stream. Rust backend handles Windows Global Shortcuts (via `tauri-plugin-global-shortcut`) and native file dialog saving/explorer reveal. A floating draggable widget provides HUD controls during recording, followed by a preview modal for playback and saving.

**Tech Stack:** Tauri 2, Rust, Vue 3, TypeScript, WebRTC (`getDisplayMedia`, `MediaRecorder`), Vuetify 3.

**Spec:** [docs/superpowers/specs/2026-10-11-screen-recorder-spec.md](file:///e:/August/August-Guide/docs/superpowers/specs/2026-10-11-screen-recorder-spec.md)

## Global Constraints

- Platform: Windows 10/11 (Tauri 2 + WebView2)
- Audio/Video Sync: WebRTC native mixed `MediaStream` (video track + audio track)
- Shortcut Scheme (Option 2):
  - **F9**: Start / Stop recording
  - **F10**: Pause / Resume recording
  - **F8**: Toggle Microphone on/off
  - **Esc**: Discard recording
  - **Ctrl+S**: Save file from Preview dialog
- Encoding: UTF-8 with proper Vietnamese accents
- Zero Redundant Checks: Follow user rules (no `arr && arr.length > 0`, use `?.`, use `??`)

---

### Task 1: Rust Backend Commands & Global Shortcut Setup

**Files:**
- Create: `src-tauri/src/commands/record_cmds.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src/services/tauriCommands.ts`

**Interfaces:**
- Produces:
  - `save_screen_recording(app: AppHandle, data: Vec<u8>, default_filename: Option<String>) -> AppResult<String>`
  - `open_in_explorer(file_path: String) -> AppResult<()>`
  - Tauri Events: `'record:toggle'`, `'record:pause_resume'`, `'record:toggle_mic'`

- [ ] **Step 1: Create `src-tauri/src/commands/record_cmds.rs`**
  - Implement `save_screen_recording` using native file dialog or file writing.
  - Implement `open_in_explorer` calling Windows `explorer.exe /select,"<path>"`.
  - Register global shortcut handlers for `F9`, `F10`, `F8` using `app.global_shortcut()`.
- [ ] **Step 2: Export command in `src-tauri/src/commands/mod.rs` and register in `lib.rs`**
  - Add `record_cmds` module.
  - Add commands to `tauri::generate_handler![..., save_screen_recording, open_in_explorer]`.
- [ ] **Step 3: Add TypeScript bindings in `src/services/tauriCommands.ts`**
  - `export async function saveScreenRecording(data: Uint8Array, defaultFilename?: string): Promise<string>`
  - `export async function openInExplorer(filePath: string): Promise<void>`
- [ ] **Step 4: Verify backend compilation**
  - Run `cargo check --manifest-path src-tauri/Cargo.toml`.

---

### Task 2: Core Media Recording Composable (`useScreenRecorder.ts`)

**Files:**
- Create: `src/composables/useScreenRecorder.ts`

**Interfaces:**
- Produces:
  - `isRecording: Ref<boolean>`
  - `isPaused: Ref<boolean>`
  - `isMicEnabled: Ref<boolean>`
  - `durationSeconds: Ref<number>`
  - `recordedBlob: Ref<Blob | null>`
  - `recordedVideoUrl: Ref<string | null>`
  - `startRecording(): Promise<void>`
  - `pauseRecording(): void`
  - `resumeRecording(): void`
  - `stopRecording(): Promise<void>`
  - `toggleMic(): void`
  - `discardRecording(): void`

- [ ] **Step 1: Implement `useScreenRecorder.ts`**
  - Request screen stream via `navigator.mediaDevices.getDisplayMedia({ video: { displaySurface: 'monitor' }, audio: false })`.
  - If mic enabled, request mic via `navigator.mediaDevices.getUserMedia({ audio: true })`.
  - Combine tracks into single `MediaStream`.
  - Initialize `MediaRecorder` with format detection (`video/webm;codecs=vp9,opus` or `video/mp4`).
  - Implement timer increment loop.
  - Implement `toggleMic()` toggling `audioTrack.enabled`.
  - Implement `stopRecording()` converting chunks to `Blob` and `URL.createObjectURL(blob)`.
- [ ] **Step 2: Verify TypeScript types with `npm run build`**

---

### Task 3: Floating Recording HUD Widget

**Files:**
- Create: `src/components/record/RecordingWidget.vue`
- Modify: `src/App.vue` (or root mount point)

**Interfaces:**
- Consumes: `useScreenRecorder.ts`
- Produces: Floating HUD bar on screen during active recording

- [ ] **Step 1: Build `RecordingWidget.vue` template & styling**
  - Compact rounded pill bar with subtle glassmorphism border.
  - Flashing red circle + `REC` badge.
  - Real-time formatted timer `MM:SS`.
  - Pause / Resume button with tooltip `F10`.
  - Microphone toggle button with tooltip `F8` (shows icon mic or mic-off).
  - Stop & Save button with tooltip `F9`.
  - Discard button (`Esc`).
  - `data-tauri-drag-region` on background for repositioning.
- [ ] **Step 2: Bind global shortcut listeners & keyboard listeners**
  - Listen for `'record:toggle'`, `'record:pause_resume'`, `'record:toggle_mic'`.
  - Listen for window `keydown` events (`F9`, `F10`, `F8`, `Escape`).
- [ ] **Step 3: Mount widget into `App.vue` with `v-if="isRecording"`**

---

### Task 4: Video Preview & Save Dialog

**Files:**
- Create: `src/components/record/RecordPreviewDialog.vue`
- Modify: `src/App.vue`

**Interfaces:**
- Consumes: `recordedVideoUrl`, `recordedBlob`, `saveScreenRecording`, `openInExplorer`

- [ ] **Step 1: Build `RecordPreviewDialog.vue`**
  - HTML5 video player with standard controls.
  - Video metadata display: duration, size in MB.
  - Button "Lưu video ra máy tính" (Hotkey `Ctrl+S`).
  - Button "Mở thư mục lưu file" (active after save).
  - Button "Xóa / Hủy bỏ".
- [ ] **Step 2: Implement save flow**
  - Read `Blob` as `ArrayBuffer` -> `Uint8Array`.
  - Call `saveScreenRecording(bytes, `AugustGuide_Record_${timestamp}.mp4`)`.
  - Display toast notification with saved path.
  - Offer 1-click button to open folder via `openInExplorer(savedPath)`.

---

### Task 5: Integration, Header Button & Settings

**Files:**
- Modify: `src/components/common/AppHeader.vue`
- Modify: `src/views/SettingsView.vue`
- Modify: `src/composables/useI18n.ts`

**Interfaces:**
- Produces: Top-bar "Quay màn hình (F9)" button, Recording settings configuration.

- [ ] **Step 1: Add "Quay màn hình" button to `AppHeader.vue`**
  - Icon `mdi-record-rec` with label `Quay video (F9)`.
  - Clicking triggers `startRecording()`.
- [ ] **Step 2: Add Recording settings in `SettingsView.vue`**
  - Option to set default mic state (Bật / Tắt mặc định).
  - Display hotkey summary table (`F9`, `F10`, `F8`, `Ctrl+S`).
- [ ] **Step 3: Add Vietnamese and English translations in `useI18n.ts`**
- [ ] **Step 4: End-to-end verification**
  - Run `cargo check` and `npm run build`.
  - Verify zero console errors and clean bundle size.
