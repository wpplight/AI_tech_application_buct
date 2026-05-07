import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Point } from '@/types'

export interface MapData {
  id: string
  name: string
  width: number
  height: number
  startPoint: Point
  endPoint: Point
  walls: Point[]
  createdAt: Date
  updatedAt: Date
}

export const useMapStore = defineStore('map', () => {
  const maps = ref<MapData[]>([])
  
  const currentMap = ref<MapData | null>(null)
  
  const mapCount = computed(() => maps.value.length)
  
  function createMap(name: string, width: number, height: number): MapData {
    const newMap: MapData = {
      id: `map_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      name,
      width,
      height,
      startPoint: { x: 1, y: 1 },
      endPoint: { x: width - 2, y: height - 2 },
      walls: [],
      createdAt: new Date(),
      updatedAt: new Date()
    }
    
    maps.value.push(newMap)
    saveMaps()
    return newMap
  }
  
  function getMapById(id: string): MapData | undefined {
    return maps.value.find(m => m.id === id)
  }
  
  function updateMap(id: string, updates: Partial<MapData>) {
    const index = maps.value.findIndex(m => m.id === id)
    if (index !== -1) {
      const existingMap = maps.value[index]!
      maps.value[index] = {
        id: existingMap.id,
        name: updates.name ?? existingMap.name,
        width: updates.width ?? existingMap.width,
        height: updates.height ?? existingMap.height,
        startPoint: updates.startPoint ?? existingMap.startPoint,
        endPoint: updates.endPoint ?? existingMap.endPoint,
        walls: updates.walls ?? existingMap.walls,
        createdAt: existingMap.createdAt,
        updatedAt: new Date()
      }
      saveMaps()
    }
  }
  
  function deleteMap(id: string) {
    const index = maps.value.findIndex(m => m.id === id)
    if (index !== -1) {
      maps.value.splice(index, 1)
      saveMaps()
    }
  }
  
  function setCurrentMap(map: MapData | null) {
    currentMap.value = map
  }
  
  function addWall(point: Point) {
    if (!currentMap.value) return
    if (!currentMap.value.walls.some(w => w.x === point.x && w.y === point.y)) {
      currentMap.value.walls.push({ ...point })
    }
  }
  
  function removeWall(point: Point) {
    if (!currentMap.value) return
    currentMap.value.walls = currentMap.value.walls.filter(
      w => !(w.x === point.x && w.y === point.y)
    )
  }
  
  function setStartPoint(point: Point) {
    if (currentMap.value) {
      currentMap.value.startPoint = { ...point }
    }
  }
  
  function setEndPoint(point: Point) {
    if (currentMap.value) {
      currentMap.value.endPoint = { ...point }
    }
  }
  
  function saveMaps() {
    try {
      const dataToSave = maps.value.map(map => ({
        ...map,
        createdAt: map.createdAt.toISOString(),
        updatedAt: map.updatedAt.toISOString()
      }))
      localStorage.setItem('wayfind_maps', JSON.stringify(dataToSave))
    } catch (error) {
      console.error('Failed to save maps:', error)
    }
  }
  
  function loadMaps() {
    try {
      const saved = localStorage.getItem('wayfind_maps')
      if (saved) {
        const parsed = JSON.parse(saved)
        maps.value = parsed.map((map: any) => ({
          ...map,
          createdAt: new Date(map.createdAt),
          updatedAt: new Date(map.updatedAt)
        }))
      }
    } catch (error) {
      console.error('Failed to load maps:', error)
    }
  }
  
  loadMaps()
  
  return {
    maps,
    currentMap,
    mapCount,
    createMap,
    getMapById,
    updateMap,
    deleteMap,
    setCurrentMap,
    addWall,
    removeWall,
    setStartPoint,
    setEndPoint,
    loadMaps
  }
})
