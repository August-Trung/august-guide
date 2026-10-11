<template>
  <div class="w-100 fill-height d-flex flex-column">
    <AppHeader />

    <v-container class="flex-grow-1 align-start pa-6" fluid>
      <!-- Loading State -->
      <div v-if="isLoadingSession" class="d-flex align-center justify-center py-12">
        <v-progress-circular indeterminate color="primary" size="48"></v-progress-circular>
      </div>

      <!-- Main Content -->
      <div v-else-if="session">
        <!-- Session Header -->
        <div class="d-flex align-center justify-space-between mb-6 flex-wrap gap-3">
          <div>
            <v-btn
              variant="text"
              prepend-icon="mdi-arrow-left"
              class="text-none pl-0 mb-2"
              color="medium-emphasis"
              @click="goBack"
            >
              {{ t('sessionView.backToDashboard') }}
            </v-btn>
            <div class="d-flex align-center gap-2">
              <h1 class="text-h4 font-weight-bold text-white mb-1">
                {{ session.title }}
              </h1>
              <v-chip :color="statusColor" size="small" class="text-uppercase font-weight-bold ml-3">
                {{ session.status === 'active' ? t('sessionView.active') : t('sessionView.completed') }}
              </v-chip>
            </div>
            <p class="text-subtitle-1 text-medium-emphasis">
              {{ session.description || t('issueDetail.noDescription') }}
            </p>
          </div>

          <div class="d-flex gap-3 flex-wrap">
            <!-- Share to Cloud Button (Only if Google Drive connected) -->
            <v-btn
              v-if="settingsStore.gdriveConnected"
              color="info"
              variant="elevated"
              size="large"
              prepend-icon="mdi-cloud-upload"
              class="text-none"
              :loading="isSharing"
              @click="handleCloudShare"
            >
              {{ t('sessionView.shareToCloud') }}
            </v-btn>

            <!-- Open Screenshots Folder Button -->
            <v-btn
              color="surface-variant"
              variant="tonal"
              size="large"
              prepend-icon="mdi-folder-image"
              class="text-none"
              :title="t('sessionView.openFolder', 'Mở thư mục lưu ảnh')"
              @click="handleOpenFolder"
            >
              {{ t('sessionView.openFolder', 'Thư mục ảnh') }}
            </v-btn>

            <!-- Export Button -->
            <v-btn
              color="secondary"
              variant="outlined"
              size="large"
              prepend-icon="mdi-export"
              class="text-none"
              @click="handleExport"
            >
              {{ t('common.export') }}
            </v-btn>

            <!-- Capture Button -->
            <v-btn
              color="primary"
              size="large"
              prepend-icon="mdi-camera"
              class="text-none font-weight-bold"
              :loading="isCapturing"
              @click="handleCapture"
            >
              {{ t('sessionView.captureMarkup', 'Chụp màn hình') }}
            </v-btn>
          </div>
        </div>

        <v-divider class="mb-6"></v-divider>

        <!-- Guide Steps List Header -->
        <div class="d-flex align-center justify-space-between mb-4 flex-wrap gap-3">
          <div class="d-flex align-center gap-2">
            <v-icon icon="mdi-image-multiple-outline" color="primary" size="24"></v-icon>
            <span class="text-h6 font-weight-bold text-white">
              Danh sách ảnh chụp hướng dẫn
            </span>
            <v-chip color="primary" size="small" variant="tonal" class="font-weight-bold ml-2">
              {{ captures.length }} ảnh
            </v-chip>
          </div>

          <div class="d-flex align-center gap-3 flex-wrap">
            <span class="text-caption text-medium-emphasis d-none d-md-inline">
              Nhấp vào ảnh để phóng to • Bấm "Sửa nét vẽ" để vẽ thêm hoặc sửa ghi chú
            </span>
            <div style="min-width: 175px;">
              <v-select
                v-model="sortBy"
                :items="sortOptions"
                item-title="title"
                item-value="value"
                density="compact"
                hide-details
                variant="solo-filled"
                flat
                bg-color="surface-variant"
                prepend-inner-icon="mdi-sort"
              ></v-select>
            </div>
          </div>
        </div>

        <!-- Captures Grid -->
        <div v-if="isLoadingCaptures" class="d-flex align-center justify-center py-12">
          <v-progress-circular indeterminate color="primary" size="48"></v-progress-circular>
        </div>

        <div v-else-if="captures.length === 0" class="text-center py-16 px-4 border rounded-lg bg-surface-variant-subtle">
          <v-icon icon="mdi-camera-plus-outline" size="64" color="primary" class="mb-3 opacity-80"></v-icon>
          <div class="text-h6 font-weight-bold text-white mb-2">Chưa có ảnh chụp nào trong bài hướng dẫn này</div>
          <p class="text-body-2 text-medium-emphasis mb-6" style="max-width: 500px; margin: 0 auto;">
            Bấm nút "Chụp màn hình" ở góc trên để chụp ảnh đầu tiên và vẽ các ghi chú, mũi tên hướng dẫn.
          </p>
          <v-btn
            color="primary"
            size="large"
            prepend-icon="mdi-camera"
            class="text-none font-weight-bold"
            :loading="isCapturing"
            @click="handleCapture"
          >
            Chụp ảnh đầu tiên
          </v-btn>
        </div>

        <v-row v-else class="gy-4">
          <v-col
            v-for="cap in sortedCaptures"
            :key="cap.id"
            cols="12"
            sm="6"
            md="4"
            lg="3"
          >
            <v-card border class="overflow-hidden h-100 d-flex flex-column capture-card elevation-2">
              <!-- Thumbnail Wrapper -->
              <div
                class="position-relative capture-thumb-wrapper"
                style="height: 190px; background: #0f172a; cursor: pointer;"
                @click="openImagePreview(cap)"
              >
                <v-img
                  :src="convertFileSrc(cap.annotatedScreenshotPath || cap.screenshotPath)"
                  cover
                  height="190"
                  class="bg-grey-darken-4"
                >
                  <template v-slot:placeholder>
                    <div class="d-flex align-center justify-center fill-height">
                      <v-progress-circular indeterminate size="28" color="primary"></v-progress-circular>
                    </div>
                  </template>
                </v-img>

                <!-- Floating Capture Number Badge -->
                <v-chip
                  color="primary"
                  size="small"
                  variant="flat"
                  class="position-absolute font-weight-bold elevation-2"
                  style="top: 10px; left: 10px; z-index: 2;"
                >
                  Ảnh {{ getStepNumber(cap.id) }}
                </v-chip>

                <!-- Quick Pin to Screen Button -->
                <v-btn
                  icon="mdi-pin-outline"
                  size="x-small"
                  color="surface"
                  variant="flat"
                  class="position-absolute elevation-2"
                  style="top: 10px; right: 10px; z-index: 2;"
                  title="Ghim nổi trên màn hình (P)"
                  @click.stop="handlePinCapture(cap)"
                ></v-btn>
              </div>

              <!-- Card Body -->
              <v-card-item class="pa-3 flex-grow-1">
                <div class="text-subtitle-2 font-weight-bold text-truncate text-white mb-1" :title="cap.windowTitle || ('Ảnh ' + getStepNumber(cap.id))">
                  {{ cap.windowTitle || ('Ảnh ' + getStepNumber(cap.id)) }}
                </div>
                <div class="text-caption text-medium-emphasis">
                  {{ formatDate(cap.createdAt) }} • {{ cap.monitorWidth }}×{{ cap.monitorHeight }}
                </div>
              </v-card-item>

              <v-divider></v-divider>

              <!-- Card Action Toolbar -->
              <v-card-actions class="px-3 py-2 d-flex align-center justify-space-between bg-surface-variant">
                <v-btn
                  icon="mdi-delete-outline"
                  size="small"
                  color="error"
                  variant="text"
                  title="Xóa ảnh này"
                  @click.stop="promptDeleteCapture(cap.id)"
                ></v-btn>

                <div class="d-flex align-center gap-1">
                  <v-btn
                    icon="mdi-magnify-plus-outline"
                    size="small"
                    variant="text"
                    color="medium-emphasis"
                    title="Xem ảnh lớn"
                    @click.stop="openImagePreview(cap)"
                  ></v-btn>

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
                </div>
              </v-card-actions>
            </v-card>
          </v-col>
        </v-row>
      </div>
    </v-container>

    <!-- Delete Capture Confirmation Dialog -->
    <ConfirmDialog
      v-model="showDeleteCaptureDialog"
      title="Xóa ảnh chụp hướng dẫn"
      message="Bạn có chắc chắn muốn xóa ảnh chụp này và toàn bộ các nét vẽ kèm theo không? Thao tác này không thể hoàn tác."
      confirm-text="Xóa"
      confirm-color="error"
      @confirm="executeDeleteCapture"
    />

    <!-- Export Session Report Dialog -->
    <ExportDialog
      v-if="session"
      v-model="showExportDialog"
      :session-id="session.id"
      :session-title="session.title"
    />

    <!-- Cloud Share Result Dialog -->
    <v-dialog v-model="showShareDialog" max-width="500">
      <v-card border>
        <v-card-title class="d-flex align-center justify-space-between py-4 px-6 border-b">
          <span class="text-h6 font-weight-bold">{{ t('sessionView.cloudShareTitle') }}</span>
          <v-btn icon="mdi-close" variant="text" density="comfortable" @click="showShareDialog = false"></v-btn>
        </v-card-title>
        
        <v-card-text class="pa-6 text-center">
          <v-icon icon="mdi-cloud-check" size="64" color="success" class="mb-4"></v-icon>
          <div class="text-h6 font-weight-bold text-white mb-2">{{ t('sessionView.cloudShareSuccess', 'Report Shared Successfully!') }}</div>
          <div class="text-body-2 text-medium-emphasis mb-6">
            {{ t('sessionView.cloudShareDesc') }}
          </div>

          <v-text-field
            v-model="shareUrl"
            readonly
            variant="outlined"
            density="comfortable"
            append-inner-icon="mdi-content-copy"
            @click:append-inner="copyShareUrl"
            class="mb-2"
          ></v-text-field>
        </v-card-text>

        <v-card-actions class="py-4 px-6 border-t d-flex justify-end">
          <v-btn color="primary" variant="flat" @click="showShareDialog = false">{{ t('common.ok', 'Done') }}</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Success Toast Notification -->
    <v-snackbar v-model="showSuccess" color="success" timeout="4000" location="top">
      {{ successMessage }}
      <template v-slot:actions>
        <v-btn variant="text" @click="showSuccess = false">{{ t('sessionView.closeBtn') }}</v-btn>
      </template>
    </v-snackbar>

    <!-- Error Toast Notification -->
    <v-snackbar v-model="showError" color="error" timeout="6000" location="top">
      {{ errorMessage }}
      <template v-slot:actions>
        <v-btn variant="text" @click="showError = false">{{ t('sessionView.closeBtn') }}</v-btn>
      </template>
    </v-snackbar>

    <!-- Fullscreen Image Preview Dialog -->
    <v-dialog v-model="showPreviewDialog" max-width="1100">
      <v-card border color="surface">
        <v-card-title class="d-flex align-center justify-space-between py-3 px-6 border-b">
          <span class="text-subtitle-1 font-weight-bold text-white">{{ previewImageTitle }}</span>
          <div class="d-flex align-center gap-2">
            <v-btn
              v-if="currentPreviewCapture"
              variant="tonal"
              size="small"
              color="primary"
              prepend-icon="mdi-pencil"
              class="text-none mr-1 font-weight-bold"
              @click="handleReEditCapture(currentPreviewCapture)"
            >
              Sửa nét vẽ
            </v-btn>
            <v-btn
              v-if="currentPreviewCapture"
              icon="mdi-pin-outline"
              variant="text"
              density="comfortable"
              title="Ghim nổi trên màn hình (P)"
              @click="handlePinCapture(currentPreviewCapture)"
            ></v-btn>
            <v-btn
              icon="mdi-folder-open-outline"
              variant="text"
              density="comfortable"
              :title="t('sessionView.openFolder', 'Mở thư mục lưu ảnh')"
              @click="handleOpenFolder"
            ></v-btn>
            <v-btn icon="mdi-close" variant="text" density="comfortable" @click="showPreviewDialog = false"></v-btn>
          </div>
        </v-card-title>
        <v-card-text class="pa-4 text-center bg-black d-flex align-center justify-center" style="max-height: 80vh; overflow: auto;">
          <img
            v-if="previewImageUrl"
            :src="previewImageUrl"
            alt="Preview"
            style="max-width: 100%; max-height: 75vh; object-fit: contain; border-radius: 4px;"
          />
        </v-card-text>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { convertFileSrc } from '@tauri-apps/api/core'
