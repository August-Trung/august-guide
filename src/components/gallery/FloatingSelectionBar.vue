<template>
  <v-slide-y-reverse-transition>
    <div
      v-if="selectedCount > 0"
      class="floating-selection-bar d-flex align-center gap-3 px-4 py-2"
    >
      <!-- Selection Counter Badge -->
      <div class="d-flex align-center gap-2 pr-2 border-right">
        <v-icon icon="mdi-checkbox-marked-circle" color="primary" size="20"></v-icon>
        <span class="text-subtitle-2 font-weight-bold text-white">
          Đã chọn {{ selectedCount }} ảnh
        </span>
      </div>

      <!-- Action Buttons -->
      <div class="d-flex align-center gap-2">
        <!-- Create Document from selection -->
        <v-btn
          color="primary"
          variant="elevated"
          size="small"
          prepend-icon="mdi-book-plus-outline"
          class="text-none font-weight-bold shadow-sm"
          @click="$emit('create-guide')"
        >
          Tạo bộ tài liệu
        </v-btn>

        <!-- Batch Delete -->
        <v-btn
          color="error"
          variant="tonal"
          size="small"
          prepend-icon="mdi-delete-outline"
          class="text-none font-weight-bold"
          @click="$emit('delete-selected')"
        >
          Xóa ({{ selectedCount }})
        </v-btn>

        <!-- Clear Selection -->
        <v-btn
          icon="mdi-close"
          variant="text"
          size="x-small"
          color="medium-emphasis"
          title="Bỏ chọn tất cả"
          @click="$emit('clear-selection')"
        ></v-btn>
      </div>
    </div>
  </v-slide-y-reverse-transition>
</template>

<script setup lang="ts">
defineProps<{
  selectedCount: number
  selectedIds: string[]
}>()

defineEmits<{
  (e: 'create-guide'): void
  (e: 'delete-selected'): void
  (e: 'clear-selection'): void
}>()
</script>

<style scoped>
.floating-selection-bar {
  position: fixed;
  bottom: 24px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 1000;
  background: rgba(15, 23, 42, 0.94);
  backdrop-filter: blur(16px);
  border: 1px solid rgba(56, 189, 248, 0.4);
  border-radius: 40px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.75), 0 0 20px rgba(56, 189, 248, 0.25);
  user-select: none;
}

.border-right {
  border-right: 1px solid rgba(255, 255, 255, 0.12);
}

.gap-2 {
  gap: 8px;
}

.gap-3 {
  gap: 12px;
}
</style>
