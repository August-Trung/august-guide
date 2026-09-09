<template>
  <v-app class="overlay-app-wrapper">
    <div class="overlay-root">
      <!-- Status bar — always visible so user knows they can press Esc -->
      <OverlayStatusBar
        v-model:collapsed="isTopBarCollapsed"
        :session-name="guideTitle"
        :issue-count="stepCount"
        @cancel="handleCancel"
        @done="handleDone"
        @copy="handleCopy"
      />

      <!-- Fullscreen screenshot and drawing canvas -->
      <div class="screenshot-wrapper">
        <AnnotationCanvas
          v-if="screenshotUrl"
          ref="annotationCanvasRef"
          :screenshot-url="screenshotUrl"
          @load="handleScreenshotLoad"
          @error="handleScreenshotError"
        />
        <!-- Loading placeholder while screenshot loads -->
        <div v-else class="loading-placeholder">
          <span>{{ loadError || t('overlay.loadingScreenshot') || 'Đang tải ảnh chụp...' }}</span>
        </div>
      </div>

      <!-- Bottom floating toolbar -->
      <AnnotationToolbar
        v-model:collapsed="isBottomBarCollapsed"
        @copy="handleCopy"
        @save="handleDone"
        @save-as="handleSaveAs"
      />

      <!-- Right side slide-in step notes drawer -->
      <StepNotesDrawer />

      <!-- Global Snackbar for Toasts in Overlay Window -->
      <v-snackbar
        v-model="uiStore.toastVisible"
        :color="toastColor"
        location="top"
        :timeout="3000"
        elevation="8"
        class="mt-4"
        style="z-index: 20000;"
      >
        <div class="d-flex align-center gap-2">
          <v-icon :icon="toastIcon" size="small"></v-icon>
          <span class="font-weight-medium">{{ uiStore.toastMessage }}</span>
        </div>
        <template v-slot:actions>
          <v-btn icon="mdi-close" variant="text" size="small" @click="uiStore.hideToast"></v-btn>
        </template>
      </v-snackbar>
    </div>
  </v-app>
</template>

<script setup lang="ts">
import { nextTick, ref, onMounted, onUnmounted, computed } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/core'
import { listenToEvent } from '@/services/tauriEvents'
import { getCapture, cancelCapture, closeOverlay, showOverlay } from '@/services/tauriCommands'
import OverlayStatusBar from '@/components/overlay/OverlayStatusBar.vue'
import AnnotationCanvas from '@/components/overlay/AnnotationCanvas.vue'
import AnnotationToolbar from '@/components/overlay/AnnotationToolbar.vue'
import StepNotesDrawer from '@/components/overlay/StepNotesDrawer.vue'
import { useOverlayStore } from '@/stores/overlayStore'
import { useUiStore } from '@/stores/uiStore'
import { useI18n } from '@/composables/useI18n'
import type { Capture } from '@/types/capture'

const uiStore = useUiStore()
const { t } = useI18n()
const overlayStore = useOverlayStore()
const annotationCanvasRef = ref<any>(null)

const captureId = ref<string | null>(null)
const capture = ref<Capture | null>(null)
const screenshotUrl = ref<string>('')
const guideTitle = ref<string>('August Guide')
const stepCount = computed(() => overlayStore.annotations.filter(a => a.showBadge).length)
const loadError = ref<string | null>(null)

const isTopBarCollapsed = ref(false)
const isBottomBarCollapsed = ref(false)

const toastColor = computed(() => {
  switch (uiStore.toastType) {
    case 'success': return 'success'
    case 'error': return 'error'
    case 'info': return 'info'
    default: return 'info'
  }
})

const toastIcon = computed(() => {
  switch (uiStore.toastType) {
    case 'success': return 'mdi-check-circle'
    case 'error': return 'mdi-alert-circle'
    case 'info': return 'mdi-information'
    default: return 'mdi-information'
  }
})

let unlistenInit: (() => void) | null = null
let hasShownOverlay = false
let revealFallbackTimer: ReturnType<typeof window.setTimeout> | null = null

const revealOverlay = async () => {
  if (hasShownOverlay) return

  await nextTick()
  if (hasShownOverlay) return
  await showOverlay()
  hasShownOverlay = true
}

