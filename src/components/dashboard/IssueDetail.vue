<template>
  <v-card v-if="issue" class="fill-height bg-transparent border-0" flat>
    <!-- Card Toolbar -->
    <div class="d-flex align-center justify-space-between mb-6">
      <v-btn
        variant="text"
        prepend-icon="mdi-arrow-left"
        class="text-none pl-0"
        color="medium-emphasis"
        @click="goBack"
      >
        {{ t('issueDetail.backToSession') }}
      </v-btn>

      <div class="d-flex gap-2">
        <v-btn
          color="primary"
          variant="flat"
          prepend-icon="mdi-export-variant"
          class="text-none"
          @click="showExportDialog = true"
        >
          {{ t('common.export') }}
        </v-btn>

        <v-btn
          color="error"
          variant="outlined"
          prepend-icon="mdi-delete"
          class="text-none"
          @click="confirmDelete"
        >
          {{ t('issueDetail.deleteIssue') }}
        </v-btn>
      </div>
    </div>

    <!-- Layout Grid -->
    <v-row>
      <!-- Left side: Screenshot and Crop preview -->
      <v-col cols="12" md="8">
        <v-card bg-color="surface" variant="outlined" color="surface-variant" class="pa-4 mb-4">
          <div class="d-flex align-center justify-space-between mb-3">
            <div class="text-subtitle-1 font-weight-bold text-white">
              {{ t('issueDetail.annotatedScreenshot') }}
            </div>
            <div class="d-flex align-center gap-2">
              <v-btn
                variant="tonal"
                color="secondary"
                size="small"
                prepend-icon="mdi-download"
                class="text-none"
                :loading="isSavingImage"
                @click="saveImageAsFile"
              >
                {{ t('settingsView.saveAs') || 'Lưu file...' }}
              </v-btn>
              <v-btn
                variant="tonal"
                color="primary"
                size="small"
                prepend-icon="mdi-content-copy"
                class="text-none"
                :loading="isCopyingImage"
                @click="copyImageToClipboard"
              >
                {{ t('issueDetail.copyImage') }}
              </v-btn>
            </div>
          </div>
          <div class="screenshot-container bg-black rounded overflow-hidden position-relative">
            <v-img
              v-if="annotatedUrl"
              :src="annotatedUrl"
              contain
              max-height="600"
              class="mx-auto"
            >
              <template v-slot:placeholder>
                <div class="d-flex align-center justify-center fill-height bg-grey-darken-3">
                  <v-progress-circular indeterminate color="primary" size="48"></v-progress-circular>
                </div>
              </template>
            </v-img>

            <!-- Floating overlay displaying issue details on the image -->
            <div class="image-info-overlay pa-3 d-flex flex-column justify-end">
              <div class="d-flex align-center gap-2 mb-1">
                <v-avatar v-if="issue.markerNumber > 0" color="primary" size="20" class="text-white font-weight-bold text-caption">
                  {{ issue.markerNumber }}
                </v-avatar>
                <span class="text-subtitle-2 font-weight-bold text-white text-truncate">{{ title }}</span>
                <v-chip size="x-small" :color="severityColor" class="font-weight-bold text-uppercase">{{ t('severities.' + severity, severity) }}</v-chip>
              </div>
              <p class="text-caption text-medium-emphasis text-truncate mb-0">{{ description || t('common.noDescription') }}</p>
            </div>
          </div>
        </v-card>

        <!-- Crop Preview -->
        <v-card bg-color="surface" variant="outlined" color="surface-variant" class="pa-4">
          <div class="text-subtitle-1 font-weight-bold text-white mb-3">
            {{ t('issueDetail.issueCrop', 'Issue Crop (Zoomed View)') }}
          </div>
          <div class="crop-container d-flex align-center justify-center py-4 bg-grey-darken-4 rounded">
            <v-img
              v-if="cropUrl"
              :src="cropUrl"
              width="200"
              height="200"
              class="border rounded"
            >
              <template v-slot:placeholder>
                <div class="d-flex align-center justify-center fill-height bg-grey-darken-3">
                  <v-progress-circular indeterminate color="primary" size="24"></v-progress-circular>
                </div>
              </template>
            </v-img>
            <div v-else class="text-caption text-medium-emphasis">{{ t('issueDetail.noCropImage', 'No crop image generated') }}</div>
          </div>
        </v-card>
      </v-col>

      <!-- Right side: Editable Metadata Form -->
      <v-col cols="12" md="4">
        <v-card bg-color="surface" variant="outlined" color="surface-variant" class="pa-4 sticky-panel">
          <div class="d-flex align-center justify-space-between mb-4">
            <div class="text-subtitle-1 font-weight-bold text-white">
              {{ t('issueDetail.issueMetadata', 'Issue Metadata') }}
            </div>
            <v-avatar v-if="issue.markerNumber > 0" color="primary" size="32" class="text-white font-weight-bold">
              {{ issue.markerNumber }}
            </v-avatar>
          </div>

          <!-- Form Fields with Auto-Save -->
          <v-form ref="formRef">
            <!-- Title -->
            <v-text-field
              v-model="title"
              :label="t('common.title')"
              variant="outlined"
              density="comfortable"
              class="mb-3"
              @blur="autoSave"
            ></v-text-field>

            <!-- Issue Type -->
            <v-select
              v-model="issueType"
              :label="t('common.type', 'Type')"
              :items="issueTypes"
              variant="outlined"
              density="comfortable"
              class="mb-3"
              @update:model-value="autoSave"
            ></v-select>

            <!-- Severity -->
            <v-select
              v-model="severity"
              :label="t('common.severity')"
              :items="severities"
              variant="outlined"
              density="comfortable"
              class="mb-3"
              @update:model-value="autoSave"
            ></v-select>

            <!-- Status -->
            <v-select
              v-model="status"
              :label="t('common.status')"
              :items="statuses"
              variant="outlined"
              density="comfortable"
              class="mb-3"
              @update:model-value="autoSave"
            ></v-select>

            <!-- Description -->
            <v-textarea
              v-model="description"
              :label="t('common.description')"
              variant="outlined"
              density="comfortable"
              rows="6"
              class="mb-3"
              @blur="autoSave"
            ></v-textarea>

            <!-- Tags -->
            <v-combobox
              v-model="selectedTags"
              :label="t('issueDetail.tags', 'Tags')"
              :items="tagStore.tags.map(t => t.name)"
              variant="outlined"
              density="comfortable"
              multiple
              chips
              closable-chips
              :placeholder="t('issueDetail.tagPlaceholder', 'Type tag and press Enter')"
              class="mb-3"
              @update:model-value="autoSave"
            ></v-combobox>
          </v-form>

          <v-divider class="my-4"></v-divider>

          <!-- System Details -->
          <div class="system-details text-caption text-medium-emphasis">
            <div class="d-flex justify-space-between mb-1">
              <span>{{ t('sessionView.createdOn') }}:</span>
              <span>{{ formatDate(issue.createdAt) }}</span>
            </div>
            <div class="d-flex justify-space-between mb-1">
              <span>{{ t('issueDetail.updatedOn', 'Updated') }}:</span>
              <span>{{ formatDate(issue.updatedAt) }}</span>
            </div>
            <div class="d-flex justify-space-between">
              <span>{{ t('issueDetail.colorBadge', 'Color badge') }}:</span>
              <span :style="{ color: issue.color }" class="font-weight-bold">
                {{ issue.color }}
              </span>
            </div>
          </div>
        </v-card>
      </v-col>
    </v-row>

    <!-- Export Dialog for AI Context Pack -->
    <ExportDialog
      v-model="showExportDialog"
      :session-id="issue.sessionId"
      :session-title="sessionTitle"
      :issue-id="issue.id"
    />
  </v-card>
