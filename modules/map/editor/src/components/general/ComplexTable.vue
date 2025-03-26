<template>
  <div class="overflow-x-auto">
    <table class="table">
      <thead v-if="!only_rows">
      <tr>
        <th v-if="show_index"></th>
        <th v-for="(head, i) in headers" :key="i">{{ head.name }}</th>
      </tr>
      </thead>
      <tbody class="h-full overflow-y-scroll">
      <tr v-for="(row, index) in data" :key="index" class="cursor-pointer"
          :class="selectedItem === row ? 'bg-primary-200/40' : 'hover:bg-primary-200/50'"
          @click="itemSelected(row, index)">
        <th v-if="show_index">{{ index + 1 }}</th>
        <td v-for="(head, hi) in headers" :key="hi">{{ row[head.field] }}</td>
      </tr>
      <tr v-if="data.length < 1">
        <th v-if="show_index"></th>
        <td><p>{{ no_data }}</p></td>
      </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { toRefs } from 'vue'

const props = defineProps<{
  data: any[],
  headers: { field: string, name: string }[],
  only_rows?: boolean,
  show_index?: boolean,
  no_data?: string,
  allowSelection?: boolean,
  selectedItem?: any
}>()

const {
  data, headers,
  only_rows = true,
  show_index = false,
  no_data,
  allowSelection,
  selectedItem
} = toRefs(props)

const emit = defineEmits(['select'])

function itemSelected(row: any, index: number) {
  if (!allowSelection) return
  emit('select', row)
}
</script>
