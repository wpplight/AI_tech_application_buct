<template>
  <div class="stats-panel">
    <h3 class="title">Statistics</h3>
    
    <div class="stats-grid">
      <!-- Steps -->
      <div class="stat-item">
        <div class="stat-label">Steps</div>
        <div class="stat-value">{{ stats.steps }}</div>
      </div>
      
      <!-- Visited Nodes -->
      <div class="stat-item">
        <div class="stat-label">Visited</div>
        <div class="stat-value">{{ stats.visitedNodes }}</div>
      </div>
      
      <!-- Path Length -->
      <div class="stat-item">
        <div class="stat-label">Path</div>
        <div class="stat-value" :class="pathClass">
          {{ stats.pathLength }}
        </div>
      </div>
      
      <!-- Status -->
      <div class="stat-item status-item">
        <div class="stat-label">Status</div>
        <div class="stat-value status-value" :class="statusClass">
          {{ statusText }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useMazeStore } from '@/stores/maze'

const store = useMazeStore()
const stats = computed(() => store.stats)

const statusText = computed(() => {
  if (stats.value.found) return 'Found ✓'
  if (store.animation.isRunning) return 'Running...'
  return 'Ready'
})

const statusClass = computed(() => {
  if (stats.value.found) return 'status-found'
  if (store.animation.isRunning) return 'status-running'
  return 'status-ready'
})

const pathClass = computed(() => {
  return stats.value.pathLength > 0 ? 'path-found' : ''
})
</script>

<style scoped>
.stats-panel {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.title {
  font-size: 1.125rem;
  font-weight: 600;
  color: white;
  margin-bottom: 0.5rem;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 0.75rem;
}

.stat-item {
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.06);
  border-radius: 10px;
  padding: 0.875rem;
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
  transition: all 0.2s ease;
}

.stat-item:hover {
  background: rgba(255, 255, 255, 0.06);
  transform: translateY(-1px);
}

.stat-label {
  font-size: 0.75rem;
  color: rgba(255, 255, 255, 0.5);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  font-weight: 500;
}

.stat-value {
  font-size: 1.5rem;
  font-weight: 700;
  color: white;
  font-family: 'JetBrains Mono', monospace;
  line-height: 1;
}

.stat-value.path-found {
  color: #22d3ee;
}

.status-item {
  grid-column: span 2;
}

.status-value {
  font-size: 1rem;
  font-weight: 600;
  padding: 0.5rem 0.75rem;
  border-radius: 8px;
  text-align: center;
}

.status-ready {
  background: rgba(255, 255, 255, 0.05);
  color: rgba(255, 255, 255, 0.7);
}

.status-running {
  background: linear-gradient(135deg, rgba(245, 158, 11, 0.2) 0%, rgba(217, 119, 6, 0.2) 100%);
  color: #f59e0b;
  animation: pulse 1s ease-in-out infinite;
}

.status-found {
  background: linear-gradient(135deg, rgba(34, 197, 94, 0.2) 0%, rgba(22, 163, 74, 0.2) 100%);
  color: #22c55e;
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
