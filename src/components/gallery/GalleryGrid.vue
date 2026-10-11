<template>
  <div class="gallery-grid-container">
    <!-- Toolbar: Filter by App, Sort, Search & Selection Controls -->
    <v-card border variant="flat" class="pa-4 mb-6 bg-surface">
      <div class="d-flex flex-wrap align-center justify-space-between gap-3">
        <!-- Left: Filters & Counters -->
        <div class="d-flex flex-wrap align-center gap-3">
          <!-- Filter by App Window Title -->
          <div style="min-width: 220px;">
            <v-select
              v-model="selectedAppFilter"
              :items="appFilterOptions"
              item-title="title"
              item-value="value"
              density="compact"
              variant="outlined"
              hide-details
              prepend-inner-icon="mdi-application-outline"
              label="Ứng dụng nguồn"
            ></v-select>
          </div>

          <!-- Sort Order -->
          <div style="min-width: 170px;">
            <v-select
              v-model="sortOrder"
              :items="sortOptions"
              item-title="title"
              item-value="value"
              density="compact"
              variant="outlined"
              hide-details
              prepend-inner-icon="mdi-sort"
              label="Sắp xếp"
            ></v-select>
          </div>

          <!-- Total Count Badge -->
          <v-chip color="primary" variant="tonal" size="default" class="font-weight-medium">
            <v-icon icon="mdi-image-multiple" start size="18"></v-icon>
            {{ filteredCaptures.length }} / {{ captures.length }} ảnh
          </v-chip>
        </div>

        <!-- Right: Multi-select Toggle / Select All -->
        <div class="d-flex align-center gap-2">
          <v-btn
            v-if="filteredCaptures.length > 0"
            variant="tonal"
            color="primary"
            size="small"
            class="text-none font-weight-medium"
            :prepend-icon="isAllSelected ? 'mdi-checkbox-blank-outline' : 'mdi-checkbox-marked-outline'"
            @click="toggleSelectAllFiltered"
          >
            {{ isAllSelected ? 'Bỏ chọn tất cả' : 'Chọn tất cả ảnh' }}
          </v-btn>
        </div>
      </div>
    </v-card>

    <!-- Empty State -->
    <div
      v-if="filteredCaptures.length === 0"
      class="text-center py-16 px-4 d-flex flex-column align-center justify-center border rounded-lg bg-surface-variant"
    >
      <v-avatar size="72" color="surface" class="mb-4 elevation-2">
        <v-icon icon="mdi-camera-outline" size="36" color="primary"></v-icon>
      </v-avatar>
      <h3 class="text-h6 font-weight-bold text-white mb-2">
        {{ captures.length === 0 ? 'Chưa có ảnh chụp nào trong thư viện' : 'Không có ảnh nào khớp bộ lọc' }}
      </h3>
      <p class="text-body-2 text-medium-emphasis max-w-md mb-4">
        {{ captures.length === 0 
          ? 'Nhấn phím tắt Alt+Shift+S (hoặc nút chụp ảnh) để chụp ảnh màn hình bất kỳ lúc nào.'
          : 'Hãy thử chọn ứng dụng khác hoặc đặt lại bộ lọc để xem các ảnh chụp khác.'
        }}
      </p>
      <v-btn
        v-if="captures.length > 0 && selectedAppFilter !== 'all'"
        variant="tonal"
        color="primary"
        size="small"
        class="text-none"
        @click="selectedAppFilter = 'all'"
      >
        Đặt lại bộ lọc
      </v-btn>
    </div>

    <!-- Timeline Grouped Grid -->
    <div v-else class="timeline-groups d-flex flex-column gap-6">
      <div
        v-for="group in groupedCaptures"
        :key="group.dateLabel"
        class="timeline-group"
      >
        <!-- Date Header -->
        <div class="d-flex align-center justify-space-between mb-3 px-1">
          <div class="d-flex align-center gap-2">
            <v-icon icon="mdi-calendar-clock" color="primary" size="20"></v-icon>
            <span class="text-subtitle-1 font-weight-bold text-white">
              {{ group.dateLabel }}
            </span>
            <v-chip size="x-small" variant="flat" color="surface-variant" class="ml-1 font-weight-medium">
              {{ group.captures.length }} ảnh
            </v-chip>
          </div>

          <!-- Select All in This Group -->
          <v-btn
            variant="text"
            size="x-small"
            color="primary"
            class="text-none"
            @click="toggleSelectGroup(group.captures)"
          >
            {{ isGroupAllSelected(group.captures) ? 'Bỏ chọn nhóm này' : 'Chọn nhóm này' }}
          </v-btn>
        </div>

        <!-- Captures Grid in this date group -->
        <v-row class="gy-4">
          <v-col
            v-for="cap in group.captures"
            :key="cap.id"
            cols="12"
            sm="6"
            md="4"
            lg="3"
          >
            <v-card
              border
              class="overflow-hidden h-100 d-flex flex-column gallery-card elevation-2"
              :class="{ 'card-selected': isSelected(cap.id) }"
            >
              <!-- Thumbnail Wrapper -->
              <div
                class="position-relative gallery-thumb-wrapper"
                style="height: 180px; background: #0f172a; cursor: pointer;"
                @click="openImagePreview(cap)"
              >
                <v-img
                  :src="convertFileSrc(cap.annotatedScreenshotPath || cap.screenshotPath)"
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

                <!-- Checkbox for Multi-select -->
                <div
                  class="position-absolute checkbox-container elevation-3"
                  style="top: 8px; left: 8px; z-index: 5;"
                  @click.stop
                >
                  <v-checkbox-btn
                    :model-value="isSelected(cap.id)"
                    color="primary"
                    density="compact"
                    class="ma-0 pa-0"
                    @update:model-value="$emit('toggle-select', cap.id)"
                  ></v-checkbox-btn>
                </div>

                <!-- Quick Pin to Screen Button -->
                <v-btn
                  icon="mdi-pin-outline"
                  size="x-small"
                  color="surface"
                  variant="flat"
                  class="position-absolute elevation-2"
                  style="top: 8px; right: 8px; z-index: 5;"
                  title="Ghim nổi trên màn hình"
                  @click.stop="handlePinCapture(cap)"
                ></v-btn>
              </div>

              <!-- Card Body: Metadata -->
              <v-card-item class="pa-3 flex-grow-1">
                <div
                  class="text-subtitle-2 font-weight-bold text-truncate text-white mb-1"
                  :title="cap.windowTitle || 'Ảnh chụp màn hình'"
                >
                  {{ cap.windowTitle || 'Ảnh chụp màn hình' }}
                </div>
                <div class="text-caption text-medium-emphasis d-flex align-center justify-space-between">
                  <span>{{ formatTimeOnly(cap.createdAt) }}</span>
                  <span>{{ cap.monitorWidth }}×{{ cap.monitorHeight }}</span>
                </div>
              </v-card-item>

              <v-divider></v-divider>

              <!-- Card Action Toolbar -->
              <v-card-actions class="px-2 py-1 d-flex align-center justify-space-between bg-surface-variant">
                <!-- Delete Single Capture -->
                <v-btn
                  icon="mdi-delete-outline"
                  size="small"
                  color="error"
                  variant="text"
                  title="Xóa ảnh này"
                  @click.stop="promptDeleteSingle(cap)"
                ></v-btn>

                <div class="d-flex align-center gap-1">
                  <!-- View Full -->
                  <v-btn
                    icon="mdi-magnify-plus-outline"
                    size="small"
                    variant="text"
                    color="medium-emphasis"
                    title="Xem ảnh lớn"
                    @click.stop="openImagePreview(cap)"
                  ></v-btn>

                  <!-- Re-edit Markup -->
                  <v-btn
                    size="small"
                    color="primary"
                    variant="tonal"
                    prepend-icon="mdi-pencil"
                    class="text-none font-weight-medium"
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
    </div>

    <!-- Image Preview Modal -->
    <v-dialog v-model="showPreviewDialog" max-width="90vw">
      <v-card bg-color="surface" class="pa-0 overflow-hidden">
        <v-card-title class="d-flex align-center justify-space-between pa-4 bg-surface-variant">
          <div class="text-subtitle-1 font-weight-bold text-white text-truncate mr-4">
            {{ previewImageTitle }}
          </div>
          <div class="d-flex align-center gap-2">
            <v-btn
              v-if="currentPreviewCapture"
              variant="tonal"
              color="primary"
              size="small"
              prepend-icon="mdi-pencil"
              class="text-none"
              @click="currentPreviewCapture && handleReEditCapture(currentPreviewCapture)"
            >
              Sửa nét vẽ
            </v-btn>
            <v-btn
              v-if="currentPreviewCapture"
              variant="tonal"
              color="secondary"
              size="small"
              prepend-icon="mdi-pin-outline"
              class="text-none"
              @click="currentPreviewCapture && handlePinCapture(currentPreviewCapture)"
            >
              Ghim nổi
            </v-btn>
            <v-btn
              icon="mdi-close"
              variant="text"
              size="small"
              color="medium-emphasis"
              @click="showPreviewDialog = false"
            ></v-btn>
          </div>
        </v-card-title>
        <v-card-text class="pa-0 text-center bg-black d-flex align-center justify-center" style="max-height: 80vh; overflow: auto;">
          <img
            v-if="previewImageUrl"
            :src="previewImageUrl"
            class="img-fluid"
            style="max-width: 100%; max-height: 80vh; object-fit: contain;"
          />
        </v-card-text>
      </v-card>
    </v-dialog>

    <!-- Confirm Single Delete Dialog -->
    <ConfirmDialog
      v-model="showDeleteDialog"
      title="Xác nhận xóa ảnh"
      message="Bạn có chắc chắn muốn xóa ảnh chụp này không? Hành động này sẽ xóa file ảnh khỏi thiết bị và không thể hoàn tác."
      confirm-text="Xóa ảnh"
      confirm-color="error"
      @confirm="executeDeleteSingle"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/core'
