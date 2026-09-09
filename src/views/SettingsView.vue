<template>
  <v-container class="py-6 px-8 max-width-container" fluid>
    <div class="d-flex align-center justify-space-between mb-6">
      <div>
        <h1 class="text-h4 font-weight-bold text-white mb-1">{{ t('common.settings') }}</h1>
        <p class="text-subtitle-1 text-medium-emphasis">{{ t('settingsView.settingsSubtitle', 'Configure August Mark to match your workflow') }}</p>
      </div>
      <v-chip color="success" variant="outlined" prepend-icon="mdi-cloud-check" size="small">
        {{ t('settingsView.settingsSaved') }}
      </v-chip>
    </div>

    <v-row>
      <v-col cols="12" md="4" lg="3">
        <v-card class="mb-4" border variant="flat">
          <v-tabs v-model="activeTab" direction="vertical" color="primary">
            <v-tab value="general" prepend-icon="mdi-cog">{{ t('settingsView.general') }}</v-tab>
            <v-tab value="capture" prepend-icon="mdi-camera">{{ t('settingsView.captureHotkeys') }}</v-tab>
            <v-tab value="sync" prepend-icon="mdi-cloud-sync">{{ t('settingsView.cloudSync') }}</v-tab>
            <v-tab value="info" prepend-icon="mdi-information">{{ t('settingsView.systemInfo') }}</v-tab>
          </v-tabs>
        </v-card>
      </v-col>

      <v-col cols="12" md="8" lg="9">
        <v-window v-model="activeTab">
          <!-- General Tab -->
          <v-window-item value="general">
            <v-card border variant="flat" class="pa-6">
              <h2 class="text-h5 font-weight-bold text-primary mb-6">{{ t('settingsView.generalPrefs') }}</h2>
              
              <!-- Theme Selection -->
              <div class="mb-6">
                <div class="text-subtitle-1 font-weight-bold mb-1">{{ t('settingsView.themeTitle') }}</div>
                <div class="text-body-2 text-medium-emphasis mb-3">{{ t('settingsView.themeDesc') }}</div>
                <v-radio-group v-model="settingsStore.theme" inline class="theme-radios">
                  <v-radio :label="t('settingsView.themeDark')" value="dark" class="mr-4"></v-radio>
                  <v-radio :label="t('settingsView.themeLight')" value="light" class="mr-4"></v-radio>
                  <v-radio :label="t('settingsView.themeSystem')" value="system"></v-radio>
                </v-radio-group>
              </div>

              <v-divider class="my-6"></v-divider>

              <!-- Language Selection -->
              <div class="mb-6">
                <div class="text-subtitle-1 font-weight-bold mb-1">{{ t('settingsView.displayLanguage') }}</div>
                <div class="text-body-2 text-medium-emphasis mb-3">{{ t('settingsView.languageDesc') }}</div>
                <v-select
                  v-model="selectedLanguage"
                  :items="languages"
                  item-title="title"
                  item-value="value"
                  variant="outlined"
                  density="comfortable"
                  max-width="300"
                ></v-select>
              </div>

              <v-divider class="my-6"></v-divider>

              <!-- Default Project -->
              <div>
                <div class="text-subtitle-1 font-weight-bold mb-1">{{ t('settingsView.defaultProject') }}</div>
                <div class="text-body-2 text-medium-emphasis mb-3">{{ t('settingsView.defaultProjectDesc') }}</div>
                <v-select
                  v-model="settingsStore.defaultProjectId"
                  :items="projects"
                  item-title="name"
                  item-value="id"
                  variant="outlined"
                  density="comfortable"
                  max-width="400"
                  :loading="projectStore.isLoading"
                ></v-select>
              </div>
            </v-card>
          </v-window-item>

          <!-- Capture & Hotkeys Tab -->
          <v-window-item value="capture">
            <v-card border variant="flat" class="pa-6">
              <h2 class="text-h5 font-weight-bold text-primary mb-6">{{ t('settingsView.capturePrefs') }}</h2>

              <!-- Delayed Capture -->
              <div class="mb-6">
                <div class="text-subtitle-1 font-weight-bold mb-1">{{ t('settingsView.delayedCapture') }}</div>
                <div class="text-body-2 text-medium-emphasis mb-3">{{ t('settingsView.delayedCaptureDesc') }}</div>
                <v-select
                  v-model="delayedCapture"
                  :items="delayOptions"
                  item-title="label"
                  item-value="value"
                  variant="outlined"
                  density="comfortable"
                  max-width="300"
                ></v-select>
              </div>

              <v-divider class="my-6"></v-divider>

              <!-- Image Quality -->
              <div class="mb-6">
                <div class="text-subtitle-1 font-weight-bold mb-1">{{ t('settingsView.screenshotQuality') }} ({{ settingsStore.screenshotQuality }}%)</div>
                <div class="text-body-2 text-medium-emphasis mb-3">{{ t('settingsView.screenshotQualityDesc') }}</div>
                <div class="d-flex align-center gap-4">
                  <v-slider
                    v-model="settingsStore.screenshotQuality"
                    :min="50"
                    :max="100"
                    :step="5"
                    color="primary"
                    track-color="surface-variant"
                    hide-details
                  ></v-slider>
                </div>
              </div>

              <v-divider class="my-6"></v-divider>

              <!-- Mouse Hold Duration -->
              <div class="mb-6">
                <div class="text-subtitle-1 font-weight-bold mb-1">{{ t('settingsView.holdDuration') }} ({{ settingsStore.holdDurationMs }}ms)</div>
                <div class="text-body-2 text-medium-emphasis mb-3">{{ t('settingsView.holdDurationDesc') }}</div>
                <v-slider
                  v-model="settingsStore.holdDurationMs"
                  :min="500"
                  :max="2000"
                  :step="100"
                  color="primary"
                  track-color="surface-variant"
                  hide-details
                ></v-slider>
              </div>

              <v-divider class="my-6"></v-divider>

              <!-- Minimize to Tray -->
              <div>
                <div class="text-subtitle-1 font-weight-bold mb-1">{{ t('settingsView.minimizeToTray') }}</div>
                <div class="text-body-2 text-medium-emphasis mb-3">{{ t('settingsView.minimizeDesc') }}</div>
                <v-switch
                  v-model="settingsStore.minimizeToTray"
                  color="primary"
                  inset
                  hide-details
                ></v-switch>
              </div>

              <v-divider class="my-6"></v-divider>

              <!-- Auto-save to custom directory -->
              <div>
                <div class="text-subtitle-1 font-weight-bold mb-1">{{ t('settingsView.autoSaveTitle') }}</div>
                <div class="text-body-2 text-medium-emphasis mb-3">{{ t('settingsView.autoSaveDesc') }}</div>
                <v-switch
                  v-model="settingsStore.autoSaveToCustomDir"
                  color="primary"
                  inset
                  hide-details
                  class="mb-3"
                ></v-switch>

                <div v-if="settingsStore.autoSaveToCustomDir" class="d-flex align-center gap-3 mt-3">
                  <v-text-field
                    v-model="settingsStore.customSaveDir"
                    :label="t('settingsView.customSaveDir')"
                    :placeholder="t('settingsView.noFolderSelected')"
                    variant="outlined"
                    density="comfortable"
                    hide-details
                    readonly
                    class="flex-grow-1"
                    prepend-inner-icon="mdi-folder-outline"
                  ></v-text-field>
                  <v-btn
                    color="primary"
                    variant="tonal"
                    prepend-icon="mdi-folder-open-outline"
                    @click="handleSelectCustomDir"
                  >
                    {{ t('settingsView.browseFolder') }}
                  </v-btn>
                  <v-btn
                    v-if="settingsStore.customSaveDir"
                    icon="mdi-open-in-new"
                    variant="text"
                    color="primary"
                    :title="t('settingsView.openFolder')"
                    @click="handleOpenCustomDir"
                  ></v-btn>
                </div>
              </div>
            </v-card>
          </v-window-item>

          <!-- Cloud Sync Tab -->
          <v-window-item value="sync">
            <v-card border variant="flat" class="pa-6">
              <h2 class="text-h5 font-weight-bold text-primary mb-6">{{ t('settingsView.cloudSync') }}</h2>
              
              <!-- Google Drive Connection -->
              <div class="mb-6">
                <div class="text-subtitle-1 font-weight-bold mb-1">{{ t('settingsView.gdriveTitle') }}</div>
                <div class="text-body-2 text-medium-emphasis mb-4">
                  {{ t('settingsView.gdriveDesc') }}
                </div>

                <div v-if="settingsStore.gdriveConnected" class="d-flex align-center flex-wrap gap-4 pa-4 border rounded bg-surface-variant">
                  <v-avatar color="success" size="50">
                    <v-icon icon="mdi-google-drive" color="white" size="30"></v-icon>
                  </v-avatar>
                  <div class="flex-grow-1">
                    <div class="font-weight-bold text-white d-flex align-center">
                      {{ t('settingsView.connectedAs') }}
                      <v-chip color="success" size="x-small" class="ml-2" variant="flat">{{ t('header.active') }}</v-chip>
                    </div>
                    <div class="text-body-2 text-medium-emphasis">{{ settingsStore.gdriveEmail || 'No email associated' }}</div>
                  </div>
                  <v-btn
                    color="error"
                    variant="outlined"
                    prepend-icon="mdi-logout"
                    :loading="settingsStore.loading"
                    @click="handleDisconnect"
                  >
                    {{ t('settingsView.disconnectBtn') }}
                  </v-btn>
                </div>

                <div v-else class="pa-4 border rounded bg-surface-variant d-flex align-center justify-space-between flex-wrap gap-4">
                  <div class="d-flex align-center gap-3">
                    <v-avatar color="grey-darken-3" size="50">
                      <v-icon icon="mdi-google-drive" color="grey-lighten-1" size="30"></v-icon>
                    </v-avatar>
                    <div>
                      <div class="font-weight-bold text-white">{{ t('settingsView.disconnected') }}</div>
                      <div class="text-body-2 text-medium-emphasis">{{ t('settingsView.notConnected') }}</div>
                    </div>
                  </div>
                  <v-btn
                    color="primary"
                    prepend-icon="mdi-login"
                    :loading="settingsStore.loading"
                    @click="handleConnect"
                  >
                    {{ t('settingsView.connectBtn') }}
                  </v-btn>
                </div>
              </div>

              <!-- Backup configuration (Only show if connected) -->
              <div v-if="settingsStore.gdriveConnected">
                <v-divider class="my-6"></v-divider>

                <!-- Auto Backup Toggle -->
                <div class="d-flex align-center justify-space-between mb-6">
                  <div>
                    <div class="text-subtitle-1 font-weight-bold mb-1">{{ t('settingsView.autoBackup') }}</div>
                    <div class="text-body-2 text-medium-emphasis">{{ t('settingsView.autoBackupDesc', 'Automatically backup database and screenshots to Google Drive on closure') }}</div>
                  </div>
                  <v-switch
                    v-model="settingsStore.autoBackup"
                    color="primary"
                    inset
                    hide-details
                  ></v-switch>
                </div>

                <v-divider class="my-6"></v-divider>

                <!-- Manual Backup & Restore Section -->
                <div>
                  <div class="text-subtitle-1 font-weight-bold mb-1">{{ t('settingsView.manualBackup') }}</div>
                  <div class="text-body-2 text-medium-emphasis mb-4">
                    {{ t('settingsView.manualBackupDesc') }}
                  </div>

                  <div class="d-flex align-center gap-4 mb-6">
                    <v-btn
                      color="primary"
                      prepend-icon="mdi-cloud-upload"
                      :loading="settingsStore.backupLoading"
                      @click="handleManualBackup"
                    >
                      {{ t('settingsView.backupNow') }}
                    </v-btn>

                    <v-btn
                      color="secondary"
                      variant="outlined"
                      prepend-icon="mdi-cloud-download"
                      :loading="settingsStore.backupLoading"
                      @click="openRestoreDialog"
                    >
                      {{ t('settingsView.restoreBtn') }}
                    </v-btn>
                  </div>

                  <div v-if="settingsStore.gdriveLastBackupTime" class="text-body-2 text-medium-emphasis">
                    <v-icon icon="mdi-clock" size="small" class="mr-1"></v-icon>
                    Last successful backup: <span class="text-white font-weight-medium">{{ settingsStore.gdriveLastBackupTime }}</span>
                  </div>
                  <div v-else class="text-body-2 text-medium-emphasis">
                    <v-icon icon="mdi-clock-alert" size="small" class="mr-1"></v-icon>
                    {{ t('settingsView.noBackups') }}
                  </div>
                </div>
              </div>
            </v-card>
          </v-window-item>

          <!-- System Info Tab -->
          <v-window-item value="info">
            <v-card border variant="flat" class="pa-6">
              <h2 class="text-h5 font-weight-bold text-primary mb-6">{{ t('settingsView.sysInfoTitle') }}</h2>
              
              <v-row class="mb-4">
                <v-col cols="12" sm="6" md="4" v-for="stat in statCards" :key="stat.title">
                  <v-card border variant="flat" class="pa-4 bg-surface-variant d-flex align-center gap-3">
                    <v-avatar size="40" :color="stat.color" rounded>
                      <v-icon :icon="stat.icon" color="white"></v-icon>
                    </v-avatar>
                    <div>
                      <div class="text-caption text-medium-emphasis">{{ stat.title }}</div>
                      <div class="text-h5 font-weight-bold text-white">{{ stat.value }}</div>
                    </div>
                  </v-card>
                </v-col>
              </v-row>

              <v-divider class="my-6"></v-divider>

              <div class="d-flex align-center justify-space-between py-2">
                <div>
                  <div class="font-weight-medium">App Version</div>
                  <div class="text-caption text-medium-emphasis">Current installed version of August Mark</div>
                </div>
                <div class="font-weight-bold text-white">{{ appVersion }}</div>
              </div>

              <v-divider></v-divider>

              <div class="d-flex align-center justify-space-between py-2">
                <div>
                  <div class="font-weight-medium">Database File Size</div>
                  <div class="text-caption text-medium-emphasis">Local SQLite database file size</div>
                </div>
                <div class="font-weight-bold text-white">{{ formatBytes(dbSize) }}</div>
              </div>

              <v-divider></v-divider>

              <div class="d-flex align-center justify-space-between py-2">
                <div>
                  <div class="font-weight-medium">{{ t('settingsView.databaseFile') }}</div>
                  <div class="text-caption text-medium-emphasis">Local storage directory for captures and DB</div>
                </div>
                <div class="d-flex align-center gap-2">
                  <div class="text-caption font-weight-bold text-primary text-truncate max-width-path">
                    {{ dbLocation }}
                  </div>
                  <v-btn
                    icon="mdi-folder-open"
                    variant="tonal"
                    size="small"
                    color="primary"
                    :title="t('sessionView.openFolder', 'Mở thư mục lưu trữ')"
                    @click="openStorageFolder"
                  ></v-btn>
                </div>
              </div>

              <v-divider class="my-6"></v-divider>

              <!-- Uninstall App Section (Danger Zone) -->
              <div class="pa-4 border rounded bg-surface-variant d-flex align-center justify-space-between flex-wrap gap-4" style="border-color: rgba(255, 82, 82, 0.4) !important;">
                <div>
                  <div class="font-weight-bold text-error d-flex align-center mb-1">
                    <v-icon icon="mdi-alert-octagon-outline" color="error" class="mr-2"></v-icon>
                    Gỡ cài đặt August Guide
                  </div>
                  <div class="text-caption text-medium-emphasis">
                    Gỡ bỏ ứng dụng hoàn toàn khỏi máy tính qua trình gỡ cài đặt hệ thống.
                  </div>
                </div>
                <v-btn
                  color="error"
                  variant="flat"
                  prepend-icon="mdi-delete-forever"
                  @click="showUninstallConfirm = true"
                >
                  Gỡ cài đặt
                </v-btn>
              </div>
            </v-card>
          </v-window-item>
        </v-window>
      </v-col>
    </v-row>

    <!-- Uninstall Confirmation Dialog -->
    <v-dialog v-model="showUninstallConfirm" max-width="480">
      <v-card border>
        <v-card-title class="text-h6 font-weight-bold py-4 px-6 border-b text-error d-flex align-center">
          <v-icon icon="mdi-alert" color="error" class="mr-2"></v-icon>
          Xác nhận gỡ cài đặt August Guide
        </v-card-title>
        <v-card-text class="pa-6">
          <p class="text-body-1 mb-2">
            Bạn có chắc chắn muốn gỡ cài đặt <strong>August Guide</strong> khỏi máy tính?
          </p>
          <p class="text-caption text-medium-emphasis mb-0">
            Thao tác này sẽ khởi động trình gỡ cài đặt của Windows và tự động đóng ứng dụng.
          </p>
        </v-card-text>
        <v-card-actions class="py-4 px-6 border-t d-flex justify-end gap-2">
          <v-btn variant="text" @click="showUninstallConfirm = false">Hủy bỏ</v-btn>
          <v-btn
            color="error"
            variant="flat"
            prepend-icon="mdi-delete-forever"
            :loading="isUninstalling"
            @click="handleUninstall"
          >
            Đồng ý gỡ cài đặt
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Restore Dialog -->
    <v-dialog v-model="restoreDialog" max-width="600">
      <v-card border>
        <v-card-title class="d-flex align-center justify-space-between py-4 px-6 border-b">
          <span class="text-h6 font-weight-bold">Restore Backup from Google Drive</span>
          <v-btn icon="mdi-close" variant="text" density="comfortable" @click="restoreDialog = false"></v-btn>
        </v-card-title>

        <v-card-text class="pa-6">
          <div class="text-body-2 text-medium-emphasis mb-4">
            {{ t('settingsView.selectBackup') }}
          </div>

          <v-alert
            type="warning"
            variant="tonal"
            border="start"
            class="mb-6"
            density="comfortable"
          >
            <strong>{{ t('common.error') }}:</strong> {{ t('settingsView.restoreWarning') }}
          </v-alert>

          <!-- Loading backups state -->
          <div v-if="loadingBackups" class="d-flex flex-column align-center py-6">
            <v-progress-circular indeterminate color="primary" class="mb-2"></v-progress-circular>
            <span class="text-caption text-medium-emphasis">{{ t('common.loading') }}</span>
          </div>

          <!-- Empty list state -->
          <div v-else-if="backupsList.length === 0" class="py-6 text-center border rounded border-dashed">
            <v-icon icon="mdi-cloud-search" size="40" class="text-medium-emphasis mb-2"></v-icon>
            <div class="text-subtitle-1 font-weight-medium">{{ t('settingsView.noBackups') }}</div>
            <div class="text-caption text-medium-emphasis">Create a backup first or check your connection</div>
          </div>

          <!-- Backups List -->
          <v-list v-else border rounded class="pa-0 mb-6 max-height-list">
            <v-list-item
              v-for="[id, name] in backupsList"
              :key="id"
              :title="formatBackupName(name)"
              :subtitle="'ID: ' + id"
              prepend-icon="mdi-zip-box"
              :active="selectedBackupId === id"
              color="primary"
              class="border-b"
              @click="selectedBackupId = id"
            >
              <template v-slot:append>
                <v-icon v-if="selectedBackupId === id" icon="mdi-check-circle" color="primary"></v-icon>
              </template>
            </v-list-item>
          </v-list>

          <!-- Or Manual File ID Input -->
          <div class="mt-4">
            <div class="text-caption font-weight-bold text-medium-emphasis mb-1">OR ENTER FILE ID MANUALLY</div>
            <v-text-field
              v-model="manualFileId"
              label="Google Drive File ID"
              variant="outlined"
              density="comfortable"
              hide-details
              placeholder="e.g. 1a2b3c4d5e..."
            ></v-text-field>
          </div>
        </v-card-text>

        <v-card-actions class="py-4 px-6 border-t d-flex justify-end gap-2">
          <v-btn variant="text" @click="restoreDialog = false">Cancel</v-btn>
          <v-btn
            color="warning"
            variant="flat"
            :disabled="!selectedBackupId && !manualFileId"
            :loading="settingsStore.backupLoading"
            @click="confirmRestore"
          >
            Confirm Restore
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Inner Confirmation Dialog -->
    <v-dialog v-model="confirmDialog" max-width="450">
      <v-card border>
        <v-card-title class="text-h6 font-weight-bold py-4 px-6 border-b">
          Are you absolutely sure?
        </v-card-title>
        <v-card-text class="pa-6">
          {{ t('settingsView.restoreWarningDesc', 'This will delete all current guide documents, steps, and screenshots on your device and replace them with the data from the backup.') }}
        </v-card-text>
        <v-card-actions class="py-4 px-6 border-t d-flex justify-end gap-2">
          <v-btn variant="text" @click="confirmDialog = false">Cancel</v-btn>
          <v-btn
            color="error"
            variant="flat"
            :loading="settingsStore.backupLoading"
            @click="executeRestore"
          >
            Yes, Replace My Data
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Error Toast Notification -->
    <v-snackbar v-model="showError" color="error" timeout="6000" location="top">
      {{ errorMessage }}
      <template v-slot:actions>
        <v-btn variant="text" @click="showError = false">Close</v-btn>
      </template>
    </v-snackbar>

    <!-- Success Toast Notification -->
    <v-snackbar v-model="showSuccess" color="success" timeout="4000" location="top">
      {{ successMessage }}
      <template v-slot:actions>
        <v-btn variant="text" @click="showSuccess = false">Close</v-btn>
      </template>
    </v-snackbar>
  </v-container>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useSettingsStore } from '@/stores/settingsStore'
