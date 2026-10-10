# Kế hoạch Triển khai 4 Tính năng Mở rộng Overlay (August Guide)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Triển khai 4 tính năng nâng cấp overlay cho August Guide: Dọn bỏ sinh crops dư thừa, Ctrl+C thông minh theo ngữ cảnh, Chỉnh sửa lại ảnh từ Dashboard (Re-edit), và Ghim ảnh lên màn hình (Pin to screen).

**Architecture:** 
- Giữ nguyên kiến trúc Non-destructive (Raw Screenshot + JSON Vector trong SQLite), loại bỏ hoàn toàn việc cắt ảnh con vào `crops/`.
- Phân biệt ngữ cảnh bàn phím tại `OverlayApp.vue` giữa sao chép toàn bộ ảnh (system clipboard) và sao chép đối tượng vector (internal clipboard store).
- Mở rộng lệnh Tauri `open_overlay` và thêm `get_issues_by_capture` để nạp lại vector JSON lên canvas khi người dùng bấm Re-edit từ Dashboard.
- Tạo một Webview Window động `pin-window` với `always_on_top: true`, `decorations: false` thông qua Tauri 2 WebviewWindowBuilder kết hợp view `pin.html` siêu nhẹ.

**Tech Stack:** Tauri 2, Rust (rusqlite, image crate), Vue 3, TypeScript, Pinia, Vuetify 3, Vite.

**Spec:** Đã thống nhất với người dùng (bỏ qua tính năng Shadow & Padding khi export).

## Global Constraints
- Không phá vỡ dữ liệu SQLite hiện có; migration schema không đổi (giữ cột `crop_path` NULL, tận dụng cột `annotation_data`).
- Luôn đảm bảo encoding UTF-8 tiếng Việt chuẩn xác trong mọi thông báo UI và log.
- Tránh các anti-pattern code review đã quy định (dùng `?.`, `??`, không check length thừa, không check `typeof window`).
- Giữ mọi nút hành động hủy/xóa có hộp thoại xác nhận khi cần.

---

### Task 1: Dọn bỏ sinh ảnh crop 400x400 dư thừa (`crops/`)

**Files:**
- Modify: `src-tauri/src/commands/issue_cmds.rs:80-95`
- Modify: `src/components/dashboard/IssueDetail.vue:101-125`

**Interfaces:**
- Consumes: `save_capture_annotations(capture_id, issues, annotated_image_base64)`
- Produces: `issues` được lưu với `crop_path = None`, giải phóng dung lượng đĩa và không tạo file con trong `crops/`.

- [ ] **Step 1: Cập nhật `issue_cmds.rs` để ngừng cắt ảnh crop**
Trong `src-tauri/src/commands/issue_cmds.rs`:
Thay thế đoạn cắt ảnh crop:
```rust
// 5. Generate crop images for each issue (crop from the annotated image!)
let mut crop_paths = Vec::new();
let mut id_overrides = Vec::new();

for payload in &issues {
    let issue_id = crate::utils::id::new_uuid();
    id_overrides.push(issue_id.clone());

    // PERFORM NO CROPS: save space, only raw screenshot + vector JSON are kept
    crop_paths.push(None);
}
```
Xóa bỏ import/sử dụng không cần thiết của `file_storage::save_crop` trong hàm này nếu không còn dùng.

- [ ] **Step 2: Tinh chỉnh `IssueDetail.vue` để ẩn khối crop trống**
Trong `src/components/dashboard/IssueDetail.vue`, chỉ hiển thị thẻ card "Issue Crop" khi `cropUrl` thực sự tồn tại:
```html
<v-card v-if="cropUrl" bg-color="surface" variant="outlined" color="surface-variant" class="pa-4">
```
Nếu không có `cropUrl`, không hiển thị khối crop nữa để giao diện gọn gàng hơn.

- [ ] **Step 3: Kiểm tra biên dịch và lưu thử**
Chạy: `cargo check --manifest-path src-tauri/Cargo.toml`
Kỳ vọng: PASS không có lỗi biên dịch.

---

