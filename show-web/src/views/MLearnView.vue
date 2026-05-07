<script setup lang="ts">
import { ref, onMounted, computed, onBeforeUnmount } from 'vue'
import { useRoute } from 'vue-router'
import { useMLearnStore } from '../stores/mlearn'
import LoadingSpinner from '../components/common/LoadingSpinner.vue'
import ErrorMessage from '../components/common/ErrorMessage.vue'
import StatCard from '../components/common/StatCard.vue'

const route = useRoute()
const store = useMLearnStore()

const activeTab = ref<'nn' | 'ga'>('nn')

// 监听路由参数
watch(
  () => route.query.tab,
  (tab) => {
    if (tab === 'nn' || tab === 'ga') {
      activeTab.value = tab
    }
  },
  { immediate: true }
)

onMounted(async () => {
  await store.checkConnection()
})

async function handleTrainNN() {
  await store.trainNN()
}

async function handleOptimizeGA() {
  await store.optimizeGA()
}

function handleCloseError() {
  store.nnError = null
  store.gaError = null
}

// 绘制收敛曲线的逻辑
const nnChartCanvas = ref<HTMLCanvasElement | null>(null)
const gaChartCanvas = ref<HTMLCanvasElement | null>(null)

function drawChart(canvas: HTMLCanvasElement | null, data: number[], color: string, label: string) {
  if (!canvas || data.length === 0) return
  
  const ctx = canvas.getContext('2d')
  if (!ctx) return
  
  const width = canvas.width
  const height = canvas.height
  const padding = 40
  
  ctx.clearRect(0, 0, width, height)
  
  // 绘制背景
  ctx.fillStyle = '#18181b'
  ctx.fillRect(0, 0, width, height)
  
  // 绘制网格
  ctx.strokeStyle = '#27272a'
  ctx.lineWidth = 1
  for (let i = 0; i <= 4; i++) {
    const y = padding + (height - 2 * padding) * i / 4
    ctx.beginPath()
    ctx.moveTo(padding, y)
    ctx.lineTo(width - padding, y)
    ctx.stroke()
  }
  
  // 绘制数据线
  if (data.length < 2) return
  
  const maxVal = Math.max(...data)
  const minVal = Math.min(...data)
  const range = maxVal - minVal || 1
  
  ctx.strokeStyle = color
  ctx.lineWidth = 2
  ctx.beginPath()
  
  data.forEach((val, i) => {
    const x = padding + (width - 2 * padding) * i / (data.length - 1)
    const y = padding + (height - 2 * padding) * (1 - (val - minVal) / range)
    
    if (i === 0) {
      ctx.moveTo(x, y)
    } else {
      ctx.lineTo(x, y)
    }
  })
  
  ctx.stroke()
  
  // 绘制标签
  ctx.fillStyle = '#a1a1aa'
  ctx.font = '12px -apple-system, BlinkMacSystemFont, sans-serif'
  ctx.fillText(label, padding, height - 10)
  ctx.fillText(`Loss: ${data[data.length - 1]?.toFixed(4)}`, width - padding - 100, height - 10)
}

onMounted(() => {
  // 初始化图表
  setTimeout(() => {
    drawChart(nnChartCanvas.value, store.lossHistory, '#f59e0b', '训练损失')
    drawChart(gaChartCanvas.value, store.fitnessHistory, '#f43f5e', '适应度')
  }, 100)
})

watch([() => store.lossHistory, () => store.fitnessHistory], () => {
  drawChart(nnChartCanvas.value, store.lossHistory, '#f59e0b', '训练损失')
  drawChart(gaChartCanvas.value, store.fitnessHistory, '#f43f5e', '适应度')
})
</script>

