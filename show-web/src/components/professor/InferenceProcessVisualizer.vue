<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { InferenceStep, AlgorithmType } from '../../api/professor'

const props = defineProps<{
  steps: InferenceStep[]
  inputFacts: string[]
  newFacts: string[]
  allFacts: string[]
  algorithm: AlgorithmType
}>()

const activeStep = ref<number | null>(null)
const showAllFacts = ref(false)

const forwardSteps = computed(() => props.steps.filter(s => s.type === 'forward'))

const iterationGroups = computed(() => {
  const groups: Map<number, InferenceStep[]> = new Map()
  for (const step of forwardSteps.value) {
    const iter = step.iteration ?? 0
    if (!groups.has(iter)) groups.set(iter, [])
    groups.get(iter)!.push(step)
  }
  return Array.from(groups.entries()).sort((a, b) => a[0] - b[0])
})

function toggleStep(idx: number) {
  activeStep.value = activeStep.value === idx ? null : idx
}

watch(() => props.steps, () => {
  activeStep.value = null
})

watch(() => props.allFacts, (newVal, oldVal) => {
  if (newVal.length > 0 && (oldVal.length === 0 || newVal.length > oldVal.length)) {
    showAllFacts.value = true
  }
})
</script>

<template>
  <div class="process-viz">
    <div class="viz-header">
      <h4 class="viz-title">推理过程可视化</h4>
      <div class="viz-summary">
        <span class="summary-item input">{{ inputFacts.length }} 输入</span>
        <span class="summary-item new">+{{ newFacts.length }} 推导</span>
        <span class="summary-item total">= {{ allFacts.length }} 总计</span>
      </div>
    </div>

    <div class="facts-overview">
      <div class="facts-section input-facts">
        <div class="facts-label">输入事实</div>
        <div class="facts-tags">
          <span v-for="f in inputFacts" :key="f" class="fact-tag input">{{ f }}</span>
          <span v-if="inputFacts.length === 0" class="empty-hint">无输入事实</span>
        </div>
      </div>
      <div v-if="newFacts.length > 0" class="facts-section new-facts">
        <div class="facts-label">推导出的新事实</div>
        <div class="facts-tags">
          <span v-for="f in newFacts" :key="f" class="fact-tag new">{{ f }}</span>
        </div>
      </div>
    </div>

    <div v-if="forwardSteps.length > 0" class="steps-visual">
      <div class="steps-label">推理步骤链</div>

      <div v-if="algorithm === 'rete'" class="rete-steps">
        <div
          v-for="(group, gIdx) in iterationGroups"
          :key="gIdx"
          class="iteration-group"
        >
          <div class="iter-header">
            <span class="iter-badge">迭代 {{ group[0] }}</span>
            <span class="iter-count">{{ group.length }} 个新事实</span>
          </div>
          <div class="iter-steps">
            <div
              v-for="(step, sIdx) in group[1]"
              :key="sIdx"
              class="step-card rete-step"
              :class="{ active: activeStep === gIdx * 100 + sIdx }"
              @click="toggleStep(gIdx * 100 + sIdx)"
            >
              <div class="step-left">
                <div class="step-num">{{ gIdx + 1 }}.{{ sIdx + 1 }}</div>
                <div class="step-connector" v-if="sIdx < group[1].length - 1"></div>
              </div>
              <div class="step-content">
                <div class="step-flow">
                  <span class="flow-from">
                    <span v-for="(af, i) in (step.accumulated_facts || []).slice(0, 3)" :key="i" class="mini-tag">{{ af }}</span>
                    <span v-if="(step.accumulated_facts?.length || 0) > 3" class="mini-tag more">+{{ (step.accumulated_facts?.length || 0) - 3 }}</span>
                  </span>
                  <svg class="flow-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="5" y1="12" x2="19" y2="12"/>
                    <polyline points="12 5 19 12 12 19"/>
                  </svg>
                  <span class="flow-to new-fact-highlight">{{ step.new_fact }}</span>
                </div>
                <div v-if="activeStep === gIdx * 100 + sIdx && step.accumulated_facts?.length" class="step-expanded">
                  <div class="expanded-label">当前已知事实</div>
                  <div class="expanded-facts">
                    <span v-for="af in step.accumulated_facts" :key="af" class="mini-tag accumulated">{{ af }}</span>
                    <span class="mini-tag plus">+ {{ step.new_fact }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div v-else class="standard-steps">
        <div
          v-for="(step, idx) in forwardSteps"
          :key="idx"
          class="step-card"
          :class="{ active: activeStep === idx }"
          @click="toggleStep(idx)"
        >
          <div class="step-left">
            <div class="step-num">{{ idx + 1 }}</div>
            <div class="step-connector" v-if="idx < forwardSteps.length - 1"></div>
          </div>
          <div class="step-content">
            <div class="step-rule">
              <span class="rule-badge">R{{ step.rule_id }}</span>
              <div class="rule-conditions">
                <span v-for="(c, i) in step.rule_conditions" :key="i" class="cond-tag">
                  {{ c }}
                  <span v-if="i < (step.rule_conditions?.length || 0) - 1" class="cond-and">AND</span>
                </span>
              </div>
              <svg class="flow-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="5" y1="12" x2="19" y2="12"/>
                <polyline points="12 5 19 12 12 19"/>
              </svg>
              <span class="conclusion-tag">{{ step.rule_conclusion }}</span>
            </div>
            <div class="step-flow">
              <span class="flow-from">
                <span v-for="(c, i) in step.rule_conditions" :key="i" class="mini-tag">{{ c }}</span>
              </span>
              <svg class="flow-arrow small" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="5" y1="12" x2="19" y2="12"/>
                <polyline points="12 5 19 12 12 19"/>
              </svg>
              <span class="flow-to new-fact-highlight">{{ step.new_fact }}</span>
            </div>
            <div v-if="activeStep === idx && step.accumulated_facts?.length" class="step-expanded">
              <div class="expanded-label">当前已知事实</div>
              <div class="expanded-facts">
                <span v-for="af in step.accumulated_facts" :key="af" class="mini-tag accumulated">{{ af }}</span>
                <span class="mini-tag plus">+ {{ step.new_fact }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-if="allFacts.length > 0" class="final-result">
      <div class="result-header" @click="showAllFacts = !showAllFacts">
        <span class="result-label">最终结果</span>
        <span class="result-toggle">{{ showAllFacts ? '收起' : '展开' }}</span>
      </div>
      <div v-if="showAllFacts" class="result-facts">
        <span v-for="f in allFacts" :key="f" class="fact-tag final" :class="{ is_new: !inputFacts.includes(f) }">
          {{ f }}
          <span v-if="!inputFacts.includes(f)" class="new-badge">新</span>
        </span>
      </div>
    </div>

    <div v-else-if="steps.length === 0" class="empty-process">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <circle cx="12" cy="12" r="10"/>
        <path d="M12 6v6l4 2"/>
      </svg>
      <p>执行推理查看过程可视化</p>
    </div>
  </div>
</template>

<style scoped>
.process-viz {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.viz-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 8px;
}

.viz-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.viz-summary {
  display: flex;
  gap: 6px;
}

.summary-item {
  padding: 2px 10px;
  border-radius: 12px;
  font-size: 11px;
  font-weight: 500;
}

.summary-item.input {
  background: rgba(59, 130, 246, 0.15);
  color: var(--accent-blue);
  border: 1px solid rgba(59, 130, 246, 0.3);
}

.summary-item.new {
  background: rgba(16, 185, 129, 0.15);
  color: var(--accent-green);
  border: 1px solid rgba(16, 185, 129, 0.3);
}

.summary-item.total {
  background: rgba(168, 85, 247, 0.15);
  color: var(--accent-purple);
  border: 1px solid rgba(168, 85, 247, 0.3);
}

.facts-overview {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.facts-section {
  padding: 12px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
}

.facts-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 8px;
}

.facts-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.fact-tag {
  padding: 3px 10px;
  border-radius: 14px;
  font-size: 12px;
  font-weight: 500;
}

.fact-tag.input {
  background: rgba(59, 130, 246, 0.1);
  border: 1px solid rgba(59, 130, 246, 0.3);
  color: var(--accent-blue);
}

.fact-tag.new {
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.3);
  color: var(--accent-green);
}

.fact-tag.final {
  background: rgba(168, 85, 247, 0.08);
  border: 1px solid rgba(168, 85, 247, 0.2);
  color: var(--text-secondary);
  position: relative;
}

.fact-tag.final.is_new {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.3);
  color: var(--accent-green);
}

