<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useProfessorStore } from '../../stores/professor'

const store = useProfessorStore()

onMounted(() => {
  store.loadPersistedHistory()
})

function formatTime(ts: number) {
  const d = new Date(ts)
  return d.toLocaleString('zh-CN', {
    month: '2-digit', day: '2-digit',
    hour: '2-digit', minute: '2-digit', second: '2-digit'
  })
}

function formatDuration(steps: any[]) {
  return `${steps.length} 步`
}

const groupedByDate = computed(() => {
  const groups: Record<string, any[]> = {}
  store.persistedHistory.forEach(entry => {
    const date = new Date(entry.timestamp).toLocaleDateString('zh-CN')
    if (!groups[date]) groups[date] = []
    groups[date].push(entry)
  })
  return groups
})

const dateKeys = computed(() => Object.keys(groupedByDate.value))
</script>

<template>
  <div class="history-panel">
    <div class="panel-header">
      <div class="header-left">
        <h3 class="panel-title">推理历史</h3>
        <span class="history-count">{{ store.persistedHistory.length }} 条记录</span>
      </div>
      <button
        v-if="store.persistedHistory.length > 0"
        class="btn-clear"
        @click="store.clearPersistedHistory()"
      >清空历史</button>
    </div>

    <div v-if="store.persistedHistory.length === 0" class="empty-state">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <circle cx="12" cy="12" r="10"/>
        <polyline points="12 6 12 12 16 14"/>
      </svg>
      <p>暂无推理历史</p>
      <p class="hint">在推理控制页面执行推理后，记录将保存在此处</p>
    </div>

    <div v-else class="history-list">
      <div v-for="date in dateKeys" :key="date" class="history-date-group">
        <div class="date-label">{{ date }}</div>
        <div
          v-for="(entry, idx) in groupedByDate[date]"
          :key="idx"
          class="history-entry"
          :class="entry.type"
        >
          <div class="entry-header">
            <span class="entry-type">{{ entry.type === 'forward' ? '正向推理' : '反向推理' }}</span>
            <span class="entry-algo">{{ entry.algorithm }}</span>
            <span class="entry-time">{{ formatTime(entry.timestamp) }}</span>
          </div>

          <div v-if="entry.type === 'forward'" class="entry-detail">
            <div class="entry-facts">
              <span class="detail-label">初始条件:</span>
              <span class="fact-tags">
                <span v-if="entry.facts.length === 0" class="empty-tag">无</span>
                <span v-for="f in entry.facts" :key="f" class="fact-tag">{{ f }}</span>
              </span>
            </div>
            <div v-if="entry.result?.new_facts?.length" class="entry-new-facts">
              <span class="detail-label">新增事实:</span>
              <span class="fact-tags">
                <span v-for="f in entry.result.new_facts" :key="f" class="new-fact-tag">{{ f }}</span>
              </span>
            </div>
          </div>

          <div v-else class="entry-detail">
            <div class="entry-goal">
              <span class="detail-label">目标:</span>
              <span class="goal-text">{{ entry.goal }}</span>
            </div>
            <div v-if="entry.result?.result" class="entry-result">
              <span class="detail-label">结果:</span>
              <span class="result-text" :class="entry.result.result">{{ entry.result.result }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.history-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.panel-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
}

.history-count {
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  padding: 2px 10px;
  border-radius: 20px;
  font-size: 12px;
}

.btn-clear {
  background: none;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-muted);
  font-size: 12px;
  padding: 4px 10px;
  cursor: pointer;
}

.btn-clear:hover {
  border-color: var(--accent-red);
  color: var(--accent-red);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px;
  gap: 12px;
}

.empty-state svg {
  width: 48px;
  height: 48px;
  color: var(--border-color);
}

.empty-state p {
  margin: 0;
  color: var(--text-muted);
  font-size: 14px;
}

.empty-state .hint {
  font-size: 12px;
  color: var(--border-color);
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.history-date-group { display: flex; flex-direction: column; gap: 8px; }

.date-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-muted);
  padding-bottom: 4px;
  border-bottom: 1px solid var(--border-color);
}

.history-entry {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  padding: 12px 16px;
  border-left: 3px solid transparent;
}

.history-entry.forward { border-left-color: var(--accent-blue); }
.history-entry.backward { border-left-color: var(--accent-purple); }

.entry-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.entry-type {
  font-size: 12px;
  font-weight: 600;
  padding: 2px 8px;
  border-radius: 4px;
}

.history-entry.forward .entry-type {
  background: rgba(59, 130, 246, 0.15);
  color: var(--accent-blue);
}

.history-entry.backward .entry-type {
  background: rgba(168, 85, 247, 0.15);
  color: var(--accent-purple);
}

.entry-algo {
  font-size: 11px;
  color: var(--text-muted);
  background: var(--bg-tertiary);
  padding: 1px 6px;
  border-radius: 4px;
}

.entry-time {
  font-size: 11px;
  color: var(--text-muted);
  margin-left: auto;
}

.entry-detail {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.entry-facts,
.entry-new-facts,
.entry-goal {
  display: flex;
  align-items: flex-start;
  gap: 8px;
}

.detail-label {
  font-size: 12px;
  color: var(--text-muted);
  flex-shrink: 0;
  min-width: 60px;
  padding-top: 2px;
}

.fact-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.fact-tag {
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
}

.empty-tag {
  color: var(--border-color);
  font-size: 12px;
}

.new-fact-tag {
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.3);
  color: var(--accent-green);
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
}

.goal-text {
  color: var(--text-secondary);
  font-size: 13px;
  font-weight: 500;
}

.result-text {
  font-size: 13px;
  font-weight: 600;
}

.result-text.succeed { color: var(--accent-green); }
.result-text.failed { color: var(--accent-red); }
.result-text.unknown { color: var(--accent-yellow); }
</style>