import { useProjectStore } from '@/stores/projectStore'
import { useUiStore } from '@/stores/uiStore'
import { getVersion } from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api/core'
import { openAppFolder } from '@/services/tauriCommands'
import { useI18n } from '@/composables/useI18n'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { open as shellOpen } from '@tauri-apps/plugin-shell'

const settingsStore = useSettingsStore()
const projectStore = useProjectStore()
const uiStore = useUiStore()
const { t } = useI18n()

const activeTab = ref('general')
const appVersion = ref('Loading...')
const dbSize = ref(0)
const dbLocation = ref('Loading...')

const openStorageFolder = async () => {
  try {
    await openAppFolder('root')
  } catch (e) {
    console.error('Failed to open storage folder:', e)
  }
}

const handleSelectCustomDir = async () => {
  try {
    const selected = await openDialog({
      directory: true,
      multiple: false,
      title: t('settingsView.browseFolder')
    })
    if (selected && typeof selected === 'string') {
      settingsStore.customSaveDir = selected
      uiStore.showToast({
        message: `${t('settingsView.customSaveDir')}: ${selected}`,
        type: 'success'
      })
    }
  } catch (e: any) {
    console.error('Failed to select directory:', e)
  }
}

const handleOpenCustomDir = async () => {
  if (settingsStore.customSaveDir) {
    try {
      await shellOpen(settingsStore.customSaveDir)
    } catch (e) {
      console.error('Failed to open directory:', e)
    }
  }
}

