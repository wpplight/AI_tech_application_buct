<template>
  <div class="display-view">
    <div class="display-header">
      <button class="btn-back" @click="goBack">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
          <path d="M19 12H5M12 19l-7-7 7-7" stroke="currentColor" stroke-width="2"/>
        </svg>
        Back
      </button>
      
      <h1 class="display-title">{{ display?.name || 'Display' }}</h1>
      
      <div class="header-info">
        <span class="algo-badge" :class="executionStore.algorithm">
          {{ executionStore.algorithm.toUpperCase() }}
        </span>
      </div>
    </div>
    
    <div class="display-layout">
      <!-- Controls Panel -->
      <div class="controls-panel">
        <!-- Algorithm Selection -->
        <div class="control-section">
          <h3 class="section-title">Algorithm</h3>
          <div class="algo-buttons">
            <button 
              v-for="algo in algorithms" 
              :key="algo"
              class="algo-btn"
              :class="{ active: executionStore.algorithm === algo }"
              @click="selectAlgorithm(algo)"
            >
              {{ algo.toUpperCase() }}
            </button>
          </div>
        </div>
        
        <!-- Execution Mode -->


        
        <!-- Playback Controls -->
        <div class="control-section">
          <h3 class="section-title">Playback</h3>
          <div class="playback-controls">
            <button 
              v-if="execution.status !== 'running'"
              class="play-btn"
              @click="startExecution"
            >
              <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                <path d="M8 5v14l11-7z"/>
              </svg>
            </button>
            
            <button 
              v-else
              class="play-btn pause"
              @click="pauseExecution"
            >
              <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/>
              </svg>
            </button>
            
            <button 
              class="control-btn"
              @click="stopExecution"
              :disabled="execution.status === 'idle'"
            >
              <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <path d="M6 6h12v12H6z"/>
              </svg>
            </button>
            
            <button 
              class="control-btn"
              @click="resetExecution"
            >
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                <path d="M17.65 6.35A7.958 7.958 0 0012 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08A5.99 5.99 0 0112 18c-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z" fill="currentColor"/>
              </svg>
            </button>
          </div>
        </div>
        
        <!-- Speed Control -->
        <div class="control-section">
          <h3 class="section-title">Speed: {{ speed }}x</h3>
          <input 
            type="range" 
            min="1" 
            max="20" 
            v-model.number="speed"
            class="speed-slider"
          />
        </div>
        
        <!-- Statistics -->
        <div class="control-section">
          <h3 class="section-title">Statistics</h3>
          <div class="stats-grid">
            <div class="stat-item">
              <div class="stat-label">Visited</div>
              <div class="stat-value">{{ execution.visitedCells.length }}</div>
            </div>
            <div class="stat-item">
              <div class="stat-label">Path</div>
              <div class="stat-value" :class="{ found: execution.found }">
                {{ execution.path.length }}
              </div>
            </div>
            <div class="stat-item full">
              <div class="stat-label">Status</div>
              <div class="stat-value status" :class="execution.status">
                {{ statusText }}
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Grid Display -->
      <div class="grid-section">
        <div class="grid-container">
          <div class="grid-wrapper">
            <div class="display-grid" :style="gridStyle">
              <div 
                v-for="(row, y) in grid.cells" 
                :key="y"
                class="grid-row"
              >
                <div
                  v-for="(cell, x) in row"
                  :key="`${x}-${y}`"
                  class="grid-cell"
                  :class="getCellClasses(cell)"
                >
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <!-- Legend -->
        <div class="legend">
          <div class="legend-item">
            <div class="legend-color start"></div>
            <span>Start</span>
          </div>
          <div class="legend-item">
            <div class="legend-color end"></div>
            <span>End</span>
          </div>
          <div class="legend-item">
            <div class="legend-color wall"></div>
            <span>Wall</span>
          </div>
          <div class="legend-item">
            <div class="legend-color visited"></div>
            <span>Visited</span>
          </div>
          <div class="legend-item">
            <div class="legend-color current"></div>
            <span>Current</span>
          </div>
          <div class="legend-item">
            <div class="legend-color path"></div>
            <span>Path</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useDisplayStore } from '@/stores/display'
import { useMapStore } from '@/stores/map'
import { useExecutionStore } from '@/stores/execution'
import type { AlgorithmType } from '@/types'

const router = useRouter()
const route = useRoute()
const displayStore = useDisplayStore()
const mapStore = useMapStore()
const executionStore = useExecutionStore()

