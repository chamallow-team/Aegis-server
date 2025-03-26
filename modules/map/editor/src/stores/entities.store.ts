import { defineStore } from 'pinia'
import { v4 as uuid } from 'uuid'

export interface Entity {
  name: string,
  id: string,

}

export const useEntitiesStore = defineStore('entities', () => {
  const entities = []
})