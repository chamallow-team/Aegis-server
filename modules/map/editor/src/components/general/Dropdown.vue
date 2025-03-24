<template>
  <div class="relative">
    <button @click="isOpen = !isOpen" class="relative z-10 block rounded-full cursor-pointer">
      <slot name="closed"></slot>
    </button>

    <button v-if="isOpen" @click="isOpen = false" tabindex="1"
            class="fixed inset-0 h-full w-full cursor-default">
    </button>

    <div v-if="isOpen"
         @click="isOpen = false"
         class="absolute left-0 mt-2 p-2 min-w-20 bg-primary-100 border border-primary-200 rounded-lg shadow-xl">
      <slot></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue'

const isOpen = ref(false)

function handleEscape(e: any) {
  if (e.key === 'Esc' || e.key === 'Escape') {
    isOpen.value = false
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleEscape)
})

onBeforeUnmount(() => {
  document.removeEventListener('keydown', handleEscape)
})
</script>