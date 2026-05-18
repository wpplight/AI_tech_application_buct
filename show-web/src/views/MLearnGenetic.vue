<template>
  <div class="genetic-page">
    <h2 class="page-title">遗传算法</h2>

    <div class="control-bar" v-if="mlearn.hasTask && mlearn.isGenetic">
      <div class="control-left">
        <span class="info-item">Epochs: <strong>{{ mlearn.totalEpochs }}</strong></span>
        <span class="info-item">Best Fitness: <strong>{{ mlearn.bestFitness?.toFixed(6) ?? '-' }}</strong></span>
      </div>
      <div class="control-right">
        <div class="epochs-control">
          <span class="epochs-label">每步 Epochs</span>
          <div class="epochs-btns">
            <button class="ep-btn" :class="{ active: mlearn.epochsPerStep === 1 }" @click="mlearn.epochsPerStep = 1">1</button>
            <button class="ep-btn" :class="{ active: mlearn.epochsPerStep === 5 }" @click="mlearn.epochsPerStep = 5">5</button>
            <button class="ep-btn" :class="{ active: mlearn.epochsPerStep === 10 }" @click="mlearn.epochsPerStep = 10">10</button>
            <button class="ep-btn" :class="{ active: mlearn.epochsPerStep === 50 }" @click="mlearn.epochsPerStep = 50">50</button>
            <button class="ep-btn" :class="{ active: mlearn.epochsPerStep === 100 }" @click="mlearn.epochsPerStep = 100">100</button>
          </div>
        </div>
        <button class="btn-train" @click="handleStep" :disabled="mlearn.isTraining">
          {{ mlearn.isTraining ? '进化中...' : '进化' }}
        </button>

      </div>
    </div>

    <div class="chart-card" v-if="chart1D">
      <div class="chart-header">
        <span class="chart-title">Ackley 函数 (1D)</span>
        <div class="chart-legend">
          <span class="legend-item">
            <span class="legend-line" style="background: #3b82f6;"></span>
            目标函数
          </span>
          <span class="legend-item">
            <span class="legend-dot" style="background: #22c55e;"></span>
            最优解 x={{ chart1D.best_gene.toFixed(4) }}
          </span>
          <span class="legend-item">
            <span class="legend-line" style="background: #ef4444; border-style: dashed;"></span>
            当前位置
          </span>
        </div>
      </div>
      <div class="chart-body">
        <Line v-if="line1DData" :data="line1DData" :options="line1DOptions" />
      </div>
      <div class="chart-footer">
        <span class="stat-item">最优基因: <strong>{{ chart1D.best_gene.toFixed(6) }}</strong></span>
        <span class="stat-item">目标函数值: <strong>{{ chart1D.best_fitness.toFixed(6) }}</strong></span>
      </div>
    </div>

    <div class="chart-card" v-if="chart2D">
      <div class="chart-header">
        <span class="chart-title">Rastrigin 变体 (2D)</span>
        <div class="view-toggle">
          <button class="toggle-btn" :class="{ active: viewMode === 'fitness' }" @click="viewMode = 'fitness'">
            适应度热图
          </button>
          <button class="toggle-btn" :class="{ active: viewMode === '3d' }" @click="viewMode = '3d'">
            3D 基因视图
          </button>
        </div>
        <div class="chart-legend">
          <span class="legend-item">
            <span class="legend-dot" style="background: #22c55e;"></span>
            最优基因 ({{ chart2D.best_gene_x.toFixed(2) }}, {{ chart2D.best_gene_y.toFixed(2) }})
          </span>
          <span class="legend-item">
            <span class="legend-dot" style="background: #ef4444;"></span>
            种群 ({{ chart2D.population_x.length }}个)
          </span>
          <span class="legend-item">Fitness: <strong>{{ chart2D.best_fitness.toFixed(4) }}</strong></span>
        </div>
      </div>
      <div class="chart-body">
        <div v-if="viewMode === '3d'" class="heatmap-body">
          <Genetic3DViewer />
        </div>
        <div v-else class="heatmap-body">
          <div class="heatmap-wrap">
            <canvas ref="heatmapCanvas" class="heatmap-canvas"></canvas>
            <div class="heatmap-marker" :style="markerStyle"></div>
            <div
              v-for="(marker, idx) in populationMarkers"
              :key="idx"
              class="population-marker"
              :class="{ 'is-best': marker.isBest }"
              :style="{ left: marker.left, top: marker.top }"
            ></div>
          </div>
          <div class="heatmap-scale">
            <div class="scale-bar"></div>
            <div class="scale-labels">
              <span>Low</span>
              <span>High</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="chart-card" v-if="chart1D">
      <div class="chart-header">
        <span class="chart-title">适应度变化</span>
        <span class="chart-subtitle">共 {{ mlearn.lossHistory.length }} 代</span>
      </div>
      <div class="chart-body chart-body-sm">
        <Line v-if="lossChartData" :data="lossChartData" :options="lossChartOptions" />
      </div>
    </div>

    <div class="empty-state" v-if="!chart1D && !chart2D">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="48" height="48" class="empty-icon">
        <circle cx="12" cy="12" r="10"/>
        <path d="M12 2c-3 3-3 7 0 10s3 7 0 10"/>
        <path d="M12 2c3 3 3 7 0 10s-3 7 0 10"/>
      </svg>
      <p>请先在「任务管理」中创建一个遗传算法任务，然后进化几步后查看结果。</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch, nextTick } from 'vue'
