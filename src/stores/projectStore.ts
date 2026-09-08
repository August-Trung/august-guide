import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { Project, CreateProjectPayload, UpdateProjectPayload } from '@/types/project'
import { useUiStore } from './uiStore'
import { useI18n } from '@/composables/useI18n'
import {
  getProjects as apiGetProjects,
  createProject as apiCreateProject,
  updateProject as apiUpdateProject,
  deleteProject as apiDeleteProject,
} from '@/services/tauriCommands'

export const useProjectStore = defineStore('project', () => {
  const projects = ref<Project[]>([])
  const activeProjectId = ref<string>('default')
  const isLoading = ref<boolean>(false)
  const error = ref<string | null>(null)

  const activeProject = computed(() => {
    return projects.value.find(p => p.id === activeProjectId.value) || null
  })

  async function fetchProjects(includeArchived: boolean = false) {
    isLoading.value = true
    error.value = null
    try {
      projects.value = await apiGetProjects(includeArchived)
      // Default to the first project or 'default' if not present
      if (!projects.value.some(p => p.id === activeProjectId.value)) {
        if (projects.value.length > 0) {
          activeProjectId.value = projects.value[0].id
        } else {
          activeProjectId.value = 'default'
        }
      }
    } catch (err: any) {
      error.value = err.message || String(err)
    } finally {
      isLoading.value = false
    }
  }

  function selectProject(id: string) {
    activeProjectId.value = id
  }

  async function createProject(payload: CreateProjectPayload) {
    const uiStore = useUiStore()
    uiStore.setLoading(true)
    error.value = null
    try {
      const newProj = await apiCreateProject(payload)
      projects.value.push(newProj)
      activeProjectId.value = newProj.id
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.projectCreated', { name: newProj.name }), type: 'success' })
      return newProj
    } catch (err: any) {
      const msg = err.message || String(err)
      error.value = msg
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.failedCreateProject', { msg }), type: 'error' })
      throw err
    } finally {
      uiStore.setLoading(false)
    }
  }

  async function updateProject(id: string, payload: UpdateProjectPayload) {
    const uiStore = useUiStore()
    uiStore.setLoading(true)
    error.value = null
    try {
      const updatedProj = await apiUpdateProject(id, payload)
      const index = projects.value.findIndex(p => p.id === id)
      if (index !== -1) {
        projects.value[index] = updatedProj
      }
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.projectUpdated', { name: updatedProj.name }), type: 'success' })
      return updatedProj
    } catch (err: any) {
      const msg = err.message || String(err)
      error.value = msg
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.failedUpdateProject', { msg }), type: 'error' })
      throw err
    } finally {
      uiStore.setLoading(false)
    }
  }

  async function deleteProject(id: string) {
    const uiStore = useUiStore()
    uiStore.setLoading(true)
    error.value = null
    try {
      await apiDeleteProject(id)
      const projName = projects.value.find(p => p.id === id)?.name || ''
      projects.value = projects.value.filter(p => p.id !== id)
      if (activeProjectId.value === id) {
        activeProjectId.value = projects.value[0]?.id || 'default'
      }
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.projectDeleted', { name: projName }), type: 'success' })
    } catch (err: any) {
      const msg = err.message || String(err)
      error.value = msg
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.failedDeleteProject', { msg }), type: 'error' })
      throw err
    } finally {
      uiStore.setLoading(false)
    }
  }

  return {
    projects,
    activeProjectId,
    activeProject,
    isLoading,
    error,
    fetchProjects,
    selectProject,
    createProject,
    updateProject,
    deleteProject,
  }
})
