<script setup lang="ts">
import type { AlgorithmType } from '../../api/professor'

defineProps<{
  mode: 'forward' | 'backward'
  algorithm: AlgorithmType
}>()

const emit = defineEmits<{
  'update:mode': [value: 'forward' | 'backward']
  'update:algorithm': [value: AlgorithmType]
  'execute': []
}>()

const algorithms = [
  { value: 'fullscan' as AlgorithmType, label: '全扫描' },
  { value: 'incremental' as AlgorithmType, label: '增量触发' },
  { value: 'rete' as AlgorithmType, label: 'Rete网络' }
]
</script>

<template>
  <div class="toolbar">
    <div class="toolbar-group">
      <button
        class="toolbar-btn"
        :class="{ active: mode === 'forward' }"
        @click="emit('update:mode', 'forward')"
      >
        正向推理
      </button>
      <button
        class="toolbar-btn"
        :class="{ active: mode === 'backward' }"
        @click="emit('update:mode', 'backward')"
      >
        反向推理
      </button>
    </div>

    <div class="toolbar-divider"></div>

    <div class="toolbar-group">
      <button
        v-for="algo in algorithms"
        :key="algo.value"
        class="toolbar-btn algo"
        :class="{ active: algorithm === algo.value }"
        @click="emit('update:algorithm', algo.value)"
      >
        {{ algo.label }}
      </button>
    </div>

    <button class="execute-btn" @click="emit('execute')">
      <svg viewBox="0 0 24 24" fill="currentColor">
        <polygon points="5 3 19 12 5 21 5 3"/>
      </svg>
    </button>
  </div>
</template>

<style scoped>
.toolbar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  flex-wrap: wrap;
}

.toolbar-group {
  display: flex;
  gap: 6px;
}

.toolbar-divider {
  width: 1px;
  height: 24px;
  background: var(--border-color);
}

.toolbar-btn {
  padding: 8px 16px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-secondary);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.toolbar-btn:hover {
  border-color: var(--text-muted);
  color: var(--text-primary);
}

.toolbar-btn.active {
  background: var(--accent-blue);
  border-color: var(--accent-blue);
  color: white;
}

.toolbar-btn.algo {
  font-size: 12px;
  padding: 8px 14px;
}

.execute-btn {
  margin-left: auto;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, var(--accent-green) 0%, var(--gradient-green-end) 100%);
  border: none;
  border-radius: 10px;
  color: white;
  cursor: pointer;
  transition: all 0.2s;
}

.execute-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
}

.execute-btn svg {
  width: 16px;
  height: 16px;
}
</style>
