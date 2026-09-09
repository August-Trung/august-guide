<template>
  <v-dialog v-model="showUpdateModal" max-width="520" :persistent="isDownloading">
    <v-card border class="overflow-hidden">
      <!-- Header with Gradient Accent -->
      <div class="pa-6 bg-surface-variant border-b d-flex align-center justify-space-between">
        <div class="d-flex align-center gap-3">
          <v-avatar color="primary" size="44" rounded>
            <v-icon icon="mdi-rocket-launch-outline" color="white" size="26"></v-icon>
          </v-avatar>
          <div>
            <div class="text-h6 font-weight-bold text-white">{{ t('updater.updateAvailableTitle', 'Đã có bản cập nhật mới!') }}</div>
            <div class="text-caption text-medium-emphasis">August Guide {{ updateManifest?.version }}</div>
          </div>
        </div>
        <v-chip color="primary" variant="flat" size="small" class="font-weight-bold">
          {{ updateManifest?.version }}
        </v-chip>
      </div>

      <!-- Card Body -->
      <v-card-text class="pa-6">
        <!-- Normal State: Release notes -->
        <div v-if="!isDownloading">
          <div class="d-flex align-center justify-space-between mb-2">
            <span class="text-subtitle-2 font-weight-bold text-primary">{{ t('updater.whatsNew', 'Những điểm mới trong phiên bản này:') }}</span>
            <span v-if="updateManifest?.date" class="text-caption text-medium-emphasis">{{ updateManifest.date }}</span>
          </div>

          <div class="pa-4 border rounded bg-background font-body text-body-2 release-notes-box mb-4">
            <div style="white-space: pre-line; word-break: break-word;">
              {{ updateManifest?.body || t('updater.defaultNotes', 'Bản cập nhật bao gồm các cải tiến hiệu năng và sửa lỗi.') }}
            </div>
          </div>

          <v-alert
            type="info"
            variant="tonal"
            density="comfortable"
            icon="mdi-information-outline"
            class="mb-0 text-caption"
          >
            {{ t('updater.autoRestartNotice', 'Ứng dụng sẽ tự động tải về, cài đặt và khởi động lại phiên bản mới.') }}
          </v-alert>
        </div>

        <!-- Downloading / Installing State -->
        <div v-else class="py-4">
          <div class="d-flex align-center justify-space-between mb-2">
            <span class="text-subtitle-2 font-weight-bold">{{ t('updater.downloading', 'Đang tải bản cập nhật...') }}</span>
            <span class="text-body-2 font-weight-bold text-primary">{{ downloadProgress }}%</span>
          </div>

          <v-progress-linear
            :model-value="downloadProgress"
            color="primary"
            height="10"
            rounded
            striped
            class="mb-3"
          ></v-progress-linear>

          <div class="d-flex justify-space-between text-caption text-medium-emphasis">
            <span>{{ formatBytes(downloadedBytes) }} / {{ totalBytes > 0 ? formatBytes(totalBytes) : '...' }}</span>
            <span>{{ t('updater.installingAuto', 'Tự động cài đặt khi hoàn tất') }}</span>
          </div>
        </div>

        <!-- Error State -->
        <v-alert
          v-if="errorMessage && !isDownloading"
          type="error"
          variant="tonal"
          density="comfortable"
          class="mt-4 mb-0"
        >
          {{ errorMessage }}
        </v-alert>
      </v-card-text>

      <!-- Card Actions -->
      <v-card-actions class="py-4 px-6 border-t d-flex justify-end gap-2">
        <v-btn
          v-if="!isDownloading"
          variant="text"
          color="medium-emphasis"
          @click="dismissModal"
        >
          {{ t('updater.later', 'Để sau') }}
        </v-btn>

        <v-btn
          color="primary"
          variant="elevated"
          :prepend-icon="isDownloading ? undefined : 'mdi-download'"
          :loading="isDownloading"
          :disabled="isDownloading"
          @click="downloadAndInstallUpdate"
        >
          {{ isDownloading ? t('updater.updating', 'Đang cập nhật...') : t('updater.updateNow', 'Cập nhật ngay') }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { useAppUpdater } from '@/composables/useAppUpdater'
import { useI18n } from '@/composables/useI18n'

const {
  showUpdateModal,
  isDownloading,
  downloadProgress,
  downloadedBytes,
  totalBytes,
  updateManifest,
  errorMessage,
  downloadAndInstallUpdate,
  dismissModal
} = useAppUpdater()

const { t } = useI18n()

function formatBytes(bytes: number, decimals = 1) {
  if (!bytes) return '0 B'
  const k = 1024
  const dm = decimals < 0 ? 0 : decimals
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`
}
</script>

<style scoped>
.gap-2 {
  gap: 8px;
}
.gap-3 {
  gap: 12px;
}
.release-notes-box {
  max-height: 180px;
  overflow-y: auto;
  line-height: 1.6;
}
</style>