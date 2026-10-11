<template>
  <v-dialog
    :model-value="modelValue"
    max-width="520px"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <v-card bg-color="surface" class="pa-4 border elevation-10">
      <v-card-title class="text-h5 font-weight-bold px-0 text-primary d-flex align-center gap-2">
        <v-icon icon="mdi-book-plus-outline" color="primary"></v-icon>
        <span>Tạo bộ tài liệu từ ảnh đã chọn</span>
      </v-card-title>

      <v-card-text class="px-0 py-4">
        <!-- Selected Count Info Banner -->
        <v-alert
          type="info"
          variant="tonal"
          density="comfortable"
          class="mb-4"
          icon="mdi-image-multiple"
        >
          <div class="text-body-2">
            Đã chọn <strong>{{ selectedCaptureIds.length }}</strong> ảnh từ thư viện để đóng gói thành một bộ tài liệu / quy trình hướng dẫn mới.
          </div>
        </v-alert>

        <v-form ref="formRef" v-model="isFormValid" @submit.prevent="handleCreate">
          <v-text-field
            v-model="title"
            label="Tên bộ tài liệu / quy trình *"
            placeholder="Ví dụ: Quy trình nghiệm thu sản phẩm, Hướng dẫn cài đặt CRM..."
            :rules="[v => !!v?.trim() || 'Vui lòng nhập tên bộ tài liệu']"
            variant="outlined"
            density="comfortable"
            class="mb-3"
            autofocus
            required
          ></v-text-field>

          <v-textarea
            v-model="description"
            label="Mô tả tóm tắt (tùy chọn)"
            placeholder="Ghi chú mục đích, đối tượng áp dụng hoặc bối cảnh của tài liệu này..."
            variant="outlined"
            density="comfortable"
            rows="3"
            counter="500"
            maxlength="500"
          ></v-textarea>
        </v-form>
      </v-card-text>

      <v-card-actions class="px-0 pt-2">
        <v-spacer></v-spacer>
        <v-btn
          variant="text"
          color="medium-emphasis"
          class="text-none"
          :disabled="isSubmitting"
          @click="handleClose"
        >
          Hủy
        </v-btn>
        <v-btn
          variant="elevated"
          color="primary"
          class="text-none px-4 font-weight-bold"
          :disabled="!isFormValid || isSubmitting || selectedCaptureIds.length === 0"
          :loading="isSubmitting"
          prepend-icon="mdi-check"
          @click="handleCreate"
        >
          Tạo bộ tài liệu
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useSessionStore } from '@/stores/sessionStore'
import type { Session } from '@/types/session'

const props = defineProps<{
  modelValue: boolean
  selectedCaptureIds: string[]
  projectId: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'created', session: Session): void
}>()

const sessionStore = useSessionStore()

const formRef = ref<any>(null)
const isFormValid = ref(false)
const title = ref('')
const description = ref('')
const isSubmitting = ref(false)

watch(
  () => props.modelValue,
  (val) => {
    if (val) {
      title.value = ''
      description.value = ''
      isSubmitting.value = false
    }
  }
)

function handleClose() {
  emit('update:modelValue', false)
}

async function handleCreate() {
  if (!title.value?.trim() || props.selectedCaptureIds.length === 0 || !props.projectId) return

  isSubmitting.value = true
  try {
    const newSession = await sessionStore.createSessionFromSelected(
      props.projectId,
      title.value.trim(),
      props.selectedCaptureIds,
      description.value.trim() || undefined
    )
    emit('created', newSession)
    emit('update:modelValue', false)
  } catch (err) {
    console.error('Failed to create guide session from selection:', err)
  } finally {
    isSubmitting.value = false
  }
}
</script>

<style scoped>
.gap-2 {
  gap: 8px;
}
</style>
