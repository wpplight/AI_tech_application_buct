<template>
  <div class="display-setup">
    <div class="setup-header">
      <button class="btn-back" @click="goBack">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
          <path d="M19 12H5M12 19l-7-7 7-7" stroke="currentColor" stroke-width="2"/>
        </svg>
        Back
      </button>
      
      <h1 class="setup-title">Create New Display</h1>
    </div>
    
    <div class="setup-form">
      <!-- Name -->
      <div class="form-section">
        <h3 class="section-title">Display Name</h3>
        <input 
          v-model="displayName" 
          type="text" 
          class="form-input"
          placeholder="My Awesome Display"
        />
      </div>
      
      <!-- Algorithm Selection -->
      <div class="form-section">
        <h3 class="section-title">Algorithm</h3>
        <div class="algo-options">
          <button 
            v-for="algo in algorithms" 
            :key="algo.id"
            class="algo-option"
            :class="{ active: selectedAlgorithm === algo.id }"
            @click="selectedAlgorithm = algo.id"
          >
            <div class="algo-icon" :style="{ background: algo.color }">
              {{ algo.label.charAt(0) }}
            </div>
            <div class="algo-info">
              <div class="algo-name">{{ algo.label }}</div>
              <div class="algo-desc">{{ algo.desc }}</div>
            </div>
          </button>
        </div>
      </div>
      
      <!-- Map Selection -->
      <div class="form-section">
        <h3 class="section-title">Select Map</h3>
        <div v-if="maps.length === 0" class="no-maps">
          <p>No maps available. Create a map first.</p>
          <button class="btn-secondary" @click="createMap">Create Map</button>
        </div>
        <div v-else class="map-options">
          <button 
            v-for="map in maps" 
            :key="map.id"
            class="map-option"
            :class="{ active: selectedMapId === map.id }"
            @click="selectedMapId = map.id"
          >
            <div class="map-preview-small">
              <div 
                v-for="(cell, i) in getPreviewCells(map)" 
                :key="i"
                class="preview-cell"
                :class="cell"
              ></div>
            </div>
            <div class="map-info">
              <div class="map-name">{{ map.name }}</div>
              <div class="map-meta">{{ map.width }}×{{ map.height }}</div>
            </div>
          </button>
        </div>
      </div>
      
      <!-- Execution Mode -->
      <div class="form-section">
        <h3 class="section-title">Execution Mode</h3>
        <div class="mode-options">
          <button 
            class="mode-option"
            :class="{ active: executionMode === 'step' }"
            @click="executionMode = 'step'"
          >
            <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
              <path d="M5 4l10 8-10 8V4z" stroke="currentColor" stroke-width="2"/>
              <path d="M19 5v14" stroke="currentColor" stroke-width="2"/>
            </svg>
            <div class="mode-name">Step by Step</div>
            <div class="mode-desc">Control execution manually</div>
          </button>
          
          <button 
            class="mode-option"
            :class="{ active: executionMode === 'instant' }"
            @click="executionMode = 'instant'"
          >
            <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
              <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" stroke="currentColor" stroke-width="2"/>
            </svg>
            <div class="mode-name">Instant</div>
            <div class="mode-desc">Show final result immediately</div>
          </button>
        </div>
      </div>
      
      <!-- Speed -->
      <div class="form-section">
        <h3 class="section-title">Speed: {{ speed }}x</h3>
        <input 
          type="range" 
          min="1" 
          max="10" 
          v-model.number="speed"
          class="speed-slider"
        />
      </div>
      
      <!-- Create Button -->
      <button 
        class="btn-create"
        @click="createDisplay"
        :disabled="!canCreate"
      >
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
          <path d="M5 3l14 9-14 9V3z" fill="currentColor"/>
        </svg>
        Create Display
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useDisplayStore } from '@/stores/display'
import { useMapStore } from '@/stores/map'
import type { AlgorithmType } from '@/types'

const router = useRouter()
const displayStore = useDisplayStore()
const mapStore = useMapStore()

const displayName = ref('New Display')
const selectedAlgorithm = ref<AlgorithmType>('bfs')
const selectedMapId = ref('')
const executionMode = ref<'step' | 'instant'>('instant')
const speed = ref(1)

const maps = computed(() => mapStore.maps)

const algorithms = [
  { id: 'bfs' as AlgorithmType, label: 'BFS', desc: 'Breadth-First Search', color: '#3b82f6' },
  { id: 'dfs' as AlgorithmType, label: 'DFS', desc: 'Depth-First Search', color: '#10b981' },
  { id: 'astar' as AlgorithmType, label: 'A*', desc: 'A* Heuristic Search', color: '#f59e0b' }
]

const canCreate = computed(() => {
  return displayName.value.trim() && selectedMapId.value
})

if (maps.value.length > 0) {
  selectedMapId.value = maps.value[0]!.id
}

function goBack() {
  router.push('/')
}

function createMap() {
  router.push('/maps/new')
}

function createDisplay() {
  if (!canCreate.value) return
  
  const newDisplay = displayStore.createDisplay(
    displayName.value,
    selectedAlgorithm.value,
    selectedMapId.value,
    executionMode.value,
    speed.value
  )
  
  router.push(`/displays/${newDisplay.id}`)
}

