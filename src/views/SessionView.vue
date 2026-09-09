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
        <div class="d-flex align-center justify-space-between mb-6">
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

          <div class="d-flex gap-3">
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
              class="text-none"
              :loading="isCapturing"
              @click="handleCapture"
            >
              {{ t('sessionView.captureMarkup', 'Chụp màn hình') }}
            </v-btn>
          </div>
        </div>

        <v-divider class="mb-6"></v-divider>

        <!-- View Tabs: Steps vs Captures -->
        <div class="d-flex align-center justify-space-between mb-4 flex-wrap gap-2">
          <v-tabs v-model="activeTab" color="primary" density="comfortable">
            <v-tab value="issues" class="text-none font-weight-bold">
              <v-icon start icon="mdi-format-list-numbered"></v-icon>
              {{ t('sessionView.issuesInSession', 'Các bước hướng dẫn') }} ({{ issues.length }})
            </v-tab>
            <v-tab value="captures" class="text-none font-weight-bold">
              <v-icon start icon="mdi-image-multiple"></v-icon>
              {{ t('sessionView.capturesInSession', 'Tất cả ảnh chụp') }} ({{ captures.length }})
            </v-tab>
          </v-tabs>

          <span v-if="activeTab === 'issues'" class="text-caption text-medium-emphasis">
            {{ t('sessionView.showingIssues', 'Hiển thị {filtered} trên {total} bước', { filtered: filteredIssues.length, total: issues.length }) }}
          </span>
          <span v-else class="text-caption text-medium-emphasis">
            {{ captures.length }} ảnh chụp trong phiên
          </span>
        </div>

        <!-- Issues / Steps Tab -->
        <div v-if="activeTab === 'issues'">
          <!-- Filter Bar -->
          <FilterBar class="mb-4" />

          <!-- Issue List -->
          <div v-if="isLoadingIssues" class="d-flex align-center justify-center py-8">
            <v-progress-circular indeterminate color="secondary" size="36"></v-progress-circular>
          </div>
          <div v-else>
            <IssueList :issues="filteredIssues" @delete="handleDeleteIssue" />
          </div>
        </div>

        <!-- Captures Grid Tab -->
        <div v-else>
          <div v-if="isLoadingCaptures" class="d-flex align-center justify-center py-8">
            <v-progress-circular indeterminate color="primary" size="36"></v-progress-circular>
          </div>
          <div v-else-if="captures.length === 0" class="text-center py-12 text-medium-emphasis">
            <v-icon icon="mdi-image-off-outline" size="48" class="mb-2"></v-icon>
            <div>Chưa có ảnh chụp nào trong bài hướng dẫn này</div>
          </div>
          <v-row v-else>
            <v-col
              v-for="cap in captures"
              :key="cap.id"
              cols="12"
              sm="6"
              md="4"
              lg="3"
            >
              <v-card border class="overflow-hidden h-100 d-flex flex-column capture-card">
                <div class="position-relative capture-thumb-wrapper" style="height: 180px; background: #0f172a; cursor: pointer;" @click="openImagePreview(cap)">
                  <v-img
                    :src="convertFileSrc(cap.screenshotPath)"
                    cover
                    height="180"
                    class="bg-grey-darken-4"
                  >
                    <template v-slot:placeholder>
                      <div class="d-flex align-center justify-center fill-height">
                        <v-progress-circular indeterminate size="24" color="primary"></v-progress-circular>
                      </div>
                    </template>
                  </v-img>
                </div>
                <v-card-item class="pa-3 flex-grow-1">
                  <div class="text-subtitle-2 font-weight-bold text-truncate text-white mb-1">
                    {{ cap.windowTitle || 'Ảnh chụp toàn màn hình' }}
                  </div>
                  <div class="text-caption text-medium-emphasis">
                    {{ formatDate(cap.createdAt) }} • {{ cap.monitorWidth }}×{{ cap.monitorHeight }}
                  </div>
                </v-card-item>
              </v-card>
            </v-col>
          </v-row>
        </div>
      </div>
    </v-container>

    <!-- Delete Issue Confirmation Dialog -->
    <ConfirmDialog
      v-model="showDeleteDialog"
      :title="t('dashboardView.deleteIssueTitle')"
      :message="t('dashboardView.deleteIssueConfirm')"
      :confirm-text="t('common.delete')"
      confirm-color="error"
      @confirm="executeDeleteIssue"
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
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import { convertFileSrc } from '@tauri-apps/api/core'
import { getSession, triggerCapture, openOverlay, shareSessionOnGdrive, getCapturesBySession, openAppFolder } from '@/services/tauriCommands'
import { useSettingsStore } from '@/stores/settingsStore'
import { useIssueStore } from '@/stores/issueStore'
import { listenToEvent } from '@/services/tauriEvents'
import AppHeader from '@/components/common/AppHeader.vue'
import FilterBar from '@/components/dashboard/FilterBar.vue'
import IssueList from '@/components/dashboard/IssueList.vue'
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
const isLoadingSession = ref(true)
const isLoadingCaptures = ref(false)
const isCapturing = ref(false)
const isSharing = ref(false)
const activeTab = ref<'issues' | 'captures'>('issues')

const showPreviewDialog = ref(false)
const previewImageUrl = ref<string | null>(null)
const previewImageTitle = ref<string>('')

const settingsStore = useSettingsStore()
const issueStore = useIssueStore()
const { issues, filteredIssues, isLoading: isLoadingIssues } = storeToRefs(issueStore)

const showDeleteDialog = ref(false)
const issueToDelete = ref<string | null>(null)
const showExportDialog = ref(false)

const loadSessionDetails = async () => {
  isLoadingSession.value = true
  try {
    session.value = await getSession(sessionId)
  } catch (err) {
    console.error('Failed to load session details:', err)
  } finally {
    isLoadingSession.value = false
  }
}

const loadIssues = async () => {
  try {
    await issueStore.fetchIssuesBySession(sessionId)
  } catch (err) {
    console.error('Failed to load issues for session:', err)
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
  previewImageUrl.value = convertFileSrc(cap.screenshotPath)
  previewImageTitle.value = cap.windowTitle || `Ảnh chụp ${formatDate(cap.createdAt)}`
  showPreviewDialog.value = true
}

let unlistenSessionUpdated: (() => void) | null = null

onMounted(async () => {
  issueStore.clearFilters()
  await settingsStore.loadSettings()
  if (sessionId) {
    await loadSessionDetails()
    await loadIssues()
    await loadCaptures()
  }

  unlistenSessionUpdated = await listenToEvent<string>('session-updated', async (updatedSessionId) => {
    if (updatedSessionId === sessionId) {
      await loadSessionDetails()
      await loadIssues()
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

const handleDeleteIssue = (id: string) => {
  issueToDelete.value = id
  showDeleteDialog.value = true
}

const executeDeleteIssue = async () => {
  if (!issueToDelete.value) return
  try {
    await issueStore.deleteIssue(issueToDelete.value)
    await loadCaptures()
  } catch (err) {
    console.error('Failed to delete issue:', err)
  } finally {
    issueToDelete.value = null
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
.gap-2 {
  gap: 8px;
}
.gap-3 {
  gap: 12px;
}
</style>
