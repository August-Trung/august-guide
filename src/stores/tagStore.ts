import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getAllTags, createTag } from '@/services/tauriCommands'
import type { Tag } from '@/types/issue'

export const useTagStore = defineStore('tag', () => {
  const tags = ref<Tag[]>([])
  const loading = ref(false)

  async function loadTags() {
    loading.value = true
    try {
      const allTags = await getAllTags()
      tags.value = allTags
    } catch (e) {
      console.error('[TagStore] Failed to load tags:', e)
    } finally {
      loading.value = false
    }
  }

  // Create a new tag and add it to store
  async function addTag(name: string, color?: string): Promise<Tag> {
    const defaultColor = color || '#4ECDC4'
    try {
      const newTag = await createTag(name, defaultColor)
      // If tag is not already in the list, add it
      if (!tags.value.some(t => t.id === newTag.id)) {
        tags.value.push(newTag)
        // Sort tags alphabetically
        tags.value.sort((a, b) => a.name.localeCompare(b.name))
      }
      return newTag
    } catch (e) {
      console.error('[TagStore] Failed to create tag:', e)
      throw e
    }
  }

  return {
    tags,
    loading,
    loadTags,
    addTag
  }
})
