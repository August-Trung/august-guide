import { ref } from 'vue'
import { check, type Update } from '@tauri-apps/plugin-updater'
import { restartApp } from '@/services/tauriCommands'
import { useUiStore } from '@/stores/uiStore'

const isChecking = ref(false)
const isDownloading = ref(false)
const downloadProgress = ref(0)
const downloadedBytes = ref(0)
const totalBytes = ref(0)
const updateAvailable = ref(false)
const updateManifest = ref<{ version: string; date?: string; body?: string } | null>(null)
const showUpdateModal = ref(false)
const errorMessage = ref('')

let activeUpdate: Update | null = null

export function useAppUpdater() {
  const uiStore = useUiStore()

  async function checkForUpdates(silent = false) {
    if (isChecking.value || isDownloading.value) return
    isChecking.value = true
    errorMessage.value = ''

    try {
      const update = await check()
      if (update && update.available) {
        activeUpdate = update
        updateAvailable.value = true
        updateManifest.value = {
          version: update.version,
          date: update.date,
          body: update.body || 'Bản cập nhật mới với nhiều cải tiến và sửa lỗi hiệu năng.'
        }
        showUpdateModal.value = true
      } else {
        updateAvailable.value = false
        activeUpdate = null
        if (!silent) {
          uiStore.showToast({
            message: 'Bạn đang sử dụng phiên bản mới nhất!',
            type: 'info'
          })
        }
      }
    } catch (err: any) {
      console.warn('[Updater] Check failed:', err)
      const errStr = err?.message || String(err)
      errorMessage.value = errStr
      if (!silent) {
        uiStore.showToast({
          message: 'Không thể kiểm tra cập nhật: ' + errStr,
          type: 'error'
        })
      }
    } finally {
      isChecking.value = false
    }
  }

  async function downloadAndInstallUpdate() {
    if (!activeUpdate || isDownloading.value) return
    isDownloading.value = true
    downloadProgress.value = 0
    downloadedBytes.value = 0
    totalBytes.value = 0
    errorMessage.value = ''

    try {
      let downloaded = 0
      let contentLength = 0

      await activeUpdate.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            contentLength = event.data.contentLength || 0
            totalBytes.value = contentLength
            downloadedBytes.value = 0
            downloadProgress.value = 0
            break
          case 'Progress':
            downloaded += event.data.chunkLength
            downloadedBytes.value = downloaded
            if (contentLength > 0) {
              downloadProgress.value = Math.min(100, Math.round((downloaded / contentLength) * 100))
            }
            break
          case 'Finished':
            downloadProgress.value = 100
            break
        }
      })

      uiStore.showToast({
        message: 'Tải cập nhật thành công! Đang khởi động lại ứng dụng...',
        type: 'success'
      })

      setTimeout(async () => {
        try {
          await restartApp()
        } catch (e) {
          console.error('[Updater] Failed to restart app:', e)
          window.location.reload()
        }
      }, 1500)
    } catch (err: any) {
      console.error('[Updater] Download & Install failed:', err)
      errorMessage.value = err?.message || String(err)
      uiStore.showToast({
        message: 'Lỗi cập nhật: ' + (err?.message || err),
        type: 'error'
      })
      isDownloading.value = false
    }
  }

  function dismissModal() {
    if (isDownloading.value) return
    showUpdateModal.value = false
  }

  return {
    isChecking,
    isDownloading,
    downloadProgress,
    downloadedBytes,
    totalBytes,
    updateAvailable,
    updateManifest,
    showUpdateModal,
    errorMessage,
    checkForUpdates,
    downloadAndInstallUpdate,
    dismissModal
  }
}