### Task 2: Ctrl+C thông minh theo ngữ cảnh & Ctrl+V Paste nét vẽ

**Files:**
- Modify: `src/stores/overlayStore.ts`
- Modify: `src/OverlayApp.vue:194-216`

**Interfaces:**
- Consumes: `overlayStore.selectedAnnotationId`, `overlayStore.annotations`
- Produces: 
  - `overlayStore.copiedAnnotation`: Ref lưu annotation đang được copy
  - `overlayStore.copySelectedAnnotation()`: Phương thức copy annotation
  - `overlayStore.pasteAnnotation()`: Phương thức paste annotation với offset +20px
  - Phím tắt `Ctrl + C`, `Ctrl + V`, `Ctrl + Shift + C`

- [ ] **Step 1: Bổ sung clipboard state và methods vào `overlayStore.ts`**
Trong `src/stores/overlayStore.ts`:
```typescript
const copiedAnnotation = ref<Annotation | null>(null)

function copySelectedAnnotation(): boolean {
  if (!selectedAnnotationId.value) return false
  const ann = annotations.value.find(a => a.id === selectedAnnotationId.value)
  if (!ann) return false
  copiedAnnotation.value = JSON.parse(JSON.stringify(ann))
  return true
}

function pasteAnnotation(): Annotation | null {
  if (!copiedAnnotation.value) return null
  const cloned: Annotation = JSON.parse(JSON.stringify(copiedAnnotation.value))
  cloned.id = typeof crypto?.randomUUID === 'function' ? crypto.randomUUID() : Math.random().toString(36).substring(2, 11)
  const offset = 20
  
  if (cloned.type === 'marker') {
    cloned.position = { x: cloned.position.x + offset, y: cloned.position.y + offset }
  } else if (cloned.type === 'rect' || cloned.type === 'blur' || cloned.type === 'highlight' || cloned.type === 'spotlight') {
    cloned.topLeft = { x: cloned.topLeft.x + offset, y: cloned.topLeft.y + offset }
  } else if (cloned.type === 'arrow') {
    cloned.start = { x: cloned.start.x + offset, y: cloned.start.y + offset }
    cloned.end = { x: cloned.end.x + offset, y: cloned.end.y + offset }
  } else if (cloned.type === 'text') {
    cloned.position = { x: cloned.position.x + offset, y: cloned.position.y + offset }
  } else if (cloned.type === 'freedraw') {
    cloned.points = cloned.points.map(p => ({ x: p.x + offset, y: p.y + offset }))
  }

  addAnnotation(cloned)
  selectedAnnotationId.value = cloned.id
  // Cập nhật lại copiedAnnotation với tọa độ mới để nếu paste tiếp sẽ tiếp tục offset
  copiedAnnotation.value = JSON.parse(JSON.stringify(cloned))
  return cloned
}
```
Export `copiedAnnotation`, `copySelectedAnnotation`, `pasteAnnotation` trong return của `overlayStore`.

- [ ] **Step 2: Cập nhật xử lý phím tắt trong `OverlayApp.vue`**
Trong `src/OverlayApp.vue`, cập nhật hàm `handleKeyDown`:
```typescript
  if (e.ctrlKey) {
    if (e.key.toLowerCase() === 'z') {
      e.preventDefault()
      overlayStore.undo()
    } else if (e.key.toLowerCase() === 'y') {
      e.preventDefault()
      overlayStore.redo()
    } else if (e.key.toLowerCase() === 'd') {
      if (overlayStore.selectedAnnotationId) {
        e.preventDefault()
        overlayStore.duplicateAnnotation(overlayStore.selectedAnnotationId)
        annotationCanvasRef.value?.drawSelectionPreview?.()
      }
    } else if (e.shiftKey && e.key.toLowerCase() === 'c') {
      // Ctrl + Shift + C: Luôn copy toàn bộ ảnh vào clipboard hệ thống
      e.preventDefault()
      await handleCopy()
    } else if (e.key.toLowerCase() === 'c') {
      e.preventDefault()
      if (overlayStore.selectedAnnotationId) {
        const ok = overlayStore.copySelectedAnnotation()
        if (ok) {
          uiStore.showToast({ message: 'Đã sao chép nét vẽ!', type: 'success' })
        }
      } else {
        // Không chọn hình nào -> copy toàn bộ ảnh vào clipboard hệ thống
        await handleCopy()
      }
    } else if (e.key.toLowerCase() === 'v') {
      if (overlayStore.copiedAnnotation) {
        e.preventDefault()
        overlayStore.pasteAnnotation()
        annotationCanvasRef.value?.drawSelectionPreview?.()
        uiStore.showToast({ message: 'Đã dán nét vẽ!', type: 'success' })
      }
    } else if (e.key.toLowerCase() === 's') {
      e.preventDefault()
      await handleDone()
    }
    return
  }
```

