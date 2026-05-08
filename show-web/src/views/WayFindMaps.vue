<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useWayfindStore } from '../stores/wayfind'

const router = useRouter()
const store = useWayfindStore()

const deleteConfirm = ref<string | null>(null)

onMounted(async () => {
  await store.fetchMaps()
})

function handleAdd() {
  router.push('/wayfind/maps/edit')
}

function handleEdit(name: string) {
  router.push({ path: '/wayfind/maps/edit', query: { name } })
}

async function handleDelete(name: string) {
  await store.deleteMap(name)
  deleteConfirm.value = null
}

function formatDate(dateStr: string): string {
  if (!dateStr) return '-'
  const date = new Date(dateStr)
  return date.toLocaleString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}
</script>

<template>
  <div class="maps-view">
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">地图管理</h2>
        <p class="page-subtitle">所有保存的地图</p>
      </div>
      <button class="add-btn" @click="handleAdd">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 5v14M5 12h14"/>
        </svg>
        新建地图
      </button>
    </div>

    <div v-if="store.savedMaps.length === 0" class="empty-state">
      <div class="empty-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M3 7l6-3 6 3 6-3v13l-6 3-6-3-6 3V7z"/>
          <path d="M9 4v13"/>
          <path d="M15 7v13"/>
        </svg>
      </div>
      <p class="empty-title">暂无地图</p>
      <p class="empty-desc">点击右上角"新建地图"开始</p>
    </div>

    <div v-else class="map-list">
      <div
        v-for="map in store.savedMaps"
        :key="map.name"
        class="map-item"
      >
        <div class="map-info" @click="handleEdit(map.name)">
          <div class="map-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M3 7l6-3 6 3 6-3v13l-6 3-6-3-6 3V7z"/>
              <path d="M9 4v13"/>
              <path d="M15 7v13"/>
            </svg>
          </div>
          <div class="map-text">
            <span class="map-name">{{ map.name }}</span>
            <span class="map-meta">{{ map.width }}×{{ map.height }}</span>
          </div>
        </div>

        <div class="map-right">
          <span class="map-date">{{ formatDate(map.modifiedAt) }}</span>
          <div class="map-actions">
            <button class="icon-btn" title="编辑" @click="handleEdit(map.name)">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
                <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
              </svg>
            </button>
            <button
              v-if="deleteConfirm !== map.name"
              class="icon-btn danger"
              title="删除"
              @click="deleteConfirm = map.name"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="3 6 5 6 21 6"/>
                <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
              </svg>
            </button>
            <div v-else class="confirm-group">
              <button class="confirm-yes" @click="handleDelete(map.name)">确认</button>
              <button class="confirm-no" @click="deleteConfirm = null">取消</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.maps-view {
  max-width: 800px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 32px;
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.page-title {
  font-size: 20px;
  font-weight: 700;
}

.page-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
}

.add-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 18px;
  background: var(--accent-blue);
  border: none;
  border-radius: 12px;
  color: white;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.add-btn:hover {
  filter: brightness(1.1);
}

.add-btn svg {
  width: 16px;
  height: 16px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 64px 32px;
  text-align: center;
}

.empty-icon {
  width: 64px;
  height: 64px;
  color: var(--border-color);
  margin-bottom: 16px;
}

.empty-icon svg {
  width: 100%;
  height: 100%;
}

.empty-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 4px;
}

.empty-desc {
  font-size: 14px;
  color: var(--text-secondary);
}

.map-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.map-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 14px;
  transition: all 0.2s;
}

.map-item:hover {
  border-color: var(--accent-blue);
}

.map-info {
  display: flex;
  align-items: center;
  gap: 16px;
  cursor: pointer;
  flex: 1;
  min-width: 0;
}

.map-icon {
  width: 36px;
  height: 36px;
  background: rgba(59, 130, 246, 0.1);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--accent-blue);
  flex-shrink: 0;
}

.map-icon svg {
  width: 20px;
  height: 20px;
}

.map-text {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.map-name {
  font-size: 15px;
  font-weight: 500;
  color: var(--text-primary);
}

.map-meta {
  font-size: 13px;
  color: var(--text-secondary);
}

.map-right {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-shrink: 0;
}

.map-date {
  font-size: 12px;
  color: var(--text-secondary);
}

.map-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.icon-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  border-radius: 8px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
}

.icon-btn svg {
  width: 16px;
  height: 16px;
}

.icon-btn:hover {
  background: var(--bg-primary);
  color: var(--text-primary);
}

.icon-btn.danger:hover {
  background: rgba(244, 63, 94, 0.1);
  color: var(--accent-red, #f43f5e);
}

.confirm-group {
  display: flex;
  gap: 4px;
}

.confirm-yes,
.confirm-no {
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 12px;
  cursor: pointer;
  border: none;
  transition: all 0.2s;
}

.confirm-yes {
  background: var(--accent-red, #f43f5e);
  color: white;
}

.confirm-no {
  background: var(--bg-primary);
  color: var(--text-secondary);
}
</style>
