<template>
  <div class="task-page">
    <div class="page-header">
      <h2 class="page-title">任务管理</h2>
      <button class="btn-create" @click="showCreate = true">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
          <circle cx="12" cy="12" r="10"/>
          <path d="M12 8v8"/>
          <path d="M8 12h8"/>
        </svg>
        创建任务
      </button>
    </div>

    <div class="create-panel" v-if="showCreate">
      <div class="create-header">
        <span class="create-title">新建训练任务</span>
        <button class="btn-close" @click="showCreate = false">&times;</button>
      </div>

      <div class="create-body">
        <div class="form-group">
          <label class="form-label">算法类型</label>
          <div class="radio-group">
            <label class="radio-item" :class="{ active: form.algorithm === 'regression' }">
              <input type="radio" v-model="form.algorithm" value="regression" />
              <span>回归拟合</span>
            </label>
            <label class="radio-item" :class="{ active: form.algorithm === 'genetic' }">
              <input type="radio" v-model="form.algorithm" value="genetic" />
              <span>遗传算法</span>
            </label>
          </div>
        </div>

        <div class="form-group" v-if="form.algorithm === 'regression'">
          <label class="form-label">拟合函数</label>
          <div class="radio-group">
            <label class="radio-item" :class="{ active: form.regressionFn === 'linear' }">
              <input type="radio" v-model="form.regressionFn" value="linear" />
              <span>线性 y=2x+1</span>
            </label>
            <label class="radio-item" :class="{ active: form.regressionFn === 'quadratic' }">
              <input type="radio" v-model="form.regressionFn" value="quadratic" />
              <span>二次 y=x²</span>
            </label>
            <label class="radio-item" :class="{ active: form.regressionFn === 'sinusoidal' }">
              <input type="radio" v-model="form.regressionFn" value="sinusoidal" />
              <span>正弦 y=sin(x)</span>
            </label>
          </div>
        </div>

        <div class="form-group" v-if="form.algorithm === 'genetic'">
          <label class="form-label">目标函数</label>
          <div class="radio-group">
            <label class="radio-item" :class="{ active: form.geneticFn === 'ackley' }">
              <input type="radio" v-model="form.geneticFn" value="ackley" />
              <span>Ackley (1D)</span>
            </label>
            <label class="radio-item" :class="{ active: form.geneticFn === 'rastrigin_variant' }">
              <input type="radio" v-model="form.geneticFn" value="rastrigin_variant" />
              <span>Rastrigin 变体 (2D)</span>
            </label>
          </div>
        </div>

        <div class="form-group" v-if="form.algorithm === 'regression'">
          <label class="form-label">学习率</label>
          <div class="radio-group">
            <label class="radio-item" v-for="lr in lrOptions" :key="lr.value" :class="{ active: form.learningRate === lr.value }">
              <input type="radio" v-model.number="form.learningRate" :value="lr.value" />
              <span>{{ lr.label }}</span>
            </label>
          </div>
        </div>

        <div class="form-group" v-if="form.algorithm === 'regression'">
          <label class="form-label">噪声</label>
          <div class="radio-group">
            <label class="radio-item" v-for="n in noiseOptions" :key="n.value" :class="{ active: form.noise === n.value }">
              <input type="radio" v-model.number="form.noise" :value="n.value" />
              <span>{{ n.label }}</span>
            </label>
          </div>
        </div>

        <div class="form-group" v-if="form.algorithm === 'regression'">
          <label class="form-label">训练范围（可选）</label>
          <div class="range-inputs">
            <div class="range-field">
              <span class="range-label">X 最小值</span>
              <input type="number" v-model.number="form.xMin" placeholder="默认自动" class="range-input" />
            </div>
            <div class="range-divider">~</div>
            <div class="range-field">
              <span class="range-label">X 最大值</span>
              <input type="number" v-model.number="form.xMax" placeholder="默认自动" class="range-input" />
            </div>
          </div>
        </div>

        <div class="form-group" v-if="form.algorithm === 'genetic'">
          <label class="form-label">变量范围</label>
          <div class="range-inputs">
            <div class="range-field">
              <span class="range-label">最小值 <span class="range-hint">({{ getRangeHint().min }} 默认)</span></span>
              <input type="number" v-model.number="form.minValue" placeholder="默认" class="range-input" />
            </div>
            <div class="range-divider">~</div>
            <div class="range-field">
              <span class="range-label">最大值 <span class="range-hint">({{ getRangeHint().max }} 默认)</span></span>
              <input type="number" v-model.number="form.maxValue" placeholder="默认" class="range-input" />
            </div>
          </div>
        </div>

        <div class="form-group" v-if="form.algorithm === 'genetic'">
          <label class="form-label">优化目标</label>
          <div class="radio-group">
            <label class="radio-item" :class="{ active: form.objective === 'minimize' }">
              <input type="radio" v-model="form.objective" value="minimize" />
              <span>最小值</span>
            </label>
            <label class="radio-item" :class="{ active: form.objective === 'maximize' }">
              <input type="radio" v-model="form.objective" value="maximize" />
              <span>最大值</span>
            </label>
          </div>
        </div>

        <div class="form-group" v-if="form.algorithm === 'genetic'">
          <label class="form-label">遗传算法参数</label>
          <div class="ga-params">
            <div class="param-row">
              <div class="param-field">
                <span class="param-label">种群规模</span>
                <input type="number" v-model.number="form.populationSize" min="10" max="1000" class="param-input" />
              </div>
              <div class="param-field">
                <span class="param-label">锦标赛大小</span>
                <input type="number" v-model.number="form.tournamentSize" min="2" max="50" class="param-input" />
              </div>
            </div>
            <div class="param-row">
              <div class="param-field">
                <span class="param-label">精英数量</span>
                <input type="number" v-model.number="form.eliteCount" min="0" max="50" class="param-input" />
              </div>
              <div class="param-field">
                <span class="param-label">精英保护</span>
                <label class="toggle">
                  <input type="checkbox" v-model="form.eliteProtect" />
                  <span class="toggle-slider"></span>
                </label>
              </div>
            </div>
            <div class="param-row">
              <div class="param-field">
                <span class="param-label">变异率</span>
                <input type="number" v-model.number="form.mutationRate" min="0" max="1" step="0.001" class="param-input" />
              </div>
              <div class="param-field">
                <span class="param-label">SBX eta</span>
                <input type="number" v-model.number="form.sbxEta" min="1" max="100" class="param-input" />
              </div>
            </div>
          </div>
        </div>

        <div class="form-actions">
          <button class="btn-primary" @click="handleCreate" :disabled="creating">
          {{ creating ? '创建中...' : '确认创建' }}
        </button>
          <button class="btn-ghost" @click="showCreate = false">取消</button>
        </div>

        <div class="error-msg" v-if="mlearn.error">{{ mlearn.error }}</div>
      </div>
    </div>

    <div class="task-list" v-if="mlearn.tasks.length > 0">
      <div class="task-card" v-for="task in mlearn.tasks" :key="task.id"
        :class="{ active: task.id === mlearn.currentTaskId }"
        @click="handleEnter(task.id)">
        <div class="task-main">
          <span class="task-badge" :class="task.algorithm === 'regression' ? 'badge-blue' : 'badge-purple'">
            {{ task.algorithm === 'regression' ? '回归' : '遗传' }}
          </span>
          <span class="task-label">{{ task.label }}</span>
        </div>
        <div class="task-meta">
          <span class="task-id">{{ task.id.slice(0, 8) }}...</span>
          <span class="task-time">{{ formatTime(task.createdAt) }}</span>
        </div>
        <div class="task-actions" @click.stop>
          <button class="btn-enter" @click="handleEnter(task.id)">进入</button>
          <button class="btn-delete" @click="handleDelete(task.id)">删除</button>
        </div>
      </div>
    </div>

    <div class="empty-state" v-else>
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="48" height="48" class="empty-icon">
        <rect x="3" y="3" width="18" height="18" rx="2"/>
        <path d="M12 8v8"/>
        <path d="M8 12h8"/>
      </svg>
      <p>暂无训练任务，点击上方「创建任务」开始</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { useMLearnStore, type TaskItem } from '../stores/mlearn'
