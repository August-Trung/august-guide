<template>
  <div class="canvas-container" ref="containerRef">
    <!-- Layer 1: Screenshot (Bottom) -->
    <canvas ref="screenshotCanvasRef" class="overlay-canvas z-screenshot"></canvas>

    <!-- Layer 2: Markers (Middle) -->
    <canvas ref="markerCanvasRef" class="overlay-canvas z-markers"></canvas>

    <!-- Layer 3: Drawing/Preview (Top) -->
    <canvas
      ref="drawingCanvasRef"
      class="overlay-canvas z-drawing"
      :style="{ cursor: canvasCursor }"
      @mousedown="handleMouseDown"
      @mousemove="handleMouseMove"
      @mouseup="handleMouseUp"
      @contextmenu="handleContextMenu"
    ></canvas>

    <!-- Floating Text Input box for Text Tool -->
    <div
      v-if="textInputState.visible"
      class="floating-input-wrapper"
      :style="{ left: textInputState.x + 'px', top: textInputState.y + 'px' }"
    >
      <input
        ref="textInputRef"
        v-model="textInputState.text"
        type="text"
        :placeholder="t('overlay.typeFeedback')"
        class="floating-text-input"
        @keydown.enter="commitTextInput"
        @keydown.esc="cancelTextInput"
      />
    </div>

    <!-- Custom Context Menu for Annotation -->
    <div
      v-if="contextMenuState.visible"
      class="custom-context-menu"
      :style="{ left: contextMenuState.x + 'px', top: contextMenuState.y + 'px' }"
    >
      <button class="context-menu-item info-item" @click="toggleSelectedBadge">
        <i class="mdi mr-1" :class="selectedAnnotation?.showBadge ? 'mdi-numeric-off' : 'mdi-numeric'"></i>
        <span>{{ selectedAnnotation?.showBadge ? 'Xóa số thứ tự (giữ nét vẽ)' : 'Gắn lại số thứ tự' }}</span>
      </button>
      <button class="context-menu-item" @click="deleteSelected">
        <i class="mdi mdi-delete-outline mr-1"></i> {{ t('overlay.eraseStroke') || 'Xóa nét vẽ này' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick, computed } from 'vue'
import { useCanvas } from '@/composables/useCanvas'
import { useAnnotation } from '@/composables/useAnnotation'
import { loadImage } from '@/utils/image'
import { useOverlayStore } from '@/stores/overlayStore'
import { useI18n } from '@/composables/useI18n'

const props = defineProps<{
  screenshotUrl: string
}>()

const emit = defineEmits<{
  (e: 'load'): void
  (e: 'error', message: string): void
}>()

const { t } = useI18n()

const containerRef = ref<HTMLDivElement | null>(null)
const screenshotCanvasRef = ref<HTMLCanvasElement | null>(null)
const markerCanvasRef = ref<HTMLCanvasElement | null>(null)
const drawingCanvasRef = ref<HTMLCanvasElement | null>(null)
const textInputRef = ref<HTMLInputElement | null>(null)

const { initCanvas, drawScreenshot } = useCanvas()

let screenshotCtx: CanvasRenderingContext2D | null = null
let markerCtx: CanvasRenderingContext2D | null = null
let drawingCtx: CanvasRenderingContext2D | null = null
let screenshotImage: HTMLImageElement | null = null

const overlayStore = useOverlayStore()

// Initialize Annotation drawing composable
const {
  textInputState,
  hoveredAnnotationId,
  contextMenuState,
  handleMouseDown,
  handleMouseMove,
  handleMouseUp,
  handleContextMenu,
  deleteAnnotationById,
  toggleSelectedBadge,
  clearHover,
  commitTextInput,
  cancelTextInput,
  redrawCommittedCanvas,
  copyToClipboard,
  getMergedCanvas
} = useAnnotation(
  markerCanvasRef,
  drawingCanvasRef,
  () => markerCtx,
  () => drawingCtx
)

const canvasCursor = computed(() => {
  if (hoveredAnnotationId.value) return 'pointer'
  return overlayStore.activeTool ? 'crosshair' : 'default'
})

const selectedAnnotation = computed(() => {
  if (!contextMenuState.value.annotationId) return null
  return overlayStore.annotations.find(a => a.id === contextMenuState.value.annotationId)
})

const deleteSelected = () => {
  if (contextMenuState.value.annotationId) {
    deleteAnnotationById(contextMenuState.value.annotationId)
  }
}

const resizeCanvases = () => {
  const width = window.innerWidth
  const height = window.innerHeight

  if (screenshotCanvasRef.value && markerCanvasRef.value && drawingCanvasRef.value) {
    screenshotCtx = initCanvas(screenshotCanvasRef.value, width, height)
    markerCtx = initCanvas(markerCanvasRef.value, width, height)
    drawingCtx = initCanvas(drawingCanvasRef.value, width, height)

    // Redraw screenshot if loaded
    if (screenshotCtx && screenshotImage) {
      drawScreenshot(screenshotCtx, screenshotImage, width, height)
    }

    // Redraw committed markers/annotations
    redrawCommittedCanvas()
  }
}

const loadScreenshot = async () => {
  if (!props.screenshotUrl) return

  try {
    screenshotImage = await loadImage(props.screenshotUrl, 'anonymous')
    resizeCanvases()
    emit('load')
  } catch {
    emit('error', 'Failed to load screenshot image in canvas.')
  }
}

// Global key handlers (Ctrl+C, Ctrl+S)
const handleKeyDown = async (e: KeyboardEvent) => {
  if (textInputState.value.visible) return

  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'c') {
    e.preventDefault()
    const copied = await copyToClipboard()
    if (copied) {
      setTimeout(() => {
        import('@/services/tauriCommands').then(({ closeOverlay }) => closeOverlay())
      }, 300)
    }
  } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
    e.preventDefault()
    const merged = getMergedCanvas()
    if (merged) {
      const dataUrl = merged.toDataURL('image/png')
      await overlayStore.saveAndClose(dataUrl)
      import('@/services/tauriCommands').then(({ closeOverlay }) => closeOverlay())
    }
  }
}

