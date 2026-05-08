/**
 * 寻路算法 API 服务
 * 地图和任务是两个独立的子系统
 */

import { API_CONFIG } from './index'

export type CellType = 0 | 1 | 2 | 3

export interface Point {
  x: number
  y: number
}

export interface MapData {
  width: number
  height: number
  startPoint?: Point
  endPoint?: Point
  grid: number[][]
}

export interface SearchStep {
  state: string
  current: Point
  neighbors: Point[]
  added: Point[]
  pruned: Point[]
  path: Point[]
  visited: Point[]
  distance: number
  expanded: number
  stepsTaken: number
}

export interface SearchResult {
  found: boolean
  distance: number
  path: Point[]
  algorithm: string
  expanded: number
}

export type PathfindingAlgorithm = 'dfs' | 'bfs' | 'astar'

export interface TaskInfo {
  taskId: string
  name: string
  mapName?: string
  state: string
  algorithm: string
  width: number
  height: number
  createdAt: string
  updatedAt: string
}

export interface MapInfo {
  name: string
  width: number
  height: number
  createdAt: string
  modifiedAt: string
}

interface ApiResponse<T = any> {
  code: number
  data?: T
  message?: string
}

export class WayfindService {
  private baseUrl: string

  constructor(baseUrl?: string) {
    this.baseUrl = baseUrl || API_CONFIG.wayfind.baseUrl
  }

  private async request<T>(method: string, path: string, body?: any): Promise<T> {
    const data = body ? JSON.stringify(body) : undefined
    const req = new Request(`${this.baseUrl}${path}`, {
      method,
      headers: { 'Content-Type': 'application/json' },
      body: data,
    })
    const resp = await fetch(req)
    const json: ApiResponse<T> = await resp.json()
    if (json.code !== 0) {
      throw new Error(json.message || `API Error: ${json.code}`)
    }
    return json.data as T
  }

  // === 系统 ===
  async healthCheck() {
    return this.request<{ status: string; taskCount: number }>('GET', '/api/v1/health')
  }

  async getAlgorithms() {
    return this.request<{ algorithms: string[]; default: string }>('GET', '/api/v1/algorithms')
  }

  // === 地图管理 (独立于任务) ===
  async listMaps() {
    return this.request<MapInfo[]>('GET', '/api/v1/maps')
  }

  async saveMap(name: string, grid: number[][], width: number, height: number) {
    return this.request<void>('POST', `/api/v1/maps/${encodeURIComponent(name)}`, {
      grid,
      width,
      height,
    })
  }

  async loadMap(name: string) {
    return this.request<{ map: MapData }>('GET', `/api/v1/maps/${encodeURIComponent(name)}`)
  }

  async deleteMap(name: string) {
    return this.request<void>('DELETE', `/api/v1/maps/${encodeURIComponent(name)}`)
  }

  // === 任务管理 ===
  async listTasks() {
    return this.request<{ tasks: TaskInfo[]; total: number }>('GET', '/api/v1/tasks')
  }

  async createTask(width: number, height: number, name: string, mapName?: string) {
    return this.request<TaskInfo>('POST', '/api/v1/tasks', { width, height, name, mapName })
  }

  async getTask(taskId: string) {
    return this.request<{ task: TaskInfo; map: MapData }>('GET', `/api/v1/tasks/${taskId}`)
  }

  async deleteTask(taskId: string) {
    return this.request<void>('DELETE', `/api/v1/tasks/${taskId}`)
  }

  // === 搜索 ===
  async getMap(taskId: string) {
    return this.request<MapData>('GET', `/api/v1/map?taskId=${taskId}`)
  }

  async setCell(taskId: string, x: number, y: number, cellType: CellType) {
    return this.request<void>('PUT', '/api/v1/map/cell', { taskId, x, y, cellType })
  }

  async getDraw(taskId: string) {
    return this.request<{ width: number; height: number; cells: number[][] }>('GET', `/api/v1/map/draw?taskId=${taskId}`)
  }

  async getFinalDraw(taskId: string) {
    return this.request<{ width: number; height: number; cells: number[][] }>('GET', `/api/v1/map/final-draw?taskId=${taskId}`)
  }

  async initSearch(taskId: string, algorithm: PathfindingAlgorithm) {
    return this.request<void>('POST', '/api/v1/search/init', { taskId, algorithm })
  }

  async stepSearch(taskId: string) {
    return this.request<{
      step: SearchStep
      draw: { width: number; height: number; cells: number[][] }
      taskState: string
    }>('POST', `/api/v1/search/step?taskId=${taskId}`)
  }

  async isSearchDone(taskId: string) {
    return this.request<{ done: boolean; state: string }>('GET', `/api/v1/search/done?taskId=${taskId}`)
  }

  async getSearchResult(taskId: string) {
    return this.request<{
      result: SearchResult
      finalDraw: { width: number; height: number; cells: number[][] }
    }>('GET', `/api/v1/search/result?taskId=${taskId}`)
  }

  async getCurrentPath(taskId: string) {
    return this.request<Point[]>('GET', `/api/v1/search/path?taskId=${taskId}`)
  }
}

export const wayfindService = new WayfindService()
export default wayfindService