</template>

<script setup lang="ts">
import { ref, computed, watch, onBeforeUnmount } from 'vue'
import { useRouter } from 'vue-router'
import { convertFileSrc } from '@tauri-apps/api/core'
import { useIssueStore } from '@/stores/issueStore'
import { useTagStore } from '@/stores/tagStore'
import { useUiStore } from '@/stores/uiStore'
import { getCapture, getSession } from '@/services/tauriCommands'
import type { Issue } from '@/types/issue'
import { formatDateTime as formatDate } from '@/utils/date'
import ExportDialog from '@/components/export/ExportDialog.vue'
import { useI18n } from '@/composables/useI18n'

const { t } = useI18n()

const props = defineProps<{
  issue: Issue
}>()

const emit = defineEmits<{
  (e: 'delete', id: string): void
}>()

const router = useRouter()
const issueStore = useIssueStore()
const tagStore = useTagStore()

const showExportDialog = ref(false)
const sessionTitle = ref('Session Report')

watch(() => props.issue.sessionId, async (newId) => {
  if (newId) {
    try {
      const session = await getSession(newId)
      sessionTitle.value = session.title
    } catch (err) {
      console.error('Failed to load session details:', err)
    }
  }
}, { immediate: true })

tagStore.loadTags()

// Local Form States
const title = ref(props.issue.title)
const description = ref(props.issue.description)
const issueType = ref(props.issue.issueType)
const severity = ref(props.issue.severity)
const status = ref(props.issue.status)
const selectedTags = ref<string[]>((props.issue.tags || []).map(t => t.name))

