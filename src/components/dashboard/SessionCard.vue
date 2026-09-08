<template>
  <v-card
    class="mx-auto d-flex flex-column fill-height session-card"
    :class="{ 'session-completed': session.status === 'completed' }"
    variant="outlined"
    color="surface-variant"
  >
    <v-card-item>
      <div class="d-flex align-center justify-space-between mb-2">
        <v-chip :color="statusColor" size="x-small" class="text-uppercase font-weight-bold">
          {{ t('statuses.' + session.status, session.status) }}
        </v-chip>
        <span class="text-caption text-medium-emphasis">
          {{ timeInfo }}
        </span>
      </div>

      <v-card-title class="text-h6 font-weight-bold text-white text-truncate pl-0">
        {{ session.title }}
      </v-card-title>

      <v-card-subtitle class="pl-0 text-truncate text-medium-emphasis mb-4">
        {{ session.description || t('common.noDescription') }}
      </v-card-subtitle>

      <!-- Stats section -->
      <v-row no-gutters class="mt-2 py-2 border-top border-bottom">
        <v-col cols="6" class="text-center border-right">
          <div class="text-h6 font-weight-bold text-primary">
            {{ session.captureCount || 0 }}
          </div>
          <div class="text-caption text-medium-emphasis text-uppercase">{{ t('common.screenshots', 'Screenshots') }}</div>
        </v-col>
        <v-col cols="6" class="text-center">
          <div class="text-h6 font-weight-bold text-secondary">
            {{ session.issueCount || 0 }}
          </div>
          <div class="text-caption text-medium-emphasis text-uppercase">{{ t('common.issues', 'Issues') }}</div>
        </v-col>
      </v-row>
    </v-card-item>

    <v-spacer></v-spacer>

    <v-card-actions class="d-flex justify-space-between px-4 pb-4 pt-0">
      <v-btn
        variant="elevated"
        color="primary"
        size="small"
        class="text-none"
        prepend-icon="mdi-eye"
        @click="openSession"
      >
        {{ t('common.open', 'Open') }}
      </v-btn>
      
      <v-menu>
        <template v-slot:activator="{ props }">
          <v-btn icon="mdi-dots-vertical" variant="text" size="small" v-bind="props"></v-btn>
        </template>
        <v-list density="compact" bg-color="surface">
          <v-list-item
            v-if="session.status === 'active'"
            prepend-icon="mdi-check-circle"
            :title="t('dashboardView.deleteSessionConfirmAll', 'Complete Session').includes('Complete') ? 'Complete Session' : t('common.complete')"
            @click="$emit('complete', session.id)"
          ></v-list-item>
          <v-list-item
            v-else-if="session.status === 'completed'"
            prepend-icon="mdi-play-circle"
            :title="t('sessionView.active')"
            @click="$emit('activate', session.id)"
          ></v-list-item>
          <v-list-item
            prepend-icon="mdi-pencil"
            :title="t('common.rename')"
            @click="$emit('rename', session)"
          ></v-list-item>
          <v-list-item
            prepend-icon="mdi-export"
            :title="t('sessionView.exportReport')"
            @click="$emit('export', session)"
          ></v-list-item>
          <v-list-item
            prepend-icon="mdi-delete"
            :title="t('dashboardView.deleteSessionTitle')"
            class="text-error"
            @click="$emit('delete', session.id)"
          ></v-list-item>
        </v-list>
      </v-menu>
    </v-card-actions>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { Session } from '@/types/session'
import { formatRelativeTime, formatDate } from '@/utils/date'
import { useI18n } from '@/composables/useI18n'

const props = defineProps<{
  session: Session
}>()

const { t } = useI18n()
const router = useRouter()

const openSession = () => {
  router.push(`/session/${props.session.id}`)
}

defineEmits<{
  (e: 'delete', id: string): void
  (e: 'complete', id: string): void
  (e: 'activate', id: string): void
  (e: 'rename', session: Session): void
  (e: 'export', session: Session): void
}>()

const statusColor = computed(() => {
  switch (props.session.status) {
    case 'active':
      return 'secondary' // Teal accent
    case 'completed':
      return 'success'   // Green
    case 'archived':
      return 'warning'   // Yellow
    default:
      return 'medium-emphasis'
  }
})

const timeInfo = computed(() => {
  if (props.session.status === 'completed' && props.session.completedAt) {
    return `${t('sessionView.completed')} ${formatDate(props.session.completedAt)}`
  }
  return `${t('dashboardView.active')} ${formatRelativeTime(props.session.createdAt)}`
})
</script>

<style scoped>
.border-top {
  border-top: 1px solid rgba(255, 255, 255, 0.08) !important;
}
.border-bottom {
  border-bottom: 1px solid rgba(255, 255, 255, 0.08) !important;
}
.border-right {
  border-right: 1px solid rgba(255, 255, 255, 0.08) !important;
}
.session-completed {
  opacity: 0.75;
  border-color: rgba(255, 255, 255, 0.05) !important;
}
</style>