import { openPinWindow, reopenOverlayForEdit } from '@/services/tauriCommands'
import ConfirmDialog from '@/components/common/ConfirmDialog.vue'
import type { Capture } from '@/types/capture'

const props = defineProps<{
  captures: Capture[]
  selectedIds: string[]
  isLoading?: boolean
}>()

const emit = defineEmits<{
  (e: 'toggle-select', id: string): void
  (e: 'toggle-select-all', ids: string[]): void
  (e: 'delete-capture', id: string): void
  (e: 'refresh'): void
}>()

// Filter by App Window Title
const selectedAppFilter = ref<string>('all')
const sortOrder = ref<'newest' | 'oldest'>('newest')

const sortOptions = [
  { title: 'Mới nhất trước', value: 'newest' },
  { title: 'Cũ nhất trước', value: 'oldest' },
]

// Extract unique app window titles
const appFilterOptions = computed(() => {
  const options = [{ title: 'Tất cả ứng dụng', value: 'all' }]
  const titles = new Set<string>()

  props.captures.forEach(cap => {
    if (cap.windowTitle?.trim()) {
      titles.add(cap.windowTitle.trim())
    }
  })

  titles.forEach(title => {
    options.push({ title, value: title })
  })

  return options
})

// Filtered captures list
const filteredCaptures = computed(() => {
  let list = props.captures.filter(cap => {
    if (selectedAppFilter.value === 'all') return true
    return cap.windowTitle?.trim() === selectedAppFilter.value
  })

  // Sort
  list = [...list].sort((a, b) => {
    const timeA = new Date(a.createdAt).getTime()
    const timeB = new Date(b.createdAt).getTime()
    return sortOrder.value === 'newest' ? timeB - timeA : timeA - timeB
  })

  return list
})