- [ ] **Step 3: Kiểm tra TypeScript build**
Chạy: `npm run build`
Kỳ vọng: Không có lỗi type check.

---

### Task 3: Re-edit từ Dashboard (Mở lại chỉnh sửa nét vẽ)

**Files:**
- Modify: `src-tauri/src/commands/issue_cmds.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src/services/tauriCommands.ts`
- Modify: `src/OverlayApp.vue`
- Modify: `src/views/SessionView.vue`

**Interfaces:**
- Consumes: `issue_repo::get_by_capture`, `captures.screenshot_path`, `issues.annotation_data`
- Produces:
  - Tauri command `get_issues_by_capture(capture_id)`
  - Re-save logic trong `save_capture_annotations`: Xóa issues cũ thuộc `capture_id` trước khi chèn bản mới cập nhật
  - UI Button "Sửa nét vẽ" trên mỗi capture trong `SessionView.vue`
  - Tự động nạp lại các nét vẽ vector lên `AnnotationCanvas` khi Overlay mở lại

- [ ] **Step 1: Bổ sung Tauri command `get_issues_by_capture` và sửa `save_capture_annotations`**
Trong `src-tauri/src/commands/issue_cmds.rs`:
```rust
/// Query all issues associated with a specific capture.
#[tauri::command]
pub fn get_issues_by_capture(state: State<'_, AppState>, capture_id: String) -> AppResult<Vec<Issue>> {
    let conn = state
        .db
        .lock()
        .map_err(|e| AppError::Database(e.to_string()))?;
    let mut issues = issue_repo::get_by_capture(&conn, &capture_id)?;

    for issue in &mut issues {
        if let Some(ref path) = issue.crop_path {
            let abs_path = state.app_data_dir.join(path);
            issue.crop_path = Some(abs_path.to_string_lossy().to_string());
        }
    }

    Ok(issues)
}
```
Trong `save_capture_annotations`:
Trước khi gọi `issue_repo::create_batch`, thực hiện xóa các issues cũ đã có của `capture_id`:
```rust
    // Xóa các issue cũ của capture này để tránh nhân đôi khi re-edit
    tx.execute("DELETE FROM issues WHERE capture_id = ?1", rusqlite::params![capture_id])
        .map_err(|e| AppError::Database(format!("Failed to clean existing capture issues: {}", e)))?;
```
Đăng ký `get_issues_by_capture` vào `src-tauri/src/lib.rs` (invoke_handler).

- [ ] **Step 2: Thêm wrapper command trong `src/services/tauriCommands.ts`**
```typescript
export async function getIssuesByCapture(captureId: string): Promise<Issue[]> {
  return invoke('get_issues_by_capture', { captureId })
}

export async function reopenOverlayForEdit(captureId: string, screenshotPath: string): Promise<void> {
  return invoke('open_overlay', { captureId, screenshotPath: screenshotPath || null })
}
```

