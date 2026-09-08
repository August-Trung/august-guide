<template>
  <transition name="slide">
    <div v-if="overlayStore.showStepNotes" class="step-notes-drawer">
      <div class="drawer-header">
        <div class="drawer-title">
          <i class="mdi mdi-format-list-numbered mr-2"></i>
          <span>{{ t('overlay.stepNotesTitle') || 'Các bước hướng dẫn' }}</span>
        </div>
        <button class="close-btn" @click="overlayStore.showStepNotes = false">
          <i class="mdi mdi-close"></i>
        </button>
      </div>

      <div class="drawer-body">
        <div v-if="badgeAnnotations.length === 0" class="empty-steps">
          <i class="mdi mdi-cursor-default-click-outline empty-icon"></i>
          <p>Chưa có bước nào được đánh số.</p>
          <span class="hint">Chọn công cụ <b>Ghim số (1)</b> hoặc bật số cho nét vẽ để thêm bước.</span>
        </div>

        <div v-else class="steps-list">
          <div
            v-for="ann in badgeAnnotations"
            :key="ann.id"
            class="step-item"
          >
            <div class="step-badge" :style="{ backgroundColor: ann.color || '#FF6B35' }">
              {{ ann.number }}
            </div>
            <div class="step-input-wrap">
              <input
                v-model="ann.stepNote"
                type="text"
                class="step-note-input"
                :placeholder="`Mô tả thao tác cho Bước ${ann.number}...`"
              />
            </div>
            <div class="step-actions">
              <button
                class="step-action-btn unbadge-btn"
                title="Bỏ đánh số (vẫn giữ hình vẽ)"
                @click="overlayStore.toggleAnnotationBadge(ann.id)"
              >
                <i class="mdi mdi-numeric-off"></i>
              </button>
              <button
                class="step-action-btn delete-btn"
                title="Xóa hình vẽ này"
                @click="overlayStore.removeAnnotation(ann.id)"
              >
                <i class="mdi mdi-delete-outline"></i>
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="drawer-footer">
        <button class="footer-btn secondary" @click="overlayStore.showStepNotes = false">
          Đóng
        </button>
      </div>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useOverlayStore } from '@/stores/overlayStore'
import { useI18n } from '@/composables/useI18n'

const overlayStore = useOverlayStore()
const { t } = useI18n()

const badgeAnnotations = computed(() => {
  return overlayStore.annotations.filter(a => a.showBadge)
})
</script>

<style scoped>
.step-notes-drawer {
  position: fixed;
  top: 60px;
  right: 20px;
  width: 360px;
  max-height: calc(100vh - 140px);
  background: rgba(26, 29, 39, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.14);
  border-radius: 16px;
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(12px);
  z-index: 10001;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  color: #ffffff;
}

.drawer-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 18px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.drawer-title {
  font-weight: 600;
  font-size: 1rem;
  display: flex;
  align-items: center;
}

.close-btn {
  background: transparent;
  border: none;
  color: rgba(255, 255, 255, 0.6);
  cursor: pointer;
  font-size: 1.25rem;
  border-radius: 6px;
  padding: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.close-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #ffffff;
}

.drawer-body {
  padding: 16px;
  overflow-y: auto;
  flex: 1;
}

.empty-steps {
  text-align: center;
  padding: 24px 12px;
  color: rgba(255, 255, 255, 0.6);
}

.empty-icon {
  font-size: 2.5rem;
  color: rgba(255, 255, 255, 0.3);
  margin-bottom: 8px;
}

.hint {
  font-size: 0.8rem;
  color: rgba(255, 255, 255, 0.4);
  display: block;
  margin-top: 6px;
}

.steps-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.step-item {
  display: flex;
  align-items: center;
  gap: 10px;
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 8px;
  padding: 8px 10px;
}

.step-badge {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  color: #ffffff;
  font-weight: bold;
  font-size: 0.85rem;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.step-input-wrap {
  flex: 1;
}

.step-note-input {
  width: 100%;
  background: transparent;
  border: none;
  color: #ffffff;
  font-size: 0.875rem;
  outline: none;
}

.step-note-input::placeholder {
  color: rgba(255, 255, 255, 0.35);
}

.step-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.step-action-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1rem;
  transition: all 0.15s ease;
}

.unbadge-btn {
  color: rgba(56, 189, 248, 0.7);
}

.unbadge-btn:hover {
  background: rgba(56, 189, 248, 0.15);
  color: #38bdf8;
}

.delete-btn {
  color: rgba(255, 82, 82, 0.6);
}

.delete-btn:hover {
  background: rgba(255, 82, 82, 0.15);
  color: #ff5252;
}

.drawer-footer {
  padding: 12px 16px;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  display: flex;
  justify-content: flex-end;
}

.footer-btn {
  background: rgba(255, 255, 255, 0.1);
  border: none;
  color: #ffffff;
  padding: 6px 16px;
  border-radius: 8px;
  font-size: 0.875rem;
  cursor: pointer;
  transition: background 0.2s;
}

.footer-btn:hover {
  background: rgba(255, 255, 255, 0.18);
}

.slide-enter-active,
.slide-leave-active {
  transition: all 0.25s ease;
}

.slide-enter-from,
.slide-leave-to {
  transform: translateX(30px);
  opacity: 0;
}

.mr-2 {
  margin-right: 8px;
}
</style>
