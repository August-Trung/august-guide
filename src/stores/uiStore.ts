import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface ToastOptions {
  message: string
  type: 'success' | 'error' | 'info'
  duration?: number
}

export const useUiStore = defineStore('ui', () => {
  const isLoading = ref(false)
  
  // Toast state
  const toastVisible = ref(false)
  const toastMessage = ref('')
  const toastType = ref<'success' | 'error' | 'info'>('info')
  const toastTimeout = ref<number | null>(null)

  function showToast(options: ToastOptions) {
    if (toastTimeout.value) {
      clearTimeout(toastTimeout.value)
    }
    toastMessage.value = options.message
    toastType.value = options.type
    toastVisible.value = true

    const duration = options.duration || 3000
    toastTimeout.value = window.setTimeout(() => {
      toastVisible.value = false
    }, duration)
  }

  function hideToast() {
    toastVisible.value = false
    if (toastTimeout.value) {
      clearTimeout(toastTimeout.value)
      toastTimeout.value = null
    }
  }

  function setLoading(loading: boolean) {
    isLoading.value = loading
  }

  return {
    isLoading,
    toastVisible,
    toastMessage,
    toastType,
    showToast,
    hideToast,
    setLoading
  }
})
