<script setup lang="ts">
import { ref, computed } from 'vue'
import type { InferenceStep, AlgorithmType, ReteTraceEvent, ReteTopology } from '../../api/professor'
import ReteTraceVisualizer from './ReteTraceVisualizer.vue'
import InferenceProcessVisualizer from './InferenceProcessVisualizer.vue'

const props = defineProps<{
  steps: InferenceStep[]
  factsCount: number
  inputFacts?: string[]
  missingFacts?: string[]
  success?: boolean
  goal?: string
  cacheHit?: boolean
  algorithm?: AlgorithmType
  newFacts?: string[]
  allFacts?: string[]
  reteTrace?: ReteTraceEvent[]
  networkTopology?: ReteTopology | null
}>()

const activeView = ref<'steps' | 'process' | 'rete'>('steps')

const isBackward = computed(() => props.inputFacts !== undefined && props.goal !== undefined)
const isRete = computed(() => props.algorithm === 'rete')
const hasTrace = computed(() => (props.reteTrace?.length ?? 0) > 0)

const viewTabs = computed(() => {
  const tabs: Array<{ id: 'steps' | 'process' | 'rete'; label: string }> = [
    { id: 'steps', label: '推理步骤' }
  ]
  if (!isBackward.value && props.steps.length > 0) {
    tabs.push({ id: 'process', label: '过程可视化' })
  }
  if (isRete.value && hasTrace.value) {
    tabs.push({ id: 'rete', label: 'Rete 网络' })
  }
  return tabs
})
</script>