.new-badge {
  font-size: 9px;
  padding: 0 4px;
  margin-left: 4px;
  background: rgba(16, 185, 129, 0.3);
  border-radius: 4px;
  color: var(--accent-green);
  font-weight: 700;
}

.empty-hint {
  font-size: 12px;
  color: var(--text-muted);
  font-style: italic;
}

.steps-visual {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.steps-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.iteration-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 8px;
}

.iter-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 0;
}

.iter-badge {
  padding: 2px 10px;
  background: rgba(168, 85, 247, 0.15);
  border: 1px solid rgba(168, 85, 247, 0.3);
  border-radius: 10px;
  font-size: 11px;
  font-weight: 600;
  color: var(--accent-purple);
}

.iter-count {
  font-size: 11px;
  color: var(--text-muted);
}

.iter-steps {
  display: flex;
  flex-direction: column;
  gap: 0;
  padding-left: 8px;
  border-left: 2px solid rgba(168, 85, 247, 0.2);
}

.step-card {
  display: flex;
  gap: 10px;
  cursor: pointer;
  padding: 6px 8px;
  border-radius: 8px;
  transition: background 0.2s;
}

.step-card:hover {
  background: var(--bg-tertiary);
}

.step-card.active {
  background: var(--bg-tertiary);
}

.step-left {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex-shrink: 0;
  width: 28px;
}

