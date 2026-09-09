<template>
  <v-sheet
    color="background"
    class="d-flex align-center justify-space-between px-6 py-3 border-bottom"
    flat
  >
    <div class="d-flex align-center gap-2">
      <!-- Active Project Info -->
      <v-icon
        icon="mdi-folder-outline"
        :color="activeProject?.color || 'primary'"
        class="mr-1"
      ></v-icon>
      <span class="text-subtitle-1 font-weight-bold text-white">
        {{ activeProject ? activeProject.name : t('common.noActiveProject') }}
      </span>
      <v-chip
        v-if="activeSessionsCount > 0"
        color="secondary"
        size="x-small"
        class="ml-2 font-weight-bold"
        variant="flat"
      >
        {{ activeSessionsCount }} {{ t('header.active') }}
      </v-chip>
    </div>

    <!-- Global Search Bar (Middle) -->
    <div class="search-bar-wrapper select-wrapper">
      <v-autocomplete
        v-model:search="searchQuery"
        v-model="selectedItem"
        :items="searchResults"
        :loading="isSearching"
        :placeholder="t('header.searchPlaceholder')"
        prepend-inner-icon="mdi-magnify"
        variant="solo-filled"
        density="compact"
        flat
        hide-details
        :no-filter="true"
        return-object
        class="search-input"
        bg-color="surface-variant"
        item-title="title"
        item-value="id"
        :menu-props="{ maxWidth: 320 }"
        style="width: 320px;"
      >
        <template v-slot:no-data>
          <div class="px-4 py-2 text-caption text-medium-emphasis">
            {{ searchQuery ? t('header.noResults') : t('header.typeToSearch') }}
          </div>
        </template>

        <template v-slot:item="{ props, item }">
          <v-list-item
            v-bind="props"
            :key="item?.id || (item as any)?.raw?.id"
            :title="item?.title || (item as any)?.raw?.title"
            :subtitle="item?.subtitle || (item as any)?.raw?.subtitle"
          >
            <template v-slot:prepend>
              <v-icon :icon="(item?.type || (item as any)?.raw?.type) === 'session' ? 'mdi-clock-outline' : 'mdi-alert-circle-outline'" size="small" class="mr-2"></v-icon>
            </template>
            <template v-slot:append>
              <v-chip size="x-small" :color="(item?.type || (item as any)?.raw?.type) === 'session' ? 'primary' : 'secondary'" class="text-uppercase font-weight-bold">
                {{ item?.type || (item as any)?.raw?.type }}
              </v-chip>
            </template>
          </v-list-item>
        </template>
      </v-autocomplete>
    </div>

    <!-- Hotkey hint / Action area -->
    <div class="d-flex align-center gap-3">
      <v-btn
        icon="mdi-folder-image"
        variant="tonal"
        size="small"
        color="secondary"
        class="rounded"
        :title="t('header.openFolder', 'Mở thư mục lưu ảnh')"
        @click="handleOpenFolder"
      ></v-btn>

      <v-btn
        color="primary"
        prepend-icon="mdi-camera"
        variant="elevated"
        size="small"
        class="text-none font-weight-bold"
        @click="handleManualCapture"
      >
        {{ t('header.captureNow', 'Chụp màn hình') }}
      </v-btn>

      <div class="d-flex align-center text-caption text-medium-emphasis bg-surface px-3 py-1 rounded border">
        <v-icon size="14" class="mr-1" color="primary">mdi-keyboard</v-icon>
        <span class="mr-1">{{ t('header.triggerCapture') }}</span>
        <kbd class="kbd-key mr-1">{{ t('header.middleClick') }}</kbd> {{ t('header.or') }} <kbd class="kbd-key ml-1">Ctrl + Shift + M</kbd>
      </div>
    </div>
  </v-sheet>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { useProjectStore } from '@/stores/projectStore'
import { useSessionStore } from '@/stores/sessionStore'
import { useRouter } from 'vue-router'
import { searchAll, openAppFolder } from '@/services/tauriCommands'
import { useI18n } from '@/composables/useI18n'
import { startCaptureWorkflow } from '@/composables/useTauriEvents'

interface SearchItem {
  id: string
  title: string
  subtitle: string
  type: 'session' | 'issue'
  sessionId: string
}

const projectStore = useProjectStore()
const { activeProject } = storeToRefs(projectStore)
const { t } = useI18n()

const sessionStore = useSessionStore()
const { sessions } = storeToRefs(sessionStore)

async function handleManualCapture() {
  await startCaptureWorkflow(projectStore, sessionStore)
}

async function handleOpenFolder() {
  try {
    await openAppFolder('screenshots')
  } catch (e) {
    console.error('Failed to open screenshots folder:', e)
  }
}

const router = useRouter()
const searchQuery = ref('')
const selectedItem = ref<SearchItem | null>(null)
const isSearching = ref(false)
const searchResults = ref<SearchItem[]>([])

let debounceTimer: ReturnType<typeof setTimeout> | null = null

const performSearch = async (val: string) => {
  console.log('[Search JS] performSearch called with input:', val)
  if (!val.trim()) {
    console.log('[Search JS] Input is empty, clearing searchResults')
    searchResults.value = []
    return
  }

  isSearching.value = true
  try {
    console.log('[Search JS] Invoking Rust command search_all with:', val)
    const res = await searchAll(val)
    console.log('[Search JS] Received searchAll result from Rust:', JSON.stringify(res))
    const items: SearchItem[] = []

    // Map sessions
    for (const s of res.sessions) {
      items.push({
        id: s.id,
        title: s.title,
        subtitle: s.description || 'Session',
        type: 'session',
        sessionId: s.id
      })
    }

    // Map issues
    for (const iss of res.issues) {
      items.push({
        id: iss.id,
        title: `[#${iss.markerNumber}] ${iss.title}`,
        subtitle: `${iss.issueType} • ${iss.severity} • ${iss.status}`,
        type: 'issue',
        sessionId: iss.sessionId
      })
    }

    searchResults.value = items
    console.log('[Search JS] Set searchResults.value to:', JSON.stringify(items))
  } catch (e) {
    console.error('[Search JS] Failed to perform search:', e)
  } finally {
    isSearching.value = false
  }
}

watch(searchQuery, (val) => {
  console.log('[Search JS] searchQuery watcher triggered with value:', val)
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    performSearch(val)
  }, 300)
})

watch(selectedItem, (item) => {
  console.log('[Search JS] selectedItem watcher triggered with value:', item)
  if (item) {
    const targetItem = { ...item }
    
    if (targetItem.type === 'session') {
      router.push(`/session/${targetItem.id}`)
    } else {
      router.push(`/issue/${targetItem.id}`)
    }

    // Reset selection safely in 300ms (after the menu is closed)
    setTimeout(() => {
      console.log('[Search JS] Resetting selectedItem and searchQuery')
      selectedItem.value = null
      searchQuery.value = ''
    }, 300)
  }
})

const activeSessionsCount = computed(() => {
  return sessions.value.filter(s => s.status === 'active').length
})
</script>

<style scoped>
.gap-2 {
  gap: 8px;
}
.gap-4 {
  gap: 16px;
}
.border-bottom {
  border-bottom: 1px solid rgba(255, 255, 255, 0.08) !important;
}
.kbd-key {
  background-color: rgba(255, 255, 255, 0.08);
  border: 1px solid rgba(255, 255, 255, 0.16);
  border-radius: 3px;
  color: #fff;
  font-family: monospace;
  font-size: 0.8rem;
  padding: 1px 5px;
}
.select-wrapper :deep(.v-field) {
  border-radius: 6px;
}
</style>
