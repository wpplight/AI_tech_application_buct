export type CellType = 'empty' | 'wall' | 'start' | 'end'
export type CellState = 'empty' | 'current' | 'visited' | 'path' | 'queue' | 'stack'
export type AlgorithmType = 'bfs' | 'dfs' | 'astar'

export interface Point {
  x: number
  y: number
}

export interface GridCell {
  type: CellType
  state: CellState
  point: Point
}

export interface Grid {
  width: number
  height: number
  cells: GridCell[][]
}

export interface AlgorithmResult {
  found: boolean
  pathLength: number
  expandedNodes: number
  path: Point[]
}

export interface AlgorithmStats {
  steps: number
  visitedNodes: number
  pathLength: number
  found: boolean
}

export interface AnimationState {
  isRunning: boolean
  isPaused: boolean
  speed: number
  currentStep: number
  totalSteps: number
}