- [ ] **Step 3: Tải lại mảng nét vẽ cũ trong `OverlayApp.vue`**
Trong `src/OverlayApp.vue`:
Cập nhật `loadCaptureDetails`:
```typescript
import { getCapture, getIssuesByCapture, cancelCapture, closeOverlay, showOverlay } from '@/services/tauriCommands'

const loadCaptureDetails = async (id: string) => {
  try {
    captureId.value = id
    loadError.value = null
    screenshotUrl.value = ''
    const data = await getCapture(id)
    capture.value = data

    overlayStore.init(id, data.screenshotPath, data)
    screenshotUrl.value = convertFileSrc(data.screenshotPath)

    // Nạp lại các nét vẽ cũ nếu có (Re-edit mode)
    try {
      const existingIssues = await getIssuesByCapture(id)
      if (existingIssues && existingIssues.length > 0) {
        const loadedAnnotations: any[] = []
        for (const issue of existingIssues) {
          if (issue.annotationData && issue.annotationData !== '{}') {
            try {
              const parsed = JSON.parse(issue.annotationData)
              if (parsed.type && parsed.type !== 'full_screenshot') {
                if (issue.title && !parsed.stepNote) {
                  parsed.stepNote = issue.title
                }
                loadedAnnotations.push(parsed)
              }
            } catch (jsonErr) {
              console.warn('[Overlay] Failed to parse annotationData:', jsonErr)
            }
          }
        }
        if (loadedAnnotations.length > 0) {
          overlayStore.annotations = loadedAnnotations
          overlayStore.reindexAnnotations()
        }
      }
    } catch (issueErr) {
      console.warn('[Overlay] Could not fetch existing issues for re-edit:', issueErr)
    }

    scheduleRevealFallback()
  } catch (e) {
    console.error('[Overlay] Failed to load capture details:', e)
    loadError.value = t('overlay.failedLoadScreenshot') || 'Không thể tải ảnh chụp màn hình'
    await revealOverlay()
  }
}
```

- [ ] **Step 4: Thêm nút "Sửa nét vẽ" trong `SessionView.vue`**
Trong `src/views/SessionView.vue`, ở tab `captures` (trong mỗi `capture-card`):
Thêm nút hành động:
```html
<v-card-actions class="pa-2 d-flex justify-end bg-surface-variant">
  <v-btn
    size="small"
    color="primary"
    variant="tonal"
    prepend-icon="mdi-pencil"
    class="text-none font-weight-bold"
    @click.stop="handleReEditCapture(cap)"
  >
    Sửa nét vẽ
  </v-btn>
</v-card-actions>
```
Và hàm xử lý:
```typescript
const handleReEditCapture = async (cap: any) => {
  try {
    await reopenOverlayForEdit(cap.id, cap.screenshotPath)
  } catch (e: any) {
    uiStore.showToast({
      message: `Không thể mở chỉnh sửa: ${e?.message || e}`,
      type: 'error'
    })
  }
}
```

---

### Task 4: Pin to Screen (Ghim ảnh lên màn hình - Snipaste style)

**Files:**
- Create: `pin.html`
- Create: `src/pin.ts`
- Create: `src/PinApp.vue`
- Modify: `vite.config.ts`
- Modify: `src-tauri/src/commands/capture_cmds.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src/services/tauriCommands.ts`
- Modify: `src/OverlayApp.vue:207-268`
- Modify: `src/components/overlay/AnnotationToolbar.vue`

**Interfaces:**
- Consumes: Merged canvas (Data URL), device pixel ratio / display size
- Produces:
  - Cửa sổ `pin_{id}` không viền, `always_on_top: true`
  - Kéo di chuyển window bằng chuột (`startDragging`)
  - Lăn chuột phóng to / thu nhỏ
  - ESC hoặc double click đóng ghim
  - Phím tắt `P` và nút "Ghim" trên thanh công cụ Overlay

