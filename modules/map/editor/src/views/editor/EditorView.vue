<template>
  <div class="flex flex-row h-screen w-screen bg-primary-200/25">
    <EditorNavbar />
    <div class="overflow-hidden m-2 rounded-xl bg-primary-100 w-full">
      <router-view name="body"></router-view>
    </div>
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
