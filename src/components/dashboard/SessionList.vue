<template>
  <div>
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
      :title="t('dashboardView.failedLoadSessions', 'Failed to load sessions')"
      :text="error"
    ></v-alert>

    <!-- Empty State -->
    <EmptyState
      v-else-if="sessions.length === 0"
      :title="t('dashboardView.noSessionsYet')"
      :description="t('dashboardView.noSessionsYetSub')"
      :action-text="t('dashboardView.createReviewSession')"
      @action="showCreateDialog = true"
    />

    <!-- Sort Toolbar -->
    <div v-else-if="sessions.length > 0" class="d-flex justify-end mb-4 align-center">
      <span class="text-caption text-medium-emphasis mr-2">{{ t('dashboardView.sortByLabel') }}</span>
      <div style="width: 220px;">
        <v-select
          v-model="sortBy"
          :items="sortOptions"
          item-title="title"
          item-value="value"
          density="compact"
          hide-details
          variant="solo-filled"
          flat
          bg-color="surface-variant"
        ></v-select>
      </div>
    </div>

    <!-- Sessions Grid -->
    <transition-group
      v-if="sessions.length > 0"
      name="list"
      tag="div"
      class="v-row"
    >
      <v-col
        v-for="session in sortedSessions"
        :key="session.id"
        cols="12"
        sm="6"
        md="4"
      >
        <SessionCard
          :session="session"
          @delete="handleDelete"
          @complete="handleComplete"
          @activate="handleActivate"
          @rename="openRenameDialog"
          @export="openExportDialog"
        />
      </v-col>
    </transition-group>

    <!-- Create Session Dialog -->
    <v-dialog v-model="showCreateDialog" max-width="500px">
      <v-card bg-color="surface" class="pa-4">
        <v-card-title class="text-h5 font-weight-bold px-0 text-primary">
          {{ t('dashboardView.newReviewSession') }}
        </v-card-title>
        <v-card-text class="px-0 py-4">
          <v-form ref="form" v-model="isFormValid" @submit.prevent="handleCreate">
            <v-text-field
              v-model="sessionTitle"
              :label="t('dashboardView.sessionTitle')"
              :placeholder="t('exportDialog.allIssues').includes('Tất cả') ? 'Ví dụ: Đánh giá trang chủ' : 'e.g. Homepage UI Audit'"
              :rules="[v => !!v || t('dashboardView.sessionTitleRequired')]"
              variant="outlined"
              density="comfortable"
              class="mb-3"
              required
            ></v-text-field>

            <v-textarea
              v-model="sessionDesc"
              :label="t('sidebar.projectDesc') + ' (' + t('sidebar.optional') + ')'"
              :placeholder="t('dashboardView.sessionDescPlaceholder')"
              variant="outlined"
              density="comfortable"
              rows="3"
            ></v-textarea>
          </v-form>
        </v-card-text>
        <v-card-actions class="px-0">
          <v-spacer></v-spacer>
          <v-btn variant="text" color="medium-emphasis" class="text-none" @click="closeDialog">
            {{ t('common.cancel') }}
          </v-btn>
          <v-btn
            variant="elevated"
            color="primary"
            class="text-none px-4"
            :disabled="!isFormValid || isCreating"
            :loading="isCreating"
            @click="handleCreate"
          >
            {{ t('common.create') }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Delete Session Confirmation Dialog -->
    <ConfirmDialog
      v-model="showDeleteDialog"
      :title="t('dashboardView.deleteSessionTitle')"
      :message="t('dashboardView.deleteSessionConfirmAll')"
      :confirm-text="t('common.delete')"
      confirm-color="error"
      @confirm="executeDelete"
    />

    <!-- Rename Session Dialog -->
    <v-dialog v-model="showRenameDialog" max-width="500px">
      <v-card bg-color="surface" class="pa-4">
        <v-card-title class="text-h5 font-weight-bold px-0 text-primary">
          {{ t('dashboardView.renameSession', 'Rename Session') }}
        </v-card-title>
        <v-card-text class="px-0 py-4">
          <v-form ref="renameForm" v-model="isRenameFormValid" @submit.prevent="handleRenameSave">
            <v-text-field
              v-model="renameTitle"
              :label="t('dashboardView.sessionTitle')"
              :placeholder="t('exportDialog.allIssues').includes('Tất cả') ? 'Ví dụ: Đánh giá trang chủ' : 'e.g. Homepage UI Audit'"
              :rules="[v => !!v || t('dashboardView.sessionTitleRequired')]"
              variant="outlined"
              density="comfortable"
              class="mb-3"
              required
            ></v-text-field>

            <v-textarea
              v-model="renameDesc"
              :label="t('sidebar.projectDesc') + ' (' + t('sidebar.optional') + ')'"
              :placeholder="t('dashboardView.sessionDescPlaceholder')"
              variant="outlined"
              density="comfortable"
              rows="3"
            ></v-textarea>
          </v-form>
        </v-card-text>
        <v-card-actions class="px-0">
          <v-spacer></v-spacer>
          <v-btn variant="text" color="medium-emphasis" class="text-none" @click="closeRenameDialog">
            {{ t('common.cancel') }}
          </v-btn>
          <v-btn
            variant="elevated"
            color="primary"
            class="text-none px-4"
            :disabled="!isRenameFormValid || isRenaming"
            :loading="isRenaming"
            @click="handleRenameSave"
          >
            {{ t('common.save') }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Export Session Report Dialog -->
    <ExportDialog
      v-model="showExportDialog"
      :session-id="exportSessionId"
      :session-title="exportSessionTitle"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useProjectStore } from '@/stores/projectStore'
import { useSessionStore } from '@/stores/sessionStore'
import { listenToEvent } from '@/services/tauriEvents'
import { useI18n } from '@/composables/useI18n'
import SessionCard from './SessionCard.vue'
import EmptyState from '@/components/common/EmptyState.vue'
import ConfirmDialog from '@/components/common/ConfirmDialog.vue'
import ExportDialog from '@/components/export/ExportDialog.vue'
import type { Session } from '@/types/session'

const { t } = useI18n()
const projectStore = useProjectStore()
const { activeProjectId } = storeToRefs(projectStore)

const sessionStore = useSessionStore()
const { sessions, sortedSessions, sortBy, isLoading, error } = storeToRefs(sessionStore)
const { fetchSessionsByProject, createSession, deleteSession, updateSession } = sessionStore

const sortOptions = computed(() => [
  { title: t('sortOptions.newest'), value: 'newest' },
  { title: t('sortOptions.oldest'), value: 'oldest' },
  { title: t('sortOptions.issues_desc'), value: 'issues_desc' },
  { title: t('sortOptions.status_active'), value: 'status' }
])

const showCreateDialog = ref(false)
const isFormValid = ref(false)
const sessionTitle = ref('')
const sessionDesc = ref('')
const isCreating = ref(false)

const showDeleteDialog = ref(false)
const sessionToDelete = ref<string | null>(null)

// Expose openCreateDialog so dashboard parent can call it
defineExpose({
  openCreateDialog: () => {
    showCreateDialog.value = true
  }
})

// Refetch sessions when active project changes
watch(activeProjectId, (newId) => {
  if (newId) {
    fetchSessionsByProject(newId)
  }
}, { immediate: true })

let unlistenSessionUpdated: (() => void) | null = null

onMounted(async () => {
  if (activeProjectId.value) {
    fetchSessionsByProject(activeProjectId.value)
  }

  unlistenSessionUpdated = await listenToEvent<string>('session-updated', (_sessionId) => {
    if (activeProjectId.value) {
      fetchSessionsByProject(activeProjectId.value)
    }
  })
})

onUnmounted(() => {
  if (unlistenSessionUpdated) {
    unlistenSessionUpdated()
  }
})

function closeDialog() {
  showCreateDialog.value = false
  sessionTitle.value = ''
  sessionDesc.value = ''
}

async function handleCreate() {
  if (!sessionTitle.value || !activeProjectId.value) return
  isCreating.value = true
  try {
    await createSession({
      projectId: activeProjectId.value,
      title: sessionTitle.value,
      description: sessionDesc.value,
    })
    closeDialog()
  } catch (err) {
    console.error('Failed to create session:', err)
  } finally {
    isCreating.value = false
  }
}

function handleDelete(id: string) {
  sessionToDelete.value = id
  showDeleteDialog.value = true
}

async function executeDelete() {
  if (!sessionToDelete.value) return
  try {
    await deleteSession(sessionToDelete.value)
  } catch (err) {
    console.error('Failed to delete session:', err)
  } finally {
    sessionToDelete.value = null
  }
}

async function handleComplete(id: string) {
  try {
    await updateSession(id, { status: 'completed' })
  } catch (err) {
    console.error('Failed to complete session:', err)
  }
}

async function handleActivate(id: string) {
  try {
    await updateSession(id, { status: 'active' })
  } catch (err) {
    console.error('Failed to reactivate session:', err)
  }
}

const showRenameDialog = ref(false)
const isRenameFormValid = ref(false)
const renameSessionId = ref<string | null>(null)
const renameTitle = ref('')
const renameDesc = ref('')
const isRenaming = ref(false)

function openRenameDialog(session: Session) {
  renameSessionId.value = session.id
  renameTitle.value = session.title
  renameDesc.value = session.description || ''
  showRenameDialog.value = true
}

function closeRenameDialog() {
  showRenameDialog.value = false
  renameSessionId.value = null
  renameTitle.value = ''
  renameDesc.value = ''
}

async function handleRenameSave() {
  if (!renameSessionId.value || !renameTitle.value) return
  isRenaming.value = true
  try {
    await updateSession(renameSessionId.value, {
      title: renameTitle.value,
      description: renameDesc.value,
    })
    closeRenameDialog()
  } catch (err) {
    console.error('Failed to rename session:', err)
  } finally {
    isRenaming.value = false
  }
}

const showExportDialog = ref(false)
const exportSessionId = ref('')
const exportSessionTitle = ref('')

function openExportDialog(session: Session) {
  exportSessionId.value = session.id
  exportSessionTitle.value = session.title
  showExportDialog.value = true
}
</script>

<style scoped>
.border-dashed {
  border: 2px dashed rgba(255, 255, 255, 0.12) !important;
  background-color: rgba(255, 255, 255, 0.02);
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
