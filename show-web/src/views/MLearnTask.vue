<template>
  <div class="task-page">
    <h2 class="page-title">创建训练任务</h2>

    <div class="form-card">
      <div class="form-group">
        <label class="form-label">算法类型</label>
        <div class="radio-group">
          <label class="radio-item" :class="{ active: mlearn.algorithm === 'regression' }">
            <input type="radio" v-model="mlearn.algorithm" value="regression" />
            <span>回归拟合</span>
          </label>
          <label class="radio-item" :class="{ active: mlearn.algorithm === 'genetic' }">
            <input type="radio" v-model="mlearn.algorithm" value="genetic" />
            <span>遗传算法</span>
          </label>
        </div>
      </div>

      <div class="form-group" v-if="mlearn.algorithm === 'regression'">
        <label class="form-label">拟合函数</label>
        <div class="radio-group">
          <label class="radio-item" :class="{ active: mlearn.regressionFn === 'linear' }">
            <input type="radio" v-model="mlearn.regressionFn" value="linear" />
            <span>线性 y=2x+1</span>
          </label>
          <label class="radio-item" :class="{ active: mlearn.regressionFn === 'quadratic' }">
            <input type="radio" v-model="mlearn.regressionFn" value="quadratic" />
            <span>二次 y=x²</span>
          </label>
          <label class="radio-item" :class="{ active: mlearn.regressionFn === 'sinusoidal' }">
            <input type="radio" v-model="mlearn.regressionFn" value="sinusoidal" />
            <span>正弦 y=sin(x)</span>
          </label>
        </div>
      </div>

      <div class="form-group" v-if="mlearn.algorithm === 'genetic'">
        <label class="form-label">目标函数</label>
        <div class="radio-group">
          <label class="radio-item" :class="{ active: mlearn.geneticFn === 'ackley' }">
            <input type="radio" v-model="mlearn.geneticFn" value="ackley" />
            <span>Ackley (1D)</span>
          </label>
          <label class="radio-item" :class="{ active: mlearn.geneticFn === 'rastrigin_variant' }">
            <input type="radio" v-model="mlearn.geneticFn" value="rastrigin_variant" />
            <span>Rastrigin 变体 (2D)</span>
          </label>
        </div>
      </div>

      <div class="form-group" v-if="mlearn.algorithm === 'regression'">
        <label class="form-label">学习率</label>
        <div class="radio-group">
          <label class="radio-item" :class="{ active: mlearn.learningRate === 0.001 }">
            <input type="radio" v-model.number="mlearn.learningRate" :value="0.001" />
            <span>0.001 (慢)</span>
          </label>
          <label class="radio-item" :class="{ active: mlearn.learningRate === 0.005 }">
            <input type="radio" v-model.number="mlearn.learningRate" :value="0.005" />
            <span>0.005</span>
          </label>
          <label class="radio-item" :class="{ active: mlearn.learningRate === 0.01 }">
            <input type="radio" v-model.number="mlearn.learningRate" :value="0.01" />
            <span>0.01 (适中)</span>
          </label>
          <label class="radio-item" :class="{ active: mlearn.learningRate === 0.05 }">
            <input type="radio" v-model.number="mlearn.learningRate" :value="0.05" />
            <span>0.05</span>
          </label>
          <label class="radio-item" :class="{ active: mlearn.learningRate === 0.1 }">
            <input type="radio" v-model.number="mlearn.learningRate" :value="0.1" />
            <span>0.1 (快)</span>
          </label>
        </div>
      </div>

      <div class="form-group" v-if="mlearn.algorithm === 'regression'">
        <label class="form-label">噪声: {{ mlearn.noise }}</label>
        <input type="range" v-model.number="mlearn.noise" min="0" max="1" step="0.05" class="range-input" />
      </div>

      <div class="form-group">
        <label class="form-label">每步 Epochs: {{ mlearn.epochsPerStep }}</label>
        <input type="range" v-model.number="mlearn.epochsPerStep" min="1" max="100" step="1" class="range-input" />
      </div>

      <div class="form-actions">
        <button class="btn-primary" @click="handleCreate" :disabled="mlearn.isTraining">
          创建任务
        </button>
        <button class="btn-danger" v-if="mlearn.hasTask" @click="handleStop">
          删除任务
        </button>
      </div>

      <div class="error-msg" v-if="mlearn.error">{{ mlearn.error }}</div>
    </div>

    <div class="task-card" v-if="mlearn.hasTask">
      <h3 class="card-title">任务状态</h3>
      <div class="stat-grid">
        <div class="stat-item">
          <div class="stat-label">Task ID</div>
          <div class="stat-value mono">{{ mlearn.currentTaskId?.slice(0, 12) }}...</div>
        </div>
        <div class="stat-item">
          <div class="stat-label">算法</div>
          <div class="stat-value">{{ mlearn.taskStatus?.algorithm }}</div>
        </div>
        <div class="stat-item">
          <div class="stat-label">总 Epochs</div>
          <div class="stat-value">{{ mlearn.totalEpochs }}</div>
        </div>
        <div class="stat-item">
          <div class="stat-label">Loss</div>
          <div class="stat-value">{{ mlearn.bestFitness?.toFixed(6) ?? '-' }}</div>
        </div>
      </div>

      <div class="train-actions">
        <button class="btn-primary" @click="handleStep" :disabled="mlearn.isTraining">
          {{ mlearn.isTraining ? '训练中...' : `训练 ${mlearn.epochsPerStep} Epochs` }}
        </button>
        <button class="btn-secondary" @click="handleMultiStep(10)" :disabled="mlearn.isTraining">
          训练 10 步
        </button>
        <button class="btn-secondary" @click="handleMultiStep(50)" :disabled="mlearn.isTraining">
          训练 50 步
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useMLearnStore } from '../stores/mlearn'

