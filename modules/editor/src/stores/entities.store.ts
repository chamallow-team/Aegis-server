import { defineStore } from 'pinia'
import { v4 as uuid } from 'uuid'
import { type Ref, ref } from 'vue'

export interface Entity {
  id: string,
  name: string,
}

export const useEntitiesStore = defineStore('entities', () => {
  const entities: Ref<Entity[]> = ref([
    { name: 'FREMM', id: uuid() },
    { name: 'Rafale', id: uuid() },
    { name: 'Mirage 2000', id: uuid() },
    { name: 'A400M', id: uuid() },
    { name: 'Tigre', id: uuid() },
    { name: 'NH90', id: uuid() },
    { name: 'VBCI', id: uuid() },
    { name: 'Leclerc', id: uuid() },
    { name: 'VAB', id: uuid() },
    { name: 'AMX10RC', id: uuid() },
    { name: 'CAESAR', id: uuid() },
    { name: 'Mistral', id: uuid() },
    { name: 'Horizon', id: uuid() },
    { name: 'Atlantique 2', id: uuid() },
    { name: 'E-3F', id: uuid() },
    { name: 'C130', id: uuid() },
    { name: 'Caracal', id: uuid() },
    { name: 'Cougar', id: uuid() },
    { name: 'Puma', id: uuid() },
    { name: 'Gazelle', id: uuid() },
    { name: 'Fennec', id: uuid() },
    { name: 'Super Puma', id: uuid() },
    { name: 'Super Frelon', id: uuid() },
    { name: 'Lynx', id: uuid() },
    { name: 'Panther', id: uuid() },
    { name: 'Alouette III', id: uuid() },
    { name: 'Alouette II', id: uuid() },
    { name: 'Super Etendard', id: uuid() },
    { name: 'Etendard IV', id: uuid() },
    { name: 'Jaguar', id: uuid() },
    { name: 'Alpha Jet', id: uuid() },
    { name: 'Transall', id: uuid() },
    { name: 'Noratlas', id: uuid() }
  ])

  function addNewEntity(entity: Entity) {
    if (entities.value.find(e => e.name === entity.name || e.id === entity.name))
      throw new Error('Entity already exists!')

    entities.value.push(entity)
  }

  return {
    // states
    entities,
    // actions
    addNewEntity
  }
})