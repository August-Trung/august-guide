<template>
  <v-dialog v-model="visible" max-width="550px" persistent>
    <v-card class="export-dialog-card pa-4">
      <v-card-title class="d-flex align-center pb-2 border-bottom">
        <v-icon color="secondary" class="mr-2">mdi-export</v-icon>
        <span class="text-h6 font-weight-bold text-white">{{ t('exportDialog.exportCenter') }}</span>
      </v-card-title>

      <!-- Tabs Header -->
      <v-tabs v-model="activeTab" bg-color="transparent" color="secondary" class="border-bottom mb-4" grow>
        <v-tab value="standard" class="text-none">{{ t('exportDialog.standardReport') }}</v-tab>
        <v-tab value="aacp" class="text-none">{{ t('exportDialog.aacpTab') }}</v-tab>
      </v-tabs>

      <v-card-text class="pa-0 pt-2">
        <v-window v-model="activeTab" class="bg-transparent border-0">
          <!-- Tab 1: Standard Reports -->
          <v-window-item value="standard" class="pa-1">
            <p class="text-body-1 text-white mb-2 font-weight-medium">
              Session: <span class="text-secondary">{{ sessionTitle }}</span>
            </p>
            <p class="text-body-2 text-medium-emphasis mb-4">
              {{ formatDescription }}
            </p>

            <!-- Options Form -->
            <div v-if="!isExporting && !statusMessage" class="mt-4">
              <!-- Format Selector -->
              <v-select
                v-model="selectedFormat"
                :items="formats"
                :label="t('exportDialog.exportFormat')"
                variant="outlined"
                density="comfortable"
                class="mb-4"
              ></v-select>

              <!-- Severity Filter -->
              <div class="mb-4">
                <span class="text-subtitle-2 text-white d-block mb-1 font-weight-medium">{{ t('exportDialog.includeSeverities') }}</span>
                <div class="d-flex flex-wrap">
                  <v-checkbox
                    v-for="sev in severityOptions"
                    :key="sev"
                    v-model="selectedSeverities"
                    :label="sev"
                    :value="sev"
                    density="compact"
                    hide-details
                    color="secondary"
                    class="mr-3 mb-1"
                  ></v-checkbox>
                </div>
              </div>

              <!-- Status Filter -->
              <div class="mb-4">
                <span class="text-subtitle-2 text-white d-block mb-1 font-weight-medium">{{ t('exportDialog.includeStatuses') }}</span>
                <div class="d-flex flex-wrap">
                  <v-checkbox
                    v-for="stat in statusOptions"
                    :key="stat"
                    v-model="selectedStatuses"
                    :label="stat"
                    :value="stat"
                    density="compact"
                    hide-details
                    color="secondary"
                    class="mr-3 mb-1"
                  ></v-checkbox>
                </div>
              </div>
            </div>
          </v-window-item>

          <!-- Tab 2: AACP (AI Agent) -->
          <v-window-item value="aacp" class="pa-1">
            <p class="text-body-2 text-medium-emphasis mb-4">
              {{ t('exportDialog.aacpDesc') }}
            </p>

            <div v-if="!isExporting && !statusMessage">
              <!-- Step 1: Issue Dropdown -->
              <div class="text-subtitle-2 text-white mb-2 font-weight-medium">
                {{ t('exportDialog.selectIssue') }}
              </div>
              <v-select
                v-model="selectedIssueId"
                :items="issuesList"
                item-title="displayName"
                item-value="id"
                variant="outlined"
                density="comfortable"
                class="mb-4"
                hide-details
              ></v-select>

              <!-- Step 2: Workspace Path Picker -->
              <div class="text-subtitle-2 text-white mb-2 font-weight-medium mt-3">
                {{ t('exportDialog.targetWorkspace') }}
              </div>
              <div class="d-flex align-center gap-2 mb-4">
                <v-text-field
                  v-model="workspacePath"
                  :placeholder="t('exportDialog.workspacePlaceholder')"
                  variant="outlined"
                  density="comfortable"
                  hide-details
                  class="flex-grow-1"
                ></v-text-field>
                <v-btn
                  color="secondary"
                  variant="outlined"
                  height="48"
                  class="text-none"
                  @click="pickWorkspaceDirectory"
                >
                  {{ t('common.browse') }}
                </v-btn>
              </div>

              <!-- Step 3: Suspected Files Textarea -->
              <div class="text-subtitle-2 text-white mb-2 font-weight-medium mt-3 d-flex justify-space-between align-center">
                <span>{{ t('exportDialog.suspectedFiles') }}</span>
                <span class="text-caption text-medium-emphasis font-weight-regular">{{ t('sidebar.optional') }}</span>
              </div>
              <v-textarea
                v-model="suspectedFilesText"
                :placeholder="t('exportDialog.suspectedPlaceholder')"
                variant="outlined"
                density="comfortable"
                rows="2"
                class="mb-4"
                hide-details
              ></v-textarea>

              <!-- ZIP Option -->
              <v-checkbox
                v-model="compressZip"
                :label="t('exportDialog.zipOption')"
                density="compact"
                color="secondary"
                class="mb-2 mt-2"
                hide-details
              ></v-checkbox>

              <!-- Step 4: Output Save Directory -->
              <div class="text-subtitle-2 text-white mb-2 font-weight-medium mt-3">
                {{ t('exportDialog.saveDestination') }}
              </div>
              <div class="d-flex align-center gap-2 mb-4">
                <v-text-field
                  v-model="outputDir"
                  :placeholder="t('exportDialog.destinationPlaceholder')"
                  variant="outlined"
                  density="comfortable"
                  hide-details
                  class="flex-grow-1"
                ></v-text-field>
                <v-btn
                  color="secondary"
                  variant="outlined"
                  height="48"
                  class="text-none"
                  @click="pickOutputDirectory"
                >
                  {{ t('common.browse') }}
                </v-btn>
              </div>
            </div>
          </v-window-item>
        </v-window>

        <!-- Loading State / Success Status -->
        <div v-if="isExporting" class="d-flex flex-column align-center justify-center py-8">
          <v-progress-circular indeterminate color="secondary" class="mb-2"></v-progress-circular>
          <span class="text-caption text-secondary">{{ t('exportDialog.generating') }}</span>
        </div>

        <!-- Inline Status Alerts -->
        <v-alert
          v-if="statusMessage && !isExporting"
          :type="statusType"
          variant="tonal"
          class="mt-4 mb-2"
          closable
          @click:close="statusMessage = ''"
        >
          {{ statusMessage }}
        </v-alert>
      </v-card-text>

      <v-card-actions class="d-flex justify-end gap-2 px-4 pb-2 mt-4">
        <v-btn
          variant="text"
          color="medium-emphasis"
          class="text-none"
          :disabled="isExporting"
          @click="close"
        >
          {{ t('common.close') }}
        </v-btn>
        
        <template v-if="!isExporting && !statusMessage">
          <!-- Standard Export Button -->
          <v-btn
            v-if="activeTab === 'standard'"
            variant="elevated"
            color="secondary"
            class="text-none"
            prepend-icon="mdi-file-document-outline"
            :disabled="isExportDisabled"
            @click="handleExport"
          >
            {{ t('exportDialog.chooseSavePath', 'Choose Save Path') }}
          </v-btn>

          <!-- AACP Export Button -->
          <v-btn
            v-else
            variant="elevated"
            color="primary"
            class="text-none"
            prepend-icon="mdi-robot"
            :disabled="isAacpExportDisabled"
            @click="handleAacpExport"
          >
            {{ t('exportDialog.exportContextPack', 'Export Context Pack') }}
          </v-btn>
        </template>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { save, open } from '@tauri-apps/plugin-dialog'
