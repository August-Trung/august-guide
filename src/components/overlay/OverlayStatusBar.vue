<template>
  <div>
    <!-- Full Top Status Bar -->
    <transition name="topbar-slide">
      <div v-if="!collapsed" class="overlay-status-bar">
        <div class="bar-left">
          <span class="session-badge">Guide</span>
          <span class="session-name">{{ sessionName || 'Hướng dẫn thao tác' }}</span>
          <span class="divider"></span>
          <span class="issue-count">{{ issueCount }} bước</span>
        </div>
        <div class="bar-right">
          <button
            type="button"
            class="btn-history mr-2"
            :disabled="!canUndo"
            :title="t('overlay.undo', 'Undo') + ' (Ctrl+Z)'"
            @click="overlayStore.undo()"
          >
            <i class="mdi mdi-undo"></i>
          </button>
          <button
            type="button"
            class="btn-history mr-3"
            :disabled="!canRedo"
            :title="t('overlay.redo', 'Redo') + ' (Ctrl+Y)'"
            @click="overlayStore.redo()"
          >
            <i class="mdi mdi-redo"></i>
          </button>

          <button
            type="button"
            class="btn-copy mr-2"
            :title="t('issueDetail.copyImage') + ' (Ctrl+C)'"
            @click="$emit('copy')"
          >
            <i class="mdi mdi-content-copy mr-1"></i> {{ t('issueDetail.copyImage') }}
          </button>

          <button class="btn-cancel" @click="$emit('cancel')">
            {{ t('overlay.discard') }} <kbd>Esc</kbd>
          </button>
          <button class="btn-done" @click="$emit('done')">
            {{ t('common.ok', 'Done') }}
          </button>

          <!-- Collapse Top Bar Button -->
          <button
            type="button"
            class="btn-collapse ml-2"
            title="Ẩn thanh trên (H)"
            @click="collapsed = true"
          >
            <i class="mdi mdi-chevron-up"></i>
          </button>
        </div>
      </div>
    </transition>

    <!-- Mini Collapsed Top Pill -->
    <transition name="topbar-pill-fade">
      <button
        v-if="collapsed"
        type="button"
        class="topbar-collapsed-pill"
        title="Hiện thanh trên (H)"
        @click="collapsed = false"
      >
        <span class="pill-badge">Guide</span>
        <span class="pill-text">{{ sessionName || 'August Guide' }} ({{ issueCount }} bước)</span>
        <i class="mdi mdi-chevron-down ml-1"></i>
      </button>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useOverlayStore } from '@/stores/overlayStore'
import { useI18n } from '@/composables/useI18n'

const collapsed = defineModel<boolean>('collapsed', { default: false })

defineProps<{
  sessionName: string
  issueCount: number
}>()

defineEmits<{
  (e: 'cancel'): void
  (e: 'done'): void
  (e: 'copy'): void
}>()

const overlayStore = useOverlayStore()
const { t } = useI18n()
const canUndo = computed(() => overlayStore.undoStack.length > 0)
const canRedo = computed(() => overlayStore.redoStack.length > 0)
</script>

<style scoped>
.overlay-status-bar {
  position: fixed;
  top: 16px;
  left: 50%;
  transform: translateX(-50%);
  height: 48px;
  min-width: 580px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  background: rgba(22, 25, 35, 0.90);
  backdrop-filter: blur(14px);
  -webkit-backdrop-filter: blur(14px);
  border: 1px solid rgba(255, 255, 255, 0.14);
  border-radius: 24px;
  color: #E8E8E8;
  z-index: 9999;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.45);
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  white-space: nowrap;
}

.bar-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.bar-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.session-badge {
  background: rgba(255, 107, 53, 0.15);
  color: #FF6B35;
  font-size: 0.72rem;
  font-weight: 800;
  padding: 3px 9px;
  border-radius: 4px;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  border: 1px solid rgba(255, 107, 53, 0.30);
}

.session-name {
  font-size: 0.95rem;
  font-weight: 600;
  color: #E8E8E8;
}

