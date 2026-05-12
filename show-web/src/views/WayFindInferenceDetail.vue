<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useWayfindStore } from '../stores/wayfind'
import type { PathfindingAlgorithm } from '../api/wayfind'

const route = useRoute()
const router = useRouter()
const store = useWayfindStore()

const isRunning = ref(false)
const timer = ref<ReturnType<typeof setInterval> | null>(null)
const speed = ref(200)

const algorithms: { value: PathfindingAlgorithm; label: string; desc: string }[] = [
  { value: 'bfs', label: 'BFS', desc: '广度优先搜索' },
  { value: 'dfs', label: 'DFS', desc: '深度优先搜索' },
  { value: 'astar', label: 'A*', desc: '启发式搜索' }
]

const mapW = computed(() => store.currentTaskMap?.width ?? 20)
const mapH = computed(() => store.currentTaskMap?.height ?? 15)
const totalCells = computed(() => mapW.value * mapH.value)

const baseColors = {
  0: '#e4e4e7',
  1: '#52525b',
  2: '#10b981',
  3: '#f43f5e',
  4: '#60a5fa',
  5: '#a855f7',
  6: '#a855f7',
} as const

type BaseColorKey = keyof typeof baseColors

function getCellColor(index: number): string {
  const draw = store.currentTaskDraw
  const w = mapW.value
  const x = index % w
  const y = Math.floor(index / w)

  if (!draw || y < 0 || y >= draw.length) return '#e4e4e7'
  const row = draw[y]
  if (!row || x < 0 || x >= row.length) return '#e4e4e7'
  const cell = row[x]
  if (cell === undefined) return '#e4e4e7'

  const color = baseColors[cell as BaseColorKey]
  return color ?? '#e4e4e7'
}

async function handleInit() {
  await store.initSearch()
  startTimer()
}

function startTimer() {
  if (timer.value) return
  isRunning.value = true
  timer.value = setInterval(async () => {
    if (!store.currentTask) {
      stopTimer()
      return
    }
    const done = await store.stepSearch()
    if (done) {
      stopTimer()
    }
  }, speed.value)
}

function stopTimer() {
  if (timer.value) {
    clearInterval(timer.value)
    timer.value = null
  }
  isRunning.value = false
}

async function handlePlayPause() {
  if (!store.currentTask) return

  if (isRunning.value) {
    stopTimer()
    return
  }

  if (store.taskState === 'idle') {
    await handleInit()
  } else if (store.isDone) {
    await store.resetSearch()
    await handleInit()
  } else {
    startTimer()
  }
}

async function handleReset() {
  stopTimer()
  await store.resetSearch()
}

function handleBack() {
  stopTimer()
  router.push('/wayfind/inference')
}

onMounted(async () => {
  const taskId = route.query.taskId as string
  if (taskId) {
    await store.selectTask(taskId)
  } else {
    router.push('/wayfind/inference')
  }
})

onUnmounted(() => {
  stopTimer()
})
</script>

