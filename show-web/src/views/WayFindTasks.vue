<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useWayfindStore } from '../stores/wayfind'

const router = useRouter()
const store = useWayfindStore()

const showCreate = ref(false)
const taskName = ref('')
const selectedMapName = ref<string | null>(null)
const deleteConfirm = ref<string | null>(null)

onMounted(async () => {
  await store.fetchTasks()
  await store.fetchMaps()
})

async function handleCreate() {
  if (!taskName.value.trim()) return
  await store.createTask(
    20, 15,
    taskName.value.trim(),
    selectedMapName.value || undefined
  )
  taskName.value = ''
  selectedMapName.value = null
  showCreate.value = false
}

async function handleLoadTask(taskId: string) {
  await store.selectTask(taskId)
}

async function handleDeleteTask(taskId: string) {
  await store.deleteTask(taskId)
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
  <div class="tasks-view">
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">任务管理</h2>
        <p class="page-subtitle">所有搜索任务</p>
      </div>
      <button class="add-btn" @click="showCreate = !showCreate">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 5v14M5 12h14"/>
        </svg>
        新建任务
      </button>
    </div>

    <!-- Create form -->
    <div v-if="showCreate" class="create-card">
      <div class="create-form">
        <div class="form-row">
          <input
            v-model="taskName"
            type="text"
            placeholder="任务名称"
            class="form-input"
            @keyup.enter="handleCreate"
          />
          <button class="btn btn-primary" @click="handleCreate" :disabled="!taskName.trim()">
            创建
          </button>
          <button class="btn btn-outline" @click="showCreate = false">
            取消
          </button>
        </div>

        <div v-if="store.savedMaps.length > 0" class="map-picker">
          <span class="picker-label">来源地图（可选）</span>
          <div class="map-options">
            <button
              class="map-opt"
              :class="{ active: selectedMapName === null }"
              @click="selectedMapName = null"
            >空白地图</button>
            <button
              v-for="m in store.savedMaps"
              :key="m.name"
              class="map-opt"
              :class="{ active: selectedMapName === m.name }"
              @click="selectedMapName = m.name"
            >{{ m.name }}</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Task list -->
    <div v-if="store.tasks.length === 0" class="empty-state">
      <div class="empty-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <polygon points="12 2 2 7 12 12 22 7 12 2"/>
          <polyline points="2 17 12 22 22 17"/>
          <polyline points="2 12 12 17 22 12"/>
        </svg>
      </div>
      <p class="empty-title">暂无任务</p>
      <p class="empty-desc">点击右上角"新建任务"开始</p>
    </div>

    <div v-else class="task-list">
      <div
        v-for="task in store.tasks"
        :key="task.taskId"
        class="task-item"
      >
        <div class="task-info" @click="handleLoadTask(task.taskId)">
          <div class="task-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <polygon points="12 2 2 7 12 12 22 7 12 2"/>
              <polyline points="2 17 12 22 22 17"/>
              <polyline points="2 12 12 17 22 12"/>
            </svg>
          </div>
          <div class="task-text">
            <div class="task-top">
              <span class="task-name">{{ task.name }}</span>
              <span class="state-badge" :class="task.state">
                {{ task.state === 'idle' ? '空闲' : task.state === 'searching' ? '搜索中' : task.state === 'done' ? '已完成' : '失败' }}
              </span>
            </div>
            <div class="task-meta">
              <span>{{ task.width }}×{{ task.height }}</span>
              <span v-if="task.mapName" class="from-map">← {{ task.mapName }}</span>
              <span v-if="task.algorithm" class="task-algo">{{ task.algorithm.toUpperCase() }}</span>
            </div>
          </div>
        </div>

        <div class="task-right">
          <span class="task-date">{{ formatDate(task.updatedAt) }}</span>
          <div class="task-actions">
            <button class="icon-btn" title="加载" @click="handleLoadTask(task.taskId)">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="11" cy="11" r="8"/>
                <path d="M21 21l-4.35-4.35"/>
              </svg>
            </button>
            <button
              v-if="deleteConfirm !== task.taskId"
              class="icon-btn danger"
              title="删除"
              @click="deleteConfirm = task.taskId"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="3 6 5 6 21 6"/>
                <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
              </svg>
            </button>
            <div v-else class="confirm-group">
              <button class="confirm-yes" @click="handleDeleteTask(task.taskId)">确认</button>
              <button class="confirm-no" @click="deleteConfirm = null">取消</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.tasks-view {
  max-width: 800px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
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

.create-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  padding: 20px;
  margin-bottom: 24px;
}

.create-form {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.form-row {
  display: flex;
  gap: 8px;
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

.map-picker {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.picker-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.map-options {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.map-opt {
  padding: 6px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-primary);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.map-opt:hover {
  border-color: var(--accent-blue);
}

.map-opt.active {
  border-color: var(--accent-blue);
  background: rgba(59,130,246,0.1);
}

.btn {
  padding: 10px 16px;
  border-radius: 10px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  border: none;
  transition: all 0.2s;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background: var(--accent-blue);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  filter: brightness(1.1);
}

.btn-outline {
  background: transparent;
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
}

.btn-outline:hover {
  border-color: var(--accent-blue);
  color: var(--text-primary);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
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

.task-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.task-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 14px;
  transition: all 0.2s;
}

.task-item:hover {
  border-color: var(--accent-blue);
}

.task-info {
  display: flex;
  align-items: center;
  gap: 16px;
  cursor: pointer;
  flex: 1;
  min-width: 0;
}

.task-icon {
  width: 36px;
  height: 36px;
  background: rgba(168, 85, 247, 0.1);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #a855f7;
  flex-shrink: 0;
}

.task-icon svg {
  width: 20px;
  height: 20px;
}

.task-text {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}

.task-top {
  display: flex;
  align-items: center;
  gap: 8px;
}

.task-name {
  font-size: 15px;
  font-weight: 500;
  color: var(--text-primary);
}

.task-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: var(--text-secondary);
}

.from-map {
  color: #a855f7;
}

.task-algo {
  color: var(--accent-blue);
  font-weight: 600;
}

.state-badge {
  padding: 2px 8px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
}

.state-badge.idle {
  background: rgba(161,161,170,0.15);
  color: var(--text-secondary);
}

.state-badge.searching {
  background: rgba(59,130,246,0.15);
  color: var(--accent-blue);
}

.state-badge.done {
  background: rgba(16,185,129,0.15);
  color: var(--accent-green);
}

.state-badge.failed {
  background: rgba(244,63,94,0.15);
  color: var(--accent-red);
}

.task-right {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-shrink: 0;
}

.task-date {
  font-size: 12px;
  color: var(--text-secondary);
}

.task-actions {
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
  background: rgba(244,63,94,0.1);
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
