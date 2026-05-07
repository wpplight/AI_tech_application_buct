<script setup lang="ts">
import { computed } from 'vue'
import type { AlgorithmType } from '../../api/professor'
import LoadingSpinner from '../common/LoadingSpinner.vue'

const props = defineProps<{
  mode: 'forward' | 'backward'
  algorithm: AlgorithmType
  loading: boolean
  goal: string
}>()

const emit = defineEmits<{
  'update:mode': [value: 'forward' | 'backward']
  'update:algorithm': [value: AlgorithmType]
  'update:goal': [value: string]
  'execute': []
  'reset': []
}>()

const algorithms = [
  { value: 'fullscan' as AlgorithmType, label: '全扫描' },
  { value: 'incremental' as AlgorithmType, label: '增量触发' },
  { value: 'rete' as AlgorithmType, label: 'Rete' }
]

const canExecute = computed(() => {
  if (props.loading) return false
  if (props.mode === 'backward' && !props.goal.trim()) return false
  return true
})
</script>

<template>
  <div class="controls">
    <div class="controls-row">
      <div class="mode-switch">
        <button
          class="mode-btn"
          :class="{ active: mode === 'forward' }"
          @click="emit('update:mode', 'forward')"
        >
          正向推理
        </button>
        <button
          class="mode-btn"
          :class="{ active: mode === 'backward' }"
          @click="emit('update:mode', 'backward')"
        >
          反向推理
        </button>
      </div>

      <div class="algo-switch">
        <button
          v-for="algo in algorithms"
          :key="algo.value"
          class="algo-btn"
          :class="{ active: algorithm === algo.value }"
          @click="emit('update:algorithm', algo.value)"
        >
          {{ algo.label }}
        </button>
      </div>
    </div>

    <div v-if="mode === 'backward'" class="goal-input">
      <input
        type="text"
        :value="goal"
        placeholder="输入目标结论..."
        @input="emit('update:goal', ($event.target as HTMLInputElement).value)"
        @keyup.enter="emit('execute')"
      />
    </div>

    <div class="actions">
      <button
        class="execute-btn"
        :class="mode"
        :disabled="!canExecute"
        @click="emit('execute')"
      >
        <LoadingSpinner v-if="loading" size="sm" />
        <template v-else>
          {{ mode === 'forward' ? '开始推理' : '验证目标' }}
        </template>
      </button>
      <button class="reset-btn" @click="emit('reset')">
        重置
      </button>
    </div>
  </div>
</template>

<style scoped>
.inference-controls {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.mode-group {
  display: flex;
  gap: 4px;
  padding: 4px;
  background: var(--bg-tertiary);
  border-radius: 10px;
}

.mode-btn {
  padding: 8px 16px;
  background: transparent;
  border: none;
  border-radius: 8px;
  color: var(--text-secondary);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.mode-btn:hover {
  color: var(--text-primary);
}

.mode-btn.active {
  background: var(--accent-blue);
  color: white;
}

.algo-switch {
  display: flex;
  gap: 6px;
}

.algo-btn {
  padding: 8px 14px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-secondary);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.algo-btn:hover {
  border-color: var(--text-muted);
  color: var(--text-primary);
}

.algo-btn.active {
  background: rgba(59, 130, 246, 0.1);
  border-color: var(--accent-blue);
  color: var(--accent-blue);
}

.goal-input input {
  width: 100%;
  padding: 12px 14px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  color: var(--text-primary);
  font-size: 13px;
  transition: all 0.2s;
}

.goal-input input:focus {
  outline: none;
  border-color: var(--accent-blue);
}

.actions {
  display: flex;
  gap: 10px;
}

.execute-btn {
  flex: 1;
  padding: 12px 20px;
  border: none;
  border-radius: 10px;
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.execute-btn.forward {
  background: linear-gradient(135deg, var(--accent-green) 0%, var(--gradient-green-end) 100%);
}

.execute-btn.backward {
  background: linear-gradient(135deg, var(--gradient-yellow-start) 0%, var(--gradient-yellow-end) 100%);
}

.execute-btn:hover:not(:disabled) {
  transform: translateY(-1px);
}

.execute-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.reset-btn {
  padding: 12px 20px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 10px;
  color: var(--text-muted);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.reset-btn:hover {
  border-color: var(--text-muted);
  color: var(--text-secondary);
}
</style>
