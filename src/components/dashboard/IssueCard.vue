<template>
  <v-card
    class="issue-card d-flex flex-column h-100"
    variant="outlined"
    color="surface-variant"
    @click="navigateToDetail"
  >
    <!-- Full Annotated Screenshot / Thumbnail Image -->
    <div class="thumbnail-wrapper">
      <v-img
        v-if="displayImageUrl"
        :src="displayImageUrl"
        cover
        class="thumbnail-img"
        height="180"
      >
        <template v-slot:placeholder>
          <div class="d-flex align-center justify-center fill-height bg-grey-darken-3">
            <v-progress-circular indeterminate color="primary" size="24"></v-progress-circular>
          </div>
        </template>
      </v-img>
      <div v-else class="no-thumbnail d-flex align-center justify-center bg-grey-darken-3" style="height: 180px;">
        <v-icon size="36" color="medium-emphasis">mdi-image-off</v-icon>
      </div>

      <!-- Floating marker number badge -->
      <v-avatar
        v-if="issue.markerNumber > 0"
        color="primary"
        size="28"
        class="marker-badge text-white font-weight-bold"
      >
        {{ issue.markerNumber }}
      </v-avatar>
    </div>

    <v-card-item class="pt-3 pb-2 flex-grow-1">
      <v-card-title class="text-subtitle-1 font-weight-bold text-white pl-0 mb-1 text-truncate">
        {{ issue.title }}
      </v-card-title>
      <v-card-subtitle class="pl-0 text-caption text-truncate text-medium-emphasis">
        {{ issue.description || t('common.noDescription') }}
      </v-card-subtitle>
      <div v-if="issue.tags && issue.tags.length > 0" class="pl-0 mt-2 d-flex flex-wrap gap-1">
        <v-chip
          v-for="tag in issue.tags"
          :key="tag.id"
          size="x-small"
          variant="tonal"
          :color="tag.color || '#4ECDC4'"
          class="font-weight-medium"
        >
          {{ tag.name }}
        </v-chip>
      </div>
    </v-card-item>

    <v-divider class="mx-4"></v-divider>

    <v-card-actions class="px-4 py-2 d-flex flex-wrap gap-1 align-center justify-space-between">
      <div class="d-flex gap-1 align-center">
        <!-- Type Chip -->
        <v-chip
          :color="typeColor"
          size="x-small"
          variant="flat"
          class="font-weight-bold"
        >
          {{ t('issueTypes.' + issue.issueType, issue.issueType) }}
        </v-chip>

        <!-- Severity Chip -->
        <v-chip
          :color="severityColor"
          size="x-small"
          variant="tonal"
          class="font-weight-bold"
        >
          {{ t('severities.' + issue.severity, issue.severity) }}
        </v-chip>

        <!-- Status Chip -->
        <v-chip
          :color="statusColor"
          size="x-small"
          variant="outlined"
          class="font-weight-bold"
        >
          {{ t('statuses.' + issue.status, issue.status) }}
        </v-chip>
      </div>

      <!-- Delete Button (stop propagation to avoid navigating to detail) -->
      <v-btn
        icon="mdi-delete"
        variant="text"
        size="small"
        color="error"
        @click.stop="$emit('delete', issue.id)"
      ></v-btn>
    </v-card-actions>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { convertFileSrc } from '@tauri-apps/api/core'
import type { Issue } from '@/types/issue'
import { useI18n } from '@/composables/useI18n'

const props = defineProps<{
  issue: Issue
}>()

defineEmits<{
  (e: 'delete', id: string): void
}>()

const { t } = useI18n()
const router = useRouter()

const displayImageUrl = computed(() => {
  if (props.issue.annotatedScreenshotPath) {
    return convertFileSrc(props.issue.annotatedScreenshotPath)
  }
  if (props.issue.screenshotPath) {
    return convertFileSrc(props.issue.screenshotPath)
  }
  if (props.issue.cropPath) {
    return convertFileSrc(props.issue.cropPath)
  }
  return ''
})

const navigateToDetail = () => {
  router.push(`/issue/${props.issue.id}`)
}

const typeColor = computed(() => {
  switch (props.issue.issueType) {
    case 'Step':
    case 'GuideStep': return '#38BDF8'
    case 'Note': return '#2ED573'
    case 'Warning': return '#FF6B35'
    case 'Tip': return '#FFD32A'
    default: return '#38BDF8'
  }
})

const severityColor = computed(() => {
  switch (props.issue.severity) {
    case 'Critical': return 'error'       // red
    case 'Major': return 'warning'       // orange
    case 'Minor': return 'success'       // green (or yellow, we use theme colors)
    case 'Info': return 'info'          // blue/gray
    default: return 'medium-emphasis'
  }
})

const statusColor = computed(() => {
  switch (props.issue.status) {
    case 'Draft': return 'medium-emphasis'
    case 'Open': return 'primary'
    case 'In Progress': return 'warning'
    case 'Resolved': return 'success'
    case 'Closed': return 'medium-emphasis'
    default: return 'medium-emphasis'
  }
})
</script>

<style scoped>
.issue-card {
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  cursor: pointer;
}

.issue-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.4);
}

.thumbnail-wrapper {
  position: relative;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  overflow: hidden;
}

.thumbnail-img {
  width: 100%;
}

.marker-badge {
  position: absolute;
  top: 8px;
  left: 8px;
  border: 2px solid #ffffff;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.4);
}

.gap-1 {
  gap: 4px;
}
</style>
