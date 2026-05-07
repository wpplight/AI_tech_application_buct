import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Grid, CellState, Point, AlgorithmType } from '@/types'
import type { MapData } from './map'
import { wailsService } from '@/composables/wails'

export type ExecutionStatus = 'idle' | 'running' | 'paused' | 'completed'

export interface ExecutionState {
  status: ExecutionStatus
  currentStep: number
  totalSteps: number
  visitedCells: Point[]
  currentCells: Point[]
  path: Point[]
  found: boolean
}

export const useExecutionStore = defineStore('execution', () => {
  const grid = ref<Grid>({
    width: 20,
    height: 15,
    cells: []
  })

  const algorithm = ref<AlgorithmType>('bfs')

  const execution = ref<ExecutionState>({
    status: 'idle',
    currentStep: 0,
    totalSteps: 0,
    visitedCells: [],
    currentCells: [],
    path: [],
    found: false
  })

  const speed = ref(1)

  let executionInterval: number | null = null
  let isBackendSearch = true

  function getCellType(value: number): 'empty' | 'wall' | 'start' | 'end' {
    switch (value) {
      case 1: return 'wall'
      case 2: return 'start'
      case 3: return 'end'
      default: return 'empty'
    }
  }

  function getCellState(value: number): CellState {
    switch (value) {
      case 10: return 'visited'
      case 20: return 'path'
      default: return 'empty'
    }
  }

  async function loadMap(mapData: MapData) {
    grid.value.width = mapData.width
    grid.value.height = mapData.height
    grid.value.cells = []

    for (let y = 0; y < mapData.height; y++) {
      const row: any[] = []
      for (let x = 0; x < mapData.width; x++) {
        let type: 'empty' | 'wall' | 'start' | 'end' = 'empty'

        if (x === mapData.startPoint.x && y === mapData.startPoint.y) {
          type = 'start'
        } else if (x === mapData.endPoint.x && y === mapData.endPoint.y) {
          type = 'end'
        } else if (mapData.walls.some((w: Point) => w.x === x && w.y === y)) {
          type = 'wall'
        }

        row.push({
          type,
          state: 'empty' as CellState,
          point: { x, y }
        })
      }
      grid.value.cells.push(row)
    }

    if (isBackendSearch) {
      try {
        const backendMapData = {
          width: mapData.width,
          height: mapData.height,
          grid: [] as number[][],
          startPoint: { x: mapData.startPoint.x, y: mapData.startPoint.y },
          endPoint: { x: mapData.endPoint.x, y: mapData.endPoint.y }
        }
        for (let y = 0; y < mapData.height; y++) {
          const row: number[] = []
          for (let x = 0; x < mapData.width; x++) {
            let cellType = 0
            if (x === mapData.startPoint.x && y === mapData.startPoint.y) {
              cellType = 2
            } else if (x === mapData.endPoint.x && y === mapData.endPoint.y) {
              cellType = 3
            } else if (mapData.walls.some((w: Point) => w.x === x && w.y === y)) {
              cellType = 1
            }
            row.push(cellType)
          }
          backendMapData.grid.push(row)
        }
        await wailsService.LoadMap(backendMapData)
      } catch (error) {
        console.error('Failed to load map to backend:', error)
      }
    }
  }

  async function loadMapFromBackend(draw: number[][]) {
    grid.value.height = draw.length
    grid.value.width = draw[0]?.length || 0
    grid.value.cells = []

    for (let y = 0; y < grid.value.height; y++) {
      const row: any[] = []
      for (let x = 0; x < grid.value.width; x++) {
        const value = draw[y]?.[x] || 0
        row.push({
          type: getCellType(value),
          state: getCellState(value),
          point: { x, y }
        })
      }
      grid.value.cells.push(row)
    }
  }

  function updateCellState(point: Point, state: CellState) {
    const cell = grid.value.cells[point.y]?.[point.x]
    if (cell) {
      cell.state = state
    }
  }

  function resetExecution() {
    if (executionInterval) {
      clearInterval(executionInterval)
      executionInterval = null
    }

    execution.value = {
      status: 'idle',
      currentStep: 0,
      totalSteps: 0,
      visitedCells: [],
      currentCells: [],
      path: [],
      found: false
    }

    for (let y = 0; y < grid.value.height; y++) {
      for (let x = 0; x < grid.value.width; x++) {
        const cell = grid.value.cells[y]?.[x]
        if (cell && cell.type !== 'start' && cell.type !== 'end') {
          cell.state = 'empty'
        }
      }
    }
  }

  async function initializeBackendSearch() {
    try {
      await wailsService.InitializeSearch(algorithm.value)
      execution.value.status = 'idle'
      execution.value.currentStep = 0
      execution.value.visitedCells = []
      execution.value.path = []
      execution.value.found = false

      await updateDrawFromBackend()
      return true
    } catch (error) {
      console.error('Failed to initialize backend search:', error)
      return false
    }
  }

  async function updateDrawFromBackend() {
    try {
      const draw = await wailsService.GetDraw()
      if (draw) {
        await loadMapFromBackend(draw)
      }
    } catch (error) {
      console.error('Failed to get draw from backend:', error)
    }
  }

  async function backendSearchStep() {
    try {
      const done = await wailsService.IsSearchDone()
      
      if (done) {
        const result = await wailsService.GetSearchResult()
        if (result) {
          execution.value.found = result.found
          execution.value.path = result.path || []
        }
        return true
      }

      await wailsService.SearchStep()
      execution.value.currentStep++

      const result = await wailsService.GetSearchResult()
      if (result) {
        execution.value.found = result.found
        execution.value.path = result.path || []
      }

      await updateDrawFromBackend()
      return false
    } catch (error) {
      console.error('Backend search step error:', error)
      return true
    }
  }

  async function startExecution() {
    if (execution.value.status === 'running') return

    if (execution.value.status === 'idle' && isBackendSearch) {
      const initialized = await initializeBackendSearch()
      if (!initialized) return
    }

    execution.value.status = 'running'
    executeStepByStepBackend()
  }

  async function executeAllStepsBackend() {
    const executeBatch = async () => {
      let steps = 0
      const maxSteps = 1000

      while (steps < maxSteps) {
        const done = await backendSearchStep()
        if (done) break
        steps++
      }

      if (executionInterval) {
        clearTimeout(executionInterval as number)
        executionInterval = null
      }

      await loadFinalStateFromBackend()
      execution.value.status = 'completed'
    }

    executionInterval = window.setTimeout(executeBatch, 0) as unknown as number
  }

  async function loadFinalStateFromBackend() {
    try {
      const draw = await wailsService.GetFinalDraw()
      if (draw) {
        await loadMapFromBackend(draw)
      }
    } catch (error) {
      console.error('Failed to get final draw from backend:', error)
    }
  }

  function executeStepByStepBackend() {
    const interval = 1000 / speed.value

    executionInterval = window.setInterval(async () => {
      const done = await backendSearchStep()
      if (done) {
        if (executionInterval) {
          clearInterval(executionInterval)
          executionInterval = null
        }
        await loadFinalStateFromBackend()
        execution.value.status = 'completed'
      }
    }, interval)
  }

  function pauseExecution() {
    if (execution.value.status === 'running') {
      execution.value.status = 'paused'
      if (executionInterval) {
        clearInterval(executionInterval)
        executionInterval = null
      }
    }
  }

  function resumeExecution() {
    if (execution.value.status === 'paused') {
      execution.value.status = 'running'
      executeStepByStepBackend()
    }
  }

  function stopExecution() {
    if (executionInterval) {
      clearInterval(executionInterval)
      executionInterval = null
    }
    resetExecution()
  }

  async function stepForward() {
    if (execution.value.currentStep < execution.value.totalSteps || execution.value.status === 'idle') {
      const done = await backendSearchStep()
      if (done) {
        execution.value.status = 'completed'
      }
    }
  }

  function stepBackward() {
    console.warn('Step backward not supported in backend mode')
  }

  function goToStep(step: number) {
    console.warn('Go to step not supported in backend mode')
  }

  function setSpeed(newSpeed: number) {
    speed.value = newSpeed
    if (execution.value.status === 'running') {
      pauseExecution()
      resumeExecution()
    }
  }

  async function setAlgorithm(newAlgorithm: AlgorithmType) {
    algorithm.value = newAlgorithm
    resetExecution()

    if (isBackendSearch) {
      try {
        await initializeBackendSearch()
      } catch (error) {
        console.error('Failed to initialize backend search:', error)
      }
    }
  }

  async function initializeBackendSearchIfNeeded() {
    if (isBackendSearch && execution.value.status === 'idle' && execution.value.currentStep === 0) {
      await initializeBackendSearch()
    }
  }

  return {
    grid,
    algorithm,
    execution,
    speed,
    loadMap,
    loadMapFromBackend,
    updateCellState,
    resetExecution,
    startExecution,
    pauseExecution,
    resumeExecution,
    stopExecution,
    stepForward,
    stepBackward,
    goToStep,
    setSpeed,
    setAlgorithm,
    initializeBackendSearch,
    initializeBackendSearchIfNeeded
  }
})
