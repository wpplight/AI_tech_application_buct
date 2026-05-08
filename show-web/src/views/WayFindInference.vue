<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useWayfindStore } from '../stores/wayfind'

const router = useRouter()
const store = useWayfindStore()

onMounted(async () => {
  await store.fetchTasks()
})

function handleEnter(taskId: string) {
  router.push({ path: '/wayfind/inference/detail', query: { taskId } })
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
  <div class="inference-view">
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">寻路推导</h2>
        <p class="page-subtitle">选择一个任务开始寻路</p>
      </div>
    </div>

    <div v-if="store.tasks.length === 0" class="empty-state">
      <div class="empty-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <circle cx="11" cy="11" r="8"/>
          <path d="M21 21l-4.35-4.35"/>
          <path d="M11 8v6"/>
          <path d="M8 11h6"/>
        </svg>
      </div>
      <p class="empty-title">暂无任务</p>
      <p class="empty-desc">请先在任务管理中创建任务</p>
    </div>

    <div v-else class="task-list">
      <div
        v-for="task in store.tasks"
        :key="task.taskId"
        class="task-item"
        @click="handleEnter(task.taskId)"
      >
        <div class="task-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <circle cx="11" cy="11" r="8"/>
            <path d="M21 21l-4.35-4.35"/>
            <path d="M11 8v6"/>
            <path d="M8 11h6"/>
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
        <div class="task-right">
          <span class="task-date">{{ formatDate(task.updatedAt) }}</span>
          <div class="enter-btn">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M5 12h14M12 5l7 7-7 7"/>
            </svg>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.inference-view {
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
  align-items: center;
  gap: 16px;
  padding: 16px 20px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.task-item:hover {
  border-color: var(--accent-blue);
}

.task-item:hover .enter-btn {
  background: var(--accent-blue);
  color: white;
}

.task-icon {
  width: 40px;
  height: 40px;
  background: rgba(59, 130, 246, 0.1);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--accent-blue);
  flex-shrink: 0;
}

.task-icon svg {
  width: 20px;
  height: 20px;
}

.task-text {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
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

.enter-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  color: var(--text-secondary);
  transition: all 0.2s;
}

.enter-btn svg {
  width: 16px;
  height: 16px;
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
</style>
