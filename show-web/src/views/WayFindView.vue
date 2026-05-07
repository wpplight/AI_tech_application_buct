<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useWayfindStore } from '../stores/wayfind'
import LoadingSpinner from '../components/common/LoadingSpinner.vue'
import ErrorMessage from '../components/common/ErrorMessage.vue'
import StatCard from '../components/common/StatCard.vue'
import type { PathfindingAlgorithm } from '../api/wayfind'

const store = useWayfindStore()

const mapWidth = ref(20)
const mapHeight = ref(15)
const selectedCell = ref<'road' | 'wall' | 'start' | 'end'>('wall')

const algorithms: { value: PathfindingAlgorithm; label: string; desc: string }[] = [
  { value: 'bfs', label: 'BFS', desc: '广度优先搜索' },
  { value: 'dfs', label: 'DFS', desc: '深度优先搜索' },
  { value: 'astar', label: 'A*', desc: '启发式搜索' }
]

const cellColors = {
  0: '#27272a',
  1: '#3f3f46',
  2: '#10b981',
  3: '#f43f5e'
}

onMounted(async () => {
  await store.checkConnection()
  if (store.isConnected) {
    await store.createMap(mapWidth.value, mapHeight.value)
  }
})

async function handleCreateMap() {
  await store.createMap(mapWidth.value, mapHeight.value)
}

async function handleCellClick(x: number, y: number) {
  await store.updateCell(x, y, selectedCell.value)
}

async function handleInitSearch() {
  await store.initSearch()
}

async function handleStepSearch() {
  await store.stepSearch()
}

async function handleRunSearch() {
  await store.runSearch()
}

function handlePauseSearch() {
  store.pauseSearch()
}

function handleReset() {
  store.resetSearch()
}

function getCellColor(cell: number, x: number, y: number) {
  if (store.currentPath.some(p => p.x === x && p.y === y)) {
    return '#3b82f6'
  }
  if (store.currentVisited.some(p => p.x === x && p.y === y)) {
    return '#60a5fa'
  }
  return cellColors[cell as keyof typeof cellColors] || cellColors[0]
}

function handleCloseError() {
  store.error = null
}
</script>

<template>
  <div class="wayfind-view">
    <div class="page-header">
      <div class="header-content">
        <h1 class="page-title">寻路算法</h1>
        <p class="page-subtitle">迷宫寻路可视化系统</p>
      </div>
      <div class="connection-status" :class="{ connected: store.isConnected }">
        <span class="status-dot"></span>
        {{ store.isConnected ? '已连接' : '未连接' }}
      </div>
    </div>

    <div v-if="store.error" class="error-container">
      <ErrorMessage :message="store.error" @close="handleCloseError" />
    </div>

    <div class="content-wrapper">
      <div class="control-panel">
        <div class="panel">
          <div class="panel-section">
            <h2 class="section-title">地图设置</h2>
            <div class="map-settings">
              <div class="setting-row">
                <label class="setting-label">宽度</label>
                <input v-model.number="mapWidth" type="number" min="5" max="50" class="setting-input" />
              </div>
              <div class="setting-row">
                <label class="setting-label">高度</label>
                <input v-model.number="mapHeight" type="number" min="5" max="50" class="setting-input" />
              </div>
              <button class="btn btn-primary" @click="handleCreateMap">
                创建地图
              </button>
            </div>
          </div>

          <div class="panel-section">
            <h2 class="section-title">算法选择</h2>
            <div class="algorithm-selector">
              <button
                v-for="algo in algorithms"
                :key="algo.value"
                class="algo-btn"
                :class="{ active: store.currentAlgorithm === algo.value }"
                @click="store.setAlgorithm(algo.value)"
              >
                <span class="algo-label">{{ algo.label }}</span>
                <span class="algo-desc">{{ algo.desc }}</span>
              </button>
            </div>
          </div>

          <div class="panel-section">
            <h2 class="section-title">画笔工具</h2>
            <div class="brush-tools">
              <button
                v-for="(label, type) in { road: '道路', wall: '墙壁', start: '起点', end: '终点' }"
                :key="type"
                class="brush-btn"
                :class="{ active: selectedCell === type }"
                @click="selectedCell = type as any"
              >
                <span class="brush-preview" :style="{ background: cellColors[type === 'road' ? 0 : type === 'wall' ? 1 : type === 'start' ? 2 : 3] }"></span>
                {{ label }}
              </button>
            </div>
          </div>

          <div class="panel-section">
            <h2 class="section-title">搜索控制</h2>
            <div class="search-controls">
              <button class="btn btn-success" @click="handleInitSearch">
                初始化
              </button>
              <button class="btn btn-outline" @click="handleStepSearch" :disabled="!store.searchId">
                单步
              </button>
              <button class="btn btn-primary" @click="handleRunSearch" :disabled="!store.searchId">
                {{ store.isSearching ? '运行中...' : '运行' }}
              </button>
              <button v-if="store.isSearching" class="btn btn-warning" @click="handlePauseSearch">
                暂停
              </button>
              <button class="btn btn-outline" @click="handleReset">
                重置
              </button>
            </div>
          </div>

          <div class="panel-section">
            <h2 class="section-title">统计信息</h2>
            <div class="stats-grid">
              <StatCard label="访问节点" :value="store.visitedCount" />
              <StatCard label="路径长度" :value="store.pathLength" unit="步" />
              <StatCard label="执行时间" :value="store.executionTime" unit="ms" />
            </div>
          </div>
        </div>
      </div>

      <div class="map-panel">
        <div class="panel">
          <div class="map-container">
            <div v-if="store.currentMap" class="map-grid" :style="{
              gridTemplateColumns: `repeat(${store.currentMap.width}, minmax(0, 1fr))`
            }">
              <div
                v-for="(row, y) in store.currentMap.grid"
                :key="`row-${y}`"
                class="grid-row"
              >
                <div
                  v-for="(cell, x) in row"
                  :key="`cell-${x}-${y}`"
                  class="grid-cell"
                  :style="{ background: getCellColor(cell, x, y) }"
                  @click="handleCellClick(x, y)"
                ></div>
              </div>
            </div>
            <div v-else class="map-placeholder">
              <p>点击"创建地图"开始</p>
            </div>
          </div>

          <div class="map-legend">
            <div class="legend-item">
              <span class="legend-color" style="background: #10b981"></span>
              起点
            </div>
            <div class="legend-item">
              <span class="legend-color" style="background: #f43f5e"></span>
              终点
            </div>
            <div class="legend-item">
              <span class="legend-color" style="background: #60a5fa"></span>
              已访问
            </div>
            <div class="legend-item">
              <span class="legend-color" style="background: #3b82f6"></span>
              最短路径
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-if="store.loading" class="loading-overlay">
      <LoadingSpinner size="lg" />
    </div>
  </div>