<template>
  <div class="mlearn-view">
    <div class="page-header">
      <div class="header-content">
        <h1 class="page-title">机器学习</h1>
        <p class="page-subtitle">Rust 实现的神经网络和遗传算法</p>
      </div>
      <div class="connection-status" :class="{ connected: store.isConnected }">
        <span class="status-dot"></span>
        {{ store.isConnected ? '已连接' : '未连接' }}
      </div>
    </div>

    <!-- Tab Navigation -->
    <div class="tab-navigation">
      <button
        class="tab-btn"
        :class="{ active: activeTab === 'nn' }"
        @click="activeTab = 'nn'"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="4" y="4" width="16" height="16" rx="2"/>
          <rect x="9" y="9" width="6" height="6"/>
          <path d="M9 1v3"/>
          <path d="M15 1v3"/>
          <path d="M9 20v3"/>
          <path d="M15 20v3"/>
          <path d="M20 9h3"/>
          <path d="M20 14h3"/>
          <path d="M1 9h3"/>
          <path d="M1 14h3"/>
        </svg>
        神经网络
      </button>
      <button
        class="tab-btn"
        :class="{ active: activeTab === 'ga' }"
        @click="activeTab = 'ga'"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M2 15c6.667-6 13.333 0 20-6"/>
          <path d="M9 22c1.798-1.998 2.518-3.995 2.807-5.993"/>
          <path d="M15 2c-1.798 1.998-2.518 3.995-2.807 5.993"/>
          <path d="M17 6l-2.5-2.5"/>
          <path d="M14 8l-1.5-1.5"/>
          <path d="M7 18l2.5 2.5"/>
          <path d="M10 16l1.5 1.5"/>
        </svg>
        遗传算法
      </button>
    </div>

    <!-- Error Messages -->
    <div v-if="store.nnError || store.gaError" class="error-container">
      <ErrorMessage :message="store.nnError || store.gaError || ''" @close="handleCloseError" />
    </div>

    <!-- Neural Network Tab -->
    <div v-if="activeTab === 'nn'" class="tab-content">
      <div class="content-grid">
        <!-- 左侧：配置面板 -->
        <div class="config-panel">
          <div class="panel-section">
            <h2 class="section-title">网络配置</h2>
            <div class="config-form">
              <div class="form-group">
                <label>输入维度</label>
                <input
                  v-model.number="store.nnConfig.inputDim"
                  type="number"
                  min="1"
                  max="10"
                  class="form-input"
                />
              </div>
              <div class="form-group">
                <label>隐藏层维度</label>
                <input
                  v-model.number="store.nnConfig.hiddenDim"
                  type="number"
                  min="4"
                  max="256"
                  class="form-input"
                />
              </div>
              <div class="form-group">
                <label>输出维度</label>
                <input
                  v-model.number="store.nnConfig.outputDim"
                  type="number"
                  min="1"
                  max="10"
                  class="form-input"
                />
              </div>
              <div class="form-group">
                <label>训练轮次</label>
                <input
                  v-model.number="store.nnConfig.epochs"
                  type="number"
                  min="100"
                  max="10000"
                  class="form-input"
                />
              </div>
              <div class="form-group">
                <label>学习率</label>
                <input
                  v-model.number="store.nnConfig.learningRate"
                  type="number"
                  min="0.0001"
                  max="1"
                  step="0.01"
                  class="form-input"
                />
              </div>
            </div>
            <button
              class="btn btn-primary btn-large"
              :disabled="store.nnLoading"
              @click="handleTrainNN"
            >
              <LoadingSpinner v-if="store.nnLoading" size="sm" />
              {{ store.nnLoading ? '训练中...' : '开始训练' }}
            </button>
          </div>

          <div class="panel-section">
            <h2 class="section-title">训练结果</h2>
            <div class="stats-grid">
              <StatCard label="最终损失" :value="store.finalLoss.toFixed(4)" />
              <StatCard label="训练时间" :value="store.trainingTime" unit="ms" />
              <StatCard label="收敛轮次" :value="store.nnTrainingProgress" />
            </div>
          </div>
        </div>

        <!-- 右侧：可视化面板 -->
        <div class="visualization-panel">
          <div class="panel-section">
            <h2 class="section-title">收敛曲线</h2>
            <div class="chart-container">
              <canvas ref="nnChartCanvas" width="600" height="300"></canvas>
              <div v-if="store.lossHistory.length === 0" class="chart-placeholder">
                <p>点击"开始训练"生成图表</p>
              </div>
            </div>
          </div>

          <div class="panel-section">
            <h2 class="section-title">网络架构</h2>
            <div class="architecture-display">
              <div class="layer input-layer">
                <span class="layer-label">输入</span>
                <span class="layer-size">{{ store.nnConfig.inputDim }}</span>
              </div>
              <div class="layer-arrow">→</div>
              <div class="layer hidden-layer">
                <span class="layer-label">隐藏层</span>
                <span class="layer-size">{{ store.nnConfig.hiddenDim }}</span>
              </div>
              <div class="layer-arrow">→</div>
              <div class="layer output-layer">
                <span class="layer-label">输出</span>
                <span class="layer-size">{{ store.nnConfig.outputDim }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Genetic Algorithm Tab -->
    <div v-if="activeTab === 'ga'" class="tab-content">
      <div class="content-grid">
        <!-- 左侧：配置面板 -->
        <div class="config-panel">
          <div class="panel-section">
            <h2 class="section-title">优化配置</h2>
            <div class="config-form">
              <div class="form-group">
                <label>基准函数</label>
                <select v-model="store.gaConfig.benchmarkFunction" class="form-input">
                  <option value="sphere">Sphere</option>
                  <option value="rastrigin">Rastrigin</option>
                  <option value="ackley">Ackley</option>
                  <option value="rosenbrock">Rosenbrock</option>
                </select>
              </div>
              <div class="form-group">
                <label>维度</label>
                <input
                  v-model.number="store.gaConfig.dimensions"
                  type="number"
                  min="2"
                  max="10"
                  class="form-input"
                />
              </div>
              <div class="form-group">
                <label>种群大小</label>
                <input
                  v-model.number="store.gaConfig.populationSize"
                  type="number"
                  min="50"
                  max="500"
                  class="form-input"
                />
              </div>
              <div class="form-group">
                <label>迭代次数</label>
                <input
                  v-model.number="store.gaConfig.generations"
                  type="number"
                  min="100"
                  max="5000"
                  class="form-input"
                />
              </div>
              <div class="form-group">
                <label>精英保护</label>
                <input
                  type="checkbox"
                  v-model="store.gaConfig.eliteProtect"
                  class="form-checkbox"
                />
              </div>
            </div>
            <button
              class="btn btn-primary btn-large"
              :disabled="store.gaLoading"
              @click="handleOptimizeGA"
            >
              <LoadingSpinner v-if="store.gaLoading" size="sm" />
              {{ store.gaLoading ? '优化中...' : '开始优化' }}
            </button>
          </div>

          <div class="panel-section">
            <h2 class="section-title">优化结果</h2>
            <div class="stats-grid">
              <StatCard label="最优适应度" :value="store.bestFitness.toFixed(4)" />
              <StatCard label="最优解" :value="`[${store.bestSolution.slice(0, 2).join(', ')}...]`" />
              <StatCard label="收敛代数" :value="store.gaProgress" />
            </div>
          </div>
        </div>

        <!-- 右侧：可视化面板 -->
        <div class="visualization-panel">
          <div class="panel-section">
            <h2 class="section-title">收敛曲线</h2>
            <div class="chart-container">
              <canvas ref="gaChartCanvas" width="600" height="300"></canvas>
              <div v-if="store.fitnessHistory.length === 0" class="chart-placeholder">
                <p>点击"开始优化"生成图表</p>
              </div>
            </div>
          </div>

          <div class="panel-section">
            <h2 class="section-title">算法参数</h2>
            <div class="params-grid">
              <div class="param-item">
                <span class="param-label">交叉算法</span>
                <span class="param-value">{{ store.gaConfig.crossoverType.toUpperCase() }}</span>
              </div>
              <div class="param-item">
                <span class="param-label">变异算法</span>
                <span class="param-value">{{ store.gaConfig.mutationType }}</span>
              </div>
              <div class="param-item">
                <span class="param-label">搜索范围</span>
                <span class="param-value">[{{ store.gaConfig.bounds[0] }}, {{ store.gaConfig.bounds[1] }}]</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mlearn-view {
  max-width: 1600px;
  margin: 0 auto;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 32px;
}

.page-title {
  font-size: 36px;
  font-weight: 700;
  letter-spacing: -0.02em;
  margin-bottom: 8px;
}

.page-subtitle {
  color: #a1a1aa;
  font-size: 16px;
}

.connection-status {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: rgba(244, 63, 94, 0.1);
  border: 1px solid rgba(244, 63, 94, 0.3);
  border-radius: 20px;
  font-size: 13px;
  color: #f43f5e;
}

.connection-status.connected {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.3);
  color: #10b981;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: currentColor;
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.error-container {
  margin-bottom: 24px;
}

/* Tab Navigation */
.tab-navigation {
  display: flex;
  gap: 8px;
  margin-bottom: 24px;
  background: #18181b;
  padding: 8px;
  border-radius: 16px;
  width: fit-content;
}

.tab-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  background: transparent;
  border: none;
  border-radius: 12px;
  color: #a1a1aa;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.tab-btn svg {
  width: 20px;
  height: 20px;
}

