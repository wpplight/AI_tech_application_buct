<template>
  <div class="algorithm-selector">
    <h3 class="title">Algorithm</h3>
    
    <div class="algorithm-list">
      <!-- BFS -->
      <div 
        class="algorithm-card"
        :class="{ 'active': algorithm === 'bfs', 'border-blue': algorithm === 'bfs' }"
        @click="selectAlgorithm('bfs')"
      >
        <div class="algorithm-info">
          <div 
            class="indicator"
            :class="{ 'active': algorithm === 'bfs' }"
            style="background: #3b82f6;"
          ></div>
          <div class="algorithm-details">
            <div class="algorithm-name">BFS</div>
            <div class="algorithm-desc">Breadth-First Search</div>
          </div>
        </div>
      </div>
      
      <!-- DFS -->
      <div 
        class="algorithm-card"
        :class="{ 'active': algorithm === 'dfs', 'border-emerald': algorithm === 'dfs' }"
        @click="selectAlgorithm('dfs')"
      >
        <div class="algorithm-info">
          <div 
            class="indicator"
            :class="{ 'active': algorithm === 'dfs' }"
            style="background: #10b981;"
          ></div>
          <div class="algorithm-details">
            <div class="algorithm-name">DFS</div>
            <div class="algorithm-desc">Depth-First Search</div>
          </div>
        </div>
      </div>
      
      <!-- A* -->
      <div 
        class="algorithm-card"
        :class="{ 'active': algorithm === 'astar', 'border-amber': algorithm === 'astar' }"
        @click="selectAlgorithm('astar')"
      >
        <div class="algorithm-info">
          <div 
            class="indicator"
            :class="{ 'active': algorithm === 'astar' }"
            style="background: #f59e0b;"
          ></div>
          <div class="algorithm-details">
            <div class="algorithm-name">A*</div>
            <div class="algorithm-desc">A* Heuristic Search</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useMazeStore } from '@/stores/maze'
import type { AlgorithmType } from '@/types'

const store = useMazeStore()
const algorithm = computed(() => store.algorithm)

function selectAlgorithm(algo: AlgorithmType) {
  store.setAlgorithm(algo)
}
</script>

<style scoped>
.algorithm-selector {
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

.algorithm-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.algorithm-card {
  position: relative;
  overflow: hidden;
  border-radius: 12px;
  padding: 1rem;
  cursor: pointer;
  transition: all 0.2s ease;
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.algorithm-card:hover {
  background: rgba(255, 255, 255, 0.08);
  border-color: rgba(255, 255, 255, 0.15);
  transform: translateX(4px);
}

.algorithm-card:active {
  transform: scale(0.98);
}

.algorithm-card.active {
  background: rgba(255, 255, 255, 0.1);
}

.algorithm-card.border-blue {
  border-color: rgba(59, 130, 246, 0.5);
}

.algorithm-card.border-emerald {
  border-color: rgba(16, 185, 129, 0.5);
}

.algorithm-card.border-amber {
  border-color: rgba(245, 158, 11, 0.5);
}

.algorithm-info {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.indicator {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  flex-shrink: 0;
  transition: all 0.3s ease;
}

.indicator.active {
  box-shadow: 0 0 12px currentColor;
  animation: pulse 1.5s ease-in-out infinite;
}

.algorithm-details {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.algorithm-name {
  font-weight: 600;
  color: white;
  font-size: 1rem;
}

.algorithm-desc {
  font-size: 0.75rem;
  color: rgba(255, 255, 255, 0.6);
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
    transform: scale(1);
  }
  50% {
    opacity: 0.8;
    transform: scale(1.1);
  }
}
</style>
