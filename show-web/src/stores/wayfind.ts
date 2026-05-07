/**
 * 寻路算法状态管理
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { wayfindService, type PathfindingAlgorithm, type MapData, type SearchStep, type SearchResult, type Point } from '../api/wayfind'

export const useWayfindStore = defineStore('wayfind', () => {
  // 状态
  const currentMap = ref<MapData | null>(null)
  const currentAlgorithm = ref<PathfindingAlgorithm>('astar')
  const searchId = ref<string | null>(null)
  const searchHistory = ref<SearchStep[]>([])
  const currentStep = ref<number>(0)
  const searchResult = ref<SearchResult | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)
  const isConnected = ref(false)
  const isSearching = ref(false)
  const isPaused = ref(false)
  const searchSpeed = ref(500) // ms

  // 计算属性
  const visitedCount = computed(() => {
    if (!searchHistory.value.length) return 0
    const lastStep = searchHistory.value[searchHistory.value.length - 1]
    return lastStep?.visited?.length || 0
  })

  const pathLength = computed(() => {
    if (!searchResult.value?.path) return 0
    return searchResult.value.path.length
  })

  const currentPath = computed(() => {
    if (!searchHistory.value.length) return []
    const step = searchHistory.value[currentStep.value]
    return step?.path || []
  })

  const currentVisited = computed(() => {
    if (!searchHistory.value.length) return []
    const step = searchHistory.value[currentStep.value]
    return step?.visited || []
  })

  const executionTime = computed(() => {
    if (!searchResult.value) return 0
    return searchResult.value.execution_time_ms || 0
  })

  // 操作
  async function checkConnection() {
    try {
      await wayfindService.createMap({ width: 5, height: 5 })
      isConnected.value = true
    } catch {
      isConnected.value = false
    }
  }

  async function createMap(width: number = 20, height: number = 15) {
    try {
      loading.value = true
      error.value = null
      const result = await wayfindService.createMap({ width, height })
      if (result.success && result.map_data) {
        currentMap.value = result.map_data
        resetSearch()
      }
      return result
    } catch (e) {
      error.value = e instanceof Error ? e.message : '创建地图失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function updateCell(x: number, y: number, type: 'road' | 'wall' | 'start' | 'end') {
    if (!currentMap.value) return
    
    try {
      const cellTypeMap = { road: 0, wall: 1, start: 2, end: 3 }
      currentMap.value.grid[y][x] = cellTypeMap[type]
      
      if (type === 'start') {
        currentMap.value.startPoint = { x, y }
      } else if (type === 'end') {
        currentMap.value.endPoint = { x, y }
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '更新单元格失败'
    }
  }

  async function initSearch() {
    if (!currentMap.value) return
    
    try {
      loading.value = true
      error.value = null
      const result = await wayfindService.initSearch({
        map_id: 'current',
        algorithm: currentAlgorithm.value
      })
      
      if (result.success) {
        searchId.value = result.search_id || null
        searchHistory.value = []
        currentStep.value = 0
        searchResult.value = null
      }
      return result
    } catch (e) {
      error.value = e instanceof Error ? e.message : '初始化搜索失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function stepSearch() {
    if (!searchId.value) return
    
    try {
      loading.value = true
      isSearching.value = true
      const step = await wayfindService.stepSearch(searchId.value)
      searchHistory.value.push(step)
      
      if (step.state === 'found' || step.state === 'not_found') {
        searchResult.value = {
          found: step.state === 'found',
          distance: step.distance,
          path: step.path,
          algorithm: currentAlgorithm.value,
          expanded: step.expanded
        }
      }
      
      return step
    } catch (e) {
      error.value = e instanceof Error ? e.message : '搜索失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function runSearch() {
    if (!searchId.value) return
    
    try {
      loading.value = true
      isSearching.value = true
      isPaused.value = false
      
      // 单步执行直到完成
      while (!isPaused.value) {
        const step = await stepSearch()
        if (step?.state === 'found' || step?.state === 'not_found') {
          break
        }
        await new Promise(resolve => setTimeout(resolve, searchSpeed.value))
      }
      
      return searchResult.value
    } catch (e) {
      error.value = e instanceof Error ? e.message : '搜索失败'
      throw e
    } finally {
      loading.value = false
      isSearching.value = false
    }
  }

  function pauseSearch() {
    isPaused.value = true
  }

  function nextStep() {
    if (currentStep.value < searchHistory.value.length - 1) {
      currentStep.value++
    }
  }

  function prevStep() {
    if (currentStep.value > 0) {
      currentStep.value--
    }
  }

  function goToStep(step: number) {
    if (step >= 0 && step < searchHistory.value.length) {
      currentStep.value = step
    }
  }

  function resetSearch() {
    searchId.value = null
    searchHistory.value = []
    currentStep.value = 0
    searchResult.value = null
    isSearching.value = false
    isPaused.value = false
  }

  function setAlgorithm(algorithm: PathfindingAlgorithm) {
    currentAlgorithm.value = algorithm
    resetSearch()
  }

  function setSpeed(speed: number) {
    searchSpeed.value = Math.max(50, Math.min(2000, speed))
  }

  return {
    // 状态
    currentMap,
    currentAlgorithm,
    searchId,
    searchHistory,
    currentStep,
    searchResult,
    loading,
    error,
    isConnected,
    isSearching,
    isPaused,
    searchSpeed,
    
    // 计算属性
    visitedCount,
    pathLength,
    currentPath,
    currentVisited,
    executionTime,
    
    // 操作
    checkConnection,
    createMap,
    updateCell,
    initSearch,
    stepSearch,
    runSearch,
    pauseSearch,
    nextStep,
    prevStep,
    goToStep,
    resetSearch,
    setAlgorithm,
    setSpeed
  }
})