// Statistics state (populated dynamically in T1.07)
const stats = ref({
  projects: 0,
  sessions: 0,
  issues: 0
})

const languages = [
  { title: 'English', value: 'en' },
  { title: 'Tiếng Việt', value: 'vi' }
]

const selectedLanguage = computed({
  get: () => settingsStore.getSettingValue<string>('language', 'en'),
  set: (value) => settingsStore.setSettingValue('language', value)
})

const delayOptions = [
  { label: 'None (Immediate)', value: 0 },
  { label: '3 seconds', value: 3 },
  { label: '5 seconds', value: 5 },
  { label: '10 seconds', value: 10 }
]

const delayedCapture = computed({
  get: () => settingsStore.getSettingValue<number>('delayed_capture', 0),
  set: (value) => settingsStore.setSettingValue('delayed_capture', value)
})

const projects = computed(() => projectStore.projects)

const statCards = computed(() => [
  { title: t('settingsView.totalProjects', 'Total Projects'), value: stats.value.projects, icon: 'mdi-folder', color: 'primary' },
  { title: t('settingsView.totalSessions', 'Total Sessions'), value: stats.value.sessions, icon: 'mdi-clipboard-list', color: 'secondary' },
  { title: t('settingsView.totalSteps', 'Steps Recorded'), value: stats.value.issues, icon: 'mdi-format-list-numbered', color: 'info' }
])