</template>

<style scoped>
.wayfind-view {
  width: 100%;
}

.page-header {
  max-width: 1600px;
  margin: 0 auto;
  padding: 0 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 24px;
}

@media (min-width: 768px) {
  .page-header {
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    padding: 0 24px;
    margin-bottom: 32px;
  }
}

.page-title {
  font-size: 28px;
  font-weight: 700;
  letter-spacing: -0.02em;
  margin-bottom: 4px;
}

@media (min-width: 768px) {
  .page-title {
    font-size: 36px;
  }
}

.page-subtitle {
  color: #a1a1aa;
  font-size: 14px;
}

@media (min-width: 768px) {
  .page-subtitle {
    font-size: 16px;
  }
}

.connection-status {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 6px 14px;
  background: rgba(244, 63, 94, 0.1);
  border: 1px solid rgba(244, 63, 94, 0.3);
  border-radius: 20px;
  font-size: 12px;
  color: #f43f5e;
}

@media (min-width: 768px) {
  .connection-status {
    font-size: 13px;
    padding: 8px 16px;
  }
}

.connection-status.connected {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.3);
  color: #10b981;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: currentColor;
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.error-container {
  max-width: 1600px;
  margin: 0 auto 20px;
  padding: 0 16px;
}

@media (min-width: 768px) {
  .error-container {
    padding: 0 24px;
    margin-bottom: 24px;
  }
}

.content-wrapper {
  max-width: 1600px;
  margin: 0 auto;
  padding: 0 16px;
  display: grid;
  grid-template-columns: 1fr;
  gap: 20px;
}

@media (min-width: 768px) {
  .content-wrapper {
    padding: 0 24px;
    gap: 24px;
  }
}

@media (min-width: 1024px) {
  .content-wrapper {
    grid-template-columns: 350px 1fr;
    padding: 0 32px;
  }
}

.control-panel,
.map-panel {
  display: flex;
  flex-direction: column;
}

.panel {
  background: #18181b;
  border: 1px solid #27272a;
  border-radius: 16px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

@media (min-width: 768px) {
  .panel {
    border-radius: 20px;
    padding: 24px;
  }
}

.panel-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: #a1a1aa;
}

@media (min-width: 768px) {
  .section-title {
    font-size: 16px;
  }
}

