<template>
  <div>
    <!-- Full Annotation Toolbar -->
    <transition name="toolbar-slide">
      <div v-if="!collapsed" class="annotation-toolbar">
        <!-- Drawing Tools -->
        <div class="tools-group">
          <!-- Marker Tool (1) -->
          <button
            type="button"
            class="toolbar-tool-btn"
            :class="{ active: activeTool === 'marker' }"
            :title="(t('overlay.drawTools.marker') || 'Ghim số') + ' (1)'"
            @click="selectTool('marker')"
          >
            <i class="mdi mdi-numeric-1-circle"></i>
          </button>

          <!-- Rectangle Tool (2) -->
          <button
            type="button"
            class="toolbar-tool-btn"
            :class="{ active: activeTool === 'rect' }"
            :title="(t('overlay.drawTools.rect') || 'Hình chữ nhật') + ' (2)'"
            @click="selectTool('rect')"
          >
            <i class="mdi mdi-rectangle-outline"></i>
          </button>

          <!-- Arrow Tool (3) -->
          <button
            type="button"
            class="toolbar-tool-btn"
            :class="{ active: activeTool === 'arrow' }"
            :title="(t('overlay.drawTools.arrow') || 'Mũi tên') + ' (3)'"
            @click="selectTool('arrow')"
          >
            <i class="mdi mdi-arrow-top-right"></i>
          </button>

          <!-- Text Tool (4) -->
          <button
            type="button"
            class="toolbar-tool-btn"
            :class="{ active: activeTool === 'text' }"
            :title="(t('overlay.drawTools.text') || 'Văn bản') + ' (4)'"
            @click="selectTool('text')"
          >
            <i class="mdi mdi-format-text"></i>
          </button>

          <!-- Freehand Tool (5) -->
          <button
            type="button"
            class="toolbar-tool-btn"
            :class="{ active: activeTool === 'freedraw' }"
            :title="(t('overlay.drawTools.freedraw') || 'Vẽ tự do') + ' (5)'"
            @click="selectTool('freedraw')"
          >
            <i class="mdi mdi-draw"></i>
          </button>

          <!-- Blur Tool (6) -->
          <button
            type="button"
            class="toolbar-tool-btn"
            :class="{ active: activeTool === 'blur' }"
            :title="(t('overlay.drawTools.blur') || 'Làm mờ thông tin') + ' (6)'"
            @click="selectTool('blur')"
          >
            <i class="mdi mdi-blur"></i>
          </button>

          <!-- Highlight Tool (7) -->
          <button
            type="button"
            class="toolbar-tool-btn"
            :class="{ active: activeTool === 'highlight' }"
            :title="(t('overlay.drawTools.highlight') || 'Tô sáng') + ' (7)'"
            @click="selectTool('highlight')"
          >
            <i class="mdi mdi-marker"></i>
          </button>

          <!-- Spotlight Tool (8) -->
          <button
            type="button"
            class="toolbar-tool-btn"
            :class="{ active: activeTool === 'spotlight' }"
            title="Chiếu sáng vùng trọng tâm (8)"
            @click="selectTool('spotlight')"
          >
            <i class="mdi mdi-lightbulb-on-outline"></i>
          </button>

          <!-- Crop / Region Snip Tool (9) -->
          <button
            type="button"
            class="toolbar-tool-btn"
            :class="{ active: activeTool === 'crop' }"
            title="Cắt vùng ảnh (9 / C)"
            @click="selectTool('crop')"
          >
            <i class="mdi mdi-crop"></i>
          </button>
        </div>

        <div class="separator"></div>

        <!-- Quick Color Palette & Stroke Width -->
        <div class="palette-group">
          <div class="color-dots">
            <button
              v-for="color in paletteColors"
              :key="color"
              type="button"
              class="color-dot-btn"
              :class="{ active: overlayStore.currentColor === color }"
              :style="{ backgroundColor: color }"
              :title="`Chọn màu: ${color}`"
              @click="overlayStore.setColor(color)"
            ></button>
          </div>

          <div class="stroke-selector">
            <button
              type="button"
              class="stroke-btn"
              :class="{ active: overlayStore.currentStrokeWidth === 2 }"
              title="Nét mỏng (2px)"
              @click="overlayStore.setStrokeWidth(2)"
            >
              <span class="stroke-line" style="height: 2px;"></span>
            </button>
            <button
              type="button"
              class="stroke-btn"
              :class="{ active: overlayStore.currentStrokeWidth === 4 }"
              title="Nét vừa (4px)"
              @click="overlayStore.setStrokeWidth(4)"
            >
              <span class="stroke-line" style="height: 4px;"></span>
            </button>
            <button
              type="button"
              class="stroke-btn"
              :class="{ active: overlayStore.currentStrokeWidth === 6 }"
              title="Nét dày (6px)"
              @click="overlayStore.setStrokeWidth(6)"
            >
              <span class="stroke-line" style="height: 6px;"></span>
            </button>
          </div>
        </div>

        <div class="separator"></div>

        <!-- Options & Actions -->
        <div class="actions-group">
          <!-- Toggle Default Badge -->
          <button
            type="button"
            class="toolbar-icon-btn"
            :class="{ active: overlayStore.defaultShowBadge }"
            title="Tự động gắn số thứ tự cho nét vẽ tiếp theo"
            @click="overlayStore.defaultShowBadge = !overlayStore.defaultShowBadge"
          >
            <i class="mdi mdi-pound"></i>
          </button>

          <!-- Toggle Step Notes Drawer -->
          <button
            type="button"
            class="toolbar-icon-btn"
            :class="{ active: overlayStore.showStepNotes }"
            title="Mở bảng ghi chú các bước (Tab)"
            @click="overlayStore.showStepNotes = !overlayStore.showStepNotes"
          >
            <i class="mdi mdi-format-list-numbered"></i>
          </button>

          <!-- Copy to Clipboard (Ctrl+C) -->
          <button
            type="button"
            class="action-pill-btn copy-btn"
            title="Sao chép ảnh vào Clipboard (Ctrl + C)"
            @click="emit('copy')"
          >
            <i class="mdi mdi-content-copy"></i>
            <span>Copy</span>
          </button>

          <!-- Save As File (Download) -->
          <button
            type="button"
            class="action-pill-btn save-as-btn"
            :title="t('settingsView.saveAsTooltip') || 'Lưu ảnh đã vẽ ra file trên máy tính'"
            @click="emit('saveAs')"
          >
            <i class="mdi mdi-download"></i>
            <span>Lưu file</span>
          </button>

          <!-- Save (Ctrl+S) -->
          <button
            type="button"
            class="action-pill-btn save-btn"
            title="Lưu ảnh hoàn tất (Ctrl + S)"
            @click="emit('save')"
          >
            <i class="mdi mdi-check"></i>
            <span>Xong</span>
          </button>

          <!-- Collapse Toolbar Button -->
          <button
            type="button"
            class="toolbar-icon-btn collapse-toggle-btn"
            title="Ẩn thanh công cụ (H)"
            @click="collapsed = true"
          >
            <i class="mdi mdi-chevron-down"></i>
          </button>
        </div>
      </div>
    </transition>

    <!-- Mini Collapsed Floating Pill -->
    <transition name="pill-fade">
      <button
        v-if="collapsed"
        type="button"
        class="toolbar-collapsed-pill"
        title="Hiện thanh công cụ (H)"
        @click="collapsed = false"
      >
        <i class="mdi mdi-palette-outline"></i>
        <span class="pill-text">Công cụ vẽ</span>
        <i class="mdi mdi-chevron-up"></i>
      </button>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useOverlayStore } from '@/stores/overlayStore'
