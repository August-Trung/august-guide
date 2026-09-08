<template>
  <div class="w-100 fill-height d-flex flex-column">
    <AppHeader />

    <v-container class="flex-grow-1 align-start pa-6" fluid>
      <!-- Loading State -->
      <div v-if="isLoading" class="d-flex align-center justify-center py-12">
        <v-progress-circular indeterminate color="primary" size="48"></v-progress-circular>
      </div>

      <!-- Error State -->
      <v-alert
        v-else-if="error"
        type="error"
        variant="tonal"
        class="mb-6"
        :title="t('issueDetail.failedLoad')"
        :text="error"
      ></v-alert>

      <!-- Details View -->
      <IssueDetail
        v-else-if="activeIssue"
        :issue="activeIssue"
        @delete="handleDelete"
      />
    </v-container>

    <!-- Delete Issue Confirmation Dialog -->
    <ConfirmDialog
      v-model="showDeleteDialog"
      :title="t('dashboardView.deleteIssueTitle')"
      :message="t('dashboardView.deleteIssueConfirmAll')"
      :confirm-text="t('common.delete')"
      confirm-color="error"
      @confirm="executeDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useIssueStore } from '@/stores/issueStore'
import { useI18n } from '@/composables/useI18n'
import AppHeader from '@/components/common/AppHeader.vue'
import IssueDetail from '@/components/dashboard/IssueDetail.vue'
import ConfirmDialog from '@/components/common/ConfirmDialog.vue'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const issueStore = useIssueStore()
const { activeIssue, isLoading, error } = storeToRefs(issueStore)

const showDeleteDialog = ref(false)
const issueToDelete = ref<string | null>(null)

onMounted(async () => {
  const issueId = route.params.id as string
  if (issueId) {
    try {
      await issueStore.fetchIssue(issueId)
    } catch (err) {
      console.error('Failed to load issue:', err)
    }
  }
})

const handleDelete = (id: string) => {
  issueToDelete.value = id
  showDeleteDialog.value = true
}

const executeDelete = async () => {
  if (!issueToDelete.value) return
  const sessionId = activeIssue.value?.sessionId
  try {
    await issueStore.deleteIssue(issueToDelete.value)
    if (sessionId) {
      router.push(`/session/${sessionId}`)
    } else {
      router.push('/')
    }
  } catch (err) {
    console.error('Failed to delete issue:', err)
  } finally {
    issueToDelete.value = null
  }
}
</script>

<style scoped>
/* View specific styles */
</style>
