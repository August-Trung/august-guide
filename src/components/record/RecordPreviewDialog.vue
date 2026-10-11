<template>
  <v-dialog
    v-model="showPreview"
    max-width="840"
    persistent
    @keydown="handleKeyDown"
  >
    <v-card border class="overflow-hidden bg-surface elevation-12">
      <!-- Header -->
      <v-card-title class="d-flex align-center justify-space-between px-5 py-4 border-bottom">
        <div class="d-flex align-center gap-2">
          <v-icon icon="mdi-video-check-outline" color="primary" size="24"></v-icon>
          <span class="text-h6 font-weight-bold text-white">Xem lại video ghi hình</span>
        </div>
        <v-btn icon="mdi-close" variant="text" size="small" @click="handleClose"></v-btn>
      </v-card-title>

      <!-- Video Player -->
      <div class="video-preview-wrapper bg-black position-relative">
        <video
          v-if="recordedUrl"
          ref="videoRef"
          :src="recordedUrl"
          controls
          playsinline
          preload="auto"
          class="preview-video"
          @loadedmetadata="handleLoadedMetadata"
        ></video>
      </div>

      <!-- Info Meta Bar -->
      <div class="d-flex align-center justify-space-between px-5 py-2 bg-surface-variant text-caption text-medium-emphasis border-bottom">
        <div>
          <span>Thời lượng: <strong class="text-white">{{ formattedDuration }}</strong></span>
          <span class="mx-2">•</span>
          <span>Dung lượng: <strong class="text-white">{{ formattedFileSize }}</strong></span>
        </div>
        <div v-if="savedFilePath" class="text-success font-weight-medium text-truncate" style="max-width: 400px;" :title="savedFilePath">
          <v-icon icon="mdi-check-circle" size="14" class="mr-1"></v-icon>
          Đã lưu: {{ savedFilePath }}
        </div>
      </div>

      <!-- Actions -->
      <v-card-actions class="px-5 py-3 d-flex align-center justify-space-between bg-surface">
        <v-btn
          variant="text"
          color="medium-emphasis"
          class="text-none"
          @click="handleClose"
        >
          Đóng
        </v-btn>

        <div class="d-flex align-center gap-2">
          <!-- Open Explorer button (after save) -->
          <v-btn
            v-if="savedFilePath"
            variant="tonal"
            color="secondary"
            prepend-icon="mdi-folder-open-outline"
            class="text-none font-weight-bold"
            @click="handleOpenFolder"
          >
            Mở thư mục lưu
          </v-btn>

          <!-- Save to Computer button -->
          <v-btn
            color="primary"
            variant="elevated"
            prepend-icon="mdi-content-save"
            class="text-none font-weight-bold"
            :loading="isSaving"
            @click="handleSave"
          >
            {{ savedFilePath ? 'Lưu bản sao khác' : 'Lưu video ra máy tính (Ctrl+S)' }}
          </v-btn>
        </div>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { save } from '@tauri-apps/plugin-dialog'
import { useScreenRecorder } from '@/composables/useScreenRecorder'
import { saveRecordingFile, openInExplorer } from '@/services/tauriCommands'
import { useUiStore } from '@/stores/uiStore'

const {
  showPreview,
  recordedUrl,
  recordedBlob,
  currentMimeType,
  formattedDuration,
  closePreview
} = useScreenRecorder()

const uiStore = useUiStore()

const videoRef = ref<HTMLVideoElement | null>(null)
const isSaving = ref(false)
const savedFilePath = ref<string | null>(null)

// Handle Chromium WebM duration infinity bug so duration and timeline seek work accurately
const handleLoadedMetadata = () => {
  const vid = videoRef.value
  if (!vid) return
  if (vid.duration === Infinity || isNaN(vid.duration)) {
    vid.currentTime = 1e101
    vid.ontimeupdate = () => {
      vid.ontimeupdate = null
      vid.currentTime = 0
    }
  }
}

const isMp4 = computed(() => (currentMimeType.value || '').toLowerCase().includes('mp4'))
const defaultExt = computed(() => isMp4.value ? 'mp4' : 'webm')

const formattedFileSize = computed(() => {
  if (!recordedBlob.value) return '0 KB'
  const bytes = recordedBlob.value.size
  if (bytes < 1024 * 1024) {
    return `${Math.round(bytes / 1024)} KB`
  }
  return `${(bytes / (1024 * 1024)).toFixed(2)} MB`
})

const handleKeyDown = (e: KeyboardEvent) => {
  if (e.ctrlKey && e.key.toLowerCase() === 's') {
    e.preventDefault()
    handleSave()
  } else if (e.key === 'Escape') {
    handleClose()
  }
}

const handleClose = () => {
  savedFilePath.value = null
  closePreview()
}

const handleSave = async () => {
  if (!recordedBlob.value) return
  isSaving.value = true

  try {
    const now = new Date()
    const pad = (n: number) => n.toString().padStart(2, '0')
    const dateStr = `${now.getFullYear()}${pad(now.getMonth() + 1)}${pad(now.getDate())}_${pad(now.getHours())}${pad(now.getMinutes())}${pad(now.getSeconds())}`
    const ext = defaultExt.value
    const defaultName = `AugustGuide_Record_${dateStr}.${ext}`

    let selectedPath = await save({
      title: 'Lưu file video ghi hình',
      defaultPath: defaultName,
      filters: isMp4.value
        ? [
            { name: 'MP4 Video (*.mp4)', extensions: ['mp4'] },
            { name: 'All Files (*.*)', extensions: ['*'] }
          ]
        : [
            { name: 'WebM Video (*.webm)', extensions: ['webm'] },
            { name: 'MP4 Video (*.mp4)', extensions: ['mp4'] },
            { name: 'All Files (*.*)', extensions: ['*'] }
          ]
    })

    if (!selectedPath) {
      isSaving.value = false
      return
    }

    // Ensure valid video extension
    if (!selectedPath.toLowerCase().endsWith('.mp4') && !selectedPath.toLowerCase().endsWith('.webm')) {
      selectedPath = `${selectedPath}.${ext}`
    }

    // Save as direct binary Uint8Array for high performance & reliability
    const arrayBuffer = await recordedBlob.value.arrayBuffer()
    const uint8Array = new Uint8Array(arrayBuffer)

    const saved = await saveRecordingFile(selectedPath, { data: uint8Array })
    savedFilePath.value = saved

    uiStore.showToast({
      message: `Đã lưu video thành công: ${saved}`,
      type: 'success',
      duration: 5000
    })
  } catch (err: any) {
    console.error('Failed to save screen recording:', err)
    uiStore.showToast({
      message: `Lỗi khi lưu video: ${err?.message || err}`,
      type: 'error',
      duration: 5000
    })
  } finally {
    isSaving.value = false
  }
}

const handleOpenFolder = async () => {
  if (!savedFilePath.value) return
  try {
    await openInExplorer(savedFilePath.value)
  } catch (err) {
    console.error('Failed to open explorer:', err)
  }
}
</script>

<style scoped>
.video-preview-wrapper {
  width: 100%;
  max-height: 480px;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.preview-video {
  width: 100%;
  max-height: 480px;
  outline: none;
}

.border-bottom {
  border-bottom: 1px solid rgba(255, 255, 255, 0.08) !important;
}

.gap-2 {
  gap: 8px;
}
</style>
