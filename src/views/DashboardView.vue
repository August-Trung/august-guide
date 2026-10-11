<template>
  <div class="w-100 fill-height d-flex flex-column">
    <!-- View Header -->
    <AppHeader />

    <v-container class="flex-grow-1 align-start pa-6" fluid>
      <!-- Title & Main Actions Banner -->
      <div class="d-flex align-center justify-space-between mb-6 flex-wrap gap-3">
        <div>
          <div class="d-flex align-center gap-2 mb-1">
            <v-icon icon="mdi-image-multiple" color="primary" size="28"></v-icon>
            <h1 class="text-h4 font-weight-bold text-white">
              Thư viện ảnh chụp
            </h1>
          </div>
          <p class="text-subtitle-1 text-medium-emphasis">
            Kho ảnh chụp màn hình trong dự án. Chọn nhiều ảnh để đóng gói thành tài liệu hướng dẫn hoặc xuất file.
          </p>
        </div>

        <div class="d-flex align-center gap-2 flex-wrap">
          <!-- Open Screenshots Folder -->
          <v-btn
            color="surface-variant"
            variant="tonal"
            size="large"
            prepend-icon="mdi-folder-image"
            class="text-none font-weight-medium"
            title="Mở thư mục chứa file ảnh trên máy tính"
            @click="handleOpenFolder"
          >
            Thư mục ảnh
          </v-btn>

          <!-- Quick Capture Screenshot -->
          <v-btn
            color="primary"
            variant="elevated"
            size="large"
            prepend-icon="mdi-camera"
            class="text-none font-weight-bold shadow-sm"
            :loading="isCapturing"
            @click="handleTriggerCapture"
          >
            Chụp màn hình
          </v-btn>
        </div>
      </div>

      <!-- Loading State -->
      <div v-if="sessionStore.isLoadingCaptures" class="d-flex align-center justify-center py-16">
        <v-progress-circular indeterminate color="primary" size="48"></v-progress-circular>
      </div>

      <!-- Gallery Grid Component -->
      <GalleryGrid
        v-else
        :captures="sessionStore.projectCaptures"
        :selected-ids="selectedIds"
        @toggle-select="handleToggleSelect"
        @toggle-select-all="handleToggleSelectAll"
        @delete-capture="handleDeleteCapture"
        @refresh="loadGallery"
      />

      <!-- Floating Selection Action Bar -->
      <FloatingSelectionBar
        :selected-count="selectedIds.length"
        :selected-ids="selectedIds"
        @create-guide="showCreateGuideDialog = true"
        @delete-selected="promptBatchDelete"
        @clear-selection="selectedIds = []"
      />

      <!-- Create Guide From Selection Dialog -->
      <CreateGuideFromSelectionDialog
        v-model="showCreateGuideDialog"
        :selected-capture-ids="selectedIds"
        :project-id="projectStore.activeProjectId"
        @created="onGuideCreated"
      />

      <!-- Batch Delete Confirmation Dialog -->
      <ConfirmDialog
        v-model="showBatchDeleteDialog"
        title="Xác nhận xóa hàng loạt"
        :message="`Bạn có chắc chắn muốn xóa ${selectedIds.length} ảnh đã chọn? Toàn bộ file ảnh trên máy tính và dữ liệu liên quan sẽ bị xóa vĩnh viễn và không thể khôi phục.`"
        confirm-text="Xóa tất cả ảnh đã chọn"
        confirm-color="error"
        @confirm="executeBatchDelete"
      />
    </v-container>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import AppHeader from '@/components/common/AppHeader.vue'
import ConfirmDialog from '@/components/common/ConfirmDialog.vue'
import GalleryGrid from '@/components/gallery/GalleryGrid.vue'
import FloatingSelectionBar from '@/components/gallery/FloatingSelectionBar.vue'
import CreateGuideFromSelectionDialog from '@/components/gallery/CreateGuideFromSelectionDialog.vue'
import { useSessionStore } from '@/stores/sessionStore'
import { useProjectStore } from '@/stores/projectStore'
import { useUiStore } from '@/stores/uiStore'
import { deleteCapture, openAppFolder } from '@/services/tauriCommands'
import { startCaptureWorkflow } from '@/composables/useTauriEvents'
import { listenToEvent } from '@/services/tauriEvents'
import type { Session } from '@/types/session'

const router = useRouter()
const sessionStore = useSessionStore()
const projectStore = useProjectStore()
const uiStore = useUiStore()

const selectedIds = ref<string[]>([])
const showCreateGuideDialog = ref(false)
const showBatchDeleteDialog = ref(false)
const isCapturing = ref(false)

let unlistenSessionUpdated: (() => void) | null = null

async function loadGallery() {
  if (projectStore.activeProjectId) {
    await sessionStore.fetchProjectCaptures(projectStore.activeProjectId)
  }
}

function handleToggleSelect(id: string) {
  const index = selectedIds.value.indexOf(id)
  if (index > -1) {
    selectedIds.value.splice(index, 1)
  } else {
    selectedIds.value.push(id)
  }
}

function handleToggleSelectAll(ids: string[]) {
  selectedIds.value = ids
}

async function handleOpenFolder() {
  try {
    await openAppFolder('screenshots')
  } catch (err: any) {
    console.error('Failed to open screenshots folder:', err)
  }
}

async function handleTriggerCapture() {
  isCapturing.value = true
  try {
    await startCaptureWorkflow(projectStore, sessionStore)
  } catch (err: any) {
    console.error('Trigger capture failed:', err)
  } finally {
    isCapturing.value = false
  }
}

async function handleDeleteCapture(id: string) {
  uiStore.setLoading(true)
  try {
    await deleteCapture(id)
    selectedIds.value = selectedIds.value.filter(sId => sId !== id)
    uiStore.showToast({ message: 'Đã xóa ảnh khỏi thư viện!', type: 'success' })
    await loadGallery()
  } catch (err: any) {
    uiStore.showToast({ message: `Lỗi khi xóa ảnh: ${err?.message || err}`, type: 'error' })
  } finally {
    uiStore.setLoading(false)
  }
}

function promptBatchDelete() {
  if (selectedIds.value.length === 0) return
  showBatchDeleteDialog.value = true
}

async function executeBatchDelete() {
  const idsToDelete = [...selectedIds.value]
  selectedIds.value = []
  showBatchDeleteDialog.value = false
  uiStore.setLoading(true)

  try {
    for (const id of idsToDelete) {
      await deleteCapture(id)
    }
    uiStore.showToast({
      message: `Đã xóa thành công ${idsToDelete.length} ảnh khỏi thư viện!`,
      type: 'success',
    })
    await loadGallery()
  } catch (err: any) {
    uiStore.showToast({
      message: `Lỗi khi xóa ảnh: ${err?.message || err}`,
      type: 'error',
    })
  } finally {
    uiStore.setLoading(false)
  }
}

function onGuideCreated(newSession: Session) {
  selectedIds.value = []
  router.push(`/session/${newSession.id}`)
}

watch(
  () => projectStore.activeProjectId,
  async (newId) => {
    if (newId) {
      selectedIds.value = []
      await loadGallery()
    }
  }
)

onMounted(async () => {
  // Ensure projects are loaded
  if (!projectStore.projects?.length) {
    await projectStore.fetchProjects()
  }
  await loadGallery()

  // Listen to capture completion events to auto-refresh gallery
  unlistenSessionUpdated = await listenToEvent<string>('session-updated', async () => {
    await loadGallery()
  })
})

onUnmounted(() => {
  if (unlistenSessionUpdated) {
    unlistenSessionUpdated()
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