import type { RegressionFunction, GeneticFunction, Objective, GeneticParams } from '../api/mlearn'

const mlearn = useMLearnStore()
const router = useRouter()

const showCreate = ref(false)
const creating = ref(false)

const form = reactive({
  algorithm: 'regression' as 'regression' | 'genetic',
  regressionFn: 'linear' as RegressionFunction,
  geneticFn: 'ackley' as GeneticFunction,
  learningRate: 0.01,
  noise: 0.1,
  xMin: undefined as number | undefined,
  xMax: undefined as number | undefined,
  minValue: undefined as number | undefined,
  maxValue: undefined as number | undefined,
  objective: 'minimize' as Objective,
  populationSize: 200,
  tournamentSize: 10,
  eliteCount: 2,
  eliteProtect: true,
  mutationRate: 0.01,
  sbxEta: 15
})

const lrOptions = [
  { value: 0.001, label: '0.001 (慢)' },
  { value: 0.005, label: '0.005' },
  { value: 0.01, label: '0.01 (适中)' },
  { value: 0.05, label: '0.05' },
  { value: 0.1, label: '0.1 (快)' }
]

const noiseOptions = [
  { value: 0, label: '无噪声' },
  { value: 0.05, label: '0.05 (低)' },
  { value: 0.1, label: '0.1 (中)' },
  { value: 0.3, label: '0.3 (高)' },
  { value: 0.5, label: '0.5 (很高)' }
]

