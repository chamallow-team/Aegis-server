<template>
  <main class="flex flex-col justify-center items-center content-center h-screen w-screen">
    <button class="btn bg-primary-400 w-fit" @click="launchNewSession">Créer un nouveau projet</button>
  </main>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useEditorStore } from '@/stores/editor.store.ts'

const editor = useEditorStore()
const router = useRouter()

function launchNewSession() {
  editor.initializeStore()
  router.push({ name: 'editor' })
}
</script>