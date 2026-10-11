<template>
  <div
    v-if="isRecording"
    ref="widgetRef"
    class="recording-hud-bar"
    :class="{ 'is-paused': isPaused, 'is-dragging': isDraggingActive }"
    :style="widgetStyle"
    @mousedown="startDrag"
  >
    <!-- Drag Grip -->
    <div class="drag-handle" title="Nắm giữ để di chuyển thanh điều khiển">
      <v-icon icon="mdi-drag-vertical" size="18" color="medium-emphasis"></v-icon>
    </div>

    <!-- Status & Timer Indicator -->
    <div class="d-flex align-center gap-2 px-2 pointer-events-none">
      <span class="rec-dot" :class="{ 'pulsing': !isPaused }"></span>
      <span class="rec-text font-weight-black">REC</span>
      <span class="rec-timer font-weight-bold font-mono">{{ formattedDuration }}</span>
      <v-chip
        v-if="isPaused"
        size="x-small"
        color="warning"
        variant="flat"
        class="font-weight-bold text-uppercase ml-1"
      >
        Tạm dừng
      </v-chip>
    </div>

    <v-divider vertical class="mx-1 my-2"></v-divider>

    <!-- Control Action Buttons with Tooltips -->
    <div class="d-flex align-center gap-1 px-1">
      <!-- Pause / Resume -->
      <v-btn
        icon
        size="small"
        variant="text"
        :color="isPaused ? 'warning' : 'default'"
        @click.stop="togglePause"
        :title="isPaused ? 'Tiếp tục ghi (F10)' : 'Tạm dừng ghi (F10)'"
      >
        <v-icon :icon="isPaused ? 'mdi-play' : 'mdi-pause'" size="18"></v-icon>
        <v-tooltip activator="parent" location="bottom">
          {{ isPaused ? 'Tiếp tục (F10)' : 'Tạm dừng (F10)' }}
        </v-tooltip>
      </v-btn>

      <!-- Toggle Microphone -->
      <v-btn
        icon
        size="small"
        variant="text"
        :color="isMicEnabled ? 'default' : 'error'"
        @click.stop="toggleMic"
        :title="isMicEnabled ? 'Tắt tiếng Mic (F8)' : 'Bật tiếng Mic (F8)'"
      >
        <v-icon :icon="isMicEnabled ? 'mdi-microphone' : 'mdi-microphone-off'" size="18"></v-icon>
        <v-tooltip activator="parent" location="bottom">
          {{ isMicEnabled ? 'Mic: Bật (F8)' : 'Mic: Tắt (F8)' }}
        </v-tooltip>
      </v-btn>

      <!-- Stop & Save (Finish) -->
      <v-btn
        color="primary"
        variant="elevated"
        size="small"
        class="px-3 text-none font-weight-bold ml-1"
        prepend-icon="mdi-stop"
        @click.stop="stopRecording"
      >
        Dừng (F9)
        <v-tooltip activator="parent" location="bottom">
          Hoàn thành & Lưu (F9)
        </v-tooltip>
      </v-btn>

      <!-- Discard / Cancel -->
      <v-btn
        icon
        size="small"
        variant="text"
        color="error"
        @click.stop="confirmDiscard"
        title="Hủy bỏ (Esc)"
      >
        <v-icon icon="mdi-close" size="18"></v-icon>
        <v-tooltip activator="parent" location="bottom">
          Hủy bỏ không lưu (Esc)
        </v-tooltip>
      </v-btn>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useScreenRecorder } from '@/composables/useScreenRecorder'

const recorder = useScreenRecorder()
const {
  isRecording,
  isPaused,
  isMicEnabled,
  formattedDuration
} = storeToRefs(recorder)

const {
  togglePause,
  toggleMic,
  stopRecording,
  discardRecording
} = recorder

// Dragging logic
const widgetRef = ref<HTMLElement | null>(null)
const position = ref<{ x: number | null; y: number | null }>({ x: null, y: null })
const isDraggingActive = ref(false)

let startMouseX = 0
let startMouseY = 0
let startElemX = 0
let startElemY = 0

