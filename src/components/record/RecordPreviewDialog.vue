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
          :src="recordedUrl"
          controls
          autoplay
          class="preview-video"
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

const {
  showPreview,
  recordedUrl,
  recordedBlob,
  formattedDuration,
  closePreview
} = useScreenRecorder()

const isSaving = ref(false)
const savedFilePath = ref<string | null>(null)

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
    const defaultName = `AugustGuide_Record_${dateStr}.mp4`

    const selectedPath = await save({
      title: 'Lưu file video ghi hình',
      defaultPath: defaultName,
      filters: [
        { name: 'Video file (*.mp4, *.webm)', extensions: ['mp4', 'webm'] }
      ]
    })

    if (!selectedPath) {
      isSaving.value = false
      return
    }

    // Convert Blob to Base64
    const reader = new FileReader()
    const base64Promise = new Promise<string>((resolve, reject) => {
      reader.onloadend = () => {
        resolve(reader.result as string)
      }
      reader.onerror = reject
    })
    reader.readAsDataURL(recordedBlob.value)
    const base64Data = await base64Promise

    const saved = await saveRecordingFile(selectedPath, base64Data)
    savedFilePath.value = saved
  } catch (err) {
    console.error('Failed to save screen recording:', err)
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
