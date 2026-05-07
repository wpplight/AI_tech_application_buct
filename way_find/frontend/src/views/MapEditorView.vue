<template>
  <div class="map-editor">
    <div class="editor-header">
      <button class="btn-back" @click="goBack">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
          <path d="M19 12H5M12 19l-7-7 7-7" stroke="currentColor" stroke-width="2"/>
        </svg>
        Back
      </button>
      
      <h1 class="editor-title">{{ isEditing ? 'Edit Map' : 'Create New Map' }}</h1>
      
      <button class="btn-save" @click="saveMap">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
          <path d="M19 21H5a2 2 0 01-2-2V5a2 2 0 012-2h11l5 5v11a2 2 0 01-2 2z" stroke="currentColor" stroke-width="2"/>
          <path d="M17 21v-8H7v8M7 3v5h8" stroke="currentColor" stroke-width="2"/>
        </svg>
        Save Map
      </button>
    </div>
    
    <div class="editor-layout">
      <!-- Toolbar -->
      <div class="toolbar">
        <div class="tool-section">
          <h3 class="tool-title">Tools</h3>
          <div class="tool-buttons">
            <button 
              class="tool-btn"
              :class="{ active: currentTool === 'wall' }"
              @click="currentTool = 'wall'"
            >
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                <rect x="3" y="3" width="18" height="18" rx="2" fill="currentColor"/>
              </svg>
              Wall
            </button>
            
            <button 
              class="tool-btn"
              :class="{ active: currentTool === 'eraser' }"
              @click="currentTool = 'eraser'"
            >
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                <path d="M20 20H7L3 16c-.6-.6-.6-1.5 0-2.1L13.1 4c.6-.6 1.5-.6 2.1 0l5.7 5.7c.6.6.6 1.5 0 2.1L15 18" stroke="currentColor" stroke-width="2"/>
              </svg>
              Eraser
            </button>
            
            <button 
              class="tool-btn"
              :class="{ active: currentTool === 'start' }"
              @click="currentTool = 'start'"
            >
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                <circle cx="12" cy="12" r="8" fill="#10b981"/>
              </svg>
              Start
            </button>
            
            <button 
              class="tool-btn"
              :class="{ active: currentTool === 'end' }"
              @click="currentTool = 'end'"
            >
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                <circle cx="12" cy="12" r="8" fill="#ef4444"/>
              </svg>
              End
            </button>
          </div>
        </div>
        
        <div class="tool-section">
          <h3 class="tool-title">Map Settings</h3>
          <div class="settings-form">
            <div class="form-group">
              <label>Name</label>
              <input 
                v-model="mapName" 
                type="text" 
                class="form-input"
                placeholder="My Awesome Map"
              />
            </div>
            
            <div class="form-row">
              <div class="form-group">
                <label>Width</label>
                <input 
                  v-model.number="mapWidth" 
                  type="number" 
                  min="5" 
                  max="50" 
                  class="form-input"
                />
              </div>
              
              <div class="form-group">
                <label>Height</label>
                <input 
                  v-model.number="mapHeight" 
                  type="number" 
                  min="5" 
                  max="50" 
                  class="form-input"
                />
              </div>
            </div>
            
            <button class="btn-reset" @click="resetMap">
              Reset Map
            </button>
          </div>
        </div>
        
        <div class="tool-section">
          <h3 class="tool-title">Statistics</h3>
          <div class="stats-grid">
            <div class="stat-item">
              <div class="stat-label">Size</div>
              <div class="stat-value">{{ mapWidth }}×{{ mapHeight }}</div>
            </div>
            <div class="stat-item">
              <div class="stat-label">Walls</div>
              <div class="stat-value">{{ wallCount }}</div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Grid -->
      <div class="grid-container">
        <div class="grid-wrapper">
          <div class="editor-grid" :style="gridStyle">
            <div 
              v-for="(row, y) in editorGrid" 
              :key="y"
              class="grid-row"
            >
              <div
                v-for="(cell, x) in row"
                :key="`${x}-${y}`"
                class="grid-cell"
                :class="getCellClasses(x, y)"
                @mousedown="handleCellClick(x, y)"
                @mouseenter="handleCellHover(x, y)"
              >
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useMapStore, type MapData } from '@/stores/map'
import { wailsService } from '@/composables/wails'
import type { Point } from '@/types'