onMounted(async () => {
  // Ensure projects are fetched
  if (projectStore.projects.length === 0) {
    await projectStore.fetchProjects()
  }

  try {
    appVersion.value = await getVersion()
  } catch (e) {
    console.error('Failed to get app version:', e)
    appVersion.value = '0.1.1'
  }

  // Load app stats if command exists (graceful fallback for early milestone steps)
  try {
    const appStats = await invoke<any>('get_app_stats')
    stats.value.projects = appStats.projectCount
    stats.value.sessions = appStats.sessionCount
    stats.value.issues = appStats.issueCount
    dbSize.value = appStats.dbSize
    dbLocation.value = appStats.dbLocation || 'Local Storage'
  } catch (e) {
    // Fallback values before T1.07 is implemented
    stats.value.projects = projectStore.projects.length
    stats.value.sessions = 0
    stats.value.issues = 0
    dbSize.value = 0
    dbLocation.value = '~/AppData/Roaming/AugustMark'
  }
})

const showError = ref(false)
const errorMessage = ref('')
const showSuccess = ref(false)
const successMessage = ref('')

// Uninstaller state
const showUninstallConfirm = ref(false)
const isUninstalling = ref(false)

async function handleUninstall() {
  isUninstalling.value = true
  try {
    const { uninstallApp } = await import('@/services/tauriCommands')
    await uninstallApp()
  } catch (e: any) {
    console.error('Failed to trigger uninstall:', e)
    errorMessage.value = e?.message || 'Không thể kích hoạt trình gỡ cài đặt'
    showError.value = true
    isUninstalling.value = false
    showUninstallConfirm.value = false
  }
}