import { exportSession, exportAacpPack, getDownloadDir } from '@/services/tauriCommands'
import { useIssueStore } from '@/stores/issueStore'
import { useSettingsStore } from '@/stores/settingsStore'
import { useI18n } from '@/composables/useI18n'

const { t } = useI18n()

const props = defineProps<{
  modelValue: boolean
  sessionId: string
  sessionTitle: string
  issueId?: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
}>()

const visible = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val),
})

const activeTab = ref<'standard' | 'aacp'>('standard')

// Standard Report State
const isExporting = ref(false)
const statusMessage = ref('')
const statusType = ref<'success' | 'error'>('success')
const selectedFormat = ref<'html' | 'pdf' | 'markdown' | 'csv'>('html')
const formats = [
  { title: 'HTML Webpage (.html)', value: 'html' },
  { title: 'PDF Document (.pdf)', value: 'pdf' },
  { title: 'Markdown Document (.md)', value: 'markdown' },
  { title: 'CSV Tabular Sheet (.csv)', value: 'csv' },
]

const severityOptions = ['Critical', 'Major', 'Minor', 'Info']
const selectedSeverities = ref<string[]>(['Critical', 'Major', 'Minor', 'Info'])
const statusOptions = ['Draft', 'Open', 'In Progress', 'Resolved', 'Closed']
const selectedStatuses = ref<string[]>(['Draft', 'Open', 'In Progress', 'Resolved', 'Closed'])

