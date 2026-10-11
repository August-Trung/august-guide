<template>
  <v-navigation-drawer permanent border width="270" color="background">
    <!-- Header/Logo area -->
    <div class="pa-4 d-flex align-center gap-2">
      <v-avatar size="32" color="primary" rounded>
        <v-icon icon="mdi-book-open-page-variant" color="white"></v-icon>
      </v-avatar>
      <span class="text-h6 font-weight-bold text-white">August Guide</span>
    </div>

    <v-divider></v-divider>

    <!-- Sidebar controls: project selector / header -->
    <div class="px-4 py-3">
      <div class="text-overline mb-2 text-medium-emphasis">{{ t('sidebar.activeProject') }}</div>
      <ProjectSelector />
    </div>

    <v-divider></v-divider>

    <!-- Navigation Views -->
    <v-list density="compact" nav class="px-2">
      <v-list-item
        prepend-icon="mdi-image-multiple"
        title="Thư viện ảnh"
        value="gallery"
        to="/"
        color="primary"
      ></v-list-item>
      <v-list-item
        prepend-icon="mdi-cog"
        :title="t('common.settings')"
        value="settings"
        to="/settings"
        color="primary"
      ></v-list-item>
      <v-list-item
        prepend-icon="mdi-information"
        :title="t('common.about')"
        value="about"
        to="/about"
        color="primary"
      ></v-list-item>
    </v-list>

    <v-divider></v-divider>

    <!-- Document Albums / Guides Section -->
    <div class="px-4 py-2 d-flex align-center justify-space-between">
      <div class="d-flex align-center gap-1">
        <v-icon icon="mdi-book-multiple-outline" size="16" color="medium-emphasis"></v-icon>
        <span class="text-overline text-medium-emphasis">Bộ tài liệu</span>
      </div>
      <v-btn
        icon="mdi-plus"
        variant="text"
        size="x-small"
        color="primary"
        title="Tạo bộ tài liệu mới"
        @click="showCreateSessionDialog = true"
      ></v-btn>
    </div>

    <v-list density="compact" nav class="px-2 pt-0 albums-list">
      <div
        v-if="sessions.length === 0"
        class="px-2 py-3 text-caption text-medium-emphasis text-center"
      >
        Chưa có bộ tài liệu nào.<br />
        Chọn ảnh trong thư viện để tạo.
      </div>
      <v-list-item
        v-for="s in sessions"
        :key="s.id"
        :to="'/session/' + s.id"
        color="primary"
        class="mb-1"
      >
        <template v-slot:prepend>
          <v-icon size="18" color="primary" class="mr-2">
            {{ s.status === 'completed' ? 'mdi-book-check-outline' : 'mdi-book-open-outline' }}
          </v-icon>
        </template>
        <v-list-item-title class="font-weight-medium text-body-2 text-truncate" :title="s.title">
          {{ s.title }}
        </v-list-item-title>
        <template v-slot:append>
          <v-chip size="x-small" variant="flat" color="surface-variant" class="font-weight-bold">
            {{ s.captureCount ?? s.issueCount ?? 0 }}
          </v-chip>
        </template>
      </v-list-item>
    </v-list>

    <v-divider></v-divider>

    <!-- Project List in Sidebar -->
    <div class="px-4 py-2 d-flex align-center justify-space-between">
      <span class="text-overline text-medium-emphasis">{{ t('sidebar.projects') }}</span>
      <v-btn
        icon="mdi-plus"
        variant="text"
        size="x-small"
        color="primary"
        title="Tạo dự án mới"
        @click="showCreateDialog = true"
      ></v-btn>
    </div>

    <v-list density="compact" nav class="px-2 pt-0 select-project-list">
      <v-list-item
        v-for="proj in projects"
        :key="proj.id"
        :active="proj.id === activeProjectId"
        color="primary"
        class="mb-1"
        @click="handleSelectProject(proj.id)"
      >
        <template v-slot:prepend>
          <v-icon size="18" :color="proj.color || 'primary'" class="mr-3">
            {{ proj.id === activeProjectId ? 'mdi-folder-open' : 'mdi-folder' }}
          </v-icon>
        </template>
        <v-list-item-title class="font-weight-medium text-body-2 text-truncate">
          {{ proj.name }}
        </v-list-item-title>
      </v-list-item>
    </v-list>

    <!-- Create Document Session Dialog -->
    <v-dialog v-model="showCreateSessionDialog" max-width="500px">
      <v-card bg-color="surface" class="pa-4">
        <v-card-title class="text-h5 font-weight-bold px-0 text-primary d-flex align-center gap-2">
          <v-icon icon="mdi-book-plus-outline" color="primary"></v-icon>
          <span>Tạo bộ tài liệu mới</span>
        </v-card-title>
        <v-card-text class="px-0 py-4">
          <v-form ref="sessionFormRef" v-model="isSessionFormValid" @submit.prevent="handleCreateSession">
            <v-text-field
              v-model="newSessionTitle"
              label="Tên bộ tài liệu / quy trình *"
              placeholder="Ví dụ: Quy trình bàn giao dự án..."
              :rules="[v => !!v?.trim() || 'Vui lòng nhập tên tài liệu']"
              variant="outlined"
              density="comfortable"
              class="mb-3"
              autofocus
              required
            ></v-text-field>

            <v-textarea
              v-model="newSessionDesc"
              label="Mô tả tóm tắt (tùy chọn)"
              placeholder="Ghi chú mục đích hoặc phạm vi của tài liệu..."
              variant="outlined"
              density="comfortable"
              rows="3"
            ></v-textarea>
          </v-form>
        </v-card-text>
        <v-card-actions class="px-0">
          <v-spacer></v-spacer>
          <v-btn
            variant="text"
            color="medium-emphasis"
            class="text-none"
            :disabled="isCreatingSession"
            @click="closeSessionDialog"
          >
            Hủy
          </v-btn>
          <v-btn
            variant="elevated"
            color="primary"
            class="text-none px-4 font-weight-bold"
            :disabled="!isSessionFormValid || isCreatingSession"
            :loading="isCreatingSession"
            @click="handleCreateSession"
          >
            Tạo tài liệu
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Create Project Dialog -->
    <v-dialog v-model="showCreateDialog" max-width="500px">
      <v-card bg-color="surface" class="pa-4">
        <v-card-title class="text-h5 font-weight-bold px-0 text-primary">
          {{ t('sidebar.createNewProject') }}
        </v-card-title>
        <v-card-text class="px-0 py-4">
          <v-form ref="form" v-model="isFormValid" @submit.prevent="handleCreate">
            <v-text-field
              v-model="newProjectName"
              :label="t('sidebar.projectName')"
              :placeholder="t('sidebar.projectNamePlaceholder')"
              :rules="[v => !!v || t('sidebar.projectNameRequired')]"
              variant="outlined"
              density="comfortable"
              class="mb-3"
              required
            ></v-text-field>

            <v-textarea
              v-model="newProjectDesc"
              :label="t('sidebar.projectDesc') + ' (' + t('sidebar.optional') + ')'"
              placeholder=""
              variant="outlined"
              density="comfortable"
              rows="3"
              class="mb-3"
            ></v-textarea>

            <div class="text-subtitle-2 mb-2 text-medium-emphasis">Màu nhấn dự án</div>
            <div class="d-flex gap-2 flex-wrap mb-2">
              <v-avatar
                v-for="color in presetColors"
                :key="color"
                :color="color"
                size="32"
                class="cursor-pointer border-avatar"
                :class="{ 'active-color': selectedColor === color }"
                @click="selectedColor = color"
              >
                <v-icon v-if="selectedColor === color" size="18" color="white">mdi-check</v-icon>
              </v-avatar>
            </div>
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
            :disabled="!isFormValid || isLoading"
            :loading="isLoading"
            @click="handleCreate"
          >
            {{ t('common.create') }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-navigation-drawer>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useProjectStore } from '@/stores/projectStore'
