<template>
  <div
    data-tauri-drag-region
    class="pin-container"
    @mousedown="handleMouseDown"
    @wheel="handleWheel"
    @dblclick="closePin"
  >
    <!-- 8-directional native resize handles -->
    <div class="resize-handle top" @mousedown.stop="startResize('North')"></div>
    <div class="resize-handle bottom" @mousedown.stop="startResize('South')"></div>
    <div class="resize-handle left" @mousedown.stop="startResize('West')"></div>
    <div class="resize-handle right" @mousedown.stop="startResize('East')"></div>
    <div class="resize-handle top-left" @mousedown.stop="startResize('NorthWest')"></div>
    <div class="resize-handle top-right" @mousedown.stop="startResize('NorthEast')"></div>
    <div class="resize-handle bottom-left" @mousedown.stop="startResize('SouthWest')"></div>
    <div class="resize-handle bottom-right" @mousedown.stop="startResize('SouthEast')"></div>

    <!-- Floating Toolbar (appears on hover) -->
    <div class="pin-toolbar" @mousedown.stop>
      <span class="pin-badge" :title="'Độ thu phóng: ' + zoomPercent + '%'">{{ zoomPercent }}%</span>
      <button
        class="pin-btn"
        @click.stop="resetZoom"
        title="Đặt lại kích thước gốc 100%"
      >
        <svg viewBox="0 0 24 24" width="14" height="14" fill="currentColor">
          <path d="M12 4V1L8 5l4 4V6c3.31 0 6 2.69 6 6 0 1.01-.25 1.97-.7 2.8l1.46 1.46A7.93 7.93 0 0 0 20 12c0-4.42-3.58-8-8-8zm0 14c-3.31 0-6-2.69-6-6 0-1.01.25-1.97.7-2.8L5.24 7.74A7.93 7.93 0 0 0 4 12c0 4.42 3.58 8 8 8v3l4-4-4-4v3z"/>
        </svg>
      </button>
      <button class="pin-btn" @click.stop="copyImage" title="Sao chép vào clipboard (Ctrl+C)">
        <svg viewBox="0 0 24 24" width="14" height="14" fill="currentColor">
          <path d="M19 21H8V7h11m0-2H8a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h11a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2m-3-4H4a2 2 0 0 0-2 2v14h2V3h12V1Z"/>
        </svg>
      </button>
      <button class="pin-btn close-btn" @click.stop="closePin" title="Đóng ghim (Esc / Double-click / Click chuột giữa)">
        <svg viewBox="0 0 24 24" width="14" height="14" fill="currentColor">
          <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12 19 6.41Z"/>
        </svg>
      </button>
    </div>

    <!-- Pinned Image Display -->
    <div data-tauri-drag-region class="image-stage">
      <img
        v-if="imageUrl"
        data-tauri-drag-region
        :src="imageUrl"
        alt="Pinned Screenshot"
        class="pinned-img"
        draggable="false"
        @load="onImageLoad"
      />
      <div v-else class="loading-box">Đang tải ảnh ghim...</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { LogicalSize } from '@tauri-apps/api/dpi'

type ResizeDirection =
  | 'East'
  | 'North'
  | 'NorthEast'
  | 'NorthWest'
  | 'South'
  | 'SouthEast'
  | 'SouthWest'
  | 'West'

const imageUrl = ref<string>('')
const zoomPercent = ref<number>(100)
const imageAspect = ref<number>(1.5)
const initialLogicalWidth = ref<number>(0)
const initialLogicalHeight = ref<number>(0)

const currentWindow = getCurrentWebviewWindow()
let unlistenResized: (() => void) | null = null

const startResize = async (direction: ResizeDirection) => {
  try {
    await currentWindow.startResizeDragging(direction)
  } catch (err) {
    console.warn('startResizeDragging error:', err)
  }
}

const handleMouseDown = async (e: MouseEvent) => {
  if (e.button === 0) {
    // Left click triggers native window dragging
    try {
      await currentWindow.startDragging()
    } catch (err) {
      console.warn('startDragging error:', err)
    }
  } else if (e.button === 1) {
    // Middle click closes pin window
    e.preventDefault()
    await closePin()
  }
}

const onImageLoad = async (e: Event) => {
  const img = e.target as HTMLImageElement
  if (!img?.naturalWidth || !img?.naturalHeight) return

  const aspect = img.naturalWidth / img.naturalHeight
  imageAspect.value = aspect

  try {
    const scaleFactor = await currentWindow.scaleFactor()
    const physicalSize = await currentWindow.innerSize()
    const currentLogicalW = physicalSize.width / scaleFactor

    // Adjust height to match the exact aspect ratio of the image without any letterbox
    const targetLogicalH = Math.round(currentLogicalW / aspect)
    initialLogicalWidth.value = Math.round(currentLogicalW)
    initialLogicalHeight.value = targetLogicalH

    await currentWindow.setSize(new LogicalSize(Math.round(currentLogicalW), targetLogicalH))
    zoomPercent.value = 100
  } catch (err) {
    console.warn('Auto-fit pin window aspect ratio failed:', err)
  }
}

const handleWheel = async (e: WheelEvent) => {
  e.preventDefault()
  if (!imageAspect.value) return

  // Wheel up (deltaY < 0): zoom in (enlarge window)
  // Wheel down (deltaY > 0): zoom out (shrink window)
  const factor = e.deltaY < 0 ? 1.08 : 0.92

  try {
    const scaleFactor = await currentWindow.scaleFactor()
    const physicalSize = await currentWindow.innerSize()
    const currentLogicalW = physicalSize.width / scaleFactor

    const newLogicalW = Math.round(currentLogicalW * factor)
    const newLogicalH = Math.round(newLogicalW / imageAspect.value)

    const minW = 160
    const maxW = 3840
    const minH = 100
    const maxH = 2160

    if (newLogicalW < minW || newLogicalH < minH || newLogicalW > maxW || newLogicalH > maxH) return

    await currentWindow.setSize(new LogicalSize(newLogicalW, newLogicalH))

    if (initialLogicalWidth.value) {
      zoomPercent.value = Math.round((newLogicalW / initialLogicalWidth.value) * 100)
    }
  } catch (err) {
    console.warn('Wheel resize window error:', err)
  }
}

