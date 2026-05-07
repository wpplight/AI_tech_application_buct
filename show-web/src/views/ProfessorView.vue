<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useProfessorStore } from '../stores/professor'
import { useConditionSetStore } from '../stores/conditionSet'
import InferenceToolbar from '../components/professor/InferenceToolbar.vue'
import ConditionPanel from '../components/professor/ConditionPanel.vue'
import InferenceResult from '../components/professor/InferenceResult.vue'
import FactsManager from '../components/professor/FactsManager.vue'
import RulesManager from '../components/professor/RulesManager.vue'
import HistoryPanel from '../components/professor/HistoryPanel.vue'
import ConditionSetManager from '../components/professor/ConditionSetManager.vue'
import LoadingSpinner from '../components/common/LoadingSpinner.vue'
import ErrorMessage from '../components/common/ErrorMessage.vue'
import type { AlgorithmType } from '../api/professor'

const store = useProfessorStore()
const csStore = useConditionSetStore()

const activeTab = ref<'inference' | 'facts' | 'rules' | 'history' | 'conditionSets'>('inference')
const inferenceMode = ref<'forward' | 'backward'>('forward')
const selectedConditionSetId = ref<number | null>(null)

const tabs: Array<{ id: typeof activeTab.value; label: string; icon: string }> = [
  { id: 'inference', label: '推理控制', icon: 'cpu' },
  { id: 'conditionSets', label: '条件集', icon: 'layers' },
  { id: 'facts', label: '事实管理', icon: 'database' },
  { id: 'rules', label: '规则库', icon: 'book' },
  { id: 'history', label: '推理历史', icon: 'clock' }
]

onMounted(async () => {
  await store.checkConnection()
  if (store.isConnected) {
    await store.loadFacts()
    await store.loadRules()
  }
  store.loadPersistedHistory()
  store.loadAllFactsCount()
  await csStore.loadSets()
})

watch(inferenceMode, async (newMode, oldMode) => {
  if (newMode !== oldMode) {
    if (newMode === 'backward') {
      await store.switchToBackward()
    } else {
      await store.switchToForward()
    }
  }
})

async function handleForwardInference() {
  await store.forwardInference({
    conditionSetId: selectedConditionSetId.value ?? undefined,
    facts: selectedConditionSetId.value ? undefined : store.facts
  })
}

async function handleBackwardInference() {
  if (!store.backwardGoal.trim()) return
  await store.backwardInference(store.backwardGoal.trim(), csStore.selectedSetId ?? undefined)
}

function selectConditionSet(csId: number) {
  csStore.selectSet(csId)
  selectedConditionSetId.value = csId
  const cs = csStore.sets.find(s => s.id === csId)
  if (cs) {
    store.setFacts([...cs.facts])
  }
}

async function handleExecute() {
  if (inferenceMode.value === 'forward') {
    await handleForwardInference()
  } else {
    await handleBackwardInference()
  }
}

async function handleReset() {
  await store.reset()
}

function handleCloseError() {
  store.error = null
}

function switchTab(tab: 'inference' | 'facts' | 'rules' | 'history' | 'conditionSets') {
  activeTab.value = tab
  if (tab === 'conditionSets') {
    csStore.loadSets()
  }
}
</script>

