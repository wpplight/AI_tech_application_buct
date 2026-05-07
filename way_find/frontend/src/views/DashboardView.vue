<template>
  <div class="dashboard">
    <div class="dashboard-header">
      <h1 class="dashboard-title">WayFind Dashboard</h1>
      <p class="dashboard-subtitle">Pathfinding Visualization Platform</p>
    </div>
    
    <div class="dashboard-grid">
      <!-- Quick Actions -->
      <div class="card quick-actions">
        <h2 class="card-title">Quick Actions</h2>
        <div class="actions-grid">
          <button class="action-card" @click="createNewMap">
            <div class="action-icon map-icon">
              <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
                <rect x="3" y="3" width="18" height="18" rx="2" stroke="currentColor" stroke-width="2"/>
                <path d="M3 9h18M9 21V9" stroke="currentColor" stroke-width="2"/>
              </svg>
            </div>
            <div class="action-text">
              <div class="action-title">Create New Map</div>
              <div class="action-desc">Design your own maze</div>
            </div>
          </button>
          
          <button class="action-card" @click="createNewDisplay">
            <div class="action-icon display-icon">
              <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
                <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
                <path d="M12 6v6l4 2" stroke="currentColor" stroke-width="2"/>
              </svg>
            </div>
            <div class="action-text">
              <div class="action-title">New Display</div>
              <div class="action-desc">Visualize an algorithm</div>
            </div>
          </button>
        </div>
      </div>
      
      <!-- Maps Section -->
      <div class="card maps-section">
        <div class="section-header">
          <h2 class="card-title">My Maps</h2>
          <button class="btn-link" @click="goToMaps">View All →</button>
        </div>
        
        <div v-if="maps.length === 0" class="empty-state">
          <div class="empty-icon">
            <svg width="48" height="48" viewBox="0 0 24 24" fill="none">
              <rect x="3" y="3" width="18" height="18" rx="2" stroke="currentColor" stroke-width="2"/>
              <path d="M3 9h18M9 21V9" stroke="currentColor" stroke-width="2"/>
            </svg>
          </div>
          <p>No maps yet. Create your first one!</p>
        </div>
        
        <div v-else class="items-list">
          <div 
            v-for="map in recentMaps" 
            :key="map.id" 
            class="item-card"
            @click="editMap(map.id)"
          >
            <div class="item-info">
              <div class="item-name">{{ map.name }}</div>
              <div class="item-meta">{{ map.width }}×{{ map.height }} • {{ map.walls.length }} walls</div>
            </div>
            <button class="btn-icon" @click.stop="deleteMap(map.id)">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                <path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2" stroke="currentColor" stroke-width="2"/>
              </svg>
            </button>
          </div>
        </div>
      </div>
      
      <!-- Displays Section -->
      <div class="card displays-section">
        <div class="section-header">
          <h2 class="card-title">My Displays</h2>
          <button class="btn-link" @click="goToDisplays">View All →</button>
        </div>
        
        <div v-if="displays.length === 0" class="empty-state">
          <div class="empty-icon">
            <svg width="48" height="48" viewBox="0 0 24 24" fill="none">
              <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
              <path d="M12 6v6l4 2" stroke="currentColor" stroke-width="2"/>
            </svg>
          </div>
          <p>No displays yet. Create your first one!</p>
        </div>
        
        <div v-else class="items-list">
          <div 
            v-for="display in recentDisplays" 
            :key="display.id" 
            class="item-card"
            @click="viewDisplay(display.id)"
          >
            <div class="item-info">
              <div class="item-name">{{ display.name }}</div>
              <div class="item-meta">
                {{ display.algorithm.toUpperCase() }} • 
                {{ display.executionMode === 'step' ? 'Step by Step' : 'Instant' }}
              </div>
            </div>
            <button class="btn-icon" @click.stop="deleteDisplay(display.id)">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                <path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2" stroke="currentColor" stroke-width="2"/>
              </svg>
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
import { useDisplayStore } from '@/stores/display'

