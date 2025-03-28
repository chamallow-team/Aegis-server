<template>
  <div class="w-full">
    <div class="flex flex-row w-full items-center content-center gap-2">
      <!-- Search -->
      <label
        class="input bg-primary-200/50 hover:bg-primary-200/40 w-full border-transparent focus:border-transparent focus:ring-0">
        <svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
          <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor">
            <circle cx="11" cy="11" r="8"></circle>
            <path d="m21 21-4.3-4.3"></path>
          </g>
        </svg>
        <input type="search" class="grow" :placeholder="search_placeholder" />
        <kbd class="kbd kbd-sm bg-primary-100">CTRL</kbd>
        <kbd class="kbd kbd-sm bg-primary-100">K</kbd>
      </label>

      <!-- Advanced filter -->
      <div v-if="show_advanced_filter"
           class="p-2 bg-primary-200/50 hover:bg-primary-200/40 cursor-pointer h-full w-auto rounded"
           @click="isAdvancedFilterOpened = !isAdvancedFilterOpened"
           style="box-shadow: 0 1px color-mix(in oklab, color-mix(in oklab, var(--color-base-content) 20%, #0000) calc(var(--depth) * 10%), #0000) inset, 0 -1px oklch(100% 0 0 / calc(var(--depth) * 0.1)) inset">
        <!-- Icon -->
        <svg v-if="!isAdvancedFilterOpened" xmlns="http://www.w3.org/2000/svg" width="24" height="24"
             viewBox="0 0 24 24"
             class="fill-white">
          <path
            d="M21 3H5a1 1 0 0 0-1 1v2.59c0 .523.213 1.037.583 1.407L10 13.414V21a1.001 1.001 0 0 0 1.447.895l4-2c.339-.17.553-.516.553-.895v-5.586l5.417-5.417c.37-.37.583-.884.583-1.407V4a1 1 0 0 0-1-1zm-6.707 9.293A.996.996 0 0 0 14 13v5.382l-2 1V13a.996.996 0 0 0-.293-.707L6 6.59V5h14.001l.002 1.583-5.71 5.71z"></path>
        </svg>
        <svg v-else xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-white">
          <path
            d="m16.192 6.344-4.243 4.242-4.242-4.242-1.414 1.414L10.535 12l-4.242 4.242 1.414 1.414 4.242-4.242 4.243 4.242 1.414-1.414L13.364 12l4.242-4.242z"></path>
        </svg>
      </div>
    </div>

    <!-- Advanced filters panel -->
    <div v-if="isAdvancedFilterOpened && show_advanced_filter" class="bg-primary-200/50 rounded p-2 w-full mt-2">
      <slot name="advanced_filter"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const {
  search_placeholder = 'Rechercher...',
  show_advanced_filter = false
} = defineProps<{
  search_placeholder?: string,
  show_advanced_filter?: boolean
}>()

const isAdvancedFilterOpened = ref(false)

const emit = defineEmits<{
  search: [query: string,]
}>()
</script>