// Watch for floating text input visibility to autofocus
watch(() => textInputState.value.visible, (visible) => {
  if (visible) {
    nextTick(() => {
      textInputRef.value?.focus()
    })
  }
})

// Watch screenshotUrl changes
watch(() => props.screenshotUrl, () => {
  loadScreenshot()
})

onMounted(() => {
  window.addEventListener('resize', resizeCanvases)
  window.addEventListener('keydown', handleKeyDown)
  loadScreenshot()
})

onUnmounted(() => {
  window.removeEventListener('resize', resizeCanvases)
  window.removeEventListener('keydown', handleKeyDown)
})

// Expose canvas elements and contexts for use in other files
defineExpose({
  screenshotCanvasRef,
  markerCanvasRef,
  drawingCanvasRef,
  hoveredAnnotationId,
  clearHover,
  copyToClipboard,
  getMergedCanvas,
  getScreenshotCtx: () => screenshotCtx,
  getMarkerCtx: () => markerCtx,
  getDrawingCtx: () => drawingCtx,
})
</script>

<style scoped>
.canvas-container {
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  background: transparent;
}

.overlay-canvas {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: block;
}

.z-screenshot {
  z-index: 1;
}

.z-markers {
  z-index: 2;
  pointer-events: none; /* Let clicks pass through to drawing canvas */
}

.z-drawing {
  z-index: 3;
}

/* Floating Input Styling */
.floating-input-wrapper {
  position: absolute;
  z-index: 1001; /* Must render above drawing canvas */
  transform: translate(0, 0); /* Direct placement at logical coords */
}

.floating-text-input {
  background: rgba(15, 17, 23, 0.9);
  border: 1px solid #ff6b35;
  border-radius: 4px;
  color: #ffffff;
  font-family: sans-serif;
  font-size: 16px;
  font-weight: 500;
  padding: 4px 8px;
  outline: none;
  min-width: 150px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  user-select: text !important;
  -webkit-user-select: text !important;
}

/* Custom Context Menu */
.custom-context-menu {
  position: fixed;
  z-index: 10000;
  background: rgba(26, 29, 39, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.16);
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
  padding: 4px;
  backdrop-filter: blur(8px);
}

.context-menu-item {
  background: transparent;
  border: none;
  color: #ff5252;
  font-family: sans-serif;
  font-size: 0.875rem;
  font-weight: 500;
  padding: 8px 16px;
  cursor: pointer;
  display: flex;
  align-items: center;
  border-radius: 4px;
  width: 100%;
  text-align: left;
  outline: none;
  transition: background 0.15s ease;
}

.context-menu-item.info-item {
  color: #38bdf8;
}

.context-menu-item.info-item:hover {
  background: rgba(56, 189, 248, 0.12);
}

.context-menu-item:hover {
  background: rgba(255, 82, 82, 0.12);
}

.mr-1 {
  margin-right: 4px;
}
</style>
