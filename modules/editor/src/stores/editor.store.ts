import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useEditorStore = defineStore('editor', () => {
  const initialized = ref(false)

  function initializeStore() {
    initialized.value = true
  }

  return {
    // States
    initialized,
    // Actions
    initializeStore
  }
})