import { Line } from 'vue-chartjs'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler,
  type ChartOptions,
  type ChartData
} from 'chart.js'
import { useMLearnStore } from '../stores/mlearn'
import Genetic3DViewer from './components/Genetic3DViewer.vue'

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend, Filler)

const mlearn = useMLearnStore()

const viewMode = ref<'fitness' | '3d'>('fitness')
const chart1D = computed(() => mlearn.genetic1DData)
const chart2D = computed(() => mlearn.genetic2DData)

const line1DData = computed((): ChartData<'line'> => {
  if (!chart1D.value) return { labels: [], datasets: [] }

  const xRange = chart1D.value.x_range
  const yTrue = chart1D.value.y_true
  const popX = chart1D.value.population_x

  const annotationIdx = xRange.findIndex(x => Math.abs(x - (chart1D.value?.best_gene ?? 0)) < 0.01)
  const annotationData = xRange.map((_, i) => i === annotationIdx ? (yTrue[i] ?? null) : null)

  const popYData: (number | null)[] = new Array(xRange.length).fill(null)
  for (const px of popX) {
    const idx = xRange.findIndex(x => Math.abs(x - px) < 0.01)
    if (idx >= 0 && idx < popYData.length) {
      popYData[idx] = yTrue[idx] ?? null
    }
  }

  return {
    labels: xRange.map(v => v.toFixed(2)),
    datasets: [
      {
        label: '目标函数',
        data: yTrue,
        borderColor: '#3b82f6',
        backgroundColor: 'rgba(59, 130, 246, 0.06)',
        borderWidth: 2,
        pointRadius: 0,
        tension: 0.4,
        fill: true,
        order: 2
      },
      {
        label: '种群',
        data: popYData,
        borderColor: 'rgba(239, 68, 68, 0.6)',
        backgroundColor: 'rgba(239, 68, 68, 0.4)',
        pointRadius: 4,
        pointBackgroundColor: '#ef4444',
        pointBorderColor: 'white',
        pointBorderWidth: 1,
        showLine: false,
        order: 1
      },
      {
        label: '最优解',
        data: annotationData,
        borderColor: '#22c55e',
        backgroundColor: '#22c55e',
        pointRadius: 8,
        pointHoverRadius: 10,
        pointBackgroundColor: '#22c55e',
        pointBorderColor: 'white',
        pointBorderWidth: 3,
        pointStyle: 'circle',
        showLine: false,
        order: 0
      }
    ]
  } as ChartData<'line'>
})

const line1DOptions = computed<ChartOptions<'line'>>(() => ({
  responsive: true,
  maintainAspectRatio: false,
  animation: { duration: 300 },
  interaction: { mode: 'index', intersect: false },
  plugins: {
    legend: { display: false },
    tooltip: {
      backgroundColor: 'rgba(30, 30, 30, 0.9)',
      padding: 10,
      cornerRadius: 6,
      callbacks: {
        label(ctx) {
          return `${ctx.dataset.label}: ${Number(ctx.parsed.y).toFixed(4)}`
        }
      }
    }
  },
  scales: {
    x: {
      display: true,
      title: { display: true, text: 'x', color: '#9ca3af', font: { size: 12 } },
      grid: { color: 'rgba(156, 163, 175, 0.1)' },
      ticks: { color: '#9ca3af', font: { size: 10 }, maxTicksLimit: 10 }
    },
    y: {
      display: true,
      title: { display: true, text: 'f(x)', color: '#9ca3af', font: { size: 12 } },
      grid: { color: 'rgba(156, 163, 175, 0.1)' },
      ticks: { color: '#9ca3af', font: { size: 10 } }
    }
  }
}))

