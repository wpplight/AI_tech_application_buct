<template>
  <div class="displays-list-view">
    <div class="view-header">
      <button class="btn-back" @click="goBack">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
          <path d="M19 12H5M12 19l-7-7 7-7" stroke="currentColor" stroke-width="2"/>
        </svg>
        Back
      </button>
      
      <h1 class="view-title">My Displays</h1>
      
      <button class="btn-create" @click="createNewDisplay">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
          <path d="M12 5v14M5 12h14" stroke="currentColor" stroke-width="2"/>
        </svg>
        New Display
      </button>
    </div>
    
    <div class="content">
      <div v-if="displays.length === 0" class="empty-state">
        <div class="empty-icon">
          <svg width="64" height="64" viewBox="0 0 24 24" fill="none">
            <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
            <path d="M12 6v6l4 2" stroke="currentColor" stroke-width="2"/>
          </svg>
        </div>
        <h2>No Displays Yet</h2>
        <p>Create a display to visualize pathfinding algorithms</p>
        <button class="btn-primary" @click="createNewDisplay">Create Display</button>
      </div>
      
      <div v-else class="displays-grid">
        <div 
          v-for="display in displays" 
          :key="display.id" 
          class="display-card"
        >
          <div class="display-header">
            <h3>{{ display.name }}</h3>
            <span class="algo-badge" :class="display.algorithm">
              {{ display.algorithm.toUpperCase() }}
            </span>
          </div>
          
          <div class="display-info">
            <div class="info-row">
              <span class="info-label">Map:</span>
              <span class="info-value">{{ getMapName(display.mapId) }}</span>
            </div>
            <div class="info-row">
              <span class="info-label">Mode:</span>
              <span class="info-value">{{ display.executionMode === 'step' ? 'Step by Step' : 'Instant' }}</span>
            </div>
            <div class="info-row">
              <span class="info-label">Speed:</span>
              <span class="info-value">{{ display.speed }}x</span>
            </div>
          </div>
          
          <div class="display-actions">
            <button class="btn-action primary" @click="viewDisplay(display.id)">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                <path d="M5 3l14 9-14 9V3z" fill="currentColor"/>
              </svg>
              View
            </button>
            <button class="btn-action danger" @click="deleteDisplay(display.id)">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                <path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2" stroke="currentColor" stroke-width="2"/>
              </svg>
              Delete
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useDisplayStore } from '@/stores/display'
import { useMapStore } from '@/stores/map'

const router = useRouter()
const displayStore = useDisplayStore()
const mapStore = useMapStore()

const displays = computed(() => displayStore.displays)

function goBack() {
  router.push('/')
}

function createNewDisplay() {
  router.push('/displays/new')
}

function viewDisplay(id: string) {
  router.push(`/displays/${id}`)
}

function deleteDisplay(id: string) {
  if (confirm('Are you sure you want to delete this display?')) {
    displayStore.deleteDisplay(id)
  }
}

function getMapName(mapId: string) {
  const map = mapStore.getMapById(mapId)
  return map ? map.name : 'Unknown Map'
}
</script>

<style scoped>
.displays-list-view {
  min-height: 100vh;
  padding: 2rem;
}

.view-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  max-width: 1200px;
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

.view-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: white;
}

.btn-create {
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

.btn-create:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
}

.content {
  max-width: 1200px;
  margin: 0 auto;
}

.empty-state {
  text-align: center;
  padding: 4rem 2rem;
  background: rgba(255, 255, 255, 0.02);
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 1.5rem;
}

.empty-icon {
  margin-bottom: 1.5rem;
  color: rgba(255, 255, 255, 0.3);
}

.empty-state h2 {
  font-size: 1.5rem;
  font-weight: 600;
  color: white;
  margin-bottom: 0.5rem;
}

.empty-state p {
  color: rgba(255, 255, 255, 0.6);
  margin-bottom: 2rem;
}

.btn-primary {
  padding: 0.75rem 2rem;
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  border: none;
  border-radius: 0.75rem;
  color: white;
  cursor: pointer;
  font-family: inherit;
  font-weight: 600;
}

.displays-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 1.5rem;
}

.display-card {
  background: rgba(255, 255, 255, 0.03);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 1rem;
  padding: 1.5rem;
  transition: all 0.3s ease;
}

.display-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
}

.display-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1.5rem;
}

.display-header h3 {
  font-size: 1.25rem;
  font-weight: 600;
  color: white;
}

.algo-badge {
  padding: 0.25rem 0.75rem;
  border-radius: 0.375rem;
  font-size: 0.75rem;
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

.display-info {
  margin-bottom: 1.5rem;
}

.info-row {
  display: flex;
  justify-content: space-between;
  padding: 0.5rem 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.06);
}

.info-row:last-child {
  border-bottom: none;
}

.info-label {
  color: rgba(255, 255, 255, 0.5);
  font-size: 0.875rem;
}

.info-value {
  color: white;
  font-size: 0.875rem;
  font-weight: 500;
}

.display-actions {
  display: flex;
  gap: 0.5rem;
}

.btn-action {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.75rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 0.5rem;
  color: rgba(255, 255, 255, 0.8);
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.875rem;
  font-family: inherit;
}

.btn-action:hover {
  background: rgba(255, 255, 255, 0.1);
  color: white;
}

.btn-action.primary {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.3);
  color: #10b981;
}

.btn-action.primary:hover {
  background: rgba(16, 185, 129, 0.2);
}

.btn-action.danger:hover {
  background: rgba(239, 68, 68, 0.1);
  border-color: rgba(239, 68, 68, 0.3);
  color: #ef4444;
}
</style>
