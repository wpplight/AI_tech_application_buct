import { ref, computed } from 'vue'
import { wailsService, type MapData, type StepData, type SearchResultData, type AlgorithmType } from '@/composables/wails'
import type { Point } from '@/types'

export function useWailsSearch() {
  const map = ref<MapData | null>(null)
  const algorithm = ref<AlgorithmType>('bfs')
  const currentStep = ref<StepData | null>(null)
  const isSearching = ref(false)
  const isDone = ref(false)
  const searchResult = ref<SearchResultData | null>(null)

  const grid = computed(() => {
    if (!map.value) return { width: 0, height: 0, cells: [] }
    
    const cells: Array<Array<{ type: 'empty' | 'wall' | 'start' | 'end'; state: 'empty'; point: { x: number; y: number } }>> = [];
    for (let y = 0; y < map.value.grid.length; y++) {
      const row: Array<{ type: 'empty' | 'wall' | 'start' | 'end'; state: 'empty'; point: { x: number; y: number } }> = [];
      for (let x = 0; x < map.value.grid[y].length; x++) {
        const cellType = map.value.grid[y][x];
        row.push({
          type: getCellType(cellType),
          state: 'empty' as const,
          point: { x, y }
        });
      }
      cells.push(row);
    }
    
    return {
      width: map.value.width,
      height: map.value.height,
      cells
    }
  })

  const execution = computed(() => ({
    status: isDone.value ? 'completed' : (isSearching.value ? 'running' : 'idle'),
    currentStep: currentStep.value?.stepsTaken || 0,
    totalSteps: searchResult.value ? searchResult.value.distance + (searchResult.value.path?.length || 0) : 0,
    visitedCells: currentStep.value?.visited || [],
    currentCells: currentStep.value?.added || [],
    path: currentStep.value?.path || searchResult.value?.path || [],
    found: searchResult.value?.found || false
  }))

  function getCellType(type: number): 'empty' | 'wall' | 'start' | 'end' {
    switch (type) {
      case 1: return 'wall'
      case 2: return 'start'
      case 3: return 'end'
      default: return 'empty'
    }
  }

  async function createMap(width: number, height: number) {
    try {
      map.value = await wailsService.CreateMap(width, height)
      resetState()
    } catch (error) {
      console.error('Failed to create map:', error)
      throw error
    }
  }

  async function loadMap(mapData: MapData) {
    try {
      await wailsService.LoadMap(mapData)
      map.value = mapData
      resetState()
    } catch (error) {
      console.error('Failed to load map:', error)
      throw error
    }
  }

  async function setCell(x: number, y: number, cellType: number) {
    try {
      await wailsService.SetCell(x, y, cellType)
      if (map.value && map.value.grid[y]) {
        map.value.grid[y][x] = cellType
        if (cellType === 2) {
          map.value.startPoint = { x, y }
        } else if (cellType === 3) {
          map.value.endPoint = { x, y }
        }
      }
    } catch (error) {
      console.error('Failed to set cell:', error)
      throw error
    }
  }

  async function initializeSearch(algo: AlgorithmType) {
    try {
      algorithm.value = algo
      await wailsService.InitializeSearch(algo)
      isSearching.value = true
      isDone.value = false
      searchResult.value = null
      currentStep.value = null
    } catch (error) {
      console.error('Failed to initialize search:', error)
      throw error
    }
  }

  async function step() {
    if (isDone.value || !isSearching.value) return

    try {
      const done = await wailsService.IsSearchDone()
      if (done) {
        isDone.value = true
        isSearching.value = false
        searchResult.value = await wailsService.GetSearchResult()
        return
      }

      currentStep.value = await wailsService.SearchStep()
      updateGridState()
    } catch (error) {
      console.error('Failed to execute step:', error)
      throw error
    }
  }

  async function runToEnd() {
    if (!isSearching.value) return

    try {
      while (!isDone.value) {
        const done = await wailsService.IsSearchDone()
        if (done) {
          isDone.value = true
          isSearching.value = false
          searchResult.value = await wailsService.GetSearchResult()
          updateGridState()
          break
        }
        currentStep.value = await wailsService.SearchStep()
        updateGridState()
      }
    } catch (error) {
      console.error('Failed to run to end:', error)
      throw error
    }
  }

  async function reset() {
    if (!map.value) return

    try {
      await loadMap(map.value)
      isSearching.value = false
      isDone.value = false
      searchResult.value = null
      currentStep.value = null
    } catch (error) {
      console.error('Failed to reset:', error)
      throw error
    }
  }

  function updateGridState() {
    if (!currentStep.value || !map.value) return

    for (let y = 0; y < map.value.height; y++) {
      const row = map.value.grid[y]
      if (!row) continue
      
      for (let x = 0; x < map.value.width; x++) {
        const cell = row[x]
        if (cell !== 1 && cell !== 2 && cell !== 3) {
          row[x] = 0
        }
      }
    }

    for (const point of currentStep.value.visited) {
      const row = map.value.grid[point.y]
      if (row && row[point.x] === 0) {
        row[point.x] = 10
      }
    }

    for (const point of currentStep.value.path) {
      const row = map.value.grid[point.y]
      if (row && (row[point.x] === 0 || row[point.x] === 10)) {
        row[point.x] = 20
      }
    }
  }

  function resetState() {
    isSearching.value = false
    isDone.value = false
    searchResult.value = null
    currentStep.value = null
  }

  return {
    map,
    grid,
    algorithm,
    execution,
    isSearching,
    isDone,
    searchResult,
    createMap,
    loadMap,
    setCell,
    initializeSearch,
    step,
    runToEnd,
    reset
  }
}