<template>
  <div class="professor-container">
    <aside class="sidebar">
      <div class="sidebar-header">
        <h1 class="sidebar-title">专家系统</h1>
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
          @click="switchTab(tab.id)"
        >
          <svg v-if="tab.icon === 'cpu'" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <rect x="4" y="4" width="16" height="16" rx="2"/>
            <rect x="9" y="9" width="6" height="6"/>
            <path d="M9 1v3M15 1v3M9 20v3M15 20v3M20 9h3M20 14h3M1 9h3M1 14h3"/>
          </svg>
          <svg v-if="tab.icon === 'database'" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <ellipse cx="12" cy="5" rx="9" ry="3"/>
            <path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"/>
            <path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"/>
          </svg>
          <svg v-if="tab.icon === 'book'" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
            <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
          </svg>
          <svg v-if="tab.icon === 'clock'" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <circle cx="12" cy="12" r="10"/>
            <polyline points="12 6 12 12 16 14"/>
          </svg>
          <svg v-if="tab.icon === 'layers'" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <polygon points="12 2 2 7 12 12 22 7 12 2"/>
            <polyline points="2 17 12 22 22 17"/>
            <polyline points="2 12 12 17 22 12"/>
          </svg>
          <span class="nav-label">{{ tab.label }}</span>
        </button>
      </nav>

      <div class="sidebar-footer">
        <div class="stats-mini">
          <div class="stat-item">
            <span class="stat-value">{{ store.allFactsCount }}</span>
            <span class="stat-label">事实</span>
          </div>
          <div class="stat-divider"></div>
          <div class="stat-item">
            <span class="stat-value">{{ store.rulesCount }}</span>
            <span class="stat-label">规则</span>
          </div>
        </div>
      </div>
    </aside>

    <main class="main-content">
      <div v-if="store.error" class="error-banner">
        <ErrorMessage :message="store.error" @close="handleCloseError" />
      </div>

      <!-- 推理控制 -->
      <div v-show="activeTab === 'inference'" class="content-section">
        <div class="section-header">
          <h2 class="section-title">推理控制</h2>
          <p class="section-desc">选择条件集并执行推理</p>
        </div>

        <InferenceToolbar
          v-model:mode="inferenceMode"
          v-model:algorithm="store.currentAlgorithm"
          @execute="handleExecute"
        />

        <div class="workspace">
          <div class="workspace-left">
            <div v-if="csStore.sets.length > 0 && inferenceMode === 'forward'" class="cs-cards">
              <button
                v-for="cs in csStore.sets"
                :key="cs.id"
                class="cs-card-btn"
                :class="{ selected: selectedConditionSetId === cs.id }"
                @click="selectConditionSet(cs.id)"
              >
                <span class="cs-card-name">{{ cs.name }}</span>
                <span class="cs-card-count">{{ cs.facts.length }} 个条件</span>
              </button>
            </div>

            <ConditionPanel
              :mode="inferenceMode"
              @execute="handleExecute"
            />
          </div>

          <div class="workspace-right">
            <InferenceResult
              :steps="store.inferenceSteps"
              :facts-count="store.factsCount"
              :input-facts="store.currentResult?.input_facts"
              :missing-facts="store.currentResult?.missing_facts"
              :success="store.currentResult?.success"
              :goal="store.currentResult?.goal"
              :cache-hit="store.currentResult?.cache_hit"
            />
          </div>
        </div>
      </div>

      <!-- 条件集管理 -->
      <div v-show="activeTab === 'conditionSets'" class="content-section">
        <div class="section-header">
          <h2 class="section-title">条件集管理</h2>
          <p class="section-desc">创建和管理条件集，推理时直接选择使用</p>
        </div>

        <ConditionSetManager />
      </div>

      <!-- 事实管理 -->
      <div v-show="activeTab === 'facts'" class="content-section">
        <div class="section-header">
          <h2 class="section-title">事实管理</h2>
          <p class="section-desc">搜索事实，查看相关规则</p>
        </div>

        <FactsManager />
      </div>

      <!-- 规则库 -->
      <div v-show="activeTab === 'rules'" class="content-section">
        <RulesManager />
      </div>

      <!-- 推理历史 -->
      <div v-show="activeTab === 'history'" class="content-section">
        <HistoryPanel />
      </div>

      <div v-if="store.loading" class="loading-overlay">
        <LoadingSpinner size="lg" />
      </div>
    </main>
  </div>
</template>

<style scoped>
.professor-container {
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
  color: var(--accent-red);
  transition: all 0.3s ease;
}

.connection-badge.connected {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.3);
  color: var(--accent-green);
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
  padding: 16px 20px;
  border-top: 1px solid var(--border-color);
  flex-shrink: 0;
}

@media (max-width: 768px) {
  .sidebar-footer {
    padding: 12px 16px;
  }
}

.theme-toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
}

.stats-mini {
  display: flex;
  align-items: center;
  justify-content: space-around;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.stat-value {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
}

.stat-label {
  font-size: 11px;
  color: var(--text-muted);
}

.stat-divider {
  width: 1px;
  height: 32px;
  background: var(--border-color);
}

.main-content {
  flex: 1;
  margin-left: 280px;
  padding: 32px;
  max-width: 1600px;
}

@media (max-width: 768px) {
  .main-content {
    margin-left: 0;
    padding: 20px 16px;
  }
}

.error-banner {
  margin-bottom: 24px;
}

.content-section {
  display: flex;
  flex-direction: column;
  gap: 24px;
  animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

.section-header {
  margin-bottom: 8px;
}

.cs-cards {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 16px;
}

.cs-card-btn {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 2px;
  padding: 8px 14px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s;
  min-width: 100px;
}

.cs-card-btn:hover {
  border-color: var(--accent-blue);
  background: var(--bg-tertiary);
}

.cs-card-btn.selected {
  border-color: var(--accent-blue);
  background: rgba(59, 130, 246, 0.1);
}

.cs-card-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  white-space: nowrap;
}

.cs-card-count {
  font-size: 11px;
  color: var(--text-muted);
}

.cs-card-btn.selected .cs-card-name {
  color: var(--accent-blue);
}

.section-title {
  font-size: 28px;
  font-weight: 700;
  letter-spacing: -0.02em;
  margin-bottom: 8px;
  color: var(--text-primary);
}

.section-desc {
  color: var(--text-secondary);
  font-size: 14px;
}

.workspace {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
  align-items: start;
}

@media (max-width: 1024px) {
  .workspace {
    grid-template-columns: 1fr;
  }
}

.workspace-left,
.workspace-right {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  padding: 24px;
  min-height: 600px;
}

@media (max-width: 1024px) {
  .workspace-left,
  .workspace-right {
    min-height: auto;
  }
}

.card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  padding: 24px;
}

