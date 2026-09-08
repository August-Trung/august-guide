import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { getAllSettings, updateSetting, connectGdrive, disconnectGdrive, backupToGdrive, restoreFromGdrive, listBackupsOnGdrive } from '@/services/tauriCommands'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<Record<string, string>>({})
  const loading = ref(false)
  const backupLoading = ref(false)

  async function loadSettings() {
    loading.value = true
    try {
      const allSettings = await getAllSettings()
      settings.value = allSettings
    } catch (e) {
      console.error('Failed to load settings:', e)
    } finally {
      loading.value = false
    }
  }

  // Get a parsed setting value, or return a default value if not set
  function getSettingValue<T>(key: string, defaultValue: T): T {
    const rawValue = settings.value[key]
    if (rawValue === undefined) {
      return defaultValue
    }
    try {
      return JSON.parse(rawValue) as T
    } catch (e) {
      console.error(`Failed to parse setting key: ${key}, value: ${rawValue}`, e)
      return defaultValue
    }
  }

  async function setSettingValue(key: string, value: any) {
    const serialized = JSON.stringify(value)
    settings.value[key] = serialized
    try {
      await updateSetting(key, serialized)
    } catch (e) {
      console.error(`Failed to update setting ${key}:`, e)
    }
  }

  // Helper computed properties
  const theme = computed({
    get: () => getSettingValue<'dark' | 'light' | 'system'>('theme', 'dark'),
    set: (value) => setSettingValue('theme', value)
  })

  const overlayTrigger = computed({
    get: () => getSettingValue<string>('overlay_trigger', 'middle_mouse_hold'),
    set: (value) => setSettingValue('overlay_trigger', value)
  })

  const holdDurationMs = computed({
    get: () => getSettingValue<number>('hold_duration_ms', 1000),
    set: (value) => setSettingValue('hold_duration_ms', value)
  })

  const screenshotQuality = computed({
    get: () => getSettingValue<number>('screenshot_quality', 90),
    set: (value) => setSettingValue('screenshot_quality', value)
  })

  const defaultProjectId = computed({
    get: () => getSettingValue<string>('default_project_id', 'default'),
    set: (value) => setSettingValue('default_project_id', value)
  })

  const autoBackup = computed({
    get: () => getSettingValue<boolean>('auto_backup', false),
    set: (value) => setSettingValue('auto_backup', value)
  })

  const gdriveConnected = computed({
    get: () => getSettingValue<boolean>('gdrive_connected', false),
    set: (value) => setSettingValue('gdrive_connected', value)
  })

  const gdriveEmail = computed({
    get: () => getSettingValue<string | null>('gdrive_user_email', null),
    set: (value) => setSettingValue('gdrive_user_email', value)
  })

  const gdriveLastBackupTime = computed({
    get: () => getSettingValue<string | null>('gdrive_last_backup_time', null),
    set: (value) => setSettingValue('gdrive_last_backup_time', value)
  })

  async function connectGDrive() {
    loading.value = true
    try {
      const email = await connectGdrive()
      gdriveConnected.value = true
      gdriveEmail.value = email
    } catch (e) {
      console.error('Failed to connect Google Drive:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function disconnectGDrive() {
    loading.value = true
    try {
      await disconnectGdrive()
      gdriveConnected.value = false
      gdriveEmail.value = null
    } catch (e) {
      console.error('Failed to disconnect Google Drive:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function backupNow() {
    backupLoading.value = true
    try {
      const time = await backupToGdrive()
      gdriveLastBackupTime.value = time
      return time
    } catch (e) {
      console.error('Failed to backup to Google Drive:', e)
      throw e
    } finally {
      backupLoading.value = false
    }
  }

  async function restoreFrom(fileId: string) {
    backupLoading.value = true
    try {
      await restoreFromGdrive(fileId)
    } catch (e) {
      console.error('Failed to restore from Google Drive:', e)
      throw e
    } finally {
      backupLoading.value = false
    }
  }

  async function listBackups() {
    try {
      return await listBackupsOnGdrive()
    } catch (e) {
      console.error('Failed to list backups on Google Drive:', e)
      throw e
    }
  }

  const minimizeToTray = computed({
    get: () => getSettingValue<boolean>('minimize_to_tray', false),
    set: (value) => setSettingValue('minimize_to_tray', value)
  })

  const language = computed({
    get: () => getSettingValue<'en' | 'vi'>('language', 'en'),
    set: (value) => setSettingValue('language', value)
  })

  return {
    settings,
    loading,
    backupLoading,
    loadSettings,
    getSettingValue,
    setSettingValue,
    theme,
    overlayTrigger,
    holdDurationMs,
    screenshotQuality,
    defaultProjectId,
    autoBackup,
    gdriveConnected,
    gdriveEmail,
    gdriveLastBackupTime,
    connectGDrive,
    disconnectGDrive,
    backupNow,
    restoreFrom,
    listBackups,
    minimizeToTray,
    language
  }
})
