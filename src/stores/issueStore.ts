import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Issue, UpdateIssuePayload } from '@/types/issue'
import { useUiStore } from './uiStore'
import { useI18n } from '@/composables/useI18n'
import {
  getIssues as apiGetIssues,
  getIssue as apiGetIssue,
  updateIssue as apiUpdateIssue,
  deleteIssue as apiDeleteIssue,
} from '@/services/tauriCommands'

export const useIssueStore = defineStore('issue', () => {
  const issues = ref<Issue[]>([])
  const activeIssue = ref<Issue | null>(null)
  const isLoading = ref<boolean>(false)
  const error = ref<string | null>(null)

  const filters = ref({
    types: [] as string[],
    severities: [] as string[],
    statuses: [] as string[],
    tags: [] as string[],
  })

  const sortBy = ref<'newest' | 'oldest' | 'severity' | 'status'>('newest')

  const filteredIssues = computed(() => {
    const list = issues.value.filter(issue => {
      if (filters.value.types.length > 0 && !filters.value.types.includes(issue.issueType)) {
        return false
      }
      if (filters.value.severities.length > 0 && !filters.value.severities.includes(issue.severity)) {
        return false
      }
      if (filters.value.statuses.length > 0 && !filters.value.statuses.includes(issue.status)) {
        return false
      }
      if (filters.value.tags.length > 0) {
        const issueTagNames = (issue.tags || []).map(t => t.name)
        const hasMatchingTag = filters.value.tags.some(t => issueTagNames.includes(t))
        if (!hasMatchingTag) {
          return false
        }
      }
      return true
    })

    if (sortBy.value === 'newest') {
      return list.sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
    }
    if (sortBy.value === 'oldest') {
      return list.sort((a, b) => new Date(a.createdAt).getTime() - new Date(b.createdAt).getTime())
    }
    if (sortBy.value === 'severity') {
      const severityOrder: Record<string, number> = { Critical: 0, Major: 1, Minor: 2, Info: 3 }
      return list.sort((a, b) => {
        const orderA = severityOrder[a.severity] ?? 99
        const orderB = severityOrder[b.severity] ?? 99
        return orderA - orderB
      })
    }
    if (sortBy.value === 'status') {
      const statusOrder: Record<string, number> = { Draft: 0, Open: 1, 'In Progress': 2, Resolved: 3, Closed: 4 }
      return list.sort((a, b) => {
        const orderA = statusOrder[a.status] ?? 99
        const orderB = statusOrder[b.status] ?? 99
        return orderA - orderB
      })
    }
    return list
  })

  function clearFilters() {
    filters.value.types = []
    filters.value.severities = []
    filters.value.statuses = []
    filters.value.tags = []
  }


  async function fetchIssuesBySession(sessionId: string) {
    isLoading.value = true
    error.value = null
    try {
      issues.value = await apiGetIssues(sessionId)
    } catch (err: any) {
      error.value = err.message || String(err)
    } finally {
      isLoading.value = false
    }
  }

  async function fetchIssue(id: string) {
    isLoading.value = true
    error.value = null
    try {
      activeIssue.value = await apiGetIssue(id)
      return activeIssue.value
    } catch (err: any) {
      error.value = err.message || String(err)
      throw err
    } finally {
      isLoading.value = false
    }
  }

  async function updateIssue(id: string, payload: UpdateIssuePayload) {
    const uiStore = useUiStore()
    uiStore.setLoading(true)
    error.value = null
    try {
      const updated = await apiUpdateIssue(id, payload)
      // Update in issues list
      const idx = issues.value.findIndex(iss => iss.id === id)
      if (idx !== -1) {
        issues.value[idx] = updated
      }
      // Update active issue
      if (activeIssue.value && activeIssue.value.id === id) {
        activeIssue.value = updated
      }
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.issueUpdated'), type: 'success' })
      return updated
    } catch (err: any) {
      const msg = err.message || String(err)
      error.value = msg
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.failedUpdateIssue', { msg }), type: 'error' })
      throw err
    } finally {
      uiStore.setLoading(false)
    }
  }

  async function deleteIssue(id: string) {
    const uiStore = useUiStore()
    uiStore.setLoading(true)
    error.value = null
    try {
      await apiDeleteIssue(id)
      issues.value = issues.value.filter(iss => iss.id !== id)
      if (activeIssue.value && activeIssue.value.id === id) {
        activeIssue.value = null
      }
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.issueDeleted'), type: 'success' })
    } catch (err: any) {
      const msg = err.message || String(err)
      error.value = msg
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.failedDeleteIssue', { msg }), type: 'error' })
      throw err
    } finally {
      uiStore.setLoading(false)
    }
  }

  return {
    issues,
    activeIssue,
    isLoading,
    error,
    filters,
    sortBy,
    filteredIssues,
    clearFilters,
    fetchIssuesBySession,
    fetchIssue,
    updateIssue,
    deleteIssue,
  }
})