const populationMarkers = computed(() => {
  if (!chart2D.value) return []
  const xGrid = chart2D.value.x_grid
  const yGrid = chart2D.value.y_grid
  const popX = chart2D.value.population_x
  const popY = chart2D.value.population_y
  if (!popX.length || !popY.length) return []
  
  const xMin = Math.min(...xGrid)
  const xMax = Math.max(...xGrid)
  const yMin = Math.min(...yGrid)
  const yMax = Math.max(...yGrid)
  
  const markers: { left: string; top: string; isBest: boolean }[] = []
  const n = Math.min(popX.length, popY.length)
  const bestX = chart2D.value.best_gene_x
  const bestY = chart2D.value.best_gene_y
  
  for (let i = 0; i < n; i++) {
    const px = popX[i] as number
    const py = popY[i] as number
    const pctX = ((px - xMin) / (xMax - xMin)) * 100
    const pctY = ((py - yMin) / (yMax - yMin)) * 100
    const isBest = Math.abs(px - bestX) < 0.1 && Math.abs(py - bestY) < 0.1
    markers.push({ left: `${pctX}%`, top: `${pctY}%`, isBest })
  }
  return markers
})

const markerStyle = computed(() => {
  if (!chart2D.value) return { display: 'none' }
  const xGrid = chart2D.value.x_grid
  const yGrid = chart2D.value.y_grid
  const xMin = Math.min(...xGrid)
  const xMax = Math.max(...xGrid)
  const yMin = Math.min(...yGrid)
  const yMax = Math.max(...yGrid)
  const pctX = ((chart2D.value.best_gene_x - xMin) / (xMax - xMin)) * 100
  const pctY = ((chart2D.value.best_gene_y - yMin) / (yMax - yMin)) * 100
  return { left: `${pctX}%`, top: `${pctY}%` }
})

const heatmapCanvas = ref<HTMLCanvasElement | null>(null)

watch(chart2D, async (data) => {
  if (!data || !heatmapCanvas.value || viewMode.value !== 'fitness') return
  await nextTick()

  const steps = Math.round(Math.sqrt(data.x_grid.length))
  const canvas = heatmapCanvas.value
  canvas.width = steps
  canvas.height = steps
  const ctx = canvas.getContext('2d')!

  const vals = data.fitness_grid
  const min = Math.min(...vals)
  const max = Math.max(...vals)
  const range = max - min || 1

  for (let i = 0; i < steps; i++) {
    for (let j = 0; j < steps; j++) {
      const idx = i * steps + j
      const t = ((vals[idx] ?? 0) - min) / range
      const r = Math.round(40 + 215 * t)
      const g = Math.round(80 * (1 - Math.abs(t - 0.5) * 2))
      const b = Math.round(220 * (1 - t))
      ctx.fillStyle = `rgb(${r},${g},${b})`
      ctx.fillRect(j, i, 1, 1)
    }
  }
}, { immediate: true, deep: true })

const lossChartData = computed<ChartData<'line'> | null>(() => {
  if (mlearn.lossHistory.length === 0) return null
  return {
    labels: mlearn.lossHistory.map((_, i) => `${i + 1}`),
    datasets: [{
      label: 'Best Fitness',
      data: mlearn.lossHistory,
      borderColor: '#8b5cf6',
      backgroundColor: 'rgba(139, 92, 246, 0.08)',
      borderWidth: 2,
      pointRadius: 2,
      pointHoverRadius: 5,
      pointBackgroundColor: '#8b5cf6',
      tension: 0.3,
      fill: true
    }]
  }
})

const lossChartOptions: ChartOptions<'line'> = {
  responsive: true,
  maintainAspectRatio: false,
  animation: { duration: 200 },
  plugins: {
    legend: { display: false },
    tooltip: {
      backgroundColor: 'rgba(30, 30, 30, 0.9)',
      padding: 8,
      cornerRadius: 6,
      callbacks: {
        label(ctx) {
          return `Fitness: ${Number(ctx.parsed.y).toFixed(6)}`
        }
      }
    }
  },
  scales: {
    x: {
      display: true,
      title: { display: true, text: 'Generation', color: '#9ca3af', font: { size: 11 } },
      grid: { color: 'rgba(156, 163, 175, 0.08)' },
      ticks: { color: '#9ca3af', font: { size: 10 }, maxTicksLimit: 15 }
    },
    y: {
      display: true,
      title: { display: true, text: 'Fitness', color: '#9ca3af', font: { size: 11 } },
      grid: { color: 'rgba(156, 163, 175, 0.08)' },
      ticks: { color: '#9ca3af', font: { size: 10 } },
      beginAtZero: false
    }
  }
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
.genetic-page {
  max-width: 800px;
}

.page-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 16px 0;
}

.control-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 10px 14px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  margin-bottom: 16px;
}