import { useI18n } from '@/composables/useI18n'

const collapsed = defineModel<boolean>('collapsed', { default: false })

const emit = defineEmits<{
  (e: 'copy'): void
  (e: 'save'): void
  (e: 'saveAs'): void
}>()

const overlayStore = useOverlayStore()
const activeTool = computed(() => overlayStore.activeTool)
const { t } = useI18n()

const paletteColors = ['#FF6B35', '#FF4757', '#2ED573', '#38BDF8', '#8E44AD', '#FFDD59']

const selectTool = (tool: any) => {
  if (overlayStore.activeTool === tool) {
    overlayStore.setTool(null)
  } else {
    overlayStore.setTool(tool)
  }
}
</script>

<style scoped>
.annotation-toolbar {
  position: fixed;
  bottom: 24px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 1000;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 6px 14px;
  background: rgba(22, 25, 35, 0.92);
  border: 1px solid rgba(255, 255, 255, 0.16);
  border-radius: 36px;
  box-shadow: 0 10px 38px rgba(0, 0, 0, 0.55);
  backdrop-filter: blur(14px);
  -webkit-backdrop-filter: blur(14px);
  pointer-events: auto;
  white-space: nowrap;
}

.tools-group,
.actions-group {
  display: flex;
  align-items: center;
  gap: 4px;
}

