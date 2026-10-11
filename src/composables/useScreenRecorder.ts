import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { listenToEvent } from '@/services/tauriEvents'

export const useScreenRecorderStore = defineStore('screenRecorder', () => {
  const isRecording = ref(false)
  const isPaused = ref(false)
  const isMicEnabled = ref(true)
  const duration = ref(0)
  const recordedBlob = ref<Blob | null>(null)
  const recordedUrl = ref<string | null>(null)
  const currentMimeType = ref<string>('video/webm')
  const showPreview = ref(false)
  const errorMessage = ref<string | null>(null)

  let screenStream: MediaStream | null = null
  let micStream: MediaStream | null = null
  let mediaRecorder: MediaRecorder | null = null
  let recordedChunks: BlobPart[] = []
  let timerInterval: ReturnType<typeof setInterval> | null = null

  // Guard locks to prevent double-trigger race conditions
  let isTransitioning = false
  let lastToggleTime = 0

  const formattedDuration = computed(() => {
    const totalSecs = duration.value
    const mins = Math.floor(totalSecs / 60)
    const secs = totalSecs % 60
    const pad = (n: number) => n.toString().padStart(2, '0')
    if (mins >= 60) {
      const hours = Math.floor(mins / 60)
      const remMins = mins % 60
      return `${pad(hours)}:${pad(remMins)}:${pad(secs)}`
    }
    return `${pad(mins)}:${pad(secs)}`
  })

  const startRecording = async () => {
    if (isRecording.value) return
    errorMessage.value = null
    recordedChunks = []
    recordedBlob.value = null
    if (recordedUrl.value) {
      URL.revokeObjectURL(recordedUrl.value)
      recordedUrl.value = null
    }

    try {
      // 1. Capture screen display
      screenStream = await navigator.mediaDevices.getDisplayMedia({
        video: {
          displaySurface: 'monitor',
          frameRate: { ideal: 30, max: 60 }
        },
        audio: false
      })

      // 2. Capture microphone if enabled
      if (isMicEnabled.value) {
        try {
          micStream = await navigator.mediaDevices.getUserMedia({
            audio: {
              echoCancellation: true,
              noiseSuppression: true,
              autoGainControl: true
            }
          })
        } catch (micErr) {
          console.warn('[ScreenRecorder] Mic capture failed or denied:', micErr)
        }
      }

      // 3. Combine tracks into single stream
      const tracks: MediaStreamTrack[] = [...screenStream.getVideoTracks()]
      if (micStream) {
        tracks.push(...micStream.getAudioTracks())
      }
      const combinedStream = new MediaStream(tracks)

      // Auto-stop if user cancels screen sharing via OS banner
      const videoTrack = screenStream.getVideoTracks()[0]
      if (videoTrack) {
        videoTrack.onended = () => {
          if (isRecording.value) {
            stopRecording()
          }
        }
      }

      // 4. Select supported container format
      let mimeType = 'video/webm'
      const candidates = [
        'video/webm;codecs=vp9,opus',
        'video/webm;codecs=vp8,opus',
        'video/webm',
        'video/mp4'
      ]
      for (const cand of candidates) {
        if (MediaRecorder.isTypeSupported(cand)) {
          mimeType = cand
          break
        }
      }
      currentMimeType.value = mimeType

      mediaRecorder = new MediaRecorder(combinedStream, {
        mimeType,
        videoBitsPerSecond: 4500000
      })

      mediaRecorder.ondataavailable = (e: BlobEvent) => {
        if (e.data && e.data.size > 0) {
          recordedChunks.push(e.data)
        }
      }

      mediaRecorder.onstop = () => {
        const actualMime = mediaRecorder?.mimeType || mimeType
        currentMimeType.value = actualMime
        const blob = new Blob(recordedChunks, { type: actualMime })
        recordedBlob.value = blob
        recordedUrl.value = URL.createObjectURL(blob)
        showPreview.value = true

        // Clean up active streams
        screenStream?.getTracks().forEach(t => t.stop())
        micStream?.getTracks().forEach(t => t.stop())
        screenStream = null
        micStream = null
      }

      mediaRecorder.start(1000)
      isRecording.value = true
      isPaused.value = false
      duration.value = 0

      // Start duration timer
      if (timerInterval) clearInterval(timerInterval)
      timerInterval = setInterval(() => {
        if (!isPaused.value) {
          duration.value++
        }
      }, 1000)
    } catch (err: any) {
      console.error('[ScreenRecorder] Start recording failed:', err)
      errorMessage.value = err?.message || 'Không thể bắt đầu ghi hình'
      discardRecording()
    }
  }

  const pauseRecording = () => {
    if (mediaRecorder && mediaRecorder.state === 'recording') {
      mediaRecorder.pause()
      isPaused.value = true
    }
  }

  const resumeRecording = () => {
    if (mediaRecorder && mediaRecorder.state === 'paused') {
      mediaRecorder.resume()
      isPaused.value = false
    }
  }

  const togglePause = () => {
    if (!isRecording.value) return
    if (isPaused.value) {
      resumeRecording()
    } else {
      pauseRecording()
    }
  }

  const toggleMic = () => {
    isMicEnabled.value = !isMicEnabled.value
    if (micStream) {
      micStream.getAudioTracks().forEach(t => {
        t.enabled = isMicEnabled.value
      })
    }
  }

  const stopRecording = async () => {
    if (!isRecording.value) return
    if (timerInterval) {
      clearInterval(timerInterval)
      timerInterval = null
    }

    isRecording.value = false
    isPaused.value = false

    if (mediaRecorder && mediaRecorder.state !== 'inactive') {
      try {
        mediaRecorder.requestData()
      } catch (_) {}
      mediaRecorder.stop()
    }
  }

  const toggleRecording = async () => {
    const now = Date.now()
    if (now - lastToggleTime < 1000 || isTransitioning) {
      console.log('[ScreenRecorder] Ignored duplicate toggle within debounce window')
      return
    }
    lastToggleTime = now
    isTransitioning = true

    try {
      if (isRecording.value) {
        await stopRecording()
      } else {
        // Do not allow starting a new recording if preview dialog is currently open
        if (showPreview.value) {
          console.log('[ScreenRecorder] Cannot start recording while preview dialog is open')
          return
        }
        await startRecording()
      }
    } finally {
      setTimeout(() => {
        isTransitioning = false
      }, 500)
    }
  }

  const discardRecording = () => {
    if (timerInterval) {
      clearInterval(timerInterval)
      timerInterval = null
    }

    screenStream?.getTracks().forEach(t => t.stop())
    micStream?.getTracks().forEach(t => t.stop())
    if (mediaRecorder && mediaRecorder.state !== 'inactive') {
      try {
        mediaRecorder.stop()
      } catch (_) {}
    }

    screenStream = null
    micStream = null
    mediaRecorder = null
    recordedChunks = []
    recordedBlob.value = null
    if (recordedUrl.value) {
      URL.revokeObjectURL(recordedUrl.value)
      recordedUrl.value = null
    }

    isRecording.value = false
    isPaused.value = false
    showPreview.value = false
    duration.value = 0
  }

  const closePreview = () => {
    showPreview.value = false
    if (recordedUrl.value) {
      URL.revokeObjectURL(recordedUrl.value)
      recordedUrl.value = null
    }
    recordedBlob.value = null
  }

  return {
    isRecording,
    isPaused,
    isMicEnabled,
    duration,
    formattedDuration,
    recordedBlob,
    recordedUrl,
    currentMimeType,
    showPreview,
    errorMessage,
    startRecording,
    pauseRecording,
    resumeRecording,
    togglePause,
    toggleMic,
    stopRecording,
    toggleRecording,
    discardRecording,
    closePreview
  }
})

