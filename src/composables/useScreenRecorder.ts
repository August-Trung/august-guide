import { ref, computed, onMounted, onUnmounted } from 'vue'
import { listenToEvent } from '@/services/tauriEvents'

const isRecording = ref(false)
const isPaused = ref(false)
const isMicEnabled = ref(true)
const duration = ref(0)
const recordedBlob = ref<Blob | null>(null)
const recordedUrl = ref<string | null>(null)
const showPreview = ref(false)
const errorMessage = ref<string | null>(null)

let screenStream: MediaStream | null = null
let micStream: MediaStream | null = null
let mediaRecorder: MediaRecorder | null = null
let recordedChunks: BlobPart[] = []
let timerInterval: ReturnType<typeof setInterval> | null = null
let unlistenToggle: (() => void) | null = null
let unlistenPauseResume: (() => void) | null = null
let unlistenToggleMic: (() => void) | null = null

export function useScreenRecorder() {
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
        const blob = new Blob(recordedChunks, { type: mimeType })
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

    if (mediaRecorder && mediaRecorder.state !== 'inactive') {
      mediaRecorder.stop()
    }

    isRecording.value = false
    isPaused.value = false
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

  const initGlobalHotkeys = async () => {
    try {
      unlistenToggle = await listenToEvent('record:toggle', () => {
        if (isRecording.value) {
          stopRecording()
        } else {
          startRecording()
        }
      })

      unlistenPauseResume = await listenToEvent('record:pause_resume', () => {
        if (isRecording.value) {
          togglePause()
        }
      })

      unlistenToggleMic = await listenToEvent('record:toggle_mic', () => {
        toggleMic()
      })
    } catch (err) {
      console.warn('[ScreenRecorder] Global hotkey registration warning:', err)
    }
  }

  const handleGlobalKeyDown = (e: KeyboardEvent) => {
    if (e.key === 'F9') {
      e.preventDefault()
      if (isRecording.value) {
        stopRecording()
      } else {
        startRecording()
      }
    } else if (e.key === 'F10') {
      e.preventDefault()
      if (isRecording.value) {
        togglePause()
      }
    } else if (e.key === 'F8') {
      e.preventDefault()
      toggleMic()
    } else if (e.key === 'Escape' && isRecording.value) {
      e.preventDefault()
      discardRecording()
    }
  }

  onMounted(() => {
    window.addEventListener('keydown', handleGlobalKeyDown)
    initGlobalHotkeys()
  })

  onUnmounted(() => {
    window.removeEventListener('keydown', handleGlobalKeyDown)
    unlistenToggle?.()
    unlistenPauseResume?.()
    unlistenToggleMic?.()
  })

  return {
    isRecording,
    isPaused,
    isMicEnabled,
    duration,
    formattedDuration,
    recordedBlob,
    recordedUrl,
    showPreview,
    errorMessage,
    startRecording,
    pauseRecording,
    resumeRecording,
    togglePause,
    toggleMic,
    stopRecording,
    discardRecording,
    closePreview
  }
}
