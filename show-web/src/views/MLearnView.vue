<template>
  <div class="layout">
    <aside class="sidebar">
      <div class="sidebar-header">
        <h1 class="sidebar-title">机器学习</h1>
        <span class="connection-badge" :class="mlearn.isConnected ? 'badge-ok' : 'badge-err'">
          {{ mlearn.isConnected ? '已连接' : '未连接' }}
        </span>
      </div>

      <nav class="sidebar-nav">
        <RouterLink class="nav-item" to="/mlearn/task">
          <svg class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <rect x="3" y="3" width="7" height="7" rx="1"/>
            <rect x="14" y="3" width="7" height="7" rx="1"/>
            <rect x="3" y="14" width="7" height="7" rx="1"/>
            <rect x="14" y="14" width="7" height="7" rx="1"/>
          </svg>
          <span>任务管理</span>
        </RouterLink>
        <RouterLink class="nav-item" to="/mlearn/regression">
          <svg class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M3 20L7 13 11 15 15 8 21 4"/>
            <path d="M3 20h18"/>
            <path d="M3 4v16"/>
          </svg>
          <span>回归拟合</span>
        </RouterLink>
        <RouterLink class="nav-item" to="/mlearn/genetic">
          <svg class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2z"/>
            <path d="M12 2c-3 3-3 7 0 10s3 7 0 10"/>
            <path d="M12 2c3 3 3 7 0 10s-3 7 0 10"/>
            <path d="M2 12h20"/>
          </svg>
          <span>遗传算法</span>
        </RouterLink>
      </nav>

      <div class="sidebar-tasks" v-if="mlearn.tasks.length > 0">
        <div class="section-label">任务列表</div>
        <div class="task-item" v-for="task in mlearn.tasks" :key="task.id"
          :class="{ active: task.id === mlearn.currentTaskId }"
          @click="handleTaskClick(task)">
          <span class="task-dot" :class="task.algorithm === 'regression' ? 'dot-blue' : 'dot-purple'"></span>
          <span class="task-name">{{ task.label }}</span>
        </div>
      </div>

      <div class="sidebar-footer" v-if="mlearn.hasTask">
        <div class="task-info">
          <div class="task-label">当前任务</div>
          <div class="task-id">{{ mlearn.currentTaskId?.slice(0, 8) }}...</div>
          <div class="task-stat">
            <span>Epochs: {{ mlearn.totalEpochs }}</span>
            <span v-if="mlearn.bestFitness !== null">Loss: {{ mlearn.bestFitness.toFixed(4) }}</span>
          </div>
        </div>
      </div>
    </aside>

    <main class="main">
      <RouterView />
    </main>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { RouterLink, RouterView, useRouter } from 'vue-router'
import { useMLearnStore, type TaskItem } from '../stores/mlearn'

const mlearn = useMLearnStore()
const router = useRouter()

onMounted(async () => {
  mlearn.checkConnection()
  await mlearn.fetchTasks()
})

async function handleTaskClick(task: TaskItem) {
  await mlearn.selectTask(task.id)
  if (task.algorithm === 'regression') {
    router.push('/mlearn/regression')
  } else {
    router.push('/mlearn/genetic')
  }
}
</script>

<style scoped>
.layout {
  display: flex;
  height: 100vh;
  background: var(--bg-primary);
  overflow: hidden;
}

.sidebar {
  width: 260px;
  min-width: 260px;
  background: var(--bg-card);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  padding: 20px 16px;
  border-bottom: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.sidebar-title {
  font-size: 16px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.connection-badge {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 10px;
}

.badge-ok {
  background: rgba(34, 197, 94, 0.15);
  color: var(--accent-green);
}

.badge-err {
  background: rgba(239, 68, 68, 0.15);
  color: #ef4444;
}

.sidebar-nav {
  padding: 8px 0;
  border-bottom: 1px solid var(--border);
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 16px;
  color: var(--text-secondary);
  text-decoration: none;
  font-size: 13px;
  transition: all 0.15s;
  border-left: 3px solid transparent;
}

.nav-item:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.nav-item.router-link-active {
  background: rgba(74, 144, 226, 0.08);
  color: var(--accent-blue);
  border-left-color: var(--accent-blue);
  font-weight: 600;
}

.nav-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

.sidebar-tasks {
  flex: 1;
  overflow-y: auto;
  padding: 8px 0;
}

.section-label {
  font-size: 11px;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  padding: 8px 16px 4px;
}

.task-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  cursor: pointer;
  transition: all 0.15s;
  border-left: 3px solid transparent;
}

.task-item:hover {
  background: var(--bg-hover);
}

.task-item.active {
  background: rgba(74, 144, 226, 0.06);
  border-left-color: var(--accent-blue);
}

.task-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.dot-blue {
  background: var(--accent-blue);
}

.dot-purple {
  background: #8b5cf6;
}

.task-name {
  font-size: 12px;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.task-item.active .task-name {
  color: var(--accent-blue);
  font-weight: 600;
}

.sidebar-footer {
  padding: 12px 16px;
  border-top: 1px solid var(--border);
}

.task-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.task-label {
  font-size: 11px;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.task-id {
  font-size: 12px;
  color: var(--accent-blue);
  font-family: monospace;
}

.task-stat {
  display: flex;
  gap: 12px;
  font-size: 12px;
  color: var(--text-secondary);
}

.main {
  flex: 1;
  overflow: auto;
  padding: 20px;
}
</style>
