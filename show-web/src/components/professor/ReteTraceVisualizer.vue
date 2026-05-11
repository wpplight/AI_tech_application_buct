<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { ReteTraceEvent, ReteTopology } from '../../api/professor'

const props = defineProps<{
  trace: ReteTraceEvent[]
  topology: ReteTopology | null
}>()

const activePhase = ref<'alpha' | 'beta' | 'terminal' | 'all'>('all')
const expandedEvents = ref<Set<number>>(new Set())

const alphaEvents = computed(() => props.trace.filter(e => e.type === 'alpha_activate'))
const betaEvents = computed(() => props.trace.filter(e => e.type === 'beta_match'))
const terminalEvents = computed(() => props.trace.filter(e => e.type === 'terminal_fire'))

const filteredTrace = computed(() => {
  if (activePhase.value === 'all') return props.trace
  const typeMap: Record<string, string> = {
    alpha: 'alpha_activate',
    beta: 'beta_match',
    terminal: 'terminal_fire'
  }
  return props.trace.filter(e => e.type === typeMap[activePhase.value])
})

const uniqueAlphaConditions = computed(() => {
  if (!props.topology) return []
  return props.topology.alpha_nodes.map(n => n.condition)
})

const firedTerminals = computed(() => {
  if (!props.topology) return []
  return props.topology.terminals.filter(t => t.fired)
})

function toggleExpand(idx: number) {
  if (expandedEvents.value.has(idx)) {
    expandedEvents.value.delete(idx)
  } else {
    expandedEvents.value.add(idx)
  }
}

function getEventIcon(type: string): string {
  switch (type) {
    case 'alpha_activate': return 'A'
    case 'beta_match': return 'B'
    case 'terminal_fire': return 'T'
    default: return '?'
  }
}

function getEventColor(type: string): string {
  switch (type) {
    case 'alpha_activate': return 'var(--accent-blue)'
    case 'beta_match': return 'var(--accent-yellow)'
    case 'terminal_fire': return 'var(--accent-green)'
    default: return 'var(--text-muted)'
  }
}

function getPhaseLabel(type: string): string {
  switch (type) {
    case 'alpha_activate': return 'Alpha 网络'
    case 'beta_match': return 'Beta 网络'
    case 'terminal_fire': return '终端触发'
    default: return '未知'
  }
}

watch(() => props.trace, () => {
  expandedEvents.value.clear()
})
</script>