<template>
  <div class="detail-view">
    <!-- Header -->
    <div class="detail-header">
      <button class="back-btn" @click="handleBack">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M19 12H5M12 19l-7-7 7-7"/>
        </svg>
        返回
      </button>
      <div class="header-center">
        <span class="task-name">{{ store.currentTask?.name || '未知任务' }}</span>
        <span class="task-meta">{{ store.currentTask?.width }}×{{ store.currentTask?.height }}</span>
      </div>
    </div>

    <!-- Controls -->
    <div class="controls">
      <!-- Algorithm -->
      <div class="control-group">
        <span class="control-label">算法</span>
        <div class="algo-btns">
          <button
            v-for="algo in algorithms"
            :key="algo.value"
            class="algo-btn"
            :class="{ active: store.currentAlgorithm === algo.value }"
            @click="store.setAlgorithm(algo.value)"
          >
            {{ algo.label }}
          </button>
        </div>
      </div>

      <!-- Speed -->
      <div class="control-group">
        <span class="control-label">速度</span>
        <div class="speed-control">
          <input
            v-model.number="speed"
            type="range"
            min="10"
            max="1000"
            step="10"
            class="speed-slider"
          />
          <span class="speed-value">{{ speed }}ms</span>
        </div>
      </div>

      <!-- Playback -->
      <div class="control-group">
        <button
          class="play-btn"
          :class="{ running: isRunning, done: store.isDone }"
          :disabled="!store.currentTask"
          @click="handlePlayPause"
        >
          <svg v-if="!isRunning" viewBox="0 0 24 24" fill="currentColor">
            <path d="M8 5v14l11-7z"/>
          </svg>
          <svg v-else viewBox="0 0 24 24" fill="currentColor">
            <path d="M6 4h4v16H6zM14 4h4v16h-4z"/>
          </svg>
          {{ isRunning ? '暂停' : store.isDone ? '重播' : '播放' }}
        </button>
        <button class="reset-btn" :disabled="!store.currentTask" @click="handleReset">
          重置
        </button>
      </div>

      <!-- Stats -->
      <div class="stats">
        <div class="stat">
          <span class="stat-value">{{ store.visitedCount }}</span>
          <span class="stat-label">访问</span>
        </div>
        <div class="stat">
          <span class="stat-value">{{ store.expandedCount }}</span>
          <span class="stat-label">扩展</span>
        </div>
        <div class="stat">
          <span class="stat-value">{{ store.pathLength }}</span>
          <span class="stat-label">路径</span>
        </div>
      </div>
    </div>

    <!-- Map -->
    <div class="map-card">
      <div
        v-if="store.currentTaskDraw && store.currentTaskDraw.length > 0"
        class="map-grid"
        :style="{ gridTemplateColumns: `repeat(${mapW}, minmax(0, 1fr))` }"
      >
        <div
          v-for="i in totalCells"
          :key="`cell-${i - 1}`"
          class="grid-cell"
          :style="{ background: getCellColor(i - 1) }"
        ></div>
      </div>
      <div v-else class="map-placeholder">
        加载中...
      </div>

      <div class="map-legend">
        <div class="legend-item"><span class="legend-color" style="background:#10b981"></span>起点</div>
        <div class="legend-item"><span class="legend-color" style="background:#f43f5e"></span>终点</div>
        <div class="legend-item"><span class="legend-color" style="background:#52525b"></span>墙壁</div>
        <div class="legend-item"><span class="legend-color" style="background:#60a5fa"></span>已访问</div>
        <div class="legend-item"><span class="legend-color" style="background:#a855f7"></span>路径</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.detail-view {
  max-width: 900px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.detail-header {
  display: flex;
  align-items: center;
  gap: 16px;
}

.back-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 10px;
  color: var(--text-secondary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
  flex-shrink: 0;
}

.back-btn:hover {
  border-color: var(--accent-blue);
  color: var(--text-primary);
}

.back-btn svg {
  width: 16px;
  height: 16px;
}

.header-center {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.task-name {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.task-meta {
  font-size: 13px;
  color: var(--text-secondary);
}

.controls {
  display: flex;
  align-items: center;
  gap: 24px;
  flex-wrap: wrap;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  padding: 16px 20px;
}

.control-group {
  display: flex;
  align-items: center;
  gap: 10px;
}

.control-label {
  font-size: 12px;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.algo-btns {
  display: flex;
  gap: 6px;
}

.algo-btn {
  padding: 6px 14px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-primary);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.algo-btn:hover {
  border-color: var(--accent-blue);
}

.algo-btn.active {
  background: rgba(59, 130, 246, 0.1);
  border-color: var(--accent-blue);
  color: var(--accent-blue);
}

.speed-control {
  display: flex;
  align-items: center;
  gap: 10px;
}

.speed-slider {
  width: 100px;
  height: 4px;
  accent-color: var(--accent-blue);
  cursor: pointer;
}

.speed-value {
  font-size: 12px;
  color: var(--text-secondary);
  min-width: 40px;
}

.play-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 20px;
  background: var(--accent-blue);
  border: none;
  border-radius: 10px;
  color: white;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.play-btn:hover:not(:disabled) {
  filter: brightness(1.1);
}

.play-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.play-btn.running {
  background: #f59e0b;
}

.play-btn.done {
  background: var(--accent-green);
}

.play-btn svg {
  width: 14px;
  height: 14px;
}

.reset-btn {
  padding: 10px 16px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 10px;
  color: var(--text-secondary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.reset-btn:hover:not(:disabled) {
  border-color: var(--accent-blue);
  color: var(--text-primary);
}

.reset-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.stats {
  display: flex;
  gap: 16px;
  margin-left: auto;
}

.stat {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
}

.stat-value {
  font-size: 20px;
  font-weight: 700;
  color: var(--accent-blue);
}

.stat-label {
  font-size: 11px;
  color: var(--text-secondary);
}

.map-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  padding: 20px;
  display: flex;
  flex-direction: column;
}

.map-grid {
  display: grid;
  gap: 2px;
  width: 100%;
}

.grid-cell {
  aspect-ratio: 1;
  border-radius: 4px;
  min-width: 14px;
  min-height: 14px;
}

.map-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 300px;
  color: var(--text-secondary);
  font-size: 14px;
}

.map-legend {
  display: flex;
  justify-content: center;
  gap: 20px;
  padding-top: 16px;
  border-top: 1px solid var(--border-color);
  margin-top: 16px;
  flex-wrap: wrap;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--text-secondary);
}

.legend-color {
  width: 14px;
  height: 14px;
  border-radius: 3px;
}
</style>