function getPreviewCells(map: any) {
  const cells: string[] = []
  const gridSize = 10
  
  for (let i = 0; i < gridSize * gridSize; i++) {
    const x = i % gridSize
    const y = Math.floor(i / gridSize)
    
    const scaleX = map.width / gridSize
    const scaleY = map.height / gridSize
    
    const mapX = Math.floor(x * scaleX)
    const mapY = Math.floor(y * scaleY)
    
    if (mapX === map.startPoint.x && mapY === map.startPoint.y) {
      cells.push('start')
    } else if (mapX === map.endPoint.x && mapY === map.endPoint.y) {
      cells.push('end')
    } else if (map.walls.some((w: any) => w.x === mapX && w.y === mapY)) {
      cells.push('wall')
    } else {
      cells.push('empty')
    }
  }
  
  return cells
}
</script>

<style scoped>
.display-setup {
  min-height: 100vh;
  padding: 2rem;
  max-width: 800px;
  margin: 0 auto;
}

.setup-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 3rem;
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

.setup-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: white;
}

.setup-form {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.form-section {
  background: rgba(255, 255, 255, 0.03);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 1rem;
  padding: 1.5rem;
}

.section-title {
  font-size: 1rem;
  font-weight: 600;
  color: white;
  margin-bottom: 1rem;
}

.form-input {
  width: 100%;
  padding: 1rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 0.75rem;
  color: white;
  font-family: inherit;
  font-size: 1rem;
}

.form-input:focus {
  outline: none;
  border-color: rgba(16, 185, 129, 0.5);
}

.algo-options {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.algo-option {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 0.75rem;
  cursor: pointer;
  transition: all 0.2s ease;
  text-align: left;
  width: 100%;
}

.algo-option:hover {
  background: rgba(255, 255, 255, 0.08);
  border-color: rgba(255, 255, 255, 0.15);
}

.algo-option.active {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.3);
}

.algo-icon {
  width: 48px;
  height: 48px;
  border-radius: 0.75rem;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 1.25rem;
  font-weight: 700;
  flex-shrink: 0;
}

.algo-info {
  flex: 1;
}

.algo-name {
  font-size: 1rem;
  font-weight: 600;
  color: white;
  margin-bottom: 0.25rem;
}

.algo-desc {
  font-size: 0.875rem;
  color: rgba(255, 255, 255, 0.6);
}

.no-maps {
  text-align: center;
  padding: 2rem;
  color: rgba(255, 255, 255, 0.6);
}

.no-maps p {
  margin-bottom: 1rem;
}

.btn-secondary {
  padding: 0.75rem 1.5rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 0.75rem;
  color: white;
  cursor: pointer;
  font-family: inherit;
  font-weight: 500;
}

.btn-secondary:hover {
  background: rgba(255, 255, 255, 0.1);
}

.map-options {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 0.75rem;
}

.map-option {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 0.75rem;
  cursor: pointer;
  transition: all 0.2s ease;
  text-align: left;
  width: 100%;
}

.map-option:hover {
  background: rgba(255, 255, 255, 0.08);
  border-color: rgba(255, 255, 255, 0.15);
}

.map-option.active {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.3);
}

.map-preview-small {
  display: grid;
  grid-template-columns: repeat(10, 3px);
  gap: 1px;
  flex-shrink: 0;
}

.preview-cell {
  width: 3px;
  height: 3px;
  border-radius: 0.5px;
}

.preview-cell.empty {
  background: #1f2937;
}

.preview-cell.wall {
  background: #4b5563;
}

.preview-cell.start {
  background: #10b981;
}

.preview-cell.end {
  background: #ef4444;
}

.map-info {
  flex: 1;
}

.map-name {
  font-size: 0.875rem;
  font-weight: 600;
  color: white;
  margin-bottom: 0.25rem;
}

.map-meta {
  font-size: 0.75rem;
  color: rgba(255, 255, 255, 0.5);
}

.mode-options {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
}

.mode-option {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.75rem;
  padding: 2rem;
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 1rem;
  cursor: pointer;
  transition: all 0.2s ease;
  text-align: center;
  color: rgba(255, 255, 255, 0.6);
}

.mode-option:hover {
  background: rgba(255, 255, 255, 0.08);
  border-color: rgba(255, 255, 255, 0.15);
  color: white;
}

.mode-option.active {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.3);
  color: #10b981;
}

.mode-name {
  font-size: 1rem;
  font-weight: 600;
  color: inherit;
}

.mode-desc {
  font-size: 0.75rem;
  opacity: 0.7;
}

.speed-slider {
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: rgba(255, 255, 255, 0.1);
  outline: none;
  -webkit-appearance: none;
}

.speed-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  cursor: pointer;
  box-shadow: 0 2px 8px rgba(16, 185, 129, 0.3);
}

.btn-create {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.75rem;
  padding: 1rem 2rem;
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  border: none;
  border-radius: 0.75rem;
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: inherit;
  font-size: 1rem;
  font-weight: 600;
}

.btn-create:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
}

.btn-create:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
