<template>
  <v-card class="mb-6 filter-bar-card px-4 py-3" variant="outlined">
    <div class="d-flex flex-wrap align-center gap-3">
      <!-- Title / Icon -->
      <div class="d-flex align-center mr-2">
        <v-icon color="primary" class="mr-2">mdi-filter-variant</v-icon>
        <span class="text-subtitle-1 font-weight-medium text-white">{{ t('common.filters') }}</span>
      </div>

      <!-- Type Select -->
      <div class="flex-grow-1 select-wrapper" style="min-width: 150px; max-width: 250px;">
        <v-select
          v-model="filters.types"
          :items="typeOptions"
          item-title="title"
          item-value="value"
          :label="t('common.issueType')"
          multiple
          chips
          closable-chips
          collapse-tags
          density="compact"
          hide-details
          variant="solo-filled"
          flat
          bg-color="surface-variant"
        ></v-select>
      </div>

      <!-- Severity Select -->
      <div class="flex-grow-1 select-wrapper" style="min-width: 150px; max-width: 250px;">
        <v-select
          v-model="filters.severities"
          :items="severityOptions"
          item-title="title"
          item-value="value"
          :label="t('common.severity')"
          multiple
          chips
          closable-chips
          collapse-tags
          density="compact"
          hide-details
          variant="solo-filled"
          flat
          bg-color="surface-variant"
        ></v-select>
      </div>

      <!-- Status Select -->
      <div class="flex-grow-1 select-wrapper" style="min-width: 150px; max-width: 250px;">
        <v-select
          v-model="filters.statuses"
          :items="statusOptions"
          item-title="title"
          item-value="value"
          :label="t('common.status')"
          multiple
          chips
          closable-chips
          collapse-tags
          density="compact"
          hide-details
          variant="solo-filled"
          flat
          bg-color="surface-variant"
        ></v-select>
      </div>

      <!-- Tags Select -->
      <div class="flex-grow-1 select-wrapper" style="min-width: 150px; max-width: 250px;">
        <v-select
          v-model="filters.tags"
          :items="tagStore.tags.map(t => t.name)"
          :label="t('common.tags')"
          multiple
          chips
          closable-chips
          collapse-tags
          density="compact"
          hide-details
          variant="solo-filled"
          flat
          bg-color="surface-variant"
        ></v-select>
      </div>

      <!-- Sort Select -->
      <div class="flex-grow-1 select-wrapper" style="min-width: 150px; max-width: 200px;">
        <v-select
          v-model="sortBy"
          :items="sortOptions"
          item-title="title"
          item-value="value"
          :label="t('common.sortBy')"
          density="compact"
          hide-details
          variant="solo-filled"
          flat
          bg-color="surface-variant"
        ></v-select>
      </div>

      <!-- Reset Button -->
      <v-btn
        v-if="hasActiveFilters"
        variant="text"
        color="secondary"
        prepend-icon="mdi-filter-off"
        class="text-none ml-auto"
        @click="clearFilters"
      >
        {{ t('common.clearFilters') }}
      </v-btn>
    </div>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useIssueStore } from '@/stores/issueStore'
import { useTagStore } from '@/stores/tagStore'
import { useI18n } from '@/composables/useI18n'

const { t } = useI18n()
const issueStore = useIssueStore()
const tagStore = useTagStore()
const { filters, sortBy } = storeToRefs(issueStore)
const { clearFilters } = issueStore

tagStore.loadTags()

const typeOptions = computed(() => [
  { title: t('issueTypes.Step'), value: 'Step' },
  { title: t('issueTypes.Note'), value: 'Note' },
  { title: t('issueTypes.Warning'), value: 'Warning' },
  { title: t('issueTypes.Tip'), value: 'Tip' }
])

const severityOptions = computed(() => [
  { title: t('severities.Info'), value: 'Info' },
  { title: t('severities.Minor'), value: 'Minor' },
  { title: t('severities.Major'), value: 'Major' },
  { title: t('severities.Critical'), value: 'Critical' }
])

const statusOptions = computed(() => [
  { title: t('statuses.Draft'), value: 'Draft' },
  { title: t('statuses.Open'), value: 'Open' },
  { title: t('statuses.In Progress'), value: 'In Progress' },
  { title: t('statuses.Resolved'), value: 'Resolved' },
  { title: t('statuses.Closed'), value: 'Closed' }
])

const sortOptions = computed(() => [
  { title: t('sortOptions.newest'), value: 'newest' },
  { title: t('sortOptions.oldest'), value: 'oldest' },
  { title: t('sortOptions.severity'), value: 'severity' }
])

const hasActiveFilters = computed(() => {
  return (
    filters.value.types.length > 0 ||
    filters.value.severities.length > 0 ||
    filters.value.statuses.length > 0 ||
    filters.value.tags.length > 0
  )
})
</script>

<style scoped>
.filter-bar-card {
  background: rgba(30, 30, 30, 0.6) !important;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.08) !important;
  border-radius: 8px;
}

.gap-3 {
  gap: 12px;
}

.select-wrapper :deep(.v-field) {
  border-radius: 6px;
}
</style>
