<template>
  <div
    class="pin-container"
    @mousedown="handleMouseDown"
    @wheel="handleWheel"
    @dblclick="closePin"
  >
    <!-- Floating Toolbar (appears on hover) -->
    <div class="pin-toolbar" @mousedown.stop>
      <span class="pin-badge">{{ Math.round(zoom * 100) }}%</span>
      <button class="pin-btn" @click.stop="copyImage" title="Sao chép vào clipboard (Ctrl+C)">
        <svg viewBox="0 0 24 24" width="14" height="14" fill="currentColor">
          <path d="M19 21H8V7h11m0-2H8a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h11a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2m-3-4H4a2 2 0 0 0-2 2v14h2V3h12V1Z"/>
        </svg>
      </button>
      <button class="pin-btn close-btn" @click.stop="closePin" title="Đóng ghim (Esc / Double-click)">
        <svg viewBox="0 0 24 24" width="14" height="14" fill="currentColor">
          <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12 19 6.41Z"/>
        </svg>
      </button>
    </div>

    <!-- Pinned Image Display -->
    <div class="image-stage" :style="{ transform: `scale(${zoom})` }">
      <img
        v-if="imageUrl"
        :src="imageUrl"
        alt="Pinned Screenshot"
        class="pinned-img"
        draggable="false"
      />
      <div v-else class="loading-box">Đang tải ảnh ghim...</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'

const imageUrl = ref<string>('')
const zoom = ref<number>(1.0)

const currentWindow = getCurrentWebviewWindow()

const handleMouseDown = async (e: MouseEvent) => {
  if (e.button === 0) {
    // Left click on background or image triggers window dragging
    try {
      await currentWindow.startDragging()
    } catch (err) {
      console.warn('startDragging error:', err)
    }
  }
}

const handleWheel = (e: WheelEvent) => {
  e.preventDefault()
  const delta = e.deltaY < 0 ? 0.1 : -0.1
  const newZoom = Math.min(4.0, Math.max(0.2, zoom.value + delta))
  zoom.value = Math.round(newZoom * 10) / 10
}

const closePin = async () => {
  try {
    await currentWindow.close()
  } catch (err) {
    console.error('Failed to close pin window:', err)
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
    await closePin()
  } else if (e.ctrlKey && e.key.toLowerCase() === 'c') {
    e.preventDefault()
    await copyImage()
  }
}

onMounted(() => {
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
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
})
</script>

<style scoped>
.pin-container {
  width: 100vw;
  height: 100vh;
  position: relative;
  overflow: hidden;
  background: #111827;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: grab;
  box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.6), 0 8px 10px -6px rgba(0, 0, 0, 0.4);
}

.pin-container:active {
  cursor: grabbing;
}

.image-stage {
  display: flex;
  align-items: center;
  justify-content: center;
  transition: transform 0.05s ease-out;
  transform-origin: center center;
}

.pinned-img {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  display: block;
  user-select: none;
  -webkit-user-drag: none;
}

.loading-box {
  color: #94a3b8;
  font-family: system-ui, -apple-system, sans-serif;
  font-size: 13px;
}

.pin-toolbar {
  position: absolute;
  top: 8px;
  right: 8px;
  display: flex;
  align-items: center;
  gap: 6px;
  background: rgba(15, 23, 42, 0.85);
  backdrop-filter: blur(8px);
  padding: 4px 8px;
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.12);
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
  font-weight: 600;
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
  transition: background 0.15s;
}

.pin-btn:hover {
  background: rgba(255, 255, 255, 0.15);
}

.close-btn:hover {
  background: #ef4444;
  color: white;
}
</style>