const router = useRouter()
const route = useRoute()
const mapStore = useMapStore()

const mapName = ref('New Map')
const mapWidth = ref(20)
const mapHeight = ref(15)
const currentTool = ref<'wall' | 'eraser' | 'start' | 'end'>('wall')
const editorGrid = ref<any[][]>([])
const startPoint = ref<Point>({ x: 1, y: 1 })
const endPoint = ref<Point>({ x: 18, y: 13 })

const isEditing = computed(() => route.params.id !== undefined)

const wallCount = computed(() => {
  let count = 0
  for (let y = 0; y < editorGrid.value.length; y++) {
    const row = editorGrid.value[y]
    if (row) {
      for (let x = 0; x < row.length; x++) {
        if (row[x] === 'wall') count++
      }
    }
  }
  return count
})

const gridStyle = computed(() => ({
  gridTemplateColumns: `repeat(${mapWidth.value}, 24px)`,
  gridTemplateRows: `repeat(${mapHeight.value}, 24px)`
}))

onMounted(() => {
  if (isEditing.value) {
    const map = mapStore.getMapById(route.params.id as string)
    if (map) {
      mapName.value = map.name
      mapWidth.value = map.width
      mapHeight.value = map.height
      startPoint.value = { ...map.startPoint }
      endPoint.value = { ...map.endPoint }
    }
  }
  initializeGrid()
})

function initializeGrid() {
  editorGrid.value = []
  for (let y = 0; y < mapHeight.value; y++) {
    const row: string[] = []
    for (let x = 0; x < mapWidth.value; x++) {
      let type = 'empty'
      
      if (x === startPoint.value.x && y === startPoint.value.y) {
        type = 'start'
      } else if (x === endPoint.value.x && y === endPoint.value.y) {
        type = 'end'
      } else if (isEditing.value) {
        const map = mapStore.getMapById(route.params.id as string)
        if (map && map.walls.some(w => w.x === x && w.y === y)) {
          type = 'wall'
        }
      }
      
      row.push(type)
    }
    editorGrid.value.push(row)
  }
}

function getCellClasses(x: number, y: number) {
  const type = editorGrid.value[y]?.[x]
  return `cell-${type}`
}

function handleCellClick(x: number, y: number) {
  const type = editorGrid.value[y]?.[x]
  const row = editorGrid.value[y]
  
  if (!row) return
  
  if (currentTool.value === 'wall') {
    if (type !== 'start' && type !== 'end') {
      row[x] = 'wall'
    }
  } else if (currentTool.value === 'eraser') {
    if (type !== 'start' && type !== 'end') {
      row[x] = 'empty'
    }
  } else if (currentTool.value === 'start') {
    if (type !== 'end') {
      const startRow = editorGrid.value[startPoint.value.y]
      if (startRow) startRow[startPoint.value.x] = 'empty'
      row[x] = 'start'
      startPoint.value = { x, y }
    }
  } else if (currentTool.value === 'end') {
    if (type !== 'start') {
      const endRow = editorGrid.value[endPoint.value.y]
      if (endRow) endRow[endPoint.value.x] = 'empty'
      row[x] = 'end'
      endPoint.value = { x, y }
    }
  }
}

function handleCellHover(x: number, y: number) {
  if (isDrawing.value) {
    handleCellClick(x, y)
  }
}

const isDrawing = ref(false)

onMounted(() => {
  document.addEventListener('mousedown', () => isDrawing.value = true)
  document.addEventListener('mouseup', () => isDrawing.value = false)
})

watch([mapWidth, mapHeight], () => {
  resizeGrid()
})

function resizeGrid() {
  const oldGrid = editorGrid.value
  const oldHeight = oldGrid.length
  const firstRow = oldGrid[0]
  const oldWidth = oldHeight > 0 && firstRow ? firstRow.length : 0
  
  editorGrid.value = []
  for (let y = 0; y < mapHeight.value; y++) {
    const row: string[] = []
    for (let x = 0; x < mapWidth.value; x++) {
      const oldRow = oldGrid[y]
      if (y < oldHeight && x < oldWidth && oldRow) {
        row.push(oldRow[x] ?? 'empty')
      } else {
        row.push('empty')
      }
    }
    editorGrid.value.push(row)
  }
  
  if (startPoint.value.x >= mapWidth.value || startPoint.value.y >= mapHeight.value) {
    startPoint.value = { x: 1, y: 1 }
  }
  if (endPoint.value.x >= mapWidth.value || endPoint.value.y >= mapHeight.value) {
    endPoint.value = { x: mapWidth.value - 2, y: mapHeight.value - 2 }
  }
}

