<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useWayfindStore } from '../stores/wayfind'

const route = useRoute()
const router = useRouter()
const store = useWayfindStore()

const activeTab = ref<'maps' | 'tasks' | 'inference' | 'algorithm'>('maps')

const tabs = [
  { id: 'maps', label: '地图管理', icon: 'map' },
  { id: 'tasks', label: '任务管理', icon: 'layers' },
  { id: 'inference', label: '寻路推导', icon: 'search' },
  { id: 'algorithm', label: '算法讲解', icon: 'book' }
]

function syncActiveTab() {
  const path = route.path
  if (path.includes('/tasks')) {
    activeTab.value = 'tasks'
  } else if (path.includes('/inference')) {
    activeTab.value = 'inference'
  } else if (path.includes('/algorithm')) {
    activeTab.value = 'algorithm'
  } else if (path.includes('/maps')) {
    activeTab.value = 'maps'
  }
}

onMounted(async () => {
  syncActiveTab()
  await store.checkConnection()
  if (store.isConnected) {
    await store.fetchTasks()
    await store.fetchMaps()
  }
})

watch(() => route.path, syncActiveTab)

function switchTab(tab: 'maps' | 'tasks' | 'inference' | 'algorithm') {
  activeTab.value = tab
  router.push(`/wayfind/${tab}`)
}
</script>

<template>
  <div class="wayfind-container">
    <aside class="sidebar">
      <div class="sidebar-header">
        <h1 class="sidebar-title">寻路算法</h1>
        <div class="connection-badge" :class="{ connected: store.isConnected }">
          <span class="connection-dot"></span>
          <span class="connection-text">{{ store.isConnected ? '在线' : '离线' }}</span>
        </div>
      </div>

      <nav class="sidebar-nav">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          class="nav-item"
          :class="{ active: activeTab === tab.id }"
          @click="switchTab(tab.id as 'maps' | 'tasks' | 'inference' | 'algorithm')"
        >
          <svg v-if="tab.icon === 'map'" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M3 7l6-3 6 3 6-3v13l-6 3-6-3-6 3V7z"/>
            <path d="M9 4v13"/>
            <path d="M15 7v13"/>
          </svg>
          <svg v-if="tab.icon === 'layers'" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <polygon points="12 2 2 7 12 12 22 7 12 2"/>
            <polyline points="2 17 12 22 22 17"/>
            <polyline points="2 12 12 17 22 12"/>
          </svg>
          <svg v-if="tab.icon === 'search'" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <circle cx="11" cy="11" r="8"/>
            <path d="M21 21l-4.35-4.35"/>
            <path d="M11 8v6"/>
            <path d="M8 11h6"/>
          </svg>
          <svg v-if="tab.icon === 'book'" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M4 19.5A2.5 2.5 0 016.5 17H20"/>
            <path d="M6.5 2H20v20H6.5A2.5 2.5 0 014 19.5v-15A2.5 2.5 0 016.5 2z"/>
            <path d="M8 7h8M8 11h5"/>
          </svg>
          <span class="nav-label">{{ tab.label }}</span>
        </button>
      </nav>

      <div v-if="store.currentTask" class="sidebar-footer">
        <div class="task-info-card">
          <div class="task-info-row">
            <span class="info-label">任务</span>
            <span class="info-value">{{ store.currentTask.name }}</span>
          </div>
          <div class="task-info-row">
            <span class="info-label">尺寸</span>
            <span class="info-value">{{ store.currentTask.width }}×{{ store.currentTask.height }}</span>
          </div>
          <div class="task-info-row">
            <span class="info-label">状态</span>
            <span class="state-badge" :class="store.taskState">
              {{ store.taskState === 'idle' ? '空闲' : store.taskState === 'searching' ? '搜索中' : store.taskState === 'done' ? '已完成' : '失败' }}
            </span>
          </div>
          <div v-if="store.currentTask.algorithm" class="task-info-row">
            <span class="info-label">算法</span>
            <span class="info-value">{{ store.currentTask.algorithm.toUpperCase() }}</span>
          </div>
        </div>
      </div>
    </aside>

    <main class="main-content">
      <RouterView />
    </main>
  </div>
</template>

<style scoped>
.wayfind-container {
  display: flex;
  min-height: 100vh;
  background: var(--bg-primary);
  color: var(--text-primary);
}

.sidebar {
  width: 280px;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  position: fixed;
  top: 0;
  left: 0;
  height: 100vh;
  z-index: 100;
}

@media (max-width: 768px) {
  .sidebar {
    width: 100%;
    height: auto;
    position: relative;
  }
}

.sidebar-header {
  padding: 24px 20px;
  border-bottom: 1px solid var(--border-color);
}

.sidebar-title {
  font-size: 20px;
  font-weight: 700;
  letter-spacing: -0.02em;
  margin-bottom: 12px;
}

.connection-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 12px;
  background: rgba(244, 63, 94, 0.1);
  border: 1px solid rgba(244, 63, 94, 0.3);
  border-radius: 16px;
  font-size: 12px;
  color: var(--accent-red, #f43f5e);
  transition: all 0.3s ease;
}

.connection-badge.connected {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.3);
  color: var(--accent-green, #10b981);
}

.connection-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: currentColor;
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.6; transform: scale(0.9); }
}

.sidebar-nav {
  flex: 1;
  padding: 16px 12px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

@media (max-width: 768px) {
  .sidebar-nav {
    flex-direction: row;
    padding: 12px;
    overflow-x: auto;
    flex: none;
  }
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: transparent;
  border: none;
  border-radius: 10px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  text-align: left;
  width: 100%;
}

@media (max-width: 768px) {
  .nav-item {
    width: auto;
    flex-shrink: 0;
    padding: 10px 14px;
  }
}

.nav-item:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.nav-item.active {
  background: rgba(59, 130, 246, 0.1);
  color: var(--accent-blue);
}

.nav-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

.nav-label {
  font-size: 14px;
  font-weight: 500;
}

.sidebar-footer {
  padding: 16px 12px;
  border-top: 1px solid var(--border-color);
}

.task-info-card {
  background: var(--bg-tertiary);
  border-radius: 12px;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.task-info-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 13px;
}

.info-label {
  color: var(--text-secondary);
}

.info-value {
  font-weight: 500;
  color: var(--text-primary);
}

.state-badge {
  padding: 2px 8px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
}

.state-badge.idle {
  background: rgba(161, 161, 170, 0.15);
  color: var(--text-secondary);
}

.state-badge.searching {
  background: rgba(59, 130, 246, 0.15);
  color: var(--accent-blue);
}

.state-badge.done {
  background: rgba(16, 185, 129, 0.15);
  color: var(--accent-green);
}

.state-badge.failed {
  background: rgba(244, 63, 94, 0.15);
  color: var(--accent-red);
}

.main-content {
  flex: 1;
  margin-left: 280px;
  min-height: 100vh;
  padding: 24px;
}

@media (max-width: 768px) {
  .main-content {
    margin-left: 0;
    padding: 16px;
  }
}
</style>