<template>
  <div class="rete-trace">
    <div class="trace-header">
      <h4 class="trace-title">Rete 网络传播过程</h4>
      <div class="trace-stats">
        <span class="stat-pill alpha">{{ alphaEvents.length }} Alpha</span>
        <span class="stat-pill beta">{{ betaEvents.length }} Beta</span>
        <span class="stat-pill terminal">{{ terminalEvents.length }} Terminal</span>
      </div>
    </div>

    <div v-if="topology" class="network-overview">
      <div class="overview-section">
        <div class="overview-label">网络结构</div>
        <div class="overview-grid">
          <div class="overview-card">
            <div class="card-icon alpha-icon">A</div>
            <div class="card-info">
              <span class="card-count">{{ topology.alpha_nodes.length }}</span>
              <span class="card-label">Alpha 节点</span>
            </div>
          </div>
          <div class="overview-card">
            <div class="card-icon beta-icon">B</div>
            <div class="card-info">
              <span class="card-count">{{ topology.beta_nodes.length }}</span>
              <span class="card-label">Beta 节点</span>
            </div>
          </div>
          <div class="overview-card">
            <div class="card-icon terminal-icon">T</div>
            <div class="card-info">
              <span class="card-count">{{ topology.terminals.length }}</span>
              <span class="card-label">Terminal 节点</span>
            </div>
          </div>
          <div class="overview-card fired">
            <div class="card-icon fired-icon">
              <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
                <polygon points="5 3 19 12 5 21 5 3"/>
              </svg>
            </div>
            <div class="card-info">
              <span class="card-count">{{ firedTerminals.length }}</span>
              <span class="card-label">已触发规则</span>
            </div>
          </div>
        </div>
      </div>

      <div v-if="firedTerminals.length > 0" class="fired-rules">
        <div class="overview-label">触发的规则</div>
        <div class="fired-list">
          <div v-for="t in firedTerminals" :key="t.rule_id" class="fired-item">
            <span class="fired-badge">R{{ t.rule_id }}</span>
            <span class="fired-cond">
              <span v-for="(c, i) in t.conditions" :key="i">
                <span class="cond-text">{{ c }}</span>
                <span v-if="i < t.conditions.length - 1" class="cond-and"> AND </span>
              </span>
            </span>
            <svg class="fired-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="5" y1="12" x2="19" y2="12"/>
              <polyline points="12 5 19 12 12 19"/>
            </svg>
            <span class="fired-conclusion">{{ t.conclusion }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="phase-filter">
      <button
        v-for="p in (['all', 'alpha', 'beta', 'terminal'] as const)"
        :key="p"
        class="phase-btn"
        :class="{ active: activePhase === p }"
        @click="activePhase = p"
      >
        {{ p === 'all' ? '全部' : p === 'alpha' ? 'Alpha 网络' : p === 'beta' ? 'Beta 网络' : 'Terminal' }}
      </button>
    </div>

    <div v-if="filteredTrace.length > 0" class="trace-flow">
      <div class="flow-label">传播链</div>
      <div class="flow-timeline">
        <div
          v-for="(event, idx) in filteredTrace"
          :key="idx"
          class="flow-event"
          :class="[event.type, { expanded: expandedEvents.has(idx) }]"
          @click="toggleExpand(idx)"
        >
          <div class="event-marker">
            <span class="marker-icon" :style="{ background: getEventColor(event.type) }">
              {{ getEventIcon(event.type) }}
            </span>
            <div class="marker-line" v-if="idx < filteredTrace.length - 1"></div>
          </div>

          <div class="event-body">
            <div class="event-header-row">
              <span class="event-phase" :style="{ color: getEventColor(event.type) }">
                {{ getPhaseLabel(event.type) }}
              </span>
              <span class="event-idx">#{{ idx + 1 }}</span>
            </div>

            <div v-if="event.type === 'alpha_activate'" class="event-detail">
              <div class="detail-row">
                <span class="detail-label">条件</span>
                <span class="detail-value condition">{{ event.condition }}</span>
              </div>
              <div class="detail-row">
                <span class="detail-label">事实</span>
                <span class="detail-value fact">{{ event.fact }}</span>
              </div>
              <div v-if="expandedEvents.has(idx)" class="detail-row">
                <span class="detail-label">下游</span>
                <span class="detail-value">{{ event.children }} 个 Beta 节点</span>
              </div>
            </div>

            <div v-else-if="event.type === 'beta_match'" class="event-detail">
              <div class="detail-row">
                <span class="detail-label">规则</span>
                <span class="detail-value rule">R{{ event.rule_id }}</span>
              </div>
              <div class="detail-row">
                <span class="detail-label">条件</span>
                <span class="detail-value condition">{{ event.condition }}</span>
              </div>
              <div v-if="event.matched_facts?.length" class="detail-row">
                <span class="detail-label">匹配</span>
                <div class="detail-tags">
                  <span v-for="f in event.matched_facts" :key="f" class="tag fact-tag">{{ f }}</span>
                </div>
              </div>
              <div v-if="event.is_chain_head !== undefined" class="detail-row">
                <span class="detail-label">类型</span>
                <span class="detail-value">{{ event.is_chain_head ? '链头节点' : '链式节点' }}</span>
              </div>
            </div>

            <div v-else-if="event.type === 'terminal_fire'" class="event-detail">
              <div class="terminal-fire-box">
                <div class="fire-conditions">
                  <span v-for="(c, i) in event.conditions" :key="i" class="fire-cond">
                    {{ c }}
                    <span v-if="i < (event.conditions?.length || 0) - 1" class="cond-and"> AND </span>
                  </span>
                </div>
                <svg class="fire-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="5" y1="12" x2="19" y2="12"/>
                  <polyline points="12 5 19 12 12 19"/>
                </svg>
                <span class="fire-conclusion">{{ event.conclusion }}</span>
              </div>
              <div v-if="event.matched_facts?.length && expandedEvents.has(idx)" class="detail-row">
                <span class="detail-label">事实</span>
                <div class="detail-tags">
                  <span v-for="f in event.matched_facts" :key="f" class="tag fact-tag">{{ f }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-else-if="trace.length === 0" class="empty-trace">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <circle cx="12" cy="12" r="10"/>
        <path d="M12 6v6l4 2"/>
      </svg>
      <p>执行 Rete 推理查看网络传播过程</p>
    </div>
  </div>
</template>

<style scoped>
.rete-trace {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.trace-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 8px;
}

.trace-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.trace-stats {
  display: flex;
  gap: 6px;
}

.stat-pill {
  padding: 2px 10px;
  border-radius: 12px;
  font-size: 11px;
  font-weight: 500;
}

.stat-pill.alpha {
  background: rgba(59, 130, 246, 0.15);
  color: var(--accent-blue);
  border: 1px solid rgba(59, 130, 246, 0.3);
}

.stat-pill.beta {
  background: rgba(245, 158, 11, 0.15);
  color: var(--accent-yellow);
  border: 1px solid rgba(245, 158, 11, 0.3);
}

.stat-pill.terminal {
  background: rgba(16, 185, 129, 0.15);
  color: var(--accent-green);
  border: 1px solid rgba(16, 185, 129, 0.3);
}

.network-overview {
  display: flex;
  flex-direction: column;
  gap: 14px;
  padding: 14px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
}

.overview-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 6px;
}

.overview-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}