.divider {
  display: inline-block;
  width: 1px;
  height: 16px;
  background: rgba(255, 255, 255, 0.15);
}

.issue-count {
  font-size: 0.85rem;
  color: rgba(232, 232, 232, 0.55);
}

/* Buttons */
.btn-cancel,
.btn-copy,
.btn-done {
  border: none;
  border-radius: 6px;
  padding: 8px 16px;
  font-size: 0.875rem;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.15s ease, transform 0.1s ease;
  font-family: inherit;
  line-height: 1;
  white-space: nowrap;
}

.btn-cancel,
.btn-copy {
  background: rgba(255, 255, 255, 0.08);
  color: #E8E8E8;
  border: 1px solid rgba(255, 255, 255, 0.12);
}

.btn-copy {
  display: flex;
  align-items: center;
  gap: 4px;
}

.btn-cancel:hover,
.btn-copy:hover {
  background: rgba(255, 255, 255, 0.14);
  transform: translateY(-1px);
}

.btn-cancel:active {
  transform: translateY(0);
}

.btn-done {
  background: #FF6B35;
  color: #fff;
}

.btn-done:hover {
  opacity: 0.88;
  transform: translateY(-1px);
}

.btn-done:active {
  transform: translateY(0);
}

.btn-collapse {
  background: transparent;
  border: none;
  color: rgba(255, 255, 255, 0.55);
  font-size: 1.15rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 6px;
  transition: all 0.15s ease;
}

.btn-collapse:hover {
  color: #ffffff;
  background: rgba(255, 255, 255, 0.12);
}

kbd {
  display: inline-block;
  margin-left: 6px;
  padding: 1px 5px;
  border-radius: 3px;
  background: rgba(255, 255, 255, 0.12);
  border: 1px solid rgba(255, 255, 255, 0.18);
  font-family: monospace;
  font-size: 0.72rem;
  color: rgba(232, 232, 232, 0.75);
  vertical-align: middle;
}

.btn-history {
  background: transparent;
  border: none;
  color: #E8E8E8;
  font-size: 1.25rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  transition: background 0.15s ease, color 0.15s ease;
}

.btn-history:hover:not(:disabled) {
  background: rgba(255, 255, 255, 0.08);
  color: #ffffff;
}

.btn-history:disabled {
  color: rgba(232, 232, 232, 0.25);
  cursor: not-allowed;
}

.mr-2 {
  margin-right: 8px;
}

.mr-3 {
  margin-right: 12px;
}

.ml-2 {
  margin-left: 8px;
}

/* Collapsed Mini Pill */
.topbar-collapsed-pill {
  position: fixed;
  top: 10px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 9999;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 5px 14px;
  background: rgba(22, 25, 35, 0.90);
  border: 1px solid rgba(255, 255, 255, 0.18);
  border-radius: 20px;
  color: #e2e8f0;
  font-size: 0.82rem;
  font-weight: 600;
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.45);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;
}

.topbar-collapsed-pill:hover {
  background: rgba(255, 107, 53, 0.9);
  color: #ffffff;
  border-color: #ff6b35;
  transform: translateX(-50%) translateY(2px);
}

.pill-badge {
  background: rgba(255, 107, 53, 0.25);
  color: #ff6b35;
  font-size: 0.68rem;
  font-weight: 800;
  padding: 1px 6px;
  border-radius: 4px;
  text-transform: uppercase;
}

.topbar-collapsed-pill:hover .pill-badge {
  background: rgba(255, 255, 255, 0.25);
  color: #ffffff;
}

.pill-text {
  user-select: none;
}

/* Transitions */
.topbar-slide-enter-active,
.topbar-slide-leave-active {
  transition: transform 0.25s cubic-bezier(0.16, 1, 0.3, 1), opacity 0.2s ease;
}

.topbar-slide-enter-from,
.topbar-slide-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(-30px);
}

.topbar-pill-fade-enter-active,
.topbar-pill-fade-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.topbar-pill-fade-enter-from,
.topbar-pill-fade-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(-10px);
}
</style>