const issueTypes = ['Step', 'Note', 'Warning', 'Tip']
const severities = ['Info', 'Minor', 'Major', 'Critical']
const statuses = ['Draft', 'Open', 'In Progress', 'Resolved', 'Closed']

const screenshotPath = ref('')

// Load capture details to get original screenshot path
const loadCapturePath = async () => {
  try {
    const capture = await getCapture(props.issue.captureId)
    screenshotPath.value = capture.screenshotPath
  } catch (err) {
    console.error('Failed to load capture info:', err)
  }
}
loadCapturePath()

// Converted asset URLs
const cropUrl = computed(() => {
  if (!props.issue.cropPath) return ''
  return convertFileSrc(props.issue.cropPath)
})

const annotatedUrl = computed(() => {
  if (props.issue.annotatedScreenshotPath) {
    return convertFileSrc(props.issue.annotatedScreenshotPath)
  }
  if (!screenshotPath.value) return ''
  // Annotated screenshot path has _annotated.png suffix
  const annotatedPath = screenshotPath.value.replace('.png', '_annotated.png')
  return convertFileSrc(annotatedPath)
})

// Update form states when issue prop changes
watch(() => props.issue, (newIssue) => {
  if (newIssue) {
    title.value = newIssue.title
    description.value = newIssue.description
    issueType.value = newIssue.issueType
    severity.value = newIssue.severity
    status.value = newIssue.status
    selectedTags.value = (newIssue.tags || []).map(t => t.name)
    loadCapturePath()
  }
}, { deep: true })

const goBack = () => {
  router.push(`/session/${props.issue.sessionId}`)
}

const confirmDelete = () => {
  emit('delete', props.issue.id)
}

const isCopyingImage = ref(false)
const isSavingImage = ref(false)

const severityColor = computed(() => {
  switch (severity.value) {
    case 'Critical': return 'error'
    case 'Major': return 'warning'
    case 'Minor': return 'success'
    case 'Info': return 'info'
    default: return 'medium-emphasis'
  }
})