const startDrag = (e: MouseEvent) => {
  // Prevent drag when clicking buttons
  if ((e.target as HTMLElement)?.closest('button')) return

  isDraggingActive.value = true
  startMouseX = e.clientX
  startMouseY = e.clientY

  if (widgetRef.value) {
    const rect = widgetRef.value.getBoundingClientRect()
    startElemX = rect.left
    startElemY = rect.top
  }

  window.addEventListener('mousemove', onDrag)
  window.addEventListener('mouseup', stopDrag)
}

const onDrag = (e: MouseEvent) => {
  if (!isDraggingActive.value) return
  const dx = e.clientX - startMouseX
  const dy = e.clientY - startMouseY

  const elemWidth = widgetRef.value?.offsetWidth ?? 280
  const elemHeight = widgetRef.value?.offsetHeight ?? 48

  const newLeft = Math.max(10, Math.min(window.innerWidth - elemWidth - 10, startElemX + dx))
  const newTop = Math.max(10, Math.min(window.innerHeight - elemHeight - 10, startElemY + dy))

  position.value = { x: newLeft, y: newTop }
}

const stopDrag = () => {
  isDraggingActive.value = false
  window.removeEventListener('mousemove', onDrag)
  window.removeEventListener('mouseup', stopDrag)
}

const widgetStyle = computed(() => {
  if (position.value.x !== null && position.value.y !== null) {
    return {
      left: `${position.value.x}px`,
      top: `${position.value.y}px`,
      bottom: 'auto',
      transform: 'none'
    }
  }
  // Default: positioned at top center (Dynamic Island) away from Chromium bottom banner
  return {
    top: '24px',
    bottom: 'auto',
    left: '50%',
    transform: 'translateX(-50%)'
  }
})

const confirmDiscard = () => {
  if (confirm('Bạn có chắc chắn muốn hủy bỏ bản quay này không? Dữ liệu chưa lưu sẽ bị xóa.')) {
    discardRecording()
  }
}
</script>

<style scoped>
.recording-hud-bar {
  position: fixed;
  z-index: 999999;
  display: flex;
  align-items: center;
  background: rgba(15, 23, 42, 0.94);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(56, 189, 248, 0.5);
  border-radius: 36px;
  padding: 4px 10px;
  box-shadow: 0 10px 35px rgba(0, 0, 0, 0.7), 0 0 15px rgba(56, 189, 248, 0.2);
  user-select: none;
  cursor: grab;
  transition: border-color 0.2s, box-shadow 0.2s;
}

.recording-hud-bar.is-dragging {
  cursor: grabbing;
  box-shadow: 0 14px 40px rgba(0, 0, 0, 0.8), 0 0 20px rgba(56, 189, 248, 0.4);
}

.recording-hud-bar.is-paused {
  border-color: rgba(245, 158, 11, 0.6);
  box-shadow: 0 10px 35px rgba(0, 0, 0, 0.7), 0 0 15px rgba(245, 158, 11, 0.2);
}

.drag-handle {
  display: flex;
  align-items: center;
  cursor: grab;
}

.pointer-events-none {
  pointer-events: none;
}

.rec-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: #ef4444;
  display: inline-block;
}

.rec-dot.pulsing {
  animation: pulse-red 1.2s infinite ease-in-out;
}

@keyframes pulse-red {
  0% {
    transform: scale(0.9);
    opacity: 0.6;
    box-shadow: 0 0 0 0 rgba(239, 68, 68, 0.7);
  }
  70% {
    transform: scale(1.15);
    opacity: 1;
    box-shadow: 0 0 0 6px rgba(239, 68, 68, 0);
  }
  100% {
    transform: scale(0.9);
    opacity: 0.6;
    box-shadow: 0 0 0 0 rgba(239, 68, 68, 0);
  }
}

.rec-text {
  font-size: 11px;
  letter-spacing: 0.5px;
  color: #ef4444;
}

.rec-timer {
  font-size: 14px;
  color: #f8fafc;
  min-width: 48px;
}

.gap-1 {
  gap: 4px;
}

.gap-2 {
  gap: 8px;
}
</style>
