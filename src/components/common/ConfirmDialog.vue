<template>
  <v-dialog
    v-slot:default
    :model-value="modelValue"
    max-width="400px"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <v-card bg-color="surface" class="pa-4">
      <v-card-title class="text-h5 font-weight-bold px-0 text-white">
        {{ title }}
      </v-card-title>
      
      <v-card-text class="px-0 py-3 text-medium-emphasis">
        {{ message }}
      </v-card-text>
      
      <v-card-actions class="px-0 pt-2">
        <v-spacer></v-spacer>
        <v-btn
          variant="text"
          color="medium-emphasis"
          class="text-none"
          @click="handleCancel"
        >
          Cancel
        </v-btn>
        <v-btn
          variant="elevated"
          :color="confirmColor"
          class="text-none px-4"
          @click="handleConfirm"
        >
          {{ confirmText }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
interface Props {
  modelValue: boolean
  title: string
  message: string
  confirmText?: string
  confirmColor?: string
}

withDefaults(defineProps<Props>(), {
  confirmText: 'Delete',
  confirmColor: 'error',
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'confirm'): void
  (e: 'cancel'): void
}>()

const handleCancel = () => {
  emit('update:modelValue', false)
  emit('cancel')
}

const handleConfirm = () => {
  emit('update:modelValue', false)
  emit('confirm')
}
</script>

<style scoped>
/* Scoped styles */
</style>
