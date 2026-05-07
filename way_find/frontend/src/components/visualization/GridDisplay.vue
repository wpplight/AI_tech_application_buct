<template>
  <div class="grid-display">
    <div class="grid-container" :style="gridStyle">
      <div 
        v-for="(row, y) in gridCells" 
        :key="y"
        class="grid-row"
      >
        <div
          v-for="(cell, x) in row"
          :key="`${x}-${y}`"
          class="grid-cell"
          :class="getCellClasses(cell)"
          @mousedown="handleCellClick(cell.point)"
          @mouseenter="handleCellHover(cell.point)"
        >
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useMazeStore } from '@/stores/maze'

const store = useMazeStore()

const gridCells = computed(() => {
  return store.grid.cells
})

const gridStyle = computed(() => ({
  gridTemplateColumns: `repeat(${store.grid.width}, 24px)`,
  gridTemplateRows: `repeat(${store.grid.height}, 24px)`
}))

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
    case 'queue':
      classes.push('cell-queue')
      break
    case 'stack':
      classes.push('cell-stack')
      break
  }
  
  return classes.join(' ')
}

function handleCellClick(point: { x: number; y: number }) {
  const cell = store.grid.cells[point.y]?.[point.x]
  if (!cell) return
  
  if (cell.type === 'start' || cell.type === 'end') {
    return
  }
  
  const newType = cell.type === 'wall' ? 'empty' : 'wall'
  store.updateCellType(point, newType)
}

function handleCellHover(point: { x: number; y: number }) {
  if (!store.animation.isRunning) {
    handleCellClick(point)
  }
}
</script>

<style scoped>
.grid-display {
  padding: 1rem;
  overflow-x: auto;
}

.grid-container {
  display: grid;
  gap: 1px;
  background: rgba(255, 255, 255, 0.05);
  padding: 2px;
  border-radius: 8px;
  width: fit-content;
  margin: 0 auto;
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

.cell-wall {
  background: linear-gradient(135deg, #374151 0%, #4b5563 100%);
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.5);
}

.cell-empty {
  background: #1f2937;
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

.cell-queue {
  background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
}

.cell-stack {
  background: linear-gradient(135deg, #ec4899 0%, #db2777 100%);
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.7;
  }
}
</style>