// Group captures by date (Hôm nay, Hôm qua, Ngày DD/MM/YYYY)
interface DateGroup {
  dateLabel: string
  captures: Capture[]
}

const groupedCaptures = computed(() => {
  const groups: DateGroup[] = []
  const groupMap = new Map<string, Capture[]>()

  filteredCaptures.value.forEach(cap => {
    const label = getDateGroupKey(cap.createdAt)
    if (!groupMap.has(label)) {
      groupMap.set(label, [])
    }
    groupMap.get(label)?.push(cap)
  })

  groupMap.forEach((caps, label) => {
    groups.push({ dateLabel: label, captures: caps })
  })

  return groups
})

function getDateGroupKey(isoString: string): string {
  try {
    const d = new Date(isoString)
    const today = new Date()
    const yesterday = new Date()
    yesterday.setDate(today.getDate() - 1)

    const isSameDate = (d1: Date, d2: Date) =>
      d1.getFullYear() === d2.getFullYear() &&
      d1.getMonth() === d2.getMonth() &&
      d1.getDate() === d2.getDate()

    const pad = (n: number) => (n < 10 ? '0' + n : String(n))
    const dateFormatted = `${pad(d.getDate())}/${pad(d.getMonth() + 1)}/${d.getFullYear()}`

    if (isSameDate(d, today)) {
      return `Hôm nay (${dateFormatted})`
    }
    if (isSameDate(d, yesterday)) {
      return `Hôm qua (${dateFormatted})`
    }
    return `Ngày ${dateFormatted}`
  } catch {
    return 'Khác'
  }
}