<template>
  <div class="inference-result">
    <div class="result-header">
      <h4 class="section-label">推理结果</h4>
      <div v-if="steps.length > 0 || missingFacts?.length" class="stats">
        <span>{{ factsCount }} 事实</span>
        <span>{{ steps.length }} 步骤</span>
        <span v-if="cacheHit" class="cache-badge">缓存</span>
      </div>
    </div>

    <div v-if="isBackward" class="backward-info">
      <div class="info-row">
        <span class="info-label">目标</span>
        <span class="info-value goal">{{ goal }}</span>
      </div>
      <div class="info-row">
        <span class="info-label">输入事实</span>
        <div class="info-tags">
          <span
            v-for="fact in inputFacts"
            :key="fact"
            class="fact-tag"
          >{{ fact }}</span>
          <span v-if="!inputFacts?.length" class="info-empty">无条件</span>
        </div>
      </div>
      <div v-if="success === false && missingFacts?.length" class="info-row">
        <span class="info-label">缺失事实</span>
        <div class="info-tags missing">
          <span
            v-for="fact in missingFacts"
            :key="fact"
            class="fact-tag missing"
          >{{ fact }}</span>
        </div>
      </div>
      <div v-if="success === true" class="info-row">
        <span class="info-label">结果</span>
        <span class="info-value success">✓ 推理成功</span>
      </div>
      <div v-else-if="success === false" class="info-row">
        <span class="info-label">结果</span>
        <span class="info-value fail">✗ 推理失败（事实不足）</span>
      </div>
    </div>

    <div v-if="steps.length > 0 || hasTrace" class="view-tabs">
      <button
        v-for="tab in viewTabs"
        :key="tab.id"
        class="view-tab"
        :class="{ active: activeView === tab.id }"
        @click="activeView = tab.id"
      >
        {{ tab.label }}
      </button>
    </div>

    <div v-if="activeView === 'steps'" class="view-content">
      <div v-if="steps.length > 0" class="steps-list">
        <div class="steps-label">推理步骤</div>
        <div
          v-for="(step, index) in steps"
          :key="index"
          class="step-item"
          :class="step.type"
        >
          <div class="step-badge">{{ index + 1 }}</div>
          <div class="step-content">
            <div class="step-type">{{ step.type === 'forward' ? '正向推理' : '反向推理' }}</div>

            <div v-if="step.type === 'forward' && step.new_fact" class="inference-flow">
              <span v-for="(cond, idx) in step.rule_conditions" :key="idx" class="cond">
                {{ cond }}
              </span>
              <svg class="arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="5" y1="12" x2="19" y2="12"/>
                <polyline points="12 5 19 12 12 19"/>
              </svg>
              <span class="result">{{ step.new_fact }}</span>
            </div>

            <div v-else-if="step.type === 'backward'" class="backward-step">
              <div class="backward-goal-row">
                <span class="backward-goal-label">目标</span>
                <span class="backward-goal">{{ step.goal }}</span>
              </div>

              <template v-if="step.rule_id">
                <div class="rule-attempt">
                  <span class="rule-label">尝试规则 #{{ step.rule_id }}</span>
                  <div class="rule-conditions-row">
                    <span
                      v-for="(cond, idx) in step.rule_conditions"
                      :key="idx"
                      class="cond-tag"
                    >{{ cond }}</span>
                    <svg class="arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <line x1="5" y1="12" x2="19" y2="12"/>
                      <polyline points="12 5 19 12 12 19"/>
                    </svg>
                    <span class="conclusion-tag">{{ step.rule_conclusion || step.goal }}</span>
                  </div>
                </div>
              </template>

              <div v-if="step.result" class="backward-result" :class="{
                success: step.result === '已知事实' || step.result === '推理成功',
                fail: step.result === '规则条件不满足' || step.result === '推理失败',
                loop: step.result.includes('循环')
              }">
                <span v-if="step.result === '已知事实'">✓ 已知事实，推理成功</span>
                <span v-else-if="step.result === '推理成功'">✓ 推理成功</span>
                <span v-else-if="step.result === '循环依赖，跳过'">⚠ 循环依赖，跳过</span>
                <span v-else-if="step.result === '规则条件不满足'">✗ 规则条件不满足</span>
                <span v-else>{{ step.result }}</span>
              </div>

              <div v-if="step.conditions && step.conditions.length" class="need-facts">
                <span class="need-label">需要满足：</span>
                <span
                  v-for="(cond, idx) in step.conditions"
                  :key="idx"
                  class="cond-tag need"
                >{{ cond }}</span>
              </div>
            </div>

            <div v-if="step.rule_id" class="step-meta">规则 #{{ step.rule_id }}</div>
          </div>
        </div>
      </div>

      <div v-else-if="!isBackward" class="empty-result">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <circle cx="12" cy="12" r="10"/>
          <path d="M12 6v6l4 2"/>
        </svg>
        <p>执行推理查看结果</p>
      </div>
    </div>

    <div v-if="activeView === 'process' && !isBackward" class="view-content">
      <InferenceProcessVisualizer
        :steps="steps"
        :input-facts="inputFacts || []"
        :new-facts="newFacts || []"
        :all-facts="allFacts || []"
        :algorithm="algorithm || 'fullscan'"
      />
    </div>

    <div v-if="activeView === 'rete' && isRete" class="view-content">
      <ReteTraceVisualizer
        :trace="reteTrace || []"
        :topology="networkTopology || null"
      />
    </div>
  </div>
</template>

<style scoped>
.inference-result {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.section-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.stats {
  display: flex;
  gap: 8px;
  font-size: 12px;
  color: var(--text-muted);
}

.view-tabs {
  display: flex;
  gap: 4px;
  padding: 4px;
  background: var(--bg-tertiary);
  border-radius: 10px;
  border: 1px solid var(--border-color);
}

.view-tab {
  flex: 1;
  padding: 8px 14px;
  background: transparent;
  border: none;
  border-radius: 8px;
  color: var(--text-secondary);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  text-align: center;
}

.view-tab:hover {
  color: var(--text-primary);
  background: var(--bg-secondary);
}

.view-tab.active {
  background: var(--accent-blue);
  color: white;
}

.view-content {
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.backward-step {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.backward-goal-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.backward-goal-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--accent-purple);
  text-transform: uppercase;
}

.backward-goal {
  font-size: 14px;
  font-weight: 600;
  color: var(--accent-purple);
}

.rule-attempt {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 8px 10px;
  background: rgba(168, 85, 247, 0.06);
  border: 1px solid rgba(168, 85, 247, 0.15);
  border-radius: 8px;
}

.rule-label {
  font-size: 11px;
  color: var(--text-muted);
  font-family: monospace;
}

.rule-conditions-row {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 4px;
}

.cond-tag {
  padding: 2px 8px;
  background: rgba(59, 130, 246, 0.1);
  border: 1px solid rgba(59, 130, 246, 0.3);
  border-radius: 8px;
  font-size: 12px;
  color: var(--text-secondary);
}

.conclusion-tag {
  padding: 2px 8px;
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.3);
  border-radius: 8px;
  font-size: 12px;
  color: var(--accent-green);
  font-weight: 500;
}

