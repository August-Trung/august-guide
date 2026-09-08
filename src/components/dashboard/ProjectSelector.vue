<template>
  <div>
    <!-- Project Picker Button -->
    <v-menu offset="y">
      <template v-slot:activator="{ props }">
        <v-btn
          v-bind="props"
          variant="outlined"
          color="secondary"
          class="w-100 text-none justify-start px-3"
          prepend-icon="mdi-folder-outline"
          append-icon="mdi-chevron-down"
        >
          <span class="text-truncate d-inline-block text-left" style="max-width: 140px;">
            {{ activeProject ? activeProject.name : t('common.selectProject') }}
          </span>
        </v-btn>
      </template>

      <v-list density="compact" bg-color="surface" width="220">
        <v-list-subheader class="text-overline">{{ t('common.projectsHeader') }}</v-list-subheader>
        
        <v-list-item
          v-for="proj in projects"
          :key="proj.id"
          :value="proj.id"
          :active="proj.id === activeProjectId"
          color="primary"
          @click="selectProject(proj.id)"
        >
          <template v-slot:prepend>
            <v-icon :color="proj.color || 'primary'" class="mr-2">mdi-folder</v-icon>
          </template>
          <v-list-item-title class="font-weight-medium text-truncate" style="max-width: 150px;">
            {{ proj.name }}
          </v-list-item-title>
        </v-list-item>

        <v-divider class="my-1"></v-divider>

        <v-list-item
          prepend-icon="mdi-plus"
          color="primary"
          @click="showCreateDialog = true"
        >
          <v-list-item-title class="font-weight-bold text-primary">
            {{ t('common.createProject') }}
          </v-list-item-title>
        </v-list-item>
      </v-list>
    </v-menu>

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
              :placeholder="t('common.description')"
              variant="outlined"
              density="comfortable"
              rows="3"
              class="mb-3"
            ></v-textarea>

            <div class="text-subtitle-2 mb-2 text-medium-emphasis">{{ t('common.projectColorAccent') }}</div>
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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useProjectStore } from '@/stores/projectStore'
import { useI18n } from '@/composables/useI18n'

const { t } = useI18n()
const projectStore = useProjectStore()
const { projects, activeProjectId, activeProject, isLoading } = storeToRefs(projectStore)
const { fetchProjects, selectProject, createProject } = projectStore

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

onMounted(() => {
  fetchProjects()
})

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
.gap-2 {
  gap: 8px;
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