@media (max-width: 640px) {
  .overview-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

.overview-card {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
}

.overview-card.fired {
  border-color: rgba(16, 185, 129, 0.4);
}

.card-icon {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 700;
  flex-shrink: 0;
}

.alpha-icon {
  background: rgba(59, 130, 246, 0.15);
  color: var(--accent-blue);
}

.beta-icon {
  background: rgba(245, 158, 11, 0.15);
  color: var(--accent-yellow);
}

.terminal-icon {
  background: rgba(16, 185, 129, 0.15);
  color: var(--accent-green);
}

.fired-icon {
  background: rgba(16, 185, 129, 0.2);
  color: var(--accent-green);
  width: 28px;
  height: 28px;
}

.fired-icon svg {
  width: 14px;
  height: 14px;
}

.card-info {
  display: flex;
  flex-direction: column;
}

.card-count {
  font-size: 18px;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1.1;
}

.card-label {
  font-size: 11px;
  color: var(--text-muted);
}

.fired-rules {
  display: flex;
  flex-direction: column;
}

.fired-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.fired-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: rgba(16, 185, 129, 0.06);
  border: 1px solid rgba(16, 185, 129, 0.2);
  border-radius: 8px;
  flex-wrap: wrap;
}

.fired-badge {
  padding: 2px 8px;
  background: rgba(59, 130, 246, 0.15);
  border: 1px solid rgba(59, 130, 246, 0.3);
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
  color: var(--accent-blue);
  font-family: monospace;
  flex-shrink: 0;
}

.fired-cond {
  display: flex;
  align-items: center;
  gap: 2px;
  flex-wrap: wrap;
}

.cond-text {
  font-size: 12px;
  color: var(--text-secondary);
}

.cond-and {
  font-size: 10px;
  color: var(--text-muted);
  font-weight: 600;
  margin: 0 2px;
}

.fired-arrow {
  width: 16px;
  height: 16px;
  color: var(--accent-green);
  flex-shrink: 0;
}

.fired-conclusion {
  font-size: 13px;
  font-weight: 600;
  color: var(--accent-green);
}

.phase-filter {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.phase-btn {
  padding: 6px 14px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-secondary);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.phase-btn:hover {
  border-color: var(--text-muted);
  color: var(--text-primary);
}

.phase-btn.active {
  background: var(--accent-blue);
  border-color: var(--accent-blue);
  color: white;
}

.trace-flow {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.flow-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.flow-timeline {
  display: flex;
  flex-direction: column;
  gap: 0;
  max-height: 500px;
  overflow-y: auto;
  padding-right: 4px;
}

.flow-event {
  display: flex;
  gap: 12px;
  cursor: pointer;
  transition: background 0.2s;
  padding: 4px 8px;
  border-radius: 8px;
}

.flow-event:hover {
  background: var(--bg-tertiary);
}

.event-marker {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex-shrink: 0;
  width: 28px;
}

.marker-icon {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  font-size: 11px;
  font-weight: 700;
  color: white;
  flex-shrink: 0;
  z-index: 1;
}

.marker-line {
  width: 2px;
  flex: 1;
  min-height: 8px;
  background: var(--border-color);
  margin: 2px 0;
}

.event-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding-bottom: 10px;
  min-width: 0;
}

.event-header-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.event-phase {
  font-size: 12px;
  font-weight: 600;
}

.event-idx {
  font-size: 10px;
  color: var(--text-muted);
  font-family: monospace;
}

.event-detail {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.detail-row {
  display: flex;
  align-items: flex-start;
  gap: 8px;
}

.detail-label {
  font-size: 11px;
  color: var(--text-muted);
  min-width: 32px;
  flex-shrink: 0;
  padding-top: 1px;
}

.detail-value {
  font-size: 12px;
  color: var(--text-secondary);
}

.detail-value.condition {
  color: var(--accent-blue);
  font-weight: 500;
}

.detail-value.fact {
  color: var(--accent-green);
  font-weight: 500;
}

.detail-value.rule {
  color: var(--accent-purple);
  font-weight: 600;
  font-family: monospace;
}

.detail-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.tag {
  padding: 1px 8px;
  border-radius: 6px;
  font-size: 11px;
}

.fact-tag {
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.25);
  color: var(--accent-green);
}

.terminal-fire-box {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: rgba(16, 185, 129, 0.06);
  border: 1px solid rgba(16, 185, 129, 0.2);
  border-radius: 8px;
  flex-wrap: wrap;
}

.fire-conditions {
  display: flex;
  align-items: center;
  gap: 2px;
  flex-wrap: wrap;
}

.fire-cond {
  font-size: 12px;
  color: var(--text-secondary);
}

.fire-arrow {
  width: 16px;
  height: 16px;
  color: var(--accent-green);
  flex-shrink: 0;
}

.fire-conclusion {
  font-size: 13px;
  font-weight: 600;
  color: var(--accent-green);
}

.empty-trace {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  gap: 12px;
  color: var(--text-muted);
  text-align: center;
}

.empty-trace svg {
  width: 36px;
  height: 36px;
  opacity: 0.4;
}

.empty-trace p {
  margin: 0;
  font-size: 13px;
}
</style>