const display = computed(() => displayStore.getDisplayById(route.params.id as string))
const grid = computed(() => executionStore.grid)
const execution = computed(() => executionStore.execution)

const speed = computed({
  get: () => executionStore.speed,
  set: (val) => executionStore.setSpeed(val)
})

const algorithms: AlgorithmType[] = ['bfs', 'dfs', 'astar']

const statusText = computed(() => {
  switch (execution.value.status) {
    case 'idle': return 'Ready'
    case 'running': return 'Running...'
    case 'paused': return 'Paused'
    case 'completed': return execution.value.found ? 'Found!' : 'Not Found'
    default: return ''
  }
})

const gridStyle = computed(() => ({
  gridTemplateColumns: `repeat(${grid.value.width}, 24px)`,
  gridTemplateRows: `repeat(${grid.value.height}, 24px)`
}))

onMounted(() => {
  if (display.value) {
    const map = mapStore.getMapById(display.value.mapId)
    if (map) {
      executionStore.loadMap(map)
      executionStore.setAlgorithm(display.value.algorithm)
      executionStore.setSpeed(display.value.speed)
    }
  } else {
    mapStore.createMap('Default', 20, 15)
    const defaultMap = mapStore.maps[0]
    if (defaultMap) {
      executionStore.loadMap(defaultMap)
    }
  }
})

watch(speed, (newSpeed) => {
  executionStore.setSpeed(newSpeed)
})

function selectAlgorithm(algo: AlgorithmType) {
  executionStore.setAlgorithm(algo)
}

function startExecution() {
  executionStore.startExecution()
}

function pauseExecution() {
  executionStore.pauseExecution()
}

function stopExecution() {
  executionStore.stopExecution()
}

function resetExecution() {
  executionStore.resetExecution()
}

function getCellClasses(cell: any) {
  const classes = ['cell']
  
  switch (cell.type) {
    case 'wall':
      classes.push('cell-wall')
      break
    case 'start':
      classes.push('cell-start')
      break
    case 'end':
      classes.push('cell-end')
      break
    default:
      classes.push('cell-empty')
  }
  
  switch (cell.state) {
    case 'current':
      classes.push('cell-current')
      break
    case 'visited':
      classes.push('cell-visited')
      break
    case 'path':
      classes.push('cell-path')
      break
  }
  
  return classes.join(' ')
}

function goBack() {
  router.push('/')
}
</script>

<style scoped>
.display-view {
  min-height: 100vh;
  padding: 2rem;
}

.display-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  max-width: 1600px;
  margin-left: auto;
  margin-right: auto;
}

.btn-back {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.25rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 0.75rem;
  color: rgba(255, 255, 255, 0.8);
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: inherit;
}

.btn-back:hover {
  background: rgba(255, 255, 255, 0.1);
  color: white;
}

.display-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: white;
}

.header-info {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.algo-badge {
  padding: 0.5rem 1rem;
  border-radius: 0.5rem;
  font-size: 0.875rem;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
}

.algo-badge.bfs {
  background: rgba(59, 130, 246, 0.2);
  color: #3b82f6;
}

.algo-badge.dfs {
  background: rgba(16, 185, 129, 0.2);
  color: #10b981;
}

.algo-badge.astar {
  background: rgba(245, 158, 11, 0.2);
  color: #f59e0b;
}

.display-layout {
  display: grid;
  grid-template-columns: 350px 1fr;
  gap: 2rem;
  max-width: 1600px;
  margin: 0 auto;
}

.controls-panel {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.control-section {
  background: rgba(255, 255, 255, 0.03);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 1rem;
  padding: 1.5rem;
}

.section-title {
  font-size: 0.875rem;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.6);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 1rem;
}

.algo-buttons {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 0.5rem;
}

.algo-btn {
  padding: 0.75rem;
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 0.5rem;
  color: rgba(255, 255, 255, 0.6);
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.875rem;
  font-weight: 600;
  font-family: inherit;
}

.algo-btn:hover {
  background: rgba(255, 255, 255, 0.08);
  color: white;
}

.algo-btn.active {
  border-color: currentColor;
}

.algo-btn.active:nth-child(1) {
  background: rgba(59, 130, 246, 0.1);
  border-color: rgba(59, 130, 246, 0.5);
  color: #3b82f6;
}

.algo-btn.active:nth-child(2) {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.5);
  color: #10b981;
}

.algo-btn.active:nth-child(3) {
  background: rgba(245, 158, 11, 0.1);
  border-color: rgba(245, 158, 11, 0.5);
  color: #f59e0b;
}