function formatTimeOnly(isoString: string): string {
  try {
    const d = new Date(isoString)
    const pad = (n: number) => (n < 10 ? '0' + n : String(n))
    return `${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}`
  } catch {
    return isoString
  }
}

// Multi-select helpers
function isSelected(id: string): boolean {
  return props.selectedIds.includes(id)
}

const isAllSelected = computed(() => {
  return (
    filteredCaptures.value.length > 0 &&
    filteredCaptures.value.every(c => props.selectedIds.includes(c.id))
  )
})

function toggleSelectAllFiltered() {
  if (isAllSelected.value) {
    // Deselect all filtered
    const filteredIdSet = new Set(filteredCaptures.value.map(c => c.id))
    const remaining = props.selectedIds.filter(id => !filteredIdSet.has(id))
    emit('toggle-select-all', remaining)
  } else {
    // Select all filtered + existing
    const allIds = Array.from(new Set([...props.selectedIds, ...filteredCaptures.value.map(c => c.id)]))
    emit('toggle-select-all', allIds)
  }
}

function isGroupAllSelected(caps: Capture[]): boolean {
  return caps.length > 0 && caps.every(c => props.selectedIds.includes(c.id))
}

function toggleSelectGroup(caps: Capture[]) {
  const capIds = caps.map(c => c.id)
  if (isGroupAllSelected(caps)) {
    // Deselect this group
    const groupSet = new Set(capIds)
    const remaining = props.selectedIds.filter(id => !groupSet.has(id))
    emit('toggle-select-all', remaining)
  } else {
    // Select this group
    const combined = Array.from(new Set([...props.selectedIds, ...capIds]))
    emit('toggle-select-all', combined)
  }
}

// Preview Dialog
const showPreviewDialog = ref(false)
const previewImageUrl = ref<string | null>(null)
const previewImageTitle = ref<string>('')
const currentPreviewCapture = ref<Capture | null>(null)

function openImagePreview(cap: Capture) {
  currentPreviewCapture.value = cap
  previewImageUrl.value = convertFileSrc(cap.annotatedScreenshotPath || cap.screenshotPath)
  previewImageTitle.value = cap.windowTitle || `Ảnh chụp ${cap.createdAt}`
  showPreviewDialog.value = true
}

// Pinning & Editing
async function handlePinCapture(cap: Capture) {
  try {
    const imgUrl = convertFileSrc(cap.annotatedScreenshotPath || cap.screenshotPath)
    await openPinWindow(imgUrl, cap.monitorWidth, cap.monitorHeight)
  } catch (err: any) {
    console.error('Failed to pin capture:', err)
  }
}

async function handleReEditCapture(cap: Capture) {
  try {
    await reopenOverlayForEdit(cap.id, cap.screenshotPath)
  } catch (err: any) {
    console.error('Failed to reopen overlay:', err)
  }
}

// Single Delete Confirmation
const showDeleteDialog = ref(false)
const captureToDelete = ref<Capture | null>(null)

function promptDeleteSingle(cap: Capture) {
  captureToDelete.value = cap
  showDeleteDialog.value = true
}

function executeDeleteSingle() {
  if (captureToDelete.value) {
    emit('delete-capture', captureToDelete.value.id)
    captureToDelete.value = null
  }
  showDeleteDialog.value = false
}
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
.gap-6 {
  gap: 24px;
}

.gallery-card {
  transition: transform 0.2s ease, box-shadow 0.2s ease, border-color 0.2s ease;
}

.gallery-card:hover {
  transform: translateY(-2px);
  border-color: rgba(56, 189, 248, 0.4);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.4) !important;
}

.card-selected {
  border-color: rgb(56, 189, 248) !important;
  box-shadow: 0 0 0 2px rgba(56, 189, 248, 0.4), 0 8px 24px rgba(0, 0, 0, 0.6) !important;
}

.checkbox-container {
  background: rgba(15, 23, 42, 0.85);
  backdrop-filter: blur(8px);
  border-radius: 6px;
  padding: 2px;
}

.max-w-md {
  max-width: 450px;
}
</style>
