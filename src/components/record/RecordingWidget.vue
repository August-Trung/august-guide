<template>
  <div
    v-if="isRecording"
    data-tauri-drag-region
    class="recording-hud-bar"
    :class="{ 'is-paused': isPaused }"
  >
    <!-- Drag Grip -->
    <div data-tauri-drag-region class="drag-handle" title="Nắm để di chuyển">
      <v-icon icon="mdi-drag-vertical" size="18" color="medium-emphasis"></v-icon>
    </div>

    <!-- Status & Timer Indicator -->
    <div data-tauri-drag-region class="d-flex align-center gap-2 px-2">
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
        <v-tooltip activator="parent" location="top">
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
        <v-tooltip activator="parent" location="top">
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
        <v-tooltip activator="parent" location="top">
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
        <v-tooltip activator="parent" location="top">
          Hủy bỏ không lưu (Esc)
        </v-tooltip>
      </v-btn>
    </div>
  </div>
</template>

<script setup lang="ts">
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

const confirmDiscard = () => {
  if (confirm('Bạn có chắc chắn muốn hủy bỏ bản quay này không? Dữ liệu chưa lưu sẽ bị xóa.')) {
    discardRecording()
  }
}
</script>

<style scoped>
.recording-hud-bar {
  position: fixed;
  bottom: 30px;
  left: 50%;
  transform: translateX(-50%);
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

.recording-hud-bar:active {
  cursor: grabbing;
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