.step-num {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 50%;
  font-size: 10px;
  font-weight: 600;
  color: var(--text-muted);
  flex-shrink: 0;
  z-index: 1;
}

.step-card.active .step-num {
  background: var(--accent-blue);
  border-color: var(--accent-blue);
  color: white;
}

.step-connector {
  width: 2px;
  flex: 1;
  min-height: 6px;
  background: var(--border-color);
  margin: 2px 0;
}

.step-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding-bottom: 8px;
  min-width: 0;
}

.step-rule {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
}

.rule-badge {
  padding: 1px 7px;
  background: rgba(168, 85, 247, 0.15);
  border: 1px solid rgba(168, 85, 247, 0.3);
  border-radius: 6px;
  font-size: 10px;
  font-weight: 600;
  color: var(--accent-purple);
  font-family: monospace;
  flex-shrink: 0;
}

.rule-conditions {
  display: flex;
  gap: 3px;
  flex-wrap: wrap;
  align-items: center;
}

.cond-tag {
  font-size: 11px;
  color: var(--text-secondary);
}

.cond-and {
  font-size: 9px;
  color: var(--text-muted);
  font-weight: 600;
  margin: 0 2px;
}

.conclusion-tag {
  padding: 1px 7px;
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.25);
  border-radius: 6px;
  font-size: 11px;
  color: var(--accent-green);
  font-weight: 500;
}

.step-flow {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
}

.flow-from {
  display: flex;
  align-items: center;
  gap: 3px;
  flex-wrap: wrap;
}

.mini-tag {
  padding: 1px 6px;
  background: rgba(59, 130, 246, 0.08);
  border: 1px solid rgba(59, 130, 246, 0.2);
  border-radius: 6px;
  font-size: 10px;
  color: var(--text-secondary);
  white-space: nowrap;
}

.mini-tag.more {
  background: rgba(168, 85, 247, 0.1);
  border-color: rgba(168, 85, 247, 0.25);
  color: var(--accent-purple);
}

.mini-tag.accumulated {
  background: rgba(59, 130, 246, 0.06);
  border-color: rgba(59, 130, 246, 0.15);
  color: var(--text-muted);
}

.mini-tag.plus {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.3);
  color: var(--accent-green);
  font-weight: 600;
}

.flow-arrow {
  width: 16px;
  height: 16px;
  color: var(--text-muted);
  flex-shrink: 0;
}

.flow-arrow.small {
  width: 12px;
  height: 12px;
}

.flow-to {
  font-size: 12px;
  color: var(--text-secondary);
}

.new-fact-highlight {
  padding: 2px 8px;
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.3);
  border-radius: 8px;
  color: var(--accent-green);
  font-weight: 600;
  font-size: 12px;
}

.step-expanded {
  margin-top: 6px;
  padding: 8px 10px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
}

.expanded-label {
  font-size: 10px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.3px;
  margin-bottom: 6px;
}

.expanded-facts {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.final-result {
  padding: 12px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
}

.result-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.result-toggle {
  font-size: 11px;
  color: var(--accent-blue);
  cursor: pointer;
}

.result-facts {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: 10px;
}

.empty-process {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  gap: 12px;
  color: var(--text-muted);
  text-align: center;
}

.empty-process svg {
  width: 36px;
  height: 36px;
  opacity: 0.4;
}

.empty-process p {
  margin: 0;
  font-size: 13px;
}
</style>