const router = useRouter()
const mapStore = useMapStore()
const displayStore = useDisplayStore()

const maps = computed(() => mapStore.maps)
const displays = computed(() => displayStore.displays)

const recentMaps = computed(() => maps.value.slice(0, 3))
const recentDisplays = computed(() => displays.value.slice(0, 3))

function createNewMap() {
  router.push('/maps/new')
}

function createNewDisplay() {
  router.push('/displays/new')
}

function goToMaps() {
  router.push('/maps')
}

function goToDisplays() {
  router.push('/displays')
}

function editMap(id: string) {
  router.push(`/maps/${id}/edit`)
}

function viewDisplay(id: string) {
  router.push(`/displays/${id}`)
}

function deleteMap(id: string) {
  if (confirm('Are you sure you want to delete this map?')) {
    mapStore.deleteMap(id)
  }
}

function deleteDisplay(id: string) {
  if (confirm('Are you sure you want to delete this display?')) {
    displayStore.deleteDisplay(id)
  }
}
</script>

<style scoped>
.dashboard {
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
}

.dashboard-header {
  margin-bottom: 3rem;
  text-align: center;
}

.dashboard-title {
  font-size: 2.5rem;
  font-weight: 700;
  color: white;
  margin-bottom: 0.5rem;
}

.dashboard-subtitle {
  font-size: 1.125rem;
  color: rgba(255, 255, 255, 0.6);
}

.dashboard-grid {
  display: grid;
  gap: 2rem;
}

.card {
  background: rgba(255, 255, 255, 0.03);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 1.5rem;
  padding: 2rem;
}

.card-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: white;
  margin-bottom: 1.5rem;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.section-header .card-title {
  margin-bottom: 0;
}

.btn-link {
  background: none;
  border: none;
  color: #10b981;
  font-size: 0.875rem;
  cursor: pointer;
  transition: color 0.2s;
}

.btn-link:hover {
  color: #34d399;
}

.quick-actions .actions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
}

.action-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 1rem;
  cursor: pointer;
  transition: all 0.3s ease;
  text-align: left;
  width: 100%;
}

.action-card:hover {
  background: rgba(255, 255, 255, 0.08);
  border-color: rgba(255, 255, 255, 0.15);
  transform: translateY(-2px);
}

.action-icon {
  width: 64px;
  height: 64px;
  border-radius: 1rem;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.map-icon {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.2) 0%, rgba(99, 102, 241, 0.2) 100%);
  color: #3b82f6;
}

.display-icon {
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.2) 0%, rgba(6, 182, 212, 0.2) 100%);
  color: #10b981;
}

.action-text {
  flex: 1;
}

.action-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: white;
  margin-bottom: 0.25rem;
}

.action-desc {
  font-size: 0.875rem;
  color: rgba(255, 255, 255, 0.6);
}

.empty-state {
  text-align: center;
  padding: 3rem 2rem;
  color: rgba(255, 255, 255, 0.5);
}

.empty-icon {
  margin-bottom: 1rem;
  opacity: 0.5;
}

.items-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.item-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem 1.25rem;
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.06);
  border-radius: 0.75rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.item-card:hover {
  background: rgba(255, 255, 255, 0.06);
  border-color: rgba(255, 255, 255, 0.1);
}

.item-info {
  flex: 1;
}

.item-name {
  font-weight: 600;
  color: white;
  margin-bottom: 0.25rem;
}

.item-meta {
  font-size: 0.75rem;
  color: rgba(255, 255, 255, 0.5);
}

.btn-icon {
  width: 32px;
  height: 32px;
  border-radius: 0.5rem;
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.2);
  color: #ef4444;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-icon:hover {
  background: rgba(239, 68, 68, 0.2);
  border-color: rgba(239, 68, 68, 0.3);
}
</style>