import { useSessionStore } from '@/stores/sessionStore'
import ProjectSelector from '@/components/dashboard/ProjectSelector.vue'
import { useI18n } from '@/composables/useI18n'

const router = useRouter()
const route = useRoute()

const projectStore = useProjectStore()
const sessionStore = useSessionStore()
const { t } = useI18n()
const { projects, activeProjectId, isLoading } = storeToRefs(projectStore)
const { sessions } = storeToRefs(sessionStore)
const { selectProject, createProject } = projectStore

function handleSelectProject(id: string) {
  selectProject(id)
  if (route.path !== '/') {
    router.push('/')
  }
}

// Watch active project to reload sessions
watch(
  () => activeProjectId.value,
  async (newId) => {
    if (newId) {
      await sessionStore.fetchSessionsByProject(newId)
    }
  },
  { immediate: true }
)

onMounted(async () => {
  if (activeProjectId.value) {
    await sessionStore.fetchSessionsByProject(activeProjectId.value)
  }
})

// Create Session State
const showCreateSessionDialog = ref(false)
const isSessionFormValid = ref(false)
const newSessionTitle = ref('')
const newSessionDesc = ref('')
const isCreatingSession = ref(false)

function closeSessionDialog() {
  showCreateSessionDialog.value = false
  newSessionTitle.value = ''
  newSessionDesc.value = ''
  isCreatingSession.value = false
}

async function handleCreateSession() {
  if (!newSessionTitle.value?.trim() || !activeProjectId.value) return

  isCreatingSession.value = true
  try {
    const created = await sessionStore.createSession({
      projectId: activeProjectId.value,
      title: newSessionTitle.value.trim(),
      description: newSessionDesc.value.trim() || undefined,
    })
    closeSessionDialog()
    router.push(`/session/${created.id}`)
  } catch (err) {
    console.error('Failed to create session:', err)
  } finally {
    isCreatingSession.value = false
  }
}

// Create Project State
const showCreateDialog = ref(false)
const isFormValid = ref(false)
const newProjectName = ref('')
const newProjectDesc = ref('')
const selectedColor = ref('#FF6B35')

const presetColors = [
  '#FF6B35', // Orange
  '#4ECDC4', // Teal
  '#FF4757', // Red
  '#FFA502', // Yellow
  '#3742FA', // Blue
  '#2ED573', // Green
  '#A29BFE', // Purple
  '#E84393', // Pink
]

function closeDialog() {
  showCreateDialog.value = false
  newProjectName.value = ''
  newProjectDesc.value = ''
  selectedColor.value = '#FF6B35'
}

async function handleCreate() {
  if (!newProjectName.value) return

  try {
    await createProject({
      name: newProjectName.value,
      description: newProjectDesc.value,
      color: selectedColor.value,
    })
    closeDialog()
  } catch (err) {
    console.error('Failed to create project:', err)
  }
}
</script>

<style scoped>
.gap-1 {
  gap: 4px;
}
.gap-2 {
  gap: 8px;
}
.albums-list {
  max-height: 220px;
  overflow-y: auto;
}
.select-project-list {
  max-height: 180px;
  overflow-y: auto;
}
.cursor-pointer {
  cursor: pointer;
}
.border-avatar {
  border: 2px solid transparent;
  transition: all 0.2s ease;
}
.border-avatar:hover {
  transform: scale(1.1);
}
.active-color {
  border-color: #ffffff;
  box-shadow: 0 0 8px rgba(255, 255, 255, 0.4);
}
</style>