.tab-btn:hover {
  color: #fafafa;
  background: #27272a;
}

.tab-btn.active {
  color: #fafafa;
  background: #3b82f6;
}

/* Content Grid */
.content-grid {
  display: grid;
  grid-template-columns: 1fr 1.5fr;
  gap: 24px;
}

@media (max-width: 1024px) {
  .content-grid {
    grid-template-columns: 1fr;
  }
}

.config-panel,
.visualization-panel {
  background: #18181b;
  border: 1px solid #27272a;
  border-radius: 20px;
  padding: 24px;
}

.panel-section {
  margin-bottom: 32px;
}

.panel-section:last-child {
  margin-bottom: 0;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 16px;
  color: #a1a1aa;
}

/* Config Form */
.config-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
  margin-bottom: 20px;
}

.form-group {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.form-group label {
  font-size: 14px;
  color: #a1a1aa;
}

.form-input {
  width: 160px;
  padding: 8px 12px;
  background: #27272a;
  border: 1px solid #3f3f46;
  border-radius: 8px;
  color: #fafafa;
  font-size: 14px;
}

.form-checkbox {
  width: 20px;
  height: 20px;
  accent-color: #3b82f6;
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 10px 16px;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background: #3b82f6;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #2563eb;
}

.btn-large {
  width: 100%;
  padding: 14px 24px;
  font-size: 16px;
}

/* Stats Grid */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

/* Chart Container */
.chart-container {
  position: relative;
  background: #18181b;
  border-radius: 12px;
  overflow: hidden;
}

.chart-container canvas {
  display: block;
  width: 100%;
  height: auto;
}

.chart-placeholder {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #52525b;
  font-size: 14px;
  background: #0f0f11;
}

/* Architecture Display */
.architecture-display {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 24px;
  background: #0f0f11;
  border-radius: 12px;
}

.layer {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  padding: 16px 24px;
  background: #27272a;
  border-radius: 12px;
}

.input-layer {
  border: 1px solid rgba(16, 185, 129, 0.5);
}

.hidden-layer {
  border: 1px solid rgba(245, 158, 11, 0.5);
}

.output-layer {
  border: 1px solid rgba(244, 63, 94, 0.5);
}

.layer-label {
  font-size: 12px;
  color: #a1a1aa;
}

.layer-size {
  font-size: 20px;
  font-weight: 700;
  color: #fafafa;
}

.layer-arrow {
  font-size: 24px;
  color: #52525b;
}

/* Params Grid */
.params-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

.param-item {
  padding: 12px;
  background: #27272a;
  border-radius: 10px;
  text-align: center;
}

.param-label {
  display: block;
  font-size: 11px;
  color: #a1a1aa;
  margin-bottom: 4px;
}

.param-value {
  font-size: 14px;
  font-weight: 600;
  color: #fafafa;
}
</style>
