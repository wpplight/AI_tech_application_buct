import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { AlgorithmType } from '@/types'

export type ExecutionMode = 'step' | 'instant'

export interface DisplayData {
  id: string
  name: string
  algorithm: AlgorithmType
  mapId: string
  executionMode: ExecutionMode
  speed: number
  createdAt: Date
}

export const useDisplayStore = defineStore('display', () => {
  const displays = ref<DisplayData[]>([])
  
  const currentDisplay = ref<DisplayData | null>(null)
  
  const displayCount = computed(() => displays.value.length)
  
  function createDisplay(
    name: string,
    algorithm: AlgorithmType,
    mapId: string,
    executionMode: ExecutionMode = 'instant',
    speed: number = 1
  ): DisplayData {
    const newDisplay: DisplayData = {
      id: `display_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      name,
      algorithm,
      mapId,
      executionMode,
      speed,
      createdAt: new Date()
    }
    
    displays.value.push(newDisplay)
    saveDisplays()
    return newDisplay
  }
  
  function getDisplayById(id: string): DisplayData | undefined {
    return displays.value.find(d => d.id === id)
  }
  
  function updateDisplay(id: string, updates: Partial<DisplayData>) {
    const index = displays.value.findIndex(d => d.id === id)
    if (index !== -1) {
      const existing = displays.value[index]!
      displays.value[index] = {
        id: existing.id,
        name: updates.name ?? existing.name,
        algorithm: updates.algorithm ?? existing.algorithm,
        mapId: updates.mapId ?? existing.mapId,
        executionMode: updates.executionMode ?? existing.executionMode,
        speed: updates.speed ?? existing.speed,
        createdAt: existing.createdAt
      }
      saveDisplays()
    }
  }
  
  function deleteDisplay(id: string) {
    const index = displays.value.findIndex(d => d.id === id)
    if (index !== -1) {
      displays.value.splice(index, 1)
      saveDisplays()
    }
  }
  
  function setCurrentDisplay(display: DisplayData | null) {
    currentDisplay.value = display
  }
  
  function saveDisplays() {
    try {
      const dataToSave = displays.value.map(display => ({
        ...display,
        createdAt: display.createdAt.toISOString()
      }))
      localStorage.setItem('wayfind_displays', JSON.stringify(dataToSave))
    } catch (error) {
      console.error('Failed to save displays:', error)
    }
  }
  
  function loadDisplays() {
    try {
      const saved = localStorage.getItem('wayfind_displays')
      if (saved) {
        const parsed = JSON.parse(saved)
        displays.value = parsed.map((display: any) => ({
          ...display,
          createdAt: new Date(display.createdAt)
        }))
      }
    } catch (error) {
      console.error('Failed to load displays:', error)
    }
  }
  
  loadDisplays()
  
  return {
    displays,
    currentDisplay,
    displayCount,
    createDisplay,
    getDisplayById,
    updateDisplay,
    deleteDisplay,
    setCurrentDisplay,
    loadDisplays
  }
})