// Restore state variables
const restoreDialog = ref(false)
const confirmDialog = ref(false)
const loadingBackups = ref(false)
const backupsList = ref<[string, string][]>([])
const selectedBackupId = ref<string | null>(null)
const manualFileId = ref('')

function formatBackupName(name: string): string {
  const match = name.match(/august_backup_(\d{4})(\d{2})(\d{2})_(\d{2})(\d{2})(\d{2})\.zip/)
  if (match) {
    const [_, y, m, d, hh, mm, ss] = match
    return `Backup — ${y}-${m}-${d} ${hh}:${mm}:${ss}`
  }
  return name
}

async function handleManualBackup() {
  try {
    const time = await settingsStore.backupNow()
    successMessage.value = `Backup created successfully at ${time}!`
    showSuccess.value = true
  } catch (e: any) {
    console.error('Backup failed:', e)
    errorMessage.value = e?.message || e || 'Backup failed'
    showError.value = true
  }
}

async function openRestoreDialog() {
  restoreDialog.value = true
  loadingBackups.value = true
  selectedBackupId.value = null
  manualFileId.value = ''
  try {
    const list = await settingsStore.listBackups()
    backupsList.value = list
  } catch (e: any) {
    console.error('Failed to load backups:', e)
    errorMessage.value = e?.message || e || 'Failed to list backups from Google Drive'
    showError.value = true
  } finally {
    loadingBackups.value = false
  }
}