const scheduleRevealFallback = () => {
  if (revealFallbackTimer) {
    window.clearTimeout(revealFallbackTimer)
  }

  revealFallbackTimer = window.setTimeout(() => {
    revealOverlay().catch((e) => {
      console.error('[Overlay] Failed to reveal overlay:', e)
    })
  }, 500)
}

const loadCaptureDetails = async (id: string) => {
  try {
    captureId.value = id
    loadError.value = null
    screenshotUrl.value = ''
    const data = await getCapture(id)
    capture.value = data

    overlayStore.init(id, data.screenshotPath, data)
    screenshotUrl.value = convertFileSrc(data.screenshotPath)
    scheduleRevealFallback()
  } catch (e) {
    console.error('[Overlay] Failed to load capture details:', e)
    loadError.value = t('overlay.failedLoadScreenshot') || 'Không thể tải ảnh chụp màn hình'
    await revealOverlay()
  }
}

const loadCaptureFromUrl = async (id: string, path: string) => {
  captureId.value = id
  loadError.value = null
  overlayStore.init(id, path)
  screenshotUrl.value = convertFileSrc(path)
  scheduleRevealFallback()
}

const handleScreenshotLoad = async () => {
  await revealOverlay()
}

const handleScreenshotError = async () => {
  loadError.value = t('overlay.failedDisplayScreenshot') || 'Không thể hiển thị ảnh chụp'
  screenshotUrl.value = ''
  await revealOverlay()
}

const isDocumentComposing = ref(false)

const onCompositionStart = () => {
  isDocumentComposing.value = true
}

const onCompositionEnd = () => {
  isDocumentComposing.value = false
}

const handleKeyDown = async (e: KeyboardEvent) => {
  const target = e.target as HTMLElement
  if (
    isDocumentComposing.value ||
    target instanceof HTMLInputElement ||
    target instanceof HTMLTextAreaElement ||
    target?.tagName === 'INPUT' ||
    target?.tagName === 'TEXTAREA' ||
    target?.isContentEditable
  ) {
    return
  }

  // Ctrl+Z (Undo) and Ctrl+Y (Redo)
  if (e.ctrlKey) {
    if (e.key.toLowerCase() === 'z') {
      e.preventDefault()
      overlayStore.undo()
    } else if (e.key.toLowerCase() === 'y') {
      e.preventDefault()
      overlayStore.redo()
    } else if (e.key.toLowerCase() === 'c') {
      e.preventDefault()
      await handleCopy()
    } else if (e.key.toLowerCase() === 's') {
      e.preventDefault()
      await handleDone()
    }
    return
  }

  if (e.key === 'Escape') {
    if (overlayStore.showStepNotes) {
      overlayStore.showStepNotes = false
    } else {
      handleCancel()
    }
  } else if (e.key === 'Delete' || e.key === 'Backspace') {
    if (annotationCanvasRef.value?.hoveredAnnotationId) {
      overlayStore.removeAnnotation(annotationCanvasRef.value.hoveredAnnotationId)
      annotationCanvasRef.value.clearHover()
    }
  } else if (e.key === 'Tab') {
    e.preventDefault()
    overlayStore.showStepNotes = !overlayStore.showStepNotes
  } else if (e.key.toLowerCase() === 'h') {
    e.preventDefault()
    const bothCollapsed = isTopBarCollapsed.value && isBottomBarCollapsed.value
    isTopBarCollapsed.value = !bothCollapsed
    isBottomBarCollapsed.value = !bothCollapsed
  } else if (e.key === '1') {
    overlayStore.setTool('marker')
  } else if (e.key === '2') {
    overlayStore.setTool('rect')
  } else if (e.key === '3') {
    overlayStore.setTool('arrow')
  } else if (e.key === '4') {
    overlayStore.setTool('text')
  } else if (e.key === '5') {
    overlayStore.setTool('freedraw')
  } else if (e.key === '6') {
    overlayStore.setTool('blur')
  } else if (e.key === '7') {
    overlayStore.setTool('highlight')
  } else if (e.key === '8') {
    overlayStore.setTool('spotlight')
  } else if (e.key === '9' || e.key.toLowerCase() === 'c') {
    overlayStore.setTool('crop')
  }
}

const handleCancel = async () => {
  if (captureId.value) {
    await cancelCapture(captureId.value)
  } else {
    await closeOverlay()
  }
}