.mode-buttons {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.mode-btn {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem;
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 0.75rem;
  color: rgba(255, 255, 255, 0.6);
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: inherit;
}

.mode-btn:hover {
  background: rgba(255, 255, 255, 0.08);
  color: white;
}

.mode-btn.active {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.3);
  color: #10b981;
}

.playback-controls {
  display: flex;
  gap: 0.75rem;
  justify-content: center;
}

.play-btn {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  border: none;
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
}

.play-btn:hover {
  transform: scale(1.05);
  box-shadow: 0 6px 20px rgba(16, 185, 129, 0.4);
}

.play-btn.pause {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
  box-shadow: 0 4px 12px rgba(245, 158, 11, 0.3);
}

.control-btn {
  width: 48px;
  height: 48px;
  border-radius: 0.75rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: rgba(255, 255, 255, 0.6);
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.control-btn:hover:not(:disabled) {
  background: rgba(255, 255, 255, 0.1);
  color: white;
}

.control-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 0.75rem;
}

.stat-item {
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.06);
  border-radius: 0.5rem;
  padding: 0.75rem;
}

.stat-item.full {
  grid-column: span 2;
}

.stat-label {
  font-size: 0.625rem;
  color: rgba(255, 255, 255, 0.5);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 0.25rem;
}

.stat-value {
  font-size: 1.5rem;
  font-weight: 700;
  color: white;
  font-family: 'JetBrains Mono', monospace;
}

.stat-value.found {
  color: #22d3ee;
}

.stat-value.status {
  font-size: 0.875rem;
  padding: 0.5rem;
  text-align: center;
  border-radius: 0.375rem;
}

.stat-value.status.idle {
  background: rgba(255, 255, 255, 0.05);
  color: rgba(255, 255, 255, 0.7);
}

.stat-value.status.running {
  background: rgba(245, 158, 11, 0.2);
  color: #f59e0b;
  animation: pulse 1s ease-in-out infinite;
}

.stat-value.status.paused {
  background: rgba(59, 130, 246, 0.2);
  color: #3b82f6;
}

.stat-value.status.completed {
  background: rgba(34, 197, 94, 0.2);
  color: #22c55e;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.7; }
}

.grid-section {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.grid-container {
  background: rgba(255, 255, 255, 0.02);
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 1.5rem;
  padding: 2rem;
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 600px;
}

.grid-wrapper {
  overflow-x: auto;
}

.display-grid {
  display: grid;
  gap: 1px;
  background: rgba(255, 255, 255, 0.05);
  padding: 2px;
  border-radius: 8px;
  width: fit-content;
}

.grid-row {
  display: contents;
}

.grid-cell {
  width: 24px;
  height: 24px;
  border-radius: 3px;
  transition: all 0.15s ease;
}

.cell-empty {
  background: #1f2937;
}

.cell-wall {
  background: linear-gradient(135deg, #374151 0%, #4b5563 100%);
}

.cell-start {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  box-shadow: 0 0 10px rgba(16, 185, 129, 0.5);
  border: 2px solid #34d399;
}

.cell-end {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  box-shadow: 0 0 10px rgba(239, 68, 68, 0.5);
  border: 2px solid #f87171;
}

.cell-current {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%) !important;
  box-shadow: 0 0 10px rgba(245, 158, 11, 0.5);
  transform: scale(1.15);
  animation: pulse 0.5s ease-in-out infinite;
}

.cell-visited {
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
}

.cell-path {
  background: linear-gradient(135deg, #22d3ee 0%, #06b6d4 100%);
  box-shadow: 0 0 8px rgba(34, 211, 238, 0.4);
}

.legend {
  display: flex;
  justify-content: center;
  gap: 2rem;
  padding: 1rem;
  background: rgba(255, 255, 255, 0.02);
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 1rem;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.75rem;
  color: rgba(255, 255, 255, 0.6);
}

.legend-color {
  width: 16px;
  height: 16px;
  border-radius: 3px;
}

.legend-color.start {
  background: #10b981;
}

.legend-color.end {
  background: #ef4444;
}

.legend-color.wall {
  background: #4b5563;
}

.legend-color.visited {
  background: #6366f1;
}

.legend-color.current {
  background: #f59e0b;
}

.legend-color.path {
  background: #22d3ee;
}

@media (max-width: 1024px) {
  .display-layout {
    grid-template-columns: 1fr;
  }
  
  .controls-panel {
    order: 2;
  }
  
  .grid-section {
    order: 1;
  }
}
</style>