let areHotkeysInitialized = false

function initGlobalListeners(store: ReturnType<typeof useScreenRecorderStore>) {
  if (areHotkeysInitialized) return
  areHotkeysInitialized = true

  listenToEvent('record:toggle', () => {
    store.toggleRecording()
  }).catch(err => console.warn('[ScreenRecorder] Tauri toggle listener warning:', err))

  listenToEvent('record:pause_resume', () => {
    store.togglePause()
  }).catch(err => console.warn('[ScreenRecorder] Tauri pause listener warning:', err))

  listenToEvent('record:toggle_mic', () => {
    store.toggleMic()
  }).catch(err => console.warn('[ScreenRecorder] Tauri mic listener warning:', err))

  window.addEventListener('keydown', (e: KeyboardEvent) => {
    if (e.key === 'F9') {
      e.preventDefault()
      store.toggleRecording()
    } else if (e.key === 'F10') {
      e.preventDefault()
      store.togglePause()
    } else if (e.key === 'F8') {
      e.preventDefault()
      store.toggleMic()
    } else if (e.key === 'Escape' && store.isRecording) {
      e.preventDefault()
      store.discardRecording()
    }
  })
}

export function useScreenRecorder() {
  const store = useScreenRecorderStore()
  initGlobalListeners(store)
  return store
}
