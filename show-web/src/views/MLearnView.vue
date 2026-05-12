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
            <circle cx="12" cy="12" r="10"/>
            <path d="M12 8v8"/>
            <path d="M8 12h8"/>
          </svg>
          <span>创建任务</span>
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
import { RouterLink, RouterView } from 'vue-router'
import { useMLearnStore } from '../stores/mlearn'

const mlearn = useMLearnStore()

onMounted(() => {
  mlearn.checkConnection()
})
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
  flex: 1;
  padding: 8px 0;
  overflow-y: auto;
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