const generateCompositeCanvas = async (): Promise<HTMLCanvasElement | null> => {
  if (!annotatedUrl.value) return null

  const img = new Image()
  img.crossOrigin = 'anonymous'
  img.src = annotatedUrl.value
  await new Promise((resolve, reject) => {
    img.onload = resolve
    img.onerror = reject
  })

  const canvas = document.createElement('canvas')
  const ctx = canvas.getContext('2d')
  if (!ctx) throw new Error('Could not get canvas context')

  const width = img.naturalWidth
  const height = img.naturalHeight

  // Proportional sizing based on the screenshot width (optimized to fit descriptions)
  const footerHeight = Math.max(90, Math.round(width * 0.09))
  const fontSizeTitle = Math.max(14, Math.round(footerHeight * 0.20))
  const fontSizeDesc = Math.max(11, Math.round(footerHeight * 0.14))
  const padding = Math.round(footerHeight * 0.14)

  canvas.width = width
  canvas.height = height + footerHeight

  // Draw original screenshot
  ctx.drawImage(img, 0, 0)

  // Draw dark footer background
  ctx.fillStyle = '#1A1D27'
  ctx.fillRect(0, height, width, footerHeight)

  // Draw border
  ctx.strokeStyle = 'rgba(255, 255, 255, 0.08)'
  ctx.lineWidth = Math.max(1, Math.round(width * 0.001))
  ctx.beginPath()
  ctx.moveTo(0, height)
  ctx.lineTo(width, height)
  ctx.stroke()

  // Draw marker circle
  const markerRadius = Math.max(12, Math.round(footerHeight * 0.18))
  const markerX = padding + markerRadius
  const markerY = height + padding + markerRadius
  ctx.fillStyle = '#FF6B35' // Primary accent
  ctx.beginPath()
  ctx.arc(markerX, markerY, markerRadius, 0, Math.PI * 2)
  ctx.fill()

  // Draw marker text
  ctx.fillStyle = '#FFFFFF'
  ctx.font = `bold ${Math.round(markerRadius * 1.1)}px sans-serif`
  ctx.textAlign = 'center'
  ctx.textBaseline = 'middle'
  ctx.fillText(String(props.issue.markerNumber), markerX, markerY)

  // 1. Calculate Severity Tag width first to compute maximum allowed title width
  ctx.font = `bold ${Math.round(fontSizeTitle * 0.75)}px sans-serif`
  const tagText = severity.value.toUpperCase()
  const tagTextWidth = ctx.measureText(tagText).width
  const tagPadding = Math.round(fontSizeTitle * 0.4)
  const tagWidth = tagTextWidth + tagPadding * 2

  // 2. Setup Title font
  ctx.font = `bold ${fontSizeTitle}px sans-serif`
  
  const titleX = markerX + markerRadius + padding
  const titleY = height + padding + (markerRadius * 0.6)
  
  let titleText = title.value || 'Issue'
  const maxTitleWidth = width - titleX - tagWidth - padding * 2
  
  // Truncate title if it exceeds the maximum allowed width to prevent overlaps
  const titleMetrics = ctx.measureText(titleText)
  if (titleMetrics.width > maxTitleWidth) {
    while (titleText.length > 0 && ctx.measureText(titleText + '...').width > maxTitleWidth) {
      titleText = titleText.slice(0, -1)
    }
    titleText = titleText + '...'
  }
  
  // Draw Title
  ctx.fillStyle = '#FFFFFF'
  ctx.textAlign = 'left'
  ctx.textBaseline = 'top'
  ctx.fillText(titleText, titleX, titleY)

  // 3. Draw Severity Tag next to the title
  const titleWidth = ctx.measureText(titleText).width
  const tagX = titleX + titleWidth + padding
  const tagY = titleY
  const tagHeight = fontSizeTitle * 1.2
  
  ctx.font = `bold ${Math.round(fontSizeTitle * 0.75)}px sans-serif`
  ctx.fillStyle = severity.value === 'Critical' ? '#FF4757' : 
                  severity.value === 'Major' ? '#FFA502' : 
                  severity.value === 'Minor' ? '#2ED573' : '#3742FA'
  
  const radius = Math.round(tagHeight * 0.25)
  ctx.beginPath()
  if (ctx.roundRect) {
    ctx.roundRect(tagX, tagY, tagWidth, tagHeight, radius)
  } else {
    ctx.rect(tagX, tagY, tagWidth, tagHeight)
  }
  ctx.fill()

  ctx.fillStyle = '#FFFFFF'
  ctx.textAlign = 'center'
  ctx.textBaseline = 'middle'
  ctx.fillText(tagText, tagX + tagWidth / 2, tagY + tagHeight / 2)

  // Draw Description
  const descX = titleX
  const descY = titleY + fontSizeTitle + padding * 0.8
  ctx.fillStyle = '#A0A5B5' // Muted text
  ctx.font = `${fontSizeDesc}px sans-serif`
  ctx.textAlign = 'left'
  ctx.textBaseline = 'top'

  const maxTextWidth = width - descX - padding
  const descText = description.value || t('common.noDescription')

  // Word wrapping with manual newlines support
  const paragraphs = descText.split('\n')
  const lines: string[] = []
  
  for (const paragraph of paragraphs) {
    const words = paragraph.split(' ')
    let line = ''
    for (let n = 0; n < words.length; n++) {
      const testLine = line + words[n] + ' '
      const metrics = ctx.measureText(testLine)
      const testWidth = metrics.width
      if (testWidth > maxTextWidth && n > 0) {
        lines.push(line)
        line = words[n] + ' '
      } else {
        line = testLine
      }
    }
    lines.push(line)
  }

  const maxLines = 2
  for (let i = 0; i < Math.min(lines.length, maxLines); i++) {
    let lineText = lines[i]
    if (i === maxLines - 1 && lines.length > maxLines) {
      lineText = lineText.trim().substring(0, Math.max(0, lineText.length - 4)) + '...'
    }
    ctx.fillText(lineText, descX, descY + (i * (fontSizeDesc + padding * 0.4)))
  }

  return canvas
}

