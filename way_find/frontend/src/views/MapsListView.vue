<template>
  <div class="maps-list-view">
    <div class="view-header">
      <button class="btn-back" @click="goBack">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
          <path d="M19 12H5M12 19l-7-7 7-7" stroke="currentColor" stroke-width="2"/>
        </svg>
        Back
      </button>
      
      <h1 class="view-title">My Maps</h1>
      
      <button class="btn-create" @click="createNewMap">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
          <path d="M12 5v14M5 12h14" stroke="currentColor" stroke-width="2"/>
        </svg>
        New Map
      </button>
    </div>
    
    <div class="content">
      <div v-if="maps.length === 0" class="empty-state">
        <div class="empty-icon">
          <svg width="64" height="64" viewBox="0 0 24 24" fill="none">
            <rect x="3" y="3" width="18" height="18" rx="2" stroke="currentColor" stroke-width="2"/>
            <path d="M3 9h18M9 21V9" stroke="currentColor" stroke-width="2"/>
          </svg>
        </div>
        <h2>No Maps Yet</h2>
        <p>Create your first map to get started</p>
        <button class="btn-primary" @click="createNewMap">Create Map</button>
      </div>
      
      <div v-else class="maps-grid">
        <div 
          v-for="map in maps" 
          :key="map.id" 
          class="map-card"
        >
          <div class="map-preview">
            <div class="preview-grid">
              <div 
                v-for="(cell, i) in getPreviewCells(map)" 
                :key="i"
                class="preview-cell"
                :class="cell"
              ></div>
            </div>
          </div>
          
          <div class="map-info">
            <h3>{{ map.name }}</h3>
            <p>{{ map.width }}×{{ map.height }} • {{ map.walls.length }} walls</p>
            <p class="map-date">{{ formatDate(map.updatedAt) }}</p>
          </div>
          
          <div class="map-actions">
            <button class="btn-action" @click="editMap(map.id)">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7" stroke="currentColor" stroke-width="2"/>
                <path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z" stroke="currentColor" stroke-width="2"/>
              </svg>
              Edit
            </button>
            <button class="btn-action danger" @click="deleteMap(map.id)">
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
import { useMapStore } from '@/stores/map'

const router = useRouter()
const mapStore = useMapStore()

const maps = computed(() => mapStore.maps)

function goBack() {
  router.push('/')
}

function createNewMap() {
  router.push('/maps/new')
}

function editMap(id: string) {
  router.push(`/maps/${id}/edit`)
}

function deleteMap(id: string) {
  if (confirm('Are you sure you want to delete this map?')) {
    mapStore.deleteMap(id)
  }
}

function getPreviewCells(map: any) {
  const cells: string[] = []
  const gridSize = 15
  
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

function formatDate(date: Date) {
  return new Date(date).toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    year: 'numeric'
  })
}
</script>

<style scoped>
.maps-list-view {
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

.maps-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
}

.map-card {
  background: rgba(255, 255, 255, 0.03);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 1rem;
  overflow: hidden;
  transition: all 0.3s ease;
}

.map-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
}

.map-preview {
  background: #0f1419;
  padding: 1.5rem;
  display: flex;
  justify-content: center;
  align-items: center;
}

.preview-grid {
  display: grid;
  grid-template-columns: repeat(15, 4px);
  gap: 1px;
}

.preview-cell {
  width: 4px;
  height: 4px;
  border-radius: 1px;
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
  padding: 1.5rem;
}

.map-info h3 {
  font-size: 1.125rem;
  font-weight: 600;
  color: white;
  margin-bottom: 0.5rem;
}

.map-info p {
  font-size: 0.875rem;
  color: rgba(255, 255, 255, 0.6);
  margin-bottom: 0.25rem;
}

.map-date {
  font-size: 0.75rem !important;
  color: rgba(255, 255, 255, 0.4) !important;
}

.map-actions {
  display: flex;
  gap: 0.5rem;
  padding: 1rem 1.5rem;
  border-top: 1px solid rgba(255, 255, 255, 0.06);
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

.btn-action.danger:hover {
  background: rgba(239, 68, 68, 0.1);
  border-color: rgba(239, 68, 68, 0.3);
  color: #ef4444;
}
</style>