.palette-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.color-dots {
  display: flex;
  align-items: center;
  gap: 5px;
}

.color-dot-btn {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  border: 2px solid rgba(255, 255, 255, 0.4);
  cursor: pointer;
  padding: 0;
  transition: transform 0.15s ease, border-color 0.15s ease;
  outline: none;
}

.color-dot-btn:hover {
  transform: scale(1.2);
  border-color: #ffffff;
}

.color-dot-btn.active {
  transform: scale(1.25);
  border-color: #ffffff;
  box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.8);
}

.stroke-selector {
  display: flex;
  align-items: center;
  gap: 4px;
}

.stroke-btn {
  background: transparent;
  border: 1px solid transparent;
  border-radius: 6px;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  padding: 0 3px;
  transition: all 0.15s ease;
}

.stroke-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  border-color: rgba(255, 255, 255, 0.2);
}

.stroke-btn.active {
  background: rgba(255, 255, 255, 0.2);
  border-color: #ff6b35;
}

.stroke-line {
  display: block;
  width: 100%;
  background-color: #ffffff;
  border-radius: 2px;
}

.separator {
  width: 1px;
  height: 22px;
  background: rgba(255, 255, 255, 0.15);
  margin: 0 4px;
}

/* Tool Buttons - Compact Icon Only with glow */
.toolbar-tool-btn {
  background: transparent;
  border: none;
  color: rgba(255, 255, 255, 0.75);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.18s ease;
  outline: none;
}

.toolbar-tool-btn i {
  font-size: 1.25rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.toolbar-tool-btn:hover {
  background: rgba(255, 255, 255, 0.12);
  color: #ffffff;
  transform: translateY(-1px);
}

.toolbar-tool-btn.active {
  background: #ff6b35;
  color: #ffffff;
  box-shadow: 0 3px 12px rgba(255, 107, 53, 0.45);
}

.toolbar-icon-btn {
  background: transparent;
  border: none;
  color: rgba(255, 255, 255, 0.75);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 34px;
  height: 34px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.18s ease;
}

.toolbar-icon-btn i {
  font-size: 1.15rem;
}

.toolbar-icon-btn:hover {
  background: rgba(255, 255, 255, 0.12);
  color: #ffffff;
}

.toolbar-icon-btn.active {
  background: #38bdf8;
  color: #0f172a;
}

.collapse-toggle-btn {
  color: rgba(255, 255, 255, 0.5);
}

.collapse-toggle-btn:hover {
  color: #ffffff;
  background: rgba(255, 255, 255, 0.15);
}

/* Action Pill Buttons (Copy, Done) */
.action-pill-btn {
  border: none;
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 6px 14px;
  border-radius: 18px;
  cursor: pointer;
  font-family: inherit;
  font-size: 0.85rem;
  font-weight: 600;
  transition: all 0.18s ease;
  white-space: nowrap;
  line-height: 1;
}

.copy-btn {
  background: rgba(56, 189, 248, 0.16);
  color: #38bdf8;
  border: 1px solid rgba(56, 189, 248, 0.28);
}

.copy-btn:hover {
  background: #38bdf8;
  color: #0f172a;
  transform: translateY(-1px);
}

.save-btn {
  background: #22c55e;
  color: #ffffff;
  box-shadow: 0 2px 8px rgba(34, 197, 94, 0.35);
}

.save-btn:hover {
  background: #16a34a;
  transform: translateY(-1px);
}

/* Floating Collapsed Pill */
.toolbar-collapsed-pill {
  position: fixed;
  bottom: 12px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 1000;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 14px;
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
}

.toolbar-collapsed-pill:hover {
  background: rgba(255, 107, 53, 0.9);
  color: #ffffff;
  border-color: #ff6b35;
  transform: translateX(-50%) translateY(-2px);
}

.pill-text {
  user-select: none;
}

/* Transitions */
.toolbar-slide-enter-active,
.toolbar-slide-leave-active {
  transition: transform 0.25s cubic-bezier(0.16, 1, 0.3, 1), opacity 0.2s ease;
}

.toolbar-slide-enter-from,
.toolbar-slide-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(30px);
}

.pill-fade-enter-active,
.pill-fade-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.pill-fade-enter-from,
.pill-fade-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(10px);
}
</style>
