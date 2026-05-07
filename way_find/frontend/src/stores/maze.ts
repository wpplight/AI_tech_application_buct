import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Grid, CellType, CellState, Point, AlgorithmType, AlgorithmStats, AnimationState } from '@/types'

export const useMazeStore = defineStore('maze', () => {
  // State
  const grid = ref<Grid>({
    width: 20,
    height: 15,
    cells: []
  })
  
  const algorithm = ref<AlgorithmType>('bfs')
  const animation = ref<AnimationState>({
    isRunning: false,
    isPaused: false,
    speed: 1,
    currentStep: 0,
    totalSteps: 0
  })
  
  const stats = ref<AlgorithmStats>({
    steps: 0,
    visitedNodes: 0,
    pathLength: 0,
    found: false
  })
  
  const startPoint = ref<Point>({ x: 1, y: 1 })
  const endPoint = ref<Point>({ x: 18, y: 13 })
  
  // Initialize grid
  function initializeGrid(width: number, height: number) {
    grid.value.width = width
    grid.value.height = height
    grid.value.cells = []
    
    for (let y = 0; y < height; y++) {
      const row: any[] = []
      for (let x = 0; x < width; x++) {
        let type: CellType = 'empty'
        if (x === startPoint.value.x && y === startPoint.value.y) {
          type = 'start'
        } else if (x === endPoint.value.x && y === endPoint.value.y) {
          type = 'end'
        }
        
        row.push({
          type,
          state: 'empty' as CellState,
          point: { x, y }
        })
      }
      grid.value.cells.push(row)
    }
  }
  
  // Update cell type
  function updateCellType(point: Point, type: CellType) {
    const cell = grid.value.cells[point.y]?.[point.x]
    if (cell) {
      cell.type = type
    }
  }
  
  // Update cell state
  function updateCellState(point: Point, state: CellState) {
    const cell = grid.value.cells[point.y]?.[point.x]
    if (cell) {
      cell.state = state
    }
  }
  
  // Reset all states
  function resetStates() {
    for (let y = 0; y < grid.value.height; y++) {
      for (let x = 0; x < grid.value.width; x++) {
        const cell = grid.value.cells[y]?.[x]
        if (cell && cell.type !== 'start' && cell.type !== 'end') {
          cell.state = 'empty'
        }
      }
    }
  }
  
  // Set algorithm
  function setAlgorithm(algo: AlgorithmType) {
    algorithm.value = algo
    reset()
  }
  
  // Animation controls
  function play() {
    animation.value.isRunning = true
    animation.value.isPaused = false
  }
  
  function pause() {
    animation.value.isPaused = true
  }
  
  function resume() {
    animation.value.isPaused = false
  }
  
  function stop() {
    animation.value.isRunning = false
    animation.value.isPaused = false
  }
  
  function reset() {
    stop()
    animation.value.currentStep = 0
    stats.value = {
      steps: 0,
      visitedNodes: 0,
      pathLength: 0,
      found: false
    }
    resetStates()
  }
  
  // Set speed
  function setSpeed(speed: number) {
    animation.value.speed = Math.max(0.5, Math.min(10, speed))
  }
  
  // Update stats
  function updateStats(newStats: Partial<AlgorithmStats>) {
    stats.value = { ...stats.value, ...newStats }
  }
  
  // Computed
  const canPlay = computed(() => !animation.value.isRunning || animation.value.isPaused)
  const canPause = computed(() => animation.value.isRunning && !animation.value.isPaused)
  
  // Initialize on creation
  initializeGrid(20, 15)
  
  return {
    // State
    grid,
    algorithm,
    animation,
    stats,
    startPoint,
    endPoint,
    
    // Actions
    initializeGrid,
    updateCellType,
    updateCellState,
    resetStates,
    setAlgorithm,
    play,
    pause,
    resume,
    stop,
    reset,
    setSpeed,
    updateStats,
    
    // Computed
    canPlay,
    canPause
  }
})