import {
  getSession,
  triggerCapture,
  openOverlay,
  reopenOverlayForEdit,
  shareSessionOnGdrive,
  getCapturesBySession,
  deleteCapture,
  openPinWindow,
  openAppFolder,
} from '@/services/tauriCommands'
import { useSettingsStore } from '@/stores/settingsStore'
import { useProjectStore } from '@/stores/projectStore'
import { listenToEvent } from '@/services/tauriEvents'
import AppHeader from '@/components/common/AppHeader.vue'
import ConfirmDialog from '@/components/common/ConfirmDialog.vue'
import ExportDialog from '@/components/export/ExportDialog.vue'
import type { Session } from '@/types/session'
import type { Capture } from '@/types/capture'
import { useI18n } from '@/composables/useI18n'
import { formatDate } from '@/utils/date'

const { t } = useI18n()

const route = useRoute()
const router = useRouter()
const sessionId = route.params.id as string

const session = ref<Session | null>(null)
const captures = ref<Capture[]>([])
const sortBy = ref<'newest' | 'oldest'>('newest')

const sortOptions = [
  { title: 'Mới nhất trước', value: 'newest' },
  { title: 'Cũ nhất trước (Bước 1 → N)', value: 'oldest' },
]

// Map capture ID to original 1-based chronological step number
const stepNumberMap = computed(() => {
  const map = new Map<string, number>()
  captures.value.forEach((cap, index) => {
    map.set(cap.id, index + 1)
  })
  return map
})