.control-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.info-item {
  font-size: 13px;
  color: var(--text-secondary);
}

.info-item strong {
  color: var(--text-primary);
  font-weight: 700;
  font-variant-numeric: tabular-nums;
}

.control-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.epochs-control {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-right: 6px;
}

.epochs-label {
  font-size: 12px;
  color: var(--text-muted);
  white-space: nowrap;
}

.epochs-btns {
  display: flex;
  gap: 2px;
}

.ep-btn {
  padding: 4px 10px;
  background: var(--bg-hover);
  color: var(--text-secondary);
  border: 1px solid var(--border);
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.15s;
}

.ep-btn:hover {
  color: var(--text-primary);
  border-color: var(--accent-blue);
}

.ep-btn.active {
  background: var(--accent-blue);
  color: white;
  border-color: var(--accent-blue);
}

.btn-train {
  padding: 6px 16px;
  background: var(--accent-green);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.15s;
}

.btn-train:hover { opacity: 0.9; }
.btn-train:disabled { opacity: 0.5; cursor: not-allowed; }

.btn-sm {
  padding: 6px 12px;
  background: var(--bg-hover);
  color: var(--text-primary);
  border: 1px solid var(--border);
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.15s;
}

.btn-sm:hover { background: var(--border); }
.btn-sm:disabled { opacity: 0.5; cursor: not-allowed; }

.chart-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  overflow: hidden;
  margin-bottom: 16px;
}

.chart-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 20px;
  border-bottom: 1px solid var(--border);
  gap: 12px;
  flex-wrap: wrap;
}

.view-toggle {
  display: flex;
  gap: 4px;
}

.toggle-btn {
  padding: 5px 12px;
  background: var(--bg-hover);
  color: var(--text-secondary);
  border: 1px solid var(--border);
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.15s;
}

.toggle-btn:hover {
  background: var(--bg-card);
  color: var(--text-primary);
}

.toggle-btn.active {
  background: var(--accent-purple);
  color: white;
  border-color: var(--accent-purple);
}

.chart-title {
  font-size: 14px;
  font-weight: 700;
  color: var(--text-primary);
}

.chart-subtitle {
  font-size: 12px;
  color: var(--text-muted);
}

.chart-legend {
  display: flex;
  align-items: center;
  gap: 16px;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--text-secondary);
}

.legend-item strong {
  color: var(--text-primary);
}

.legend-line {
  width: 20px;
  height: 3px;
  border-radius: 2px;
}

.legend-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  border: 2px solid white;
  box-shadow: 0 0 0 1px rgba(0,0,0,0.1);
}

.chart-body {
  padding: 16px 20px 20px;
  height: 400px;
}

.chart-body-sm {
  height: 240px;
}

.chart-footer {
  display: flex;
  gap: 24px;
  padding: 12px 20px;
  border-top: 1px solid var(--border);
  background: rgba(34, 197, 94, 0.03);
}

.stat-item {
  font-size: 13px;
  color: var(--text-secondary);
}

.stat-item strong {
  color: var(--accent-green);
  font-weight: 700;
  font-variant-numeric: tabular-nums;
}

.heatmap-body {
  padding: 20px;
  display: flex;
  gap: 16px;
  align-items: center;
  justify-content: center;
}

.heatmap-wrap {
  position: relative;
  width: 400px;
  height: 400px;
  border: 1px solid var(--border);
  border-radius: 4px;
  overflow: hidden;
}

.heatmap-canvas {
  width: 100%;
  height: 100%;
  image-rendering: pixelated;
}

.heatmap-marker {
  position: absolute;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: #22c55e;
  border: 3px solid white;
  box-shadow: 0 0 8px rgba(34, 197, 94, 0.6);
  transform: translate(-50%, -50%);
  pointer-events: none;
  z-index: 2;
}

.population-marker {
  position: absolute;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: rgba(239, 68, 68, 0.6);
  border: 1px solid rgba(255, 255, 255, 0.5);
  transform: translate(-50%, -50%);
  pointer-events: none;
  z-index: 1;
}

.population-marker.is-best {
  background: #22c55e;
  width: 12px;
  height: 12px;
  border: 2px solid white;
  box-shadow: 0 0 6px rgba(34, 197, 94, 0.6);
  z-index: 3;
}

.heatmap-scale {
  display: flex;
  flex-direction: column;
  gap: 8px;
  align-items: center;
}

.scale-bar {
  width: 16px;
  height: 200px;
  border-radius: 4px;
  background: linear-gradient(to bottom, rgb(40, 80, 220), rgb(128, 40, 128), rgb(255, 80, 40));
}

.scale-labels {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  height: 200px;
  font-size: 11px;
  color: var(--text-muted);
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