function getRangeHint() {
  if (form.geneticFn === 'ackley') {
    return { min: '-5.12', max: '5.12' }
  } else {
    return { min: '-50', max: '50' }
  }
}

async function handleCreate() {
  creating.value = true
  const geneticParams: GeneticParams | undefined = form.algorithm === 'genetic' ? {
    population_size: form.populationSize,
    tournament_size: form.tournamentSize,
    elite_count: form.eliteCount,
    elite_protect: form.eliteProtect,
    mutation_rate: form.mutationRate,
    sbx_eta: form.sbxEta
  } : undefined
  const id = await mlearn.createTask({
    algorithm: form.algorithm,
    regressionFn: form.regressionFn,
    geneticFn: form.geneticFn,
    learningRate: form.learningRate,
    noise: form.noise,
    xMin: form.xMin,
    xMax: form.xMax,
    minValue: form.minValue,
    maxValue: form.maxValue,
    objective: form.objective,
    geneticParams
  })
  creating.value = false
  if (id) {
    showCreate.value = false
    await handleEnter(id)
  }
}

async function handleEnter(taskId: string) {
  await mlearn.selectTask(taskId)
  const task = mlearn.tasks.find(t => t.id === taskId)
  if (task?.algorithm === 'regression') {
    router.push('/mlearn/regression')
  } else {
    router.push('/mlearn/genetic')
  }
}

async function handleDelete(taskId: string) {
  await mlearn.removeTask(taskId)
}

function formatTime(ts: number): string {
  const d = new Date(ts)
  const h = d.getHours().toString().padStart(2, '0')
  const m = d.getMinutes().toString().padStart(2, '0')
  const s = d.getSeconds().toString().padStart(2, '0')
  return `${h}:${m}:${s}`
}
</script>

<style scoped>
.task-page {
  max-width: 720px;
}

.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
}

.page-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.btn-create {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: var(--accent-blue);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.15s;
}

.btn-create:hover {
  opacity: 0.9;
}

.create-panel {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  margin-bottom: 20px;
  overflow: hidden;
}

.create-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 20px;
  border-bottom: 1px solid var(--border);
  background: rgba(74, 144, 226, 0.04);
}

.create-title {
  font-size: 14px;
  font-weight: 700;
  color: var(--text-primary);
}

.btn-close {
  background: none;
  border: none;
  font-size: 20px;
  color: var(--text-muted);
  cursor: pointer;
  line-height: 1;
  padding: 0 4px;
}

.btn-close:hover {
  color: var(--text-primary);
}