.arrow {
  width: 12px;
  height: 12px;
  color: var(--text-muted);
  flex-shrink: 0;
}

.backward-result {
  padding: 6px 10px;
  border-radius: 8px;
  font-size: 12px;
  font-weight: 500;
}

.backward-result.success {
  background: rgba(16, 185, 129, 0.1);
  color: var(--accent-green);
}

.backward-result.fail {
  background: rgba(239, 68, 68, 0.1);
  color: var(--accent-red);
}

.backward-result.loop {
  background: rgba(245, 158, 11, 0.1);
  color: var(--accent-yellow);
}

.need-facts {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 4px;
}

.need-label {
  font-size: 11px;
  color: var(--text-muted);
}

.cond-tag.need {
  background: rgba(239, 68, 68, 0.08);
  border-color: rgba(239, 68, 68, 0.25);
  color: var(--accent-red);
  opacity: 0.8;
}

.cache-badge {
  padding: 1px 6px;
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.3);
  border-radius: 6px;
  color: var(--accent-green);
  font-size: 11px;
}

.backward-info {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 14px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
}

.info-row {
  display: flex;
  align-items: flex-start;
  gap: 12px;
}

.info-label {
  font-size: 12px;
  color: var(--text-muted);
  min-width: 56px;
  flex-shrink: 0;
  padding-top: 2px;
}

.info-value {
  font-size: 14px;
  font-weight: 500;
}

.info-value.goal {
  color: var(--accent-purple);
}

.info-value.success {
  color: var(--accent-green);
}

.info-value.fail {
  color: var(--accent-red);
}

.info-empty {
  font-size: 13px;
  color: var(--text-muted);
}

.info-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.fact-tag {
  padding: 3px 10px;
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.3);
  border-radius: 14px;
  font-size: 12px;
  color: var(--accent-green);
}

.fact-tag.missing {
  background: rgba(239, 68, 68, 0.1);
  border-color: rgba(239, 68, 68, 0.3);
  color: var(--accent-red);
}

.steps-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 4px;
}

.steps-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.step-item {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 12px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
}

.step-item.backward {
  border-left: 3px solid var(--accent-purple);
}

.step-badge {
  min-width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-tertiary);
  border-radius: 50%;
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  flex-shrink: 0;
}

.step-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.step-type {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.inference-flow {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 4px;
}

.cond {
  padding: 2px 8px;
  background: rgba(59, 130, 246, 0.1);
  border: 1px solid rgba(59, 130, 246, 0.3);
  border-radius: 6px;
  font-size: 12px;
  color: var(--text-secondary);
}

.result {
  padding: 2px 8px;
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.3);
  border-radius: 6px;
  font-size: 12px;
  color: var(--accent-green);
  font-weight: 500;
}

.goal {
  font-size: 13px;
  color: var(--text-primary);
}

.goal-result {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 2px;
}

.step-meta {
  font-size: 11px;
  color: var(--text-muted);
  font-family: monospace;
}

.empty-result {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  gap: 12px;
  color: var(--text-muted);
  text-align: center;
}

.empty-result svg {
  width: 40px;
  height: 40px;
  opacity: 0.4;
}

.empty-result p {
  margin: 0;
  font-size: 14px;
}
</style>
