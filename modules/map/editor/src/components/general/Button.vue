<template>
  <div :class="props.class || ''" class="px-2 py-1 bg-primary-100 border-primary-200 rounded-box">
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  class?: any
}>()
</script>