const mlearn = useMLearnStore()

async function handleCreate() {
  await mlearn.createTask()
}

async function handleStop() {
  await mlearn.removeTask()
}

async function handleStep() {
  await mlearn.doStep()
  await mlearn.fetchInference()
}

async function handleMultiStep(steps: number) {
  await mlearn.doMultiStep(steps)
  await mlearn.fetchInference()
}
</script>

<style scoped>
.task-page {
  max-width: 640px;
}

.page-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 20px 0;
}

.form-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
}

.radio-group {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.radio-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  border: 1px solid var(--border);
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  color: var(--text-secondary);
  transition: all 0.15s;
}

.radio-item.active {
  border-color: var(--accent-blue);
  background: rgba(74, 144, 226, 0.08);
  color: var(--accent-blue);
}

.radio-item input {
  display: none;
}

.range-input {
  width: 100%;
  accent-color: var(--accent-blue);
}

.form-actions {
  display: flex;
  gap: 10px;
  padding-top: 8px;
}

.train-actions {
  display: flex;
  gap: 10px;
  margin-top: 16px;
}

.btn-primary {
  padding: 10px 20px;
  background: var(--accent-blue);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.15s;
}

.btn-primary:hover {
  opacity: 0.9;
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  padding: 10px 16px;
  background: var(--bg-hover);
  color: var(--text-primary);
  border: 1px solid var(--border);
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s;
}

.btn-secondary:hover {
  background: var(--border);
}

.btn-secondary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-danger {
  padding: 10px 20px;
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
  border: 1px solid rgba(239, 68, 68, 0.3);
  border-radius: 6px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
}

.error-msg {
  color: #ef4444;
  font-size: 13px;
  padding: 8px 12px;
  background: rgba(239, 68, 68, 0.08);
  border-radius: 4px;
}

.task-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 20px;
  margin-top: 16px;
}

.card-title {
  font-size: 15px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 14px 0;
}

.stat-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.stat-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.stat-label {
  font-size: 11px;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.stat-value {
  font-size: 14px;
  color: var(--text-primary);
  font-weight: 600;
}

.stat-value.mono {
  font-family: monospace;
  font-size: 12px;
}
</style>
