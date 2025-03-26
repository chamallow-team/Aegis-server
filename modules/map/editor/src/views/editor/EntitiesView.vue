<template>
  <div class="flex flex-row justify-between w-full h-full">
    <!-- Entities list -->
    <div class="flex flex-col p-2 w-full">
      <SearchBar show_advanced_filter>
        <template #advanced_filter>
          Filtre avancé à faire =)
        </template>
      </SearchBar>

      <ComplexTable :data="filteredValue" :headers="tableHeaders" show_index
                    allow-selection :selected-item="selectedEntity"
                    no_data="Aucune entité trouvée"
                    @select="newSelectedEntity"></ComplexTable>
    </div>

    <!-- Right side with entity details/editing -->
    <RightSide>
      {{ JSON.stringify(selectedEntity, null, 2) }}
    </RightSide>
  </div>
</template>

<script setup lang="ts">
import RightSide from '@/components/general/RightSide.vue'
import { computed, ref, type Ref } from 'vue'
import { type Entity, useEntitiesStore } from '@/stores/entities.store.ts'
import SearchBar from '@/components/editor/entities/SearchBar.vue'
import ComplexTable from '@/components/general/ComplexTable.vue'

const selectedEntity: Ref<Entity | null> = ref(null)
const entitiesStore = useEntitiesStore()

const tableHeaders = [{ name: 'Name', field: 'name' }]

const filteredValue = computed(function() {
  return entitiesStore.entities
})

function newSelectedEntity(new_entity: Entity) {
  selectedEntity.value = new_entity
}
</script>