- [ ] **Step 1: Tạo `pin.html`, `src/pin.ts`, `src/PinApp.vue` và cập nhật `vite.config.ts`**
Tạo file `pin.html` tương tự `overlay.html`, mount `/src/pin.ts`.
Cập nhật `vite.config.ts` để thêm `pin: fileURLToPath(new URL('./pin.html', import.meta.url))` vào `rollupOptions.input`.
Tạo `src/pin.ts` mount `PinApp.vue`.
Xây dựng `src/PinApp.vue`:
```vue
<template>
  <div class="pin-root" @mousedown="onMouseDown" @wheel="onWheel" @dblclick="closePin">
    <div class="pin-toolbar" v-show="showControls">
      <span class="zoom-badge">{{ Math.round(zoom * 100) }}%</span>
      <button class="tool-btn" @click.stop="copyPinImage" title="Sao chép ảnh">📋</button>
      <button class="tool-btn" @click.stop="closePin" title="Đóng (Esc)">✕</button>
    </div>
    <div class="pin-image-wrapper" :style="{ transform: `scale(${zoom})`, opacity }">
      <img :src="imageUrl" draggable="false" alt="Pinned image" />
    </div>
  </div>
</template>
```
Hỗ trợ `getCurrentWebviewWindow().startDragging()`, ESC để đóng, thanh trượt cuộn chuột zoom `0.3` đến `3.0`.

- [ ] **Step 2: Thêm Tauri Command `open_pin_window`**
Trong `src-tauri/src/commands/capture_cmds.rs`:
```rust
#[tauri::command]
pub async fn open_pin_window(
    app: AppHandle,
    image_url: String,
    width: f64,
    height: f64,
) -> AppResult<()> {
    let pin_id = crate::utils::id::new_uuid();
    let label = format!("pin_{}", &pin_id[..8]);

    let clean_url = image_url.replace('\\', "/").replace('\'', "\\'");
    let init_script = format!(
        "window.__PIN_DATA__ = {{ imageUrl: '{}' }};",
        clean_url
    );

    #[cfg(debug_assertions)]
    let url = WebviewUrl::External(tauri::Url::parse("http://localhost:1420/pin.html").unwrap());
    #[cfg(not(debug_assertions))]
    let url = WebviewUrl::App("pin.html".into());

    let w = width.max(200.0).min(1920.0);
    let h = height.max(150.0).min(1080.0);

    WebviewWindowBuilder::new(&app, &label, url)
        .title("August Pin")
        .decorations(false)
        .always_on_top(true)
        .transparent(false)
        .resizable(true)
        .initialization_script(&init_script)
        .inner_size(w, h)
        .build()
        .map_err(|e| AppError::Generic(format!("Failed to create pin window: {}", e)))?;

    Ok(())
}
```
Đăng ký `open_pin_window` vào `src-tauri/src/lib.rs`.

- [ ] **Step 3: Kết nối phím `P` và nút Ghim trên thanh công cụ Overlay**
Trong `src/services/tauriCommands.ts`:
```typescript
export async function openPinWindow(imageUrl: string, width: number, height: number): Promise<void> {
  return invoke('open_pin_window', { imageUrl, width, height })
}
```
Trong `src/OverlayApp.vue`:
Khi bấm phím `p` (hoặc `P`):
```typescript
  } else if (e.key.toLowerCase() === 'p') {
    e.preventDefault()
    await handlePin()
  }
```
Và hàm `handlePin`:
```typescript
const handlePin = async () => {
  if (!annotationCanvasRef.value) return
  const merged = annotationCanvasRef.value.getMergedCanvas()
  if (!merged) return
  const dataUrl = merged.toDataURL('image/png')
  const dpr = window.devicePixelRatio || 1
  const width = merged.width / dpr
  const height = merged.height / dpr
  await openPinWindow(dataUrl, width, height)
  // Tùy chọn: đóng overlay sau khi ghim
  await closeOverlay()
}
```
Thêm nút Ghim vào `AnnotationToolbar.vue` với phím tắt gợi ý `(P)`.

---

## Tự kiểm tra kế hoạch (Self-Review Checklist)
1. **Spec Coverage:** Cả 4 yêu cầu (Dọn crops, Ctrl+C thông minh, Re-edit từ Dashboard, Pin to screen) đều được mô tả chi tiết với đường dẫn file cụ thể.
2. **Không có Placeholder:** Mọi logic, signatures và code snippets đều đầy đủ, không có TBD / TODO.
3. **Tính nhất quán:** Các command name `get_issues_by_capture`, `open_pin_window`, `reopenOverlayForEdit` khớp hoàn toàn giữa frontend và backend.