function resetMap() {
  initializeGrid()
}

async function saveMap() {
  const walls: Point[] = []
  for (let y = 0; y < editorGrid.value.length; y++) {
    const row = editorGrid.value[y]
    if (row) {
      for (let x = 0; x < row.length; x++) {
        if (row[x] === 'wall') {
          walls.push({ x, y })
        }
      }
    }
  }
  
  const mapData = {
    name: mapName.value,
    width: mapWidth.value,
    height: mapHeight.value,
    startPoint: startPoint.value,
    endPoint: endPoint.value,
    walls,
    grid: editorGrid.value.map(row => row.map(cell => {
      if (cell === 'wall') return 1
      if (cell === 'start') return 2
      if (cell === 'end') return 3
      return 0
    }))
  }
  
  await wailsService.LoadMap(mapData)
  
  const name = mapName.value || 'Untitled'
  await wailsService.SaveMap(name)
  
  if (isEditing.value) {
    mapStore.updateMap(route.params.id as string, {
      name: mapName.value,
      width: mapWidth.value,
      height: mapHeight.value,
      startPoint: startPoint.value,
      endPoint: endPoint.value,
      walls
    })
  } else {
    mapStore.createMap(mapName.value, mapWidth.value, mapHeight.value)
  }
  
  router.push('/')
}

function goBack() {
  router.push('/')
}
</script>

<style scoped>
.map-editor {
  min-height: 100vh;
  padding: 2rem;
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  max-width: 1400px;
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

.editor-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: white;
}

.btn-save {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  border: none;
  border-radius: 0.75rem;
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: inherit;
  font-weight: 600;
}

.btn-save:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
}

.editor-layout {
  display: grid;
  grid-template-columns: 300px 1fr;
  gap: 2rem;
  max-width: 1400px;
  margin: 0 auto;
}

.toolbar {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.tool-section {
  background: rgba(255, 255, 255, 0.03);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 1rem;
  padding: 1.5rem;
}

.tool-title {
  font-size: 0.875rem;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.6);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 1rem;
}

.tool-buttons {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 0.5rem;
}

.tool-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  padding: 1rem;
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 0.75rem;
  color: rgba(255, 255, 255, 0.6);
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.75rem;
  font-family: inherit;
}

.tool-btn:hover {
  background: rgba(255, 255, 255, 0.08);
  color: white;
}

.tool-btn.active {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.3);
  color: #10b981;
}

.settings-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group label {
  font-size: 0.75rem;
  color: rgba(255, 255, 255, 0.6);
  font-weight: 500;
}

.form-input {
  padding: 0.75rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 0.5rem;
  color: white;
  font-family: inherit;
  font-size: 0.875rem;
}

.form-input:focus {
  outline: none;
  border-color: rgba(16, 185, 129, 0.5);
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.75rem;
}

.btn-reset {
  padding: 0.75rem;
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.2);
  border-radius: 0.5rem;
  color: #ef4444;
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: inherit;
  font-weight: 500;
}

.btn-reset:hover {
  background: rgba(239, 68, 68, 0.2);
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

.stat-label {
  font-size: 0.625rem;
  color: rgba(255, 255, 255, 0.5);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 0.25rem;
}

.stat-value {
  font-size: 1.25rem;
  font-weight: 700;
  color: white;
  font-family: 'JetBrains Mono', monospace;
}

.grid-container {
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.02);
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 1.5rem;
  padding: 2rem;
  min-height: 600px;
}

.grid-wrapper {
  overflow-x: auto;
}

.editor-grid {
  display: grid;
  gap: 1px;
  background: rgba(255, 255, 255, 0.05);
  padding: 2px;
  border-radius: 8px;
}

.grid-row {
  display: contents;
}

.grid-cell {
  width: 24px;
  height: 24px;
  border-radius: 3px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.grid-cell:hover {
  transform: scale(1.1);
  z-index: 10;
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

@media (max-width: 1024px) {
  .editor-layout {
    grid-template-columns: 1fr;
  }
  
  .toolbar {
    order: 2;
  }
  
  .grid-container {
    order: 1;
  }
}
</style>