const isExportDisabled = computed(() => {
  return selectedSeverities.value.length === 0 || selectedStatuses.value.length === 0
})

// AACP State
const issueStore = useIssueStore()
const settingsStore = useSettingsStore()

const selectedIssueId = ref<string | null>(null)
const workspacePath = ref('')
const outputDir = ref('')
const suspectedFilesText = ref('')
const compressZip = ref(true)

const issuesList = computed(() => {
  const list = issueStore.issues.map(iss => ({
    id: iss.id,
    displayName: `#${iss.markerNumber} - ${iss.title} (${iss.issueType})`
  }))
  list.unshift({
    id: 'all',
    displayName: t('exportDialog.allIssues')
  })
  return list
})

const suspectedFiles = computed(() => {
  return suspectedFilesText.value
    .split('\n')
    .map(s => s.trim())
    .filter(s => s.length > 0)
})

const isAacpExportDisabled = computed(() => {
  return !selectedIssueId.value || !workspacePath.value.trim() || !outputDir.value.trim()
})

const formatDescription = computed(() => {
  switch (selectedFormat.value) {
    case 'html':
      return 'This will generate a self-contained HTML document containing all annotated screenshots, crops, details, and metadata. All assets are base64-embedded, meaning the file can be shared and opened offline in any web browser.'
    case 'pdf':
      return "This will generate a portable PDF document of the session report using the system's Edge browser printer. Ideal for sharing and printing."
    case 'markdown':
      return 'This will generate a Markdown (.md) document of the session report. Images will be saved inside a folder alongside the markdown file, making the report portable.'
    case 'csv':
      return 'This will generate a spreadsheet-compatible CSV file containing tabular data of the issues (ID, Title, Severity, Type, Status, Description, ScreenshotPath).'
    default:
      return ''
  }
})

// Watch visible state to initialize and load data
watch(visible, async (newVal) => {
  if (newVal) {
    statusMessage.value = ''
    isExporting.value = false
    
    // Load issues for the session
    await issueStore.fetchIssuesBySession(props.sessionId)
    
    // Pre-select tab and issue based on props
    if (props.issueId) {
      activeTab.value = 'aacp'
      selectedIssueId.value = props.issueId
    } else {
      activeTab.value = 'standard'
      selectedIssueId.value = 'all'
    }
    
    // Load last workspace path and output path
    await settingsStore.loadSettings()
    workspacePath.value = settingsStore.getSettingValue<string>('last_workspace_path', '')
    
    const savedOutputDir = settingsStore.getSettingValue<string>('last_aacp_output_dir', '')
    if (savedOutputDir) {
      outputDir.value = savedOutputDir
    } else {
      try {
        const downloadFolder = await getDownloadDir()
        outputDir.value = downloadFolder
      } catch (err) {
        console.error('Failed to get download directory:', err)
        outputDir.value = ''
      }
    }
    suspectedFilesText.value = ''
  }
})

