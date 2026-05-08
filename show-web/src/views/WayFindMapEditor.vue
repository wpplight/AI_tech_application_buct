<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { wayfindService } from '../api/wayfind'

const router = useRouter()

type CellType = 0 | 1 | 2 | 3

const cellColors: Record<number, string> = {
  0: '#e4e4e7',
  1: '#52525b',
  2: '#10b981',
  3: '#f43f5e',
}

const mapName = ref('新地图')
const mapWidth = ref(20)
const mapHeight = ref(15)
const grid = reactive<CellType[][]>([])
const loading = ref(false)
const isDrawing = ref(false)

const selectedCell = ref<CellType>(1)

const tools: { key: string; label: string; cellType: CellType }[] = [
  { key: 'wall', label: '墙壁', cellType: 1 },
  { key: 'start', label: '起点', cellType: 2 },
  { key: 'end', label: '终点', cellType: 3 },
]

function initGrid(w: number, h: number) {
  grid.length = 0
  for (let y = 0; y < h; y++) {
    const row: CellType[] = []
    for (let x = 0; x < w; x++) {
      row.push(0)
    }
    grid.push(row)
  }
}

function getCellColor(cell: CellType): string {
  return cellColors[cell] || cellColors[0]
}

function paintCell(x: number, y: number) {
  if (y < 0 || y >= grid.length || x < 0 || x >= grid[0].length) return

  const current = grid[y][x]
  const target = selectedCell.value

  if (current === target) {
    grid[y][x] = 0
    return
  }

  if (target === 2) {
    for (let gy = 0; gy < grid.length; gy++) {
      for (let gx = 0; gx < grid[gy].length; gx++) {
        if (grid[gy][gx] === 2) grid[gy][gx] = 0
      }
    }
  }

  if (target === 3) {
    for (let gy = 0; gy < grid.length; gy++) {
      for (let gx = 0; gx < grid[gy].length; gx++) {
        if (grid[gy][gx] === 3) grid[gy][gx] = 0
      }
    }
  }

  grid[y][x] = target
}

function handleCellMouseDown(x: number, y: number) {
  isDrawing.value = true
  paintCell(x, y)
}

function handleCellMouseEnter(x: number, y: number) {
  if (isDrawing.value) {
    paintCell(x, y)
  }
}

function handleMouseUp() {
  isDrawing.value = false
}

async function handleSave() {
  const name = mapName.value.trim() || '新地图'
  loading.value = true
  try {
    await wayfindService.saveMap(name, grid as number[][], mapWidth.value, mapHeight.value)
    router.push('/wayfind/maps')
  } finally {
    loading.value = false
  }
}

async function handleLoad(name: string) {
  loading.value = true
  try {
    const { map } = await wayfindService.loadMap(name)
    mapName.value = name
    mapWidth.value = map.width
    mapHeight.value = map.height
    grid.length = 0
    for (let y = 0; y < map.height; y++) {
      const row: CellType[] = []
      for (let x = 0; x < map.width; x++) {
        row.push(map.grid[y][x] as CellType)
      }
      grid.push(row)
    }
  } finally {
    loading.value = false
  }
}

function handleBack() {
  router.push('/wayfind/maps')
}

const name = router.currentRoute.value.query.name as string | undefined
if (name) {
  handleLoad(name)
} else {
  initGrid(20, 15)
}
</script>

<template>
  <div class="editor-view" @mouseup="handleMouseUp" @mouseleave="handleMouseUp">
    <div class="editor-header">
      <button class="back-btn" @click="handleBack">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M19 12H5M12 19l-7-7 7-7"/>
        </svg>
        返回
      </button>
      <div class="header-center">
        <input v-model="mapName" type="text" class="name-input" placeholder="地图名称" />
      </div>
      <div class="header-actions">
        <button class="action-btn primary" :disabled="loading" @click="handleSave">
          {{ loading ? '保存中...' : '保存' }}
        </button>
      </div>
    </div>

    <div class="brush-bar">
      <button
        v-for="tool in tools"
        :key="tool.key"
        class="brush-btn"
        :class="{ active: selectedCell === tool.cellType }"
        @click="selectedCell = tool.cellType"
      >
        <span class="brush-dot" :style="{ background: cellColors[tool.cellType] }"></span>
        {{ tool.label }}
      </button>
    </div>

    <div class="map-area">
      <div
        class="map-grid"
        :style="{ gridTemplateColumns: `repeat(${mapWidth}, minmax(0, 1fr))` }"
      >
        <template v-for="(row, y) in grid" :key="y">
          <div
            v-for="(cell, x) in row"
            :key="`${x}-${y}`"
            class="grid-cell"
            :style="{ background: getCellColor(cell) }"
            @mousedown.prevent="handleCellMouseDown(x, y)"
            @mouseenter="handleCellMouseEnter(x, y)"
          ></div>
        </template>
      </div>
    </div>

    <div class="map-legend">
      <div class="legend-item"><span class="legend-color" style="background:#10b981"></span>起点 (S)</div>
      <div class="legend-item"><span class="legend-color" style="background:#f43f5e"></span>终点 (E)</div>
      <div class="legend-item"><span class="legend-color" style="background:#52525b"></span>墙壁</div>
      <div class="legend-item"><span class="legend-color" style="background:#e4e4e7; border: 1px solid #d4d4d8;"></span>道路</div>
    </div>
  </div>
</template>

<style scoped>
.editor-view {
  display: flex;
  flex-direction: column;
  max-width: 900px;
  user-select: none;
}

.editor-header {
  display: flex;
  align-items: center;
  gap: 16px;
  padding-bottom: 20px;
  border-bottom: 1px solid var(--border-color);
  margin-bottom: 20px;
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

.back-btn svg { width: 16px; height: 16px; }

.header-center { flex: 1; }

.name-input {
  width: 100%;
  max-width: 300px;
  padding: 8px 14px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  color: var(--text-primary);
  font-size: 14px;
  font-weight: 500;
  text-align: center;
}

.name-input:focus {
  outline: none;
  border-color: var(--accent-blue);
}

.header-actions { display: flex; gap: 8px; flex-shrink: 0; }

.action-btn {
  padding: 8px 16px;
  border-radius: 10px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.action-btn.primary {
  background: var(--accent-blue);
  color: white;
}

.action-btn.primary:hover:not(:disabled) { filter: brightness(1.1); }
.action-btn:disabled { opacity: 0.5; cursor: not-allowed; }

.brush-bar {
  display: flex;
  gap: 8px;
  margin-bottom: 20px;
  flex-wrap: wrap;
}

.brush-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.brush-btn:hover { border-color: var(--accent-blue); }
.brush-btn.active { border-color: var(--accent-blue); background: rgba(59,130,246,0.1); }

.brush-dot { width: 14px; height: 14px; border-radius: 4px; }

.map-area {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  padding: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 400px;
}

.map-grid {
  display: grid;
  gap: 2px;
  width: 100%;
  max-width: 700px;
}

.grid-cell {
  aspect-ratio: 1;
  border-radius: 4px;
  cursor: crosshair;
  min-width: 16px;
  min-height: 16px;
}

.map-legend {
  display: flex;
  justify-content: center;
  gap: 24px;
  padding-top: 16px;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--text-secondary);
}

.legend-color { width: 14px; height: 14px; border-radius: 3px; }
</style>