const handleCopy = async () => {
  if (annotationCanvasRef.value) {
    const copied = await annotationCanvasRef.value.copyToClipboard()
    if (copied) {
      try {
        let annotatedBase64: string | undefined = undefined
        const merged = annotationCanvasRef.value.getMergedCanvas()
        if (merged) {
          annotatedBase64 = merged.toDataURL('image/png')
        }
        await overlayStore.saveAndClose(annotatedBase64)
      } catch (e) {
        console.error('[OverlayApp] Auto-save on copy error:', e)
      }
      setTimeout(() => {
        closeOverlay()
      }, 250)
    }
  }
}

const handleDone = async () => {
  try {
    let annotatedBase64: string | undefined = undefined
    if (annotationCanvasRef.value) {
      const merged = annotationCanvasRef.value.getMergedCanvas()
      if (merged) {
        annotatedBase64 = merged.toDataURL('image/png')
      }
    }

    await overlayStore.saveAndClose(annotatedBase64)
    await closeOverlay()
  } catch (e: any) {
    console.error('[OverlayApp] Failed to save guide annotations:', e?.message || e)
  }
}

const handleSaveAs = async () => {
  const merged = annotationCanvasRef.value?.getMergedCanvas?.()
  if (!merged) return

  try {
    const { save } = await import('@tauri-apps/plugin-dialog')
    const { saveImageToFile } = await import('@/services/tauriCommands')

    const timestamp = new Date().toISOString().replace(/[-:T.]/g, '').slice(0, 14)
    const filePath = await save({
      defaultPath: `August_Guide_${timestamp}.png`,
      filters: [{ name: 'Hình ảnh (PNG)', extensions: ['png'] }],
      title: t('settingsView.saveAs') || 'Lưu ảnh ra file...'
    })

    if (filePath) {
      const dataUrl = merged.toDataURL('image/png')
      await saveImageToFile(filePath, dataUrl)
      uiStore.showToast({
        message: t('settingsView.imageSavedTo', { path: filePath }) || `Đã lưu ảnh vào: ${filePath}`,
        type: 'success'
      })
    }
  } catch (e: any) {
    console.error('[OverlayApp] Failed to save image as:', e)
    uiStore.showToast({
      message: `Lỗi khi lưu ảnh: ${e?.message || String(e)}`,
      type: 'error'
    })
  }
}

onMounted(async () => {
  const initialData = (window as any).__INITIAL_DATA__
  if (initialData && initialData.captureId && initialData.screenshotPath) {
    await loadCaptureFromUrl(initialData.captureId, initialData.screenshotPath)
  } else {
    const params = new URLSearchParams(window.location.search)
    const idParam = params.get('captureId')
    const screenshotPathParam = params.get('screenshotPath')

    if (idParam && screenshotPathParam) {
      await loadCaptureFromUrl(idParam, screenshotPathParam)
    } else if (idParam) {
      await loadCaptureDetails(idParam)
    }
  }

  unlistenInit = await listenToEvent<string>('overlay:init', async (id) => {
    if (!captureId.value) {
      await loadCaptureDetails(id)
    }
  })

  window.addEventListener('keydown', handleKeyDown)
  document.addEventListener('compositionstart', onCompositionStart)
  document.addEventListener('compositionend', onCompositionEnd)
})

onUnmounted(() => {
  if (unlistenInit) unlistenInit()
  if (revealFallbackTimer) window.clearTimeout(revealFallbackTimer)
  window.removeEventListener('keydown', handleKeyDown)
  document.removeEventListener('compositionstart', onCompositionStart)
  document.removeEventListener('compositionend', onCompositionEnd)
  overlayStore.reset()
})
</script>

<style>
*, *::before, *::after {
  box-sizing: border-box;
}

html, body, #app {
  margin: 0 !important;
  padding: 0 !important;
  width: 100vw !important;
  height: 100vh !important;
  overflow: hidden !important;
  background: #121212 !important;
  background-color: #121212 !important;
}

.v-application {
  background: #121212 !important;
  background-color: #121212 !important;
}

.v-application__wrap {
  background: transparent !important;
  background-color: transparent !important;
  min-height: 100vh !important;
}
</style>

<style scoped>
.overlay-root {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  background: transparent;
  margin: 0;
  padding: 0;
}

.screenshot-wrapper {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 1;
  background: transparent;
  user-select: none;
  -webkit-user-select: none;
}

.loading-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.6);
  color: rgba(255, 255, 255, 0.7);
  font-family: -apple-system, sans-serif;
  font-size: 1rem;
}
</style>
