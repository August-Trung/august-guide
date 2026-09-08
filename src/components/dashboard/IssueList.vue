<template>
  <div>
    <!-- Empty State -->
    <div v-if="issues.length === 0" class="d-flex flex-column align-center justify-center py-12 text-center">
      <v-icon size="64" color="medium-emphasis" class="mb-4">mdi-clipboard-text-outline</v-icon>
      <h3 class="text-h6 font-weight-bold text-white mb-2">{{ t('issueDetail.noIssuesFound') }}</h3>
      <p class="text-body-2 text-medium-emphasis max-width-text">
        {{ t('issueDetail.noIssuesFilterSub') }}
      </p>
    </div>

    <!-- Issues Grid -->
    <transition-group
      v-else
      name="list"
      tag="div"
      class="v-row"
    >
      <v-col
        v-for="issue in issues"
        :key="issue.id"
        cols="12"
        sm="6"
        md="4"
        lg="3"
      >
        <IssueCard :issue="issue" @delete="$emit('delete', $event)" />
      </v-col>
    </transition-group>
  </div>
</template>

<script setup lang="ts">
import type { Issue } from '@/types/issue'
import IssueCard from './IssueCard.vue'
import { useI18n } from '@/composables/useI18n'

defineProps<{
  issues: Issue[]
}>()

defineEmits<{
  (e: 'delete', id: string): void
}>()

const { t } = useI18n()
</script>

<style scoped>
.max-width-text {
  max-width: 400px;
}

/* Ensure proper gutters and padding for transition-group with class v-row */
.v-row {
  --v-grid-gutters: 24px;
  margin: -12px;
}
.v-col {
  padding: 12px;
}
</style>

