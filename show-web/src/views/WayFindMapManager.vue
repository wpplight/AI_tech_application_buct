<script setup lang="ts">
import { ref } from 'vue'
import { useWayfindStore } from '../stores/wayfind'
import { useRouter } from 'vue-router'

const store = useWayfindStore()
const router = useRouter()

const saveName = ref('')
const deleteConfirm = ref<string | null>(null)

async function handleSaveMap() {
  if (!saveName.value.trim()) return
  await store.saveMap(saveName.value.trim())
  saveName.value = ''
}

async function handleLoadMap(name: string) {
  await store.loadMap(name)
  router.push('/wayfind/inference')
}

async function handleDeleteMap(name: string) {
  await store.deleteMap(name)
  deleteConfirm.value = null
}

async function handleLoadTask(taskId: string) {
  await store.loadTask(taskId)
  router.push('/wayfind/inference')
}

async function handleDeleteTask(taskId: string) {
  await store.deleteTask(taskId)
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

function handleCloseError() {
  store.error = null
}
</script>

<template>
  <div class="map-manager">
    <div v-if="store.error" class="error-banner">
      <div class="error-card">
        <span>{{ store.error }}</span>
        <button class="close-btn" @click="handleCloseError">×</button>
      </div>
    </div>

    <div class="section-header">
      <h2 class="section-title">地图管理</h2>
      <p class="section-desc">保存当前地图、加载已保存地图、管理任务</p>
    </div>

    <div class="card">
      <h3 class="card-title">保存当前地图</h3>
      <div v-if="store.currentTask" class="save-form">
        <input
          v-model="saveName"
          type="text"
          placeholder="输入地图名称"
          class="form-input"
          @keyup.enter="handleSaveMap"
        />
        <button class="btn btn-primary" @click="handleSaveMap" :disabled="!saveName.trim()">
          保存
        </button>
      </div>
      <div v-else class="empty-state">
        <p>暂无任务，请先创建任务</p>
      </div>
    </div>

    <div class="card">
      <div class="card-header">
        <h3 class="card-title">已保存的地图</h3>
        <button class="btn btn-sm btn-outline" @click="store.fetchSavedMaps()">
          刷新
        </button>
      </div>
      <div v-if="store.savedMaps.length === 0" class="empty-state">
        <p>暂无已保存的地图</p>
      </div>
      <div v-else class="list">
        <div
          v-for="map in store.savedMaps"
          :key="map.name"
          class="list-item"
        >
          <div class="item-info">
            <span class="item-name">{{ map.name }}</span>
            <span class="item-meta">{{ map.width }}×{{ map.height }}</span>
            <span class="item-date">{{ formatDate(map.modifiedAt) }}</span>
          </div>
          <div class="item-actions">
            <button class="btn btn-sm btn-primary" @click="handleLoadMap(map.name)">
              加载
            </button>
            <button
              v-if="deleteConfirm !== map.name"
              class="btn btn-sm btn-outline"
              @click="deleteConfirm = map.name"
            >
              删除
            </button>
            <div v-else class="confirm-delete">
              <span>确认?</span>
              <button class="btn btn-sm btn-danger" @click="handleDeleteMap(map.name)">
                是
              </button>
              <button class="btn btn-sm btn-outline" @click="deleteConfirm = null">
                否
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="card">
      <div class="card-header">
        <h3 class="card-title">当前任务</h3>
        <button class="btn btn-sm btn-outline" @click="store.fetchAllTasks()">
          刷新
        </button>
      </div>
      <div v-if="store.allTasks.length === 0" class="empty-state">
        <p>暂无任务</p>
      </div>
      <div v-else class="list">
        <div
          v-for="task in store.allTasks"
          :key="task.taskId"
          class="list-item"
          :class="{ active: store.currentTask?.taskId === task.taskId }"
        >
          <div class="item-info">
            <span class="item-name">{{ task.name }}</span>
            <span class="item-id">{{ task.taskId }}</span>
            <span class="item-meta">{{ task.width }}×{{ task.height }}</span>
            <span class="state-badge" :class="task.state">
              {{ task.state === 'idle' ? '空闲' : task.state === 'searching' ? '搜索中' : task.state === 'done' ? '已完成' : '失败' }}
            </span>
            <span v-if="task.algorithm" class="item-algo">{{ task.algorithm.toUpperCase() }}</span>
          </div>
          <div class="item-actions">
            <button
              v-if="store.currentTask?.taskId !== task.taskId"
              class="btn btn-sm btn-primary"
              @click="handleLoadTask(task.taskId)"
            >
              加载
            </button>
            <span v-else class="current-label">当前</span>
            <button class="btn btn-sm btn-outline" @click="handleDeleteTask(task.taskId)">
              删除
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.map-manager {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.error-banner {
  margin-bottom: 8px;
}

.error-card {
  background: rgba(244, 63, 94, 0.1);
  border: 1px solid rgba(244, 63, 94, 0.3);
  border-radius: 12px;
  padding: 12px 16px;
  color: var(--accent-red, #f43f5e);
  font-size: 14px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.close-btn {
  background: none;
  border: none;
  font-size: 20px;
  cursor: pointer;
  color: inherit;
  padding: 0 4px;
}

.section-header {
  margin-bottom: 8px;
}

.section-title {
  font-size: 20px;
  font-weight: 700;
  margin-bottom: 4px;
}

.section-desc {
  font-size: 14px;
  color: var(--text-secondary);
}

.card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.card-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-secondary);
}

.save-form {
  display: flex;
  gap: 12px;
}

.form-input {
  flex: 1;
  padding: 10px 14px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  color: var(--text-primary);
  font-size: 14px;
}

.form-input:focus {
  outline: none;
  border-color: var(--accent-blue);
}

.empty-state {
  text-align: center;
  padding: 24px;
  color: var(--text-secondary);
  font-size: 14px;
}

.list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.list-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--bg-primary);
  border-radius: 12px;
  gap: 16px;
}

.list-item.active {
  border: 1px solid var(--accent-blue);
  background: rgba(59, 130, 246, 0.05);
}

.item-info {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
  min-width: 0;
}

.item-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.item-id {
  font-size: 10px;
  font-family: monospace;
  color: var(--text-secondary);
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.item-meta {
  font-size: 12px;
  color: var(--text-secondary);
}

.item-date {
  font-size: 11px;
  color: var(--text-secondary);
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

.item-algo {
  font-size: 11px;
  color: #a855f7;
  font-weight: 600;
}

.item-actions {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
}

.confirm-delete {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--accent-red);
}

.current-label {
  font-size: 12px;
  color: var(--accent-blue);
  padding: 6px 12px;
}

.btn {
  padding: 8px 16px;
  border-radius: 10px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-sm {
  padding: 6px 12px;
  font-size: 12px;
  border-radius: 8px;
}

.btn-primary {
  background: var(--accent-blue);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  filter: brightness(1.1);
}

.btn-danger {
  background: var(--accent-red);
  color: white;
}

.btn-outline {
  background: transparent;
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
}

.btn-outline:hover:not(:disabled) {
  border-color: var(--accent-blue);
  color: var(--text-primary);
}
</style>
