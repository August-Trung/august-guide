<template>
  <div class="d-flex flex-column align-center justify-center text-center py-12 px-4 border-dashed rounded-lg w-100">
    <v-avatar size="64" color="surface-variant" class="mb-4">
      <v-icon size="32" :color="iconColor" :icon="icon"></v-icon>
    </v-avatar>
    <h3 class="text-h6 font-weight-bold text-white mb-1">
      {{ title }}
    </h3>
    <p class="text-subtitle-2 text-medium-emphasis mb-6" style="max-width: 420px;">
      {{ description }}
    </p>
    <v-btn
      v-if="actionText"
      color="primary"
      class="text-none"
      :prepend-icon="actionIcon"
      @click="$emit('action')"
    >
      {{ actionText }}
    </v-btn>
  </div>
</template>

<script setup lang="ts">
interface Props {
  icon?: string
  iconColor?: string
  title: string
  description: string
  actionText?: string
  actionIcon?: string
}

withDefaults(defineProps<Props>(), {
  icon: 'mdi-folder-open',
  iconColor: 'medium-emphasis',
  actionIcon: 'mdi-plus',
})

defineEmits<{
  (e: 'action'): void
}>()
</script>

<style scoped>
.border-dashed {
  border: 2px dashed rgba(255, 255, 255, 0.12) !important;
  background-color: rgba(255, 255, 255, 0.02);
}
</style>