const resetZoom = async () => {
  if (!initialLogicalWidth.value || !initialLogicalHeight.value) return
  try {
    await currentWindow.setSize(new LogicalSize(initialLogicalWidth.value, initialLogicalHeight.value))
    zoomPercent.value = 100
  } catch (err) {
    console.warn('Reset zoom error:', err)
  }
}

const closePin = async () => {
  try {
    await invoke('close_pin_window', { label: currentWindow.label })
  } catch (err) {
    console.warn('close_pin_window fallback:', err)
    try {
      await currentWindow.close()
    } catch (e) {
      console.error('Failed to close pin window:', e)
    }
  }
}

const copyImage = async () => {
  if (!imageUrl.value) return
  try {
    const res = await fetch(imageUrl.value)
    const blob = await res.blob()
    await navigator.clipboard.write([
      new ClipboardItem({ 'image/png': blob })
    ])
  } catch (err) {
    console.error('Failed to copy pinned image:', err)
  }
}

const handleKeyDown = async (e: KeyboardEvent) => {
  if (e.key === 'Escape') {
    e.preventDefault()
    await closePin()
  } else if (e.ctrlKey && e.key.toLowerCase() === 'c') {
    e.preventDefault()
    await copyImage()
  }
}

onMounted(async () => {
  const pinData = (window as any).__PIN_DATA__
  if (pinData?.imageUrl) {
    imageUrl.value = pinData.imageUrl
  } else {
    const params = new URLSearchParams(window.location.search)
    const imgParam = params.get('img')
    if (imgParam) {
      imageUrl.value = imgParam
    }
  }

  window.addEventListener('keydown', handleKeyDown)

  try {
    unlistenResized = await currentWindow.onResized(async ({ payload: size }) => {
      if (initialLogicalWidth.value) {
        const sf = await currentWindow.scaleFactor()
        const lw = size.width / sf
        zoomPercent.value = Math.round((lw / initialLogicalWidth.value) * 100)
      }
    })
  } catch (err) {
    console.warn('onResized listener registration failed:', err)
  }
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
  unlistenResized?.()
})
</script>

<style scoped>
.pin-container {
  width: 100vw;
  height: 100vh;
  position: relative;
  overflow: hidden;
  background: #090d16;
  border: 1px solid rgba(56, 189, 248, 0.5);
  box-sizing: border-box;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: grab;
  user-select: none;
  box-shadow: 0 12px 32px rgba(0, 0, 0, 0.7);
}

.pin-container:active {
  cursor: grabbing;
}

.image-stage {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  pointer-events: auto;
}

.pinned-img {
  width: 100%;
  height: 100%;
  object-fit: contain;
  display: block;
  user-select: none;
  -webkit-user-drag: none;
  cursor: grab;
}

.pinned-img:active {
  cursor: grabbing;
}

.loading-box {
  color: #94a3b8;
  font-family: system-ui, -apple-system, sans-serif;
  font-size: 13px;
}

/* Floating Toolbar */
.pin-toolbar {
  position: absolute;
  top: 8px;
  right: 8px;
  display: flex;
  align-items: center;
  gap: 4px;
  background: rgba(15, 23, 42, 0.92);
  backdrop-filter: blur(8px);
  padding: 3px 6px;
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.18);
  opacity: 0;
  transition: opacity 0.2s ease;
  z-index: 100;
  cursor: default;
}

.pin-container:hover .pin-toolbar {
  opacity: 1;
}

.pin-badge {
  color: #38bdf8;
  font-size: 11px;
  font-family: system-ui, -apple-system, sans-serif;
  font-weight: 700;
  padding: 0 4px;
}

.pin-btn {
  background: transparent;
  border: none;
  color: #e2e8f0;
  padding: 4px;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.15s, color 0.15s;
}

.pin-btn:hover {
  background: rgba(255, 255, 255, 0.15);
}

.close-btn:hover {
  background: #ef4444;
  color: white;
}

/* 8-Directional Native Resize Drag Handles */
.resize-handle {
  position: absolute;
  z-index: 95;
  background: transparent;
}

.resize-handle.top {
  top: 0;
  left: 10px;
  right: 10px;
  height: 6px;
  cursor: ns-resize;
}

.resize-handle.bottom {
  bottom: 0;
  left: 10px;
  right: 10px;
  height: 6px;
  cursor: ns-resize;
}

.resize-handle.left {
  top: 10px;
  bottom: 10px;
  left: 0;
  width: 6px;
  cursor: ew-resize;
}

.resize-handle.right {
  top: 10px;
  bottom: 10px;
  right: 0;
  width: 6px;
  cursor: ew-resize;
}

.resize-handle.top-left {
  top: 0;
  left: 0;
  width: 12px;
  height: 12px;
  cursor: nwse-resize;
}

.resize-handle.top-right {
  top: 0;
  right: 0;
  width: 12px;
  height: 12px;
  cursor: nesw-resize;
}

.resize-handle.bottom-left {
  bottom: 0;
  left: 0;
  width: 12px;
  height: 12px;
  cursor: nesw-resize;
}

.resize-handle.bottom-right {
  bottom: 0;
  right: 0;
  width: 12px;
  height: 12px;
  cursor: nwse-resize;
}
</style>