.create-body {
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

.radio-item:hover {
  border-color: var(--accent-blue);
  color: var(--text-primary);
}

.radio-item.active {
  border-color: var(--accent-blue);
  background: rgba(74, 144, 226, 0.08);
  color: var(--accent-blue);
}

.radio-item input {
  display: none;
}

.form-actions {
  display: flex;
  gap: 10px;
  padding-top: 4px;
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

.btn-ghost {
  padding: 10px 20px;
  background: none;
  color: var(--text-secondary);
  border: 1px solid var(--border);
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s;
}

.btn-ghost:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.error-msg {
  color: #ef4444;
  font-size: 13px;
  padding: 8px 12px;
  background: rgba(239, 68, 68, 0.08);
  border-radius: 4px;
}

.range-inputs {
  display: flex;
  align-items: center;
  gap: 12px;
}

.range-field {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.range-label {
  font-size: 11px;
  color: var(--text-muted);
}

.range-input {
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  font-size: 13px;
  color: var(--text-primary);
  background: var(--bg-primary);
  width: 120px;
  outline: none;
  transition: border-color 0.15s;
}

.range-input:focus {
  border-color: var(--accent-blue);
}

.range-hint {
  font-size: 10px;
  color: var(--text-muted);
  font-weight: normal;
}

.range-divider {
  font-size: 16px;
  color: var(--text-muted);
  margin-top: 18px;
}

.ga-params {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 12px;
  background: rgba(139, 92, 246, 0.04);
  border: 1px solid var(--border);
  border-radius: 6px;
}

.param-row {
  display: flex;
  gap: 12px;
}

.param-field {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
}

.param-label {
  font-size: 11px;
  color: var(--text-muted);
}

.param-input {
  padding: 6px 10px;
  border: 1px solid var(--border);
  border-radius: 4px;
  font-size: 12px;
  color: var(--text-primary);
  background: var(--bg-primary);
  width: 100%;
  outline: none;
  transition: border-color 0.15s;
}

.param-input:focus {
  border-color: var(--accent-blue);
}

.toggle {
  position: relative;
  display: inline-block;
  width: 36px;
  height: 20px;
  margin-top: 2px;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: 0.3s;
  border-radius: 20px;
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 14px;
  width: 14px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.3s;
  border-radius: 50%;
}

.toggle input:checked + .toggle-slider {
  background-color: #8b5cf6;
}

.toggle input:checked + .toggle-slider:before {
  transform: translateX(16px);
}

.task-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.task-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 14px 18px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.15s;
}

.task-card:hover {
  border-color: var(--accent-blue);
  background: rgba(74, 144, 226, 0.03);
}

.task-card.active {
  border-color: var(--accent-blue);
  background: rgba(74, 144, 226, 0.06);
}

.task-main {
  display: flex;
  align-items: center;
  gap: 10px;
  flex: 1;
  min-width: 0;
}

.task-badge {
  padding: 3px 10px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.5px;
  flex-shrink: 0;
}

.badge-blue {
  background: rgba(74, 144, 226, 0.12);
  color: var(--accent-blue);
}

.badge-purple {
  background: rgba(139, 92, 246, 0.12);
  color: #8b5cf6;
}

.task-label {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.task-meta {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 2px;
  flex-shrink: 0;
}

.task-id {
  font-size: 11px;
  color: var(--text-muted);
  font-family: monospace;
}

.task-time {
  font-size: 11px;
  color: var(--text-muted);
}

.task-actions {
  display: flex;
  gap: 6px;
  flex-shrink: 0;
}

.btn-enter {
  padding: 6px 14px;
  background: var(--accent-blue);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.15s;
}

.btn-enter:hover {
  opacity: 0.9;
}

.btn-delete {
  padding: 6px 14px;
  background: rgba(239, 68, 68, 0.08);
  color: #ef4444;
  border: 1px solid rgba(239, 68, 68, 0.2);
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.15s;
}

.btn-delete:hover {
  background: rgba(239, 68, 68, 0.15);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  padding: 60px 40px;
  text-align: center;
  color: var(--text-muted);
  font-size: 14px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  border-style: dashed;
}

.empty-icon {
  opacity: 0.3;
}
</style>