const getStepNumber = (capId: string) => {
  return stepNumberMap.value.get(capId) ?? 1
}

const sortedCaptures = computed(() => {
  const list = [...captures.value]
  if (sortBy.value === 'newest') {
    return list.reverse()
  }
  return list
})

const isLoadingSession = ref(true)
const isLoadingCaptures = ref(false)
const isCapturing = ref(false)
const isSharing = ref(false)

const showPreviewDialog = ref(false)
const previewImageUrl = ref<string | null>(null)
const previewImageTitle = ref<string>('')
const currentPreviewCapture = ref<Capture | null>(null)

const settingsStore = useSettingsStore()
const projectStore = useProjectStore()

// If active project is changed while viewing this session, redirect back to dashboard
watch(() => projectStore.activeProjectId, (newProjectId) => {
  if (session.value?.projectId && newProjectId && session.value.projectId !== newProjectId) {
    router.push('/')
  }
})

const showDeleteCaptureDialog = ref(false)
const captureToDelete = ref<string | null>(null)
const showExportDialog = ref(false)

const loadSessionDetails = async () => {
  isLoadingSession.value = true
  try {
    session.value = await getSession(sessionId)
    if (session.value?.projectId && session.value.projectId !== projectStore.activeProjectId) {
      projectStore.activeProjectId = session.value.projectId
    }
  } catch (err) {
    console.error('Failed to load session details:', err)
  } finally {
    isLoadingSession.value = false
  }
}