function confirmRestore() {
  confirmDialog.value = true
}

async function executeRestore() {
  const fileId = selectedBackupId.value || manualFileId.value
  if (!fileId) return
  
  confirmDialog.value = false
  try {
    await settingsStore.restoreFrom(fileId)
    restoreDialog.value = false
    
    successMessage.value = 'Database and screenshots successfully restored!'
    showSuccess.value = true
    
    setTimeout(() => {
      window.location.reload()
    }, 1500)
  } catch (e: any) {
    console.error('Restore failed:', e)
    errorMessage.value = e?.message || e || 'Restore failed'
    showError.value = true
  }
}

async function handleConnect() {
  try {
    await settingsStore.connectGDrive()
  } catch (e: any) {
    console.error('Failed to connect:', e)
    errorMessage.value = e?.message || e || 'Failed to connect to Google Drive'
    showError.value = true
  }
}

async function handleDisconnect() {
  try {
    await settingsStore.disconnectGDrive()
  } catch (e) {
    console.error('Failed to disconnect:', e)
  }
}

function formatBytes(bytes: number, decimals = 2) {
  if (!bytes) return '0 Bytes'
  const k = 1024
  const dm = decimals < 0 ? 0 : decimals
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`
}
</script>

<script lang="ts">
export default {
  name: 'SettingsView'
}
</script>

<style scoped>
.max-width-container {
  max-width: 1200px;
  margin: 0 auto;
}
.gap-3 {
  gap: 12px;
}
.gap-4 {
  gap: 16px;
}
.max-width-path {
  max-width: 300px;
  display: inline-block;
}
.max-height-list {
  max-height: 250px;
  overflow-y: auto;
}
</style>