.map-settings {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.setting-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.setting-label {
  flex: 1;
  font-size: 14px;
  color: #a1a1aa;
}

.setting-input {
  width: 80px;
  padding: 8px 12px;
  background: #27272a;
  border: 1px solid #3f3f46;
  border-radius: 8px;
  color: #fafafa;
  font-size: 14px;
}

@media (min-width: 768px) {
  .setting-input {
    width: 100px;
    padding: 10px 14px;
    border-radius: 10px;
  }
}

.algorithm-selector {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.algo-btn {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 14px;
  background: #27272a;
  border: 1px solid #3f3f46;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;
}

@media (min-width: 768px) {
  .algo-btn {
    padding: 12px 16px;
    border-radius: 12px;
  }
}

.algo-btn:hover {
  border-color: #52525b;
}

.algo-btn.active {
  background: rgba(59, 130, 246, 0.1);
  border-color: rgba(59, 130, 246, 0.5);
}

.algo-label {
  font-size: 13px;
  font-weight: 600;
  color: #fafafa;
}

@media (min-width: 768px) {
  .algo-label {
    font-size: 14px;
  }
}

.algo-desc {
  font-size: 11px;
  color: #a1a1aa;
}

@media (min-width: 768px) {
  .algo-desc {
    font-size: 12px;
  }
}

.brush-tools {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 8px;
}

.brush-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: #27272a;
  border: 1px solid #3f3f46;
  border-radius: 8px;
  color: #fafafa;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

@media (min-width: 768px) {
  .brush-btn {
    padding: 10px 14px;
    font-size: 13px;
    border-radius: 10px;
  }
}

.brush-btn:hover {
  border-color: #52525b;
}

.brush-btn.active {
  border-color: #3b82f6;
  background: rgba(59, 130, 246, 0.1);
}

.brush-preview {
  width: 14px;
  height: 14px;
  border-radius: 4px;
}

@media (min-width: 768px) {
  .brush-preview {
    width: 16px;
    height: 16px;
    border-radius: 4px;
  }
}

.search-controls {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 8px;
}

.btn {
  padding: 8px 14px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
}

@media (min-width: 768px) {
  .btn {
    padding: 10px 16px;
    font-size: 14px;
    gap: 8px;
  }
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background: #3b82f6;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #2563eb;
}

.btn-success {
  background: #10b981;
  color: white;
}

.btn-success:hover:not(:disabled) {
  background: #059669;
}

.btn-warning {
  background: #f59e0b;
  color: white;
}

.btn-outline {
  background: transparent;
  border: 1px solid #3f3f46;
  color: #a1a1aa;
}

.btn-outline:hover:not(:disabled) {
  border-color: #52525b;
  color: #fafafa;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 8px;
}

@media (min-width: 768px) {
  .stats-grid {
    gap: 12px;
  }
}

.map-container {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #0f0f11;
  border-radius: 12px;
  padding: 16px;
  min-height: 300px;
}

@media (min-width: 768px) {
  .map-container {
    min-height: 400px;
    padding: 24px;
    border-radius: 16px;
  }
}

.map-grid {
  display: grid;
  gap: 2px;
  width: 100%;
  max-width: 100%;
  aspect-ratio: auto;
}

.grid-row {
  display: contents;
}

.grid-cell {
  aspect-ratio: 1;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s;
  min-width: 12px;
  min-height: 12px;
}

@media (min-width: 768px) {
  .grid-cell {
    min-width: 16px;
    min-height: 16px;
  }
}

@media (min-width: 1024px) {
  .grid-cell {
    min-width: 20px;
    min-height: 20px;
  }
}

.grid-cell:hover {
  transform: scale(1.1);
  box-shadow: 0 0 8px rgba(255, 255, 255, 0.2);
  z-index: 1;
}

.map-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  color: #52525b;
  font-size: 14px;
}

@media (min-width: 768px) {
  .map-placeholder {
    font-size: 16px;
  }
}

.map-legend {
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
  gap: 16px;
  padding-top: 16px;
  border-top: 1px solid #27272a;
  margin-top: 16px;
}

@media (min-width: 768px) {
  .map-legend {
    gap: 24px;
    padding-top: 20px;
    margin-top: 20px;
  }
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: #a1a1aa;
}

@media (min-width: 768px) {
  .legend-item {
    font-size: 13px;
  }
}

.legend-color {
  width: 14px;
  height: 14px;
  border-radius: 4px;
}

@media (min-width: 768px) {
  .legend-color {
    width: 16px;
    height: 16px;
  }
}

.loading-overlay {
  position: fixed;
  inset: 0;
  background: rgba(9, 9, 11, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}
</style>
