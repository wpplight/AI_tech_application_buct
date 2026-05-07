<script setup lang="ts">
defineProps<{
  facts: string[]
  loading?: boolean
}>()

const emit = defineEmits<{
  'add': [fact: string]
  'remove': [fact: string]
}>()

const availableFacts = [
  '有脊索', '体温恒定', '身上有毛发', '会哺乳', '能飞行', '有羽毛',
  '会游泳', '有鳞片', '有爪', '有犬齿', '吃肉', '能爪抓',
  '有斑点', '有条纹', '能鸣叫', '善跑', '有长颈',
  '有长腿', '是黑白两色', '会偷东西', '有黑条纹', '有暗斑点'
]
</script>

<template>
  <div class="condition-manager">
    <div class="current-conditions">
      <h4 class="section-label">当前条件</h4>
      <div class="facts-list">
        <div
          v-for="fact in facts"
          :key="fact"
          class="fact-tag"
        >
          <span>{{ fact }}</span>
          <button @click="emit('remove', fact)">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"/>
              <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </button>
        </div>
        <div v-if="facts.length === 0" class="empty">
          暂无条件
        </div>
      </div>
    </div>

    <div class="quick-add">
      <h4 class="section-label">快速添加</h4>
      <div class="facts-grid">
        <button
          v-for="fact in availableFacts"
          :key="fact"
          class="fact-btn"
          :class="{ selected: facts.includes(fact) }"
          :disabled="facts.includes(fact)"
          @click="emit('add', fact)"
        >
          {{ fact }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.condition-manager {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.section-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 12px;
}

.current-conditions {
  min-height: 80px;
}

.facts-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.fact-tag {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.3);
  border-radius: 18px;
  font-size: 13px;
  color: var(--accent-green);
}

.fact-tag button {
  padding: 2px;
  background: transparent;
  border: none;
  color: currentColor;
  cursor: pointer;
  opacity: 0.6;
  transition: all 0.2s;
  display: flex;
  align-items: center;
}

.fact-tag button:hover {
  opacity: 1;
  transform: rotate(90deg);
}

.fact-tag button svg {
  width: 12px;
  height: 12px;
}

.empty {
  color: var(--text-muted);
  font-size: 13px;
  padding: 20px;
  text-align: center;
}

.quick-add {
  padding-top: 16px;
  border-top: 1px solid var(--border-color);
}

.facts-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  max-height: 180px;
  overflow-y: auto;
}

.fact-btn {
  padding: 6px 12px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 14px;
  color: var(--text-primary);
  font-size: 11px;
  cursor: pointer;
  transition: all 0.2s;
}

.fact-btn:hover:not(:disabled) {
  background: var(--border-color);
  border-color: var(--text-muted);
}

.fact-btn.selected {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.5);
  color: var(--accent-green);
  cursor: default;
}
</style>
