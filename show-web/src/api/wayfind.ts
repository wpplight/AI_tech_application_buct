/**
 * 寻路算法 API 服务
 * 提供迷宫寻路系统的 API 调用封装
 */

import { API_CONFIG, buildUrl } from './index'

// 单元格类型
export type CellType = 0 | 1 | 2 | 3  // 0=道路, 1=墙壁, 2=起点, 3=终点

// 点坐标
export interface Point {
  x: number
  y: number
}

// 地图数据
export interface MapData {
  width: number
  height: number
  startPoint: Point
  endPoint: Point
  grid: number[][]
}

// 搜索状态
export type SearchState = 'ready' | 'running' | 'found' | 'not_found'

// 搜索步骤
export interface SearchStep {
  state: SearchState
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

// 搜索结果
export interface SearchResult {
  found: boolean
  distance: number
  path: Point[]
  algorithm: string
  expanded?: number
  execution_time_ms?: number
  optimal?: boolean
}

// 算法类型
export type PathfindingAlgorithm = 'dfs' | 'bfs' | 'astar'

// 地图创建请求
export interface CreateMapRequest {
  width: number
  height: number
  obstacles?: Point[]
}

// 单元格更新请求
export interface UpdateCellRequest {
  cells: Array<{
    x: number
    y: number
    type: 'road' | 'wall' | 'start' | 'end'
  }>
}

// 搜索初始化请求
export interface InitSearchRequest {
  map_id: string
  algorithm: PathfindingAlgorithm
}

// 算法对比结果
export interface AlgorithmComparison {
  map_id: string
  results: {
    dfs: SearchResult
    bfs: SearchResult
    astar: SearchResult
  }
  recommendation: string
}

/**
 * 寻路算法 API 服务类
 */
export class WayfindService {
  private baseUrl: string

  constructor(baseUrl?: string) {
    this.baseUrl = baseUrl || API_CONFIG.wayfind.baseUrl
  }

  /**
   * 创建新地图
   */
  async createMap(request: CreateMapRequest): Promise<{ success: boolean; map_id?: string; map_data?: MapData }> {
    const url = buildUrl(this.baseUrl, API_CONFIG.wayfind.endpoints.maps)
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request)
    })
    return response.json()
  }

  /**
   * 获取地图
   */
  async getMap(mapId: string): Promise<{ map_id: string; map_data: MapData }> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.wayfind.endpoints.maps}/${mapId}`)
    const response = await fetch(url)
    return response.json()
  }

  /**
   * 更新地图单元格
   */
  async updateCells(mapId: string, request: UpdateCellRequest): Promise<{ success: boolean; updated_count: number }> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.wayfind.endpoints.maps}/${mapId}/cells`)
    const response = await fetch(url, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request)
    })
    return response.json()
  }

  /**
   * 初始化搜索
   */
  async initSearch(request: InitSearchRequest): Promise<{
    success: boolean
    search_id?: string
    algorithm?: PathfindingAlgorithm
    initialized?: boolean
  }> {
    const url = buildUrl(this.baseUrl, API_CONFIG.wayfind.endpoints.search)
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request)
    })
    return response.json()
  }

  /**
   * 执行单步搜索
   */
  async stepSearch(searchId: string): Promise<SearchStep> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.wayfind.endpoints.search}/${searchId}/step`)
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' }
    })
    return response.json()
  }

  /**
   * 执行完整搜索
   */
  async runSearch(searchId: string): Promise<SearchResult> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.wayfind.endpoints.search}/${searchId}/run`)
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' }
    })
    return response.json()
  }

  /**
   * 获取搜索历史
   */
  async getSearchHistory(searchId: string): Promise<{
    search_id: string
    algorithm: PathfindingAlgorithm
    total_steps: number
    history: SearchStep[]
  }> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.wayfind.endpoints.search}/${searchId}/history`)
    const response = await fetch(url)
    return response.json()
  }

  /**
   * 算法对比
   */
  async compareAlgorithms(mapId: string): Promise<AlgorithmComparison> {
    const url = buildUrl(this.baseUrl, API_CONFIG.wayfind.endpoints.compare)
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ map_id: mapId })
    })
    return response.json()
  }
}

// 导出单例
export const wayfindService = new WayfindService()

export default wayfindService
