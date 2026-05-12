/**
 * 寻路算法状态管理
 * 两套独立的子系统:
 *   地图 (MapData) → 独立编辑网格，保存到磁盘
 *   任务 (Task)   → 基于地图创建，支持搜索
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { wayfindService } from '../api/wayfind'
import type { PathfindingAlgorithm, MapData, SearchStep, SearchResult, TaskInfo, MapInfo } from '../api/wayfind'

export const useWayfindStore = defineStore('wayfind', () => {
  // === 地图编辑 (独立于任务) ===
  const savedMaps = ref<MapInfo[]>([])
  const editingMap = ref<MapData | null>(null)  // 正在编辑的地图
  const editingMapName = ref<string>('')         // 正在编辑的地图名称

  // === 任务 ===
  const tasks = ref<TaskInfo[]>([])
  const currentTask = ref<TaskInfo | null>(null)
  const currentTaskMap = ref<MapData | null>(null)  // 任务关联的地图
  const currentTaskDraw = ref<number[][] | null>(null)

  async function setCell(x: number, y: number, cellType: 0 | 1 | 2 | 3) {
    if (!currentTask.value) return
    try {
      await wayfindService.setCell(currentTask.value.taskId, x, y, cellType)
      const draw = await wayfindService.getDraw(currentTask.value.taskId)
      currentTaskDraw.value = draw.cells
      const { task } = await wayfindService.getTask(currentTask.value.taskId)
      currentTask.value = task
    } catch (e) {
      error.value = e instanceof Error ? e.message : '更新单元格失败'
    }
  }

  // === 搜索 ===
  const currentAlgorithm = ref<PathfindingAlgorithm>('astar')
  const searchStep = ref<SearchStep | null>(null)
  const searchResult = ref<SearchResult | null>(null)

  // === 全局 ===
  const loading = ref(false)
  const error = ref<string | null>(null)
  const isConnected = ref(false)

  // === 计算属性 ===
  const visitedCount = computed(() => searchStep.value?.visited?.length || 0)
  const pathLength = computed(() => searchResult.value?.path?.length || searchStep.value?.path?.length || 0)
  const expandedCount = computed(() => searchStep.value?.expanded || searchResult.value?.expanded || 0)
  const taskState = computed(() => currentTask.value?.state || 'idle')
  const isDone = computed(() => taskState.value === 'done' || taskState.value === 'failed')
  const isSearching = computed(() => taskState.value === 'searching')

  // === 连接检查 ===
  async function checkConnection() {
    try {
      const health = await wayfindService.healthCheck()
      isConnected.value = health.status === 'ok'
    } catch {
      isConnected.value = false
    }
    return isConnected.value
  }

  // === 地图管理 ===
  async function fetchMaps() {
    try {
      savedMaps.value = await wayfindService.listMaps()
    } catch (e) {
      error.value = e instanceof Error ? e.message : '获取地图列表失败'
    }
  }

  function createNewMap(width: number, height: number) {
    const grid: number[][] = []
    for (let y = 0; y < height; y++) {
      grid[y] = []
      for (let x = 0; x < width; x++) {
        grid[y][x] = 0
      }
    }
    editingMap.value = { width, height, grid }
    editingMapName.value = ''
  }

  async function loadMapForEdit(name: string) {
    try {
      loading.value = true
      const { map } = await wayfindService.loadMap(name)
      editingMap.value = map
      editingMapName.value = name
      return map
    } catch (e) {
      error.value = e instanceof Error ? e.message : '加载地图失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function saveMap(name: string) {
    if (!editingMap.value) return
    try {
      await wayfindService.saveMap(name, editingMap.value.grid, editingMap.value.width, editingMap.value.height)
      editingMapName.value = name
      await fetchMaps()
    } catch (e) {
      error.value = e instanceof Error ? e.message : '保存地图失败'
    }
  }

  async function deleteMap(name: string) {
    try {
      await wayfindService.deleteMap(name)
      savedMaps.value = savedMaps.value.filter(m => m.name !== name)
      if (editingMapName.value === name) {
        editingMap.value = null
        editingMapName.value = ''
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '删除地图失败'
    }
  }

  // === 地图编辑 (本地更新) ===
  function setEditingCell(x: number, y: number, cellType: 0 | 1 | 2 | 3) {
    if (!editingMap.value) return
    editingMap.value.grid[y][x] = cellType
    if (cellType === 2) {
      editingMap.value.startPoint = { x, y }
    } else if (cellType === 3) {
      editingMap.value.endPoint = { x, y }
    }
  }

  // === 任务管理 ===
  async function fetchTasks() {
    try {
      const result = await wayfindService.listTasks()
      tasks.value = result.tasks
    } catch (e) {
      error.value = e instanceof Error ? e.message : '获取任务列表失败'
    }
  }

  async function createTask(width: number, height: number, name: string, mapName?: string) {
    try {
      loading.value = true
      const task = await wayfindService.createTask(width, height, name, mapName)
      await fetchTasks()
      return task
    } catch (e) {
      error.value = e instanceof Error ? e.message : '创建任务失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function selectTask(taskId: string) {
    try {
      loading.value = true
      const { task, map } = await wayfindService.getTask(taskId)
      currentTask.value = task
      currentTaskMap.value = map
      const draw = await wayfindService.getDraw(taskId)
      currentTaskDraw.value = draw.cells
      searchStep.value = null
      searchResult.value = null
      return task
    } catch (e) {
      error.value = e instanceof Error ? e.message : '加载任务失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deleteTask(taskId: string) {
    try {
      await wayfindService.deleteTask(taskId)
      tasks.value = tasks.value.filter(t => t.taskId !== taskId)
      if (currentTask.value?.taskId === taskId) {
        currentTask.value = null
        currentTaskMap.value = null
        currentTaskDraw.value = null
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '删除任务失败'
    }
  }

  // === 搜索 ===
  function setAlgorithm(algorithm: PathfindingAlgorithm) {
    currentAlgorithm.value = algorithm
  }

  async function initSearch() {
    if (!currentTask.value) return
    try {
      loading.value = true
      await wayfindService.initSearch(currentTask.value.taskId, currentAlgorithm.value)
      const { task } = await wayfindService.getTask(currentTask.value.taskId)
      currentTask.value = task
      searchStep.value = null
      searchResult.value = null
      const draw = await wayfindService.getDraw(currentTask.value.taskId)
      currentTaskDraw.value = draw.cells
    } catch (e) {
      error.value = e instanceof Error ? e.message : '初始化搜索失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function stepSearch(): Promise<boolean> {
    if (!currentTask.value) return false
    try {
      const result = await wayfindService.stepSearch(currentTask.value.taskId)
      searchStep.value = result.step
      currentTaskDraw.value = result.draw.cells
      const { task } = await wayfindService.getTask(currentTask.value.taskId)
      currentTask.value = task

      if (result.taskState === 'done' || result.taskState === 'failed') {
        const finalResult = await wayfindService.getSearchResult(currentTask.value.taskId)
        searchResult.value = finalResult.result
        currentTaskDraw.value = finalResult.finalDraw.cells
        return true
      }
      return false
    } catch (e) {
      error.value = e instanceof Error ? e.message : '搜索失败'
      return false
    }
  }

  async function runSearch() {
    if (!currentTask.value) return
    try {
      loading.value = true
      while (true) {
        const { done } = await wayfindService.isSearchDone(currentTask.value!.taskId)
        if (done) break
        await stepSearch()
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '搜索失败'
    } finally {
      loading.value = false
    }
  }

  async function resetSearch() {
    if (!currentTask.value) return
    try {
      const result = await wayfindService.resetSearch(currentTask.value.taskId)
      const { task, map } = await wayfindService.getTask(currentTask.value.taskId)
      currentTask.value = task
      currentTaskMap.value = map
      searchStep.value = null
      searchResult.value = null
      currentTaskDraw.value = result.draw.cells
    } catch (e) {
      error.value = e instanceof Error ? e.message : '重置失败'
    }
  }

  return {
    // 地图
    savedMaps,
    editingMap,
    editingMapName,
    fetchMaps,
    createNewMap,
    loadMapForEdit,
    saveMap,
    deleteMap,
    setEditingCell,
    // 任务
    tasks,
    currentTask,
    currentTaskMap,
    currentTaskDraw,
    fetchTasks,
    createTask,
    selectTask,
    deleteTask,
    setCell,
    // 搜索
    currentAlgorithm,
    searchStep,
    searchResult,
    visitedCount,
    pathLength,
    expandedCount,
    taskState,
    isDone,
    isSearching,
    setAlgorithm,
    initSearch,
    stepSearch,
    runSearch,
    resetSearch,
    // 全局
    loading,
    error,
    isConnected,
    checkConnection,
  }
})