const copyImageToClipboard = async () => {
  if (!annotatedUrl.value) return
  isCopyingImage.value = true
  const uiStore = useUiStore()
  try {
    const canvas = await generateCompositeCanvas()
    if (!canvas) throw new Error('Could not generate canvas')

    // Copy canvas to clipboard as blob
    const blob: Blob | null = await new Promise((resolve) => canvas.toBlob(resolve, 'image/png'))
    if (!blob) throw new Error('Could not create blob')

    await navigator.clipboard.write([
      new ClipboardItem({
        [blob.type]: blob
      })
    ])
    uiStore.showToast({ message: t('issueDetail.copyImageSuccess'), type: 'success' })
  } catch (err: any) {
    console.error('Failed to copy image:', err)
    uiStore.showToast({ message: t('issueDetail.copyImageError'), type: 'error' })
  } finally {
    isCopyingImage.value = false
  }
}

const saveImageAsFile = async () => {
  if (!annotatedUrl.value) return
  isSavingImage.value = true
  const uiStore = useUiStore()
  try {
    const canvas = await generateCompositeCanvas()
    if (!canvas) throw new Error('Could not generate canvas')

    const { save } = await import('@tauri-apps/plugin-dialog')
    const { saveImageToFile } = await import('@/services/tauriCommands')

    const safeTitle = (title.value || 'Step')
      .replace(/[^a-zA-Z0-9_\-]/g, '_')
      .replace(/_+/g, '_')
      .substring(0, 30)
    const timestamp = new Date().toISOString().replace(/[-:T.]/g, '').slice(0, 14)
    const defaultName = `August_Guide_${props.issue.markerNumber > 0 ? `Step${props.issue.markerNumber}_` : ''}${safeTitle}_${timestamp}.png`

    const filePath = await save({
      defaultPath: defaultName,
      filters: [{ name: 'Hình ảnh (PNG)', extensions: ['png'] }],
      title: t('settingsView.saveAs') || 'Lưu ảnh ra file...'
    })

    if (filePath) {
      const dataUrl = canvas.toDataURL('image/png')
      await saveImageToFile(filePath, dataUrl)
      uiStore.showToast({
        message: t('settingsView.imageSavedTo', { path: filePath }) || `Đã lưu ảnh vào: ${filePath}`,
        type: 'success'
      })
    }
  } catch (err: any) {
    console.error('Failed to save image as:', err)
    uiStore.showToast({
      message: `Lỗi khi lưu ảnh: ${err?.message || String(err)}`,
      type: 'error'
    })
  } finally {
    isSavingImage.value = false
  }
}

const autoSave = async () => {
  const currentTags = (props.issue.tags || []).map(t => t.name).sort()
  const newTags = [...selectedTags.value].map(t => t.trim()).filter(Boolean).sort()
  const tagsChanged = JSON.stringify(currentTags) !== JSON.stringify(newTags)

  // Only update if changed
  if (
    title.value === props.issue.title &&
    description.value === props.issue.description &&
    issueType.value === props.issue.issueType &&
    severity.value === props.issue.severity &&
    status.value === props.issue.status &&
    !tagsChanged
  ) {
    return
  }

  try {
    // Process tags: if they are new, create them in SQLite
    const tagNames: string[] = []
    for (const name of selectedTags.value) {
      const trimmed = name.trim()
      if (!trimmed) continue
      const existing = tagStore.tags.find(t => t.name.toLowerCase() === trimmed.toLowerCase())
      if (existing) {
        tagNames.push(existing.name)
      } else {
        try {
          const newTag = await tagStore.addTag(trimmed)
          tagNames.push(newTag.name)
        } catch (e) {
          console.error('Failed to create new tag:', e)
          tagNames.push(trimmed)
        }
      }
    }

    await issueStore.updateIssue(props.issue.id, {
      title: title.value,
      description: description.value,
      issueType: issueType.value,
      severity: severity.value,
      status: status.value,
      tags: tagNames
    })
  } catch (err) {
    console.error('Failed to auto-save issue:', err)
  }
}

onBeforeUnmount(() => {
  autoSave()
})
</script>

<style scoped>
.screenshot-container {
  border: 1px solid rgba(255, 255, 255, 0.12);
  min-height: 200px;
  display: flex;
  align-items: center;
}

.position-relative {
  position: relative;
}

.image-info-overlay {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background: linear-gradient(to top, rgba(15, 17, 23, 0.95) 0%, rgba(15, 17, 23, 0.75) 70%, rgba(15, 17, 23, 0) 100%);
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  pointer-events: none;
  z-index: 2;
}

.sticky-panel {
  position: sticky;
  top: 24px;
}

.gap-2 {
  gap: 8px;
}
</style>