const loadCaptures = async () => {
  isLoadingCaptures.value = true
  try {
    captures.value = await getCapturesBySession(sessionId)
  } catch (err) {
    console.error('Failed to load captures for session:', err)
  } finally {
    isLoadingCaptures.value = false
  }
}

const handleOpenFolder = async () => {
  try {
    await openAppFolder('screenshots')
  } catch (e) {
    console.error('Failed to open screenshots folder:', e)
  }
}

const openImagePreview = (cap: Capture) => {
  currentPreviewCapture.value = cap
  previewImageUrl.value = convertFileSrc(cap.annotatedScreenshotPath || cap.screenshotPath)
  previewImageTitle.value = cap.windowTitle || `Ảnh chụp ${formatDate(cap.createdAt)}`
  showPreviewDialog.value = true
}

const handlePinCapture = async (cap: Capture) => {
  try {
    const imgUrl = convertFileSrc(cap.annotatedScreenshotPath || cap.screenshotPath)
    await openPinWindow(imgUrl, cap.monitorWidth, cap.monitorHeight)
  } catch (err: any) {
    console.error('Failed to pin capture:', err)
  }
}

let unlistenSessionUpdated: (() => void) | null = null

onMounted(async () => {
  await settingsStore.loadSettings()
  if (sessionId) {
    await loadSessionDetails()
    await loadCaptures()
  }

  unlistenSessionUpdated = await listenToEvent<string>('session-updated', async (updatedSessionId) => {
    if (updatedSessionId === sessionId) {
      await loadSessionDetails()
      await loadCaptures()
    }
  })
})