const close = () => {
  if (isExporting.value) return
  visible.value = false
  statusMessage.value = ''
}

const handleExport = async () => {
  statusMessage.value = ''
  try {
    const format = selectedFormat.value
    const safeTitle = props.sessionTitle.replace(/[^a-z0-9]/gi, '_').toLowerCase()
    let defaultFilename = `${safeTitle}_report`
    let extension = 'html'
    let formatFilterName = 'HTML Document'
    
    if (format === 'pdf') {
      extension = 'pdf'
      formatFilterName = 'PDF Document'
    } else if (format === 'markdown') {
      extension = 'md'
      formatFilterName = 'Markdown Document'
    } else if (format === 'csv') {
      extension = 'csv'
      formatFilterName = 'CSV File'
    }
    
    defaultFilename = `${defaultFilename}.${extension}`

    const selectedPath = await save({
      title: `Export ${format.toUpperCase()} Report`,
      defaultPath: defaultFilename,
      filters: [
        {
          name: formatFilterName,
          extensions: [extension],
        },
      ],
    })

    if (!selectedPath) {
      return
    }

    isExporting.value = true
    await exportSession(
      props.sessionId,
      selectedPath,
      format,
      selectedSeverities.value,
      selectedStatuses.value
    )
    
    statusType.value = 'success'
    statusMessage.value = `Report exported successfully to: ${selectedPath}`
  } catch (err: any) {
    console.error('Failed to export session:', err)
    statusType.value = 'error'
    statusMessage.value = `Export failed: ${err.message || String(err)}`
  } finally {
    isExporting.value = false
  }
}

const pickWorkspaceDirectory = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Workspace Directory',
      defaultPath: workspacePath.value || undefined
    })
    
    if (selected && typeof selected === 'string') {
      workspacePath.value = selected
    }
  } catch (err) {
    console.error('Failed to pick directory:', err)
  }
}

const pickOutputDirectory = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Destination Output Directory',
      defaultPath: outputDir.value || undefined
    })
    
    if (selected && typeof selected === 'string') {
      outputDir.value = selected
    }
  } catch (err) {
    console.error('Failed to pick destination directory:', err)
  }
}

const handleAacpExport = async () => {
  if (!selectedIssueId.value || !workspacePath.value.trim() || !outputDir.value.trim()) return
  
  statusMessage.value = ''
  isExporting.value = true
  
  try {
    // Save settings
    await settingsStore.setSettingValue('last_workspace_path', workspacePath.value.trim())
    await settingsStore.setSettingValue('last_aacp_output_dir', outputDir.value.trim())

    const finalPath = await exportAacpPack(
      props.sessionId,
      selectedIssueId.value,
      workspacePath.value.trim(),
      suspectedFiles.value,
      outputDir.value.trim(),
      compressZip.value
    )
    
    statusType.value = 'success'
    statusMessage.value = `AI Context Pack exported successfully to:\n${finalPath}`
  } catch (err: any) {
    console.error('Failed to export AACP:', err)
    statusType.value = 'error'
    statusMessage.value = `Export failed: ${err.message || String(err)}`
  } finally {
    isExporting.value = false
  }
}
</script>

<style scoped>
.export-dialog-card {
  background: #1e1e1e !important;
  border: 1px solid rgba(255, 255, 255, 0.08) !important;
  border-radius: 8px;
}

.border-bottom {
  border-bottom: 1px solid rgba(255, 255, 255, 0.08) !important;
}

.gap-2 {
  gap: 8px;
}
</style>