.rules-container {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.rules-stats {
  display: flex;
  gap: 32px;
  padding-bottom: 24px;
  border-bottom: 1px solid var(--border-color);
}

.rule-stat {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.stat-number {
  font-size: 28px;
  font-weight: 700;
  color: var(--accent-blue);
}

.stat-label {
  font-size: 13px;
  color: var(--text-muted);
}

.rules-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  max-height: 600px;
  overflow-y: auto;
}

.rule-item {
  padding: 16px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  transition: all 0.3s ease;
}

.rule-item:hover {
  border-color: var(--text-muted);
}

.rule-id {
  font-size: 12px;
  color: var(--text-muted);
  font-family: monospace;
  display: block;
  margin-bottom: 12px;
}

.rule-content {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.rule-conditions {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.condition-tag {
  padding: 4px 10px;
  background: rgba(59, 130, 246, 0.1);
  border: 1px solid rgba(59, 130, 246, 0.3);
  border-radius: 12px;
  font-size: 12px;
  color: var(--accent-blue);
}

.rule-arrow {
  width: 16px;
  height: 16px;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.rule-conclusion {
  padding: 4px 10px;
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.3);
  border-radius: 12px;
  font-size: 12px;
  color: var(--accent-green);
  font-weight: 500;
}

.rules-more {
  text-align: center;
  color: var(--text-muted);
  font-size: 13px;
  padding: 16px;
}

.history-container {
  max-height: 700px;
  overflow-y: auto;
}

.history-item {
  display: flex;
  gap: 16px;
  padding: 16px;
  background: var(--bg-tertiary);
  border-radius: 12px;
  margin-bottom: 12px;
  border-left: 3px solid;
  transition: all 0.3s ease;
}

.history-item:hover {
  transform: translateX(4px);
}

.history-item.forward {
  border-left-color: var(--accent-green);
}

.history-item.backward {
  border-left-color: var(--accent-yellow);
}

.history-item:last-child {
  margin-bottom: 0;
}

.history-number {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  background: var(--border-color);
  border-radius: 50%;
  font-size: 12px;
  font-weight: 600;
  flex-shrink: 0;
}

.history-item.forward .history-number {
  background: rgba(16, 185, 129, 0.2);
  color: var(--accent-green);
}

.history-item.backward .history-number {
  background: rgba(245, 158, 11, 0.2);
  color: var(--accent-yellow);
}

.history-content {
  flex: 1;
}

.history-type {
  font-size: 11px;
  color: var(--text-secondary);
  margin-bottom: 8px;
  font-weight: 500;
}

.history-flow {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
  padding: 10px;
  background: var(--bg-primary);
  border-radius: 8px;
  margin-bottom: 8px;
}

.cond-text {
  font-size: 12px;
  color: var(--text-secondary);
}

.cond-plus {
  color: var(--text-muted);
  margin: 0 2px;
}

.rule-icon {
  width: 16px;
  height: 16px;
  color: var(--accent-blue);
}

.new-fact {
  font-size: 13px;
  color: var(--accent-green);
  font-weight: 500;
}

.history-goal {
  font-size: 13px;
  color: var(--accent-yellow);
  font-weight: 500;
  margin-bottom: 8px;
}

.goal-result {
  font-size: 12px;
  color: var(--text-secondary);
}

.step-meta {
  font-size: 11px;
  color: var(--text-muted);
}

.history-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: var(--text-muted);
  text-align: center;
}

.history-empty svg {
  width: 48px;
  height: 48px;
  margin-bottom: 16px;
  opacity: 0.5;
}

.history-empty p {
  margin: 0;
}

.history-empty .empty-hint {
  font-size: 12px;
  margin-top: 8px;
}

.loading-overlay {
  position: fixed;
  inset: 0;
  background: rgba(9, 9, 11, 0.8);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}
</style>
