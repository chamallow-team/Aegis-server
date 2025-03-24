<template>
  <div>
    <EditorNavbar />
  </div>
</template>

<script setup lang="ts">
import { onBeforeMount, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useEditorStore } from '@/stores/editor.store.ts'
import EditorNavbar from '@/components/editor/EditorNavbar.vue'

const editorStore = useEditorStore()
const router = useRouter()

onBeforeMount(() => {
  if (!editorStore.initialized) {
    router.push({ name: 'home' })
  }
})
</script>