onUnmounted(() => {
  if (unlistenSessionUpdated) {
    unlistenSessionUpdated()
  }
})

const goBack = () => {
  router.push('/')
}

const handleCapture = async () => {
  if (!sessionId) return
  isCapturing.value = true
  try {
    const res = await triggerCapture(sessionId)
    await openOverlay(res.captureId, res.screenshotPath)
  } catch (err) {
    console.error('Capture trigger failed:', err)
  } finally {
    isCapturing.value = false
  }
}

const handleReEditCapture = async (cap: Capture) => {
  try {
    await reopenOverlayForEdit(cap.id, cap.screenshotPath)
  } catch (err: any) {
    errorMessage.value = `Không thể mở chỉnh sửa nét vẽ: ${err?.message || err}`
    showError.value = true
  }
}

const promptDeleteCapture = (id: string) => {
  captureToDelete.value = id
  showDeleteCaptureDialog.value = true
}

const executeDeleteCapture = async () => {
  if (!captureToDelete.value) return
  try {
    await deleteCapture(captureToDelete.value)
    successMessage.value = 'Đã xóa ảnh chụp hướng dẫn!'
    showSuccess.value = true
    await loadCaptures()
    await loadSessionDetails()
  } catch (err: any) {
    console.error('Failed to delete capture:', err)
    errorMessage.value = `Không thể xóa ảnh chụp: ${err?.message || err}`
    showError.value = true
  } finally {
    captureToDelete.value = null
  }
}

const handleExport = () => {
  showExportDialog.value = true
}

const showShareDialog = ref(false)
const shareUrl = ref('')
const showSuccess = ref(false)
const successMessage = ref('')
const showError = ref(false)
const errorMessage = ref('')

async function handleCloudShare() {
  if (!sessionId) return
  isSharing.value = true
  try {
    const url = await shareSessionOnGdrive(sessionId)
    shareUrl.value = url
    showShareDialog.value = true
  } catch (err: any) {
    console.error('Cloud share failed:', err)
    errorMessage.value = err?.message || err || t('sessionView.failedShare')
    showError.value = true
  } finally {
    isSharing.value = false
  }
}

async function copyShareUrl() {
  try {
    await navigator.clipboard.writeText(shareUrl.value)
    successMessage.value = t('sessionView.copiedToClipboard')
    showSuccess.value = true
  } catch (err) {
    console.error('Failed to copy text:', err)
  }
}

const statusColor = computed(() => {
  if (!session.value) return 'medium-emphasis'
  switch (session.value.status) {
    case 'active': return 'secondary'
    case 'completed': return 'success'
    case 'archived': return 'warning'
    default: return 'medium-emphasis'
  }
})
</script>

<style scoped>
.gap-1 {
  gap: 4px;
}
.gap-2 {
  gap: 8px;
}
.gap-3 {
  gap: 12px;
}
.bg-surface-variant-subtle {
  background: rgba(255, 255, 255, 0.02);
}
.capture-card {
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}
.capture-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5) !important;
}
</style>
