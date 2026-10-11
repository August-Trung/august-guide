import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { Session, CreateSessionPayload, UpdateSessionPayload } from '@/types/session'
import { Capture } from '@/types/capture'
import { useUiStore } from './uiStore'
import { useI18n } from '@/composables/useI18n'
import {
  getSessions as apiGetSessions,
  getSessionsByProject as apiGetSessionsByProject,
  createSession as apiCreateSession,
  updateSession as apiUpdateSession,
  deleteSession as apiDeleteSession,
  getAllCapturesByProject,
  createSessionFromCaptures as apiCreateSessionFromCaptures,
} from '@/services/tauriCommands'

export const useSessionStore = defineStore('session', () => {
  const sessions = ref<Session[]>([])
  const isLoading = ref<boolean>(false)
  const error = ref<string | null>(null)
  const sortBy = ref<'newest' | 'oldest' | 'issues_desc' | 'status'>('newest')

  const sortedSessions = computed(() => {
    const list = [...sessions.value]
    if (sortBy.value === 'newest') {
      return list.sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
    }
    if (sortBy.value === 'oldest') {
      return list.sort((a, b) => new Date(a.createdAt).getTime() - new Date(b.createdAt).getTime())
    }
    if (sortBy.value === 'issues_desc') {
      return list.sort((a, b) => (b.issueCount || 0) - (a.issueCount || 0))
    }
    if (sortBy.value === 'status') {
      const statusOrder: Record<string, number> = { active: 0, completed: 1, archived: 2 }
      return list.sort((a, b) => {
        const orderA = statusOrder[a.status] ?? 99
        const orderB = statusOrder[b.status] ?? 99
        return orderA - orderB
      })
    }
    return list
  })

  async function fetchSessions() {
    isLoading.value = true
    error.value = null
    try {
      sessions.value = await apiGetSessions()
    } catch (err: any) {
      error.value = err.message || String(err)
    } finally {
      isLoading.value = false
    }
  }

  async function fetchSessionsByProject(projectId: string) {
    isLoading.value = true
    error.value = null
    try {
      sessions.value = await apiGetSessionsByProject(projectId)
    } catch (err: any) {
      error.value = err.message || String(err)
    } finally {
      isLoading.value = false
    }
  }

  async function createSession(payload: CreateSessionPayload) {
    const uiStore = useUiStore()
    uiStore.setLoading(true)
    error.value = null
    try {
      const newSession = await apiCreateSession(payload)
      sessions.value.unshift(newSession)
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.sessionCreated', { title: newSession.title }), type: 'success' })
      return newSession
    } catch (err: any) {
      const msg = err.message || String(err)
      error.value = msg
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.failedCreateSession', { msg }), type: 'error' })
      throw err
    } finally {
      uiStore.setLoading(false)
    }
  }

  async function updateSession(id: string, payload: UpdateSessionPayload) {
    const uiStore = useUiStore()
    uiStore.setLoading(true)
    error.value = null
    try {
      const updatedSession = await apiUpdateSession(id, payload)
      const index = sessions.value.findIndex(s => s.id === id)
      if (index !== -1) {
        sessions.value[index] = updatedSession
      }
      
      const { t } = useI18n()
      if (payload.status === 'completed') {
        uiStore.showToast({ message: t('toasts.sessionCompleted', { title: updatedSession.title }), type: 'success' })
      } else {
        uiStore.showToast({ message: t('toasts.sessionUpdated', { title: updatedSession.title }), type: 'success' })
      }
      return updatedSession
    } catch (err: any) {
      const msg = err.message || String(err)
      error.value = msg
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.failedUpdateSession', { msg }), type: 'error' })
      throw err
    } finally {
      uiStore.setLoading(false)
    }
  }

  async function deleteSession(id: string) {
    const uiStore = useUiStore()
    uiStore.setLoading(true)
    error.value = null
    try {
      const sessionTitle = sessions.value.find(s => s.id === id)?.title || ''
      await apiDeleteSession(id)
      sessions.value = sessions.value.filter(s => s.id !== id)
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.sessionDeleted', { title: sessionTitle }), type: 'success' })
    } catch (err: any) {
      const msg = err.message || String(err)
      error.value = msg
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.failedDeleteSession', { msg }), type: 'error' })
      throw err
    } finally {
      uiStore.setLoading(false)
    }
  }

  const projectCaptures = ref<Capture[]>([])
  const isLoadingCaptures = ref<boolean>(false)

  async function fetchProjectCaptures(projectId: string) {
    isLoadingCaptures.value = true
    try {
      projectCaptures.value = await getAllCapturesByProject(projectId)
    } catch (err: any) {
      console.error('Failed to load project captures:', err)
    } finally {
      isLoadingCaptures.value = false
    }
  }

  async function createSessionFromSelected(
    projectId: string,
    title: string,
    captureIds: string[],
    description?: string
  ) {
    const uiStore = useUiStore()
    uiStore.setLoading(true)
    try {
      const newSession = await apiCreateSessionFromCaptures(projectId, title, captureIds, description)
      sessions.value.unshift(newSession)
      uiStore.showToast({ message: `Đã tạo bộ tài liệu "${newSession.title}" từ ${captureIds.length} ảnh!`, type: 'success' })
      return newSession
    } catch (err: any) {
      console.error('Failed to create session from captures:', err)
      uiStore.showToast({ message: `Lỗi khi tạo bộ tài liệu: ${err.message || err}`, type: 'error' })
      throw err
    } finally {
      uiStore.setLoading(false)
    }
  }

  return {
    sessions,
    isLoading,
    error,
    sortBy,
    sortedSessions,
    projectCaptures,
    isLoadingCaptures,
    fetchSessions,
    fetchSessionsByProject,
    fetchProjectCaptures,
    createSession,
    createSessionFromSelected,
    updateSession,
    deleteSession,
  }
})
