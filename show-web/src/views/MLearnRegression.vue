<template>
  <div class="regression-page">
    <h2 class="page-title">回归拟合</h2>

    <div class="control-bar" v-if="mlearn.hasTask && mlearn.isRegression">
      <div class="control-left">
        <span class="info-item">Epochs: <strong>{{ mlearn.totalEpochs }}</strong></span>
        <span class="info-item">Loss: <strong>{{ mlearn.bestFitness?.toFixed(6) ?? '-' }}</strong></span>
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
          {{ mlearn.isTraining ? '训练中...' : '训练' }}
        </button>
      </div>
    </div>

    <div class="chart-card" v-if="chartData">
      <div class="chart-header">
        <div class="tab-switcher">
          <button
            v-for="tab in chartTabs"
            :key="tab.key"
            class="tab-btn"
            :class="{ active: activeTab === tab.key }"
            @click="activeTab = tab.key"
          >
            {{ tab.label }}
          </button>
        </div>
      </div>
      <div class="chart-body">
        <Line v-if="activeTab === 'fit'" :data="lineChartData" :options="lineChartOptions" />
        <Line v-else-if="activeTab === 'train'" :data="trainChartData ?? emptyChartData" :options="lossChartOptions" />
        <Line v-else-if="activeTab === 'gen'" :data="genChartData ?? emptyChartData" :options="lossChartOptions" />
        <Line v-else-if="activeTab === 'recall'" :data="recallChartData ?? emptyChartData" :options="scatterChartOptions" />
      </div>
    </div>

    <div class="empty-state" v-if="!chartData">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="48" height="48" class="empty-icon">
        <path d="M3 20L7 13 11 15 15 8 21 4"/>
        <path d="M3 20h18"/>
        <path d="M3 4v16"/>
      </svg>
      <p>请先在「任务管理」中创建一个回归任务，然后训练几步后查看拟合曲线。</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
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

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend, Filler)

const mlearn = useMLearnStore()

const activeTab = ref('fit')

const chartTabs = [
  { key: 'fit', label: '拟合曲线' },
  { key: 'train', label: '训练曲线' },
  { key: 'gen', label: '泛化曲线' },
  { key: 'recall', label: '回想曲线' }
]

const emptyChartData: ChartData<'line'> = { labels: [], datasets: [] }

const chartData = computed(() => mlearn.regressionData)

function trueFn(fn: string, x: number): number {
  switch (fn) {
    case 'linear': return 2 * x + 1
    case 'quadratic': return x * x
    case 'sinusoidal': return Math.sin(x)
    default: return 0
  }
}

const lineChartData = computed<ChartData<'line'>>(() => {
  if (!chartData.value) return { labels: [], datasets: [] }
  const fn = mlearn.currentTask?.fn ?? 'linear'
  const xCurve = chartData.value.x_curve
  const yTrue = xCurve.map(x => trueFn(fn, x))
  return {
    labels: xCurve.map(v => v.toFixed(2)),
    datasets: [
      {
        label: '标准曲线',
        data: yTrue,
        borderColor: '#3b82f6',
        backgroundColor: 'rgba(59, 130, 246, 0.05)',
        borderWidth: 2.5,
        pointRadius: 0,
        tension: 0.4,
        fill: false,
        order: 2
      },
      {
        label: '拟合曲线',
        data: chartData.value.y_curve,
        borderColor: '#ef4444',
        backgroundColor: 'rgba(239, 68, 68, 0.05)',
        borderWidth: 2.5,
        pointRadius: 0,
        tension: 0.4,
        fill: false,
        order: 1
      }
    ]
  }
})

const lineChartOptions = computed<ChartOptions<'line'>>(() => ({
  responsive: true,
  maintainAspectRatio: false,
  animation: { duration: 300 },
  interaction: { mode: 'index', intersect: false },
  plugins: {
    legend: { display: false },
    tooltip: {
      backgroundColor: 'rgba(30, 30, 30, 0.9)',
      titleFont: { size: 12 },
      bodyFont: { size: 12 },
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
      title: { display: true, text: 'y', color: '#9ca3af', font: { size: 12 } },
      grid: { color: 'rgba(156, 163, 175, 0.1)' },
      ticks: { color: '#9ca3af', font: { size: 10 } }
    }
  }
}))

const trainChartData = computed<ChartData<'line'> | null>(() => {
  const h = mlearn.trainingHistory
  if (!h || h.records.length === 0) return null
  return {
    labels: h.records.map(r => `${r.epoch}`),
    datasets: [{
      label: '训练 Loss',
      data: h.records.map(r => r.train_loss),
      borderColor: '#3b82f6',
      backgroundColor: 'rgba(59, 130, 246, 0.08)',
      borderWidth: 2,
      pointRadius: 0,
      tension: 0.3,
      fill: true
    }]
  }
})

const genChartData = computed<ChartData<'line'> | null>(() => {
  const h = mlearn.trainingHistory
  if (!h || h.records.length === 0) return null
  return {
    labels: h.records.map(r => `${r.epoch}`),
    datasets: [
      {
        label: '训练 Loss',
        data: h.records.map(r => r.train_loss),
        borderColor: '#3b82f6',
        backgroundColor: 'rgba(59, 130, 246, 0.06)',
        borderWidth: 2,
        pointRadius: 0,
        tension: 0.3,
        fill: true
      },
      {
        label: '验证 Loss',
        data: h.records.map(r => r.val_loss),
        borderColor: '#8b5cf6',
        backgroundColor: 'rgba(139, 92, 246, 0.06)',
        borderWidth: 2,
        pointRadius: 0,
        tension: 0.3,
        fill: true
      }
    ]
  }
})

const recallChartData = computed<ChartData<'line'> | null>(() => {
  const r = mlearn.recallData
  if (!r || r.y_true.length === 0) return null
  const min = Math.min(...r.y_true)
  const max = Math.max(...r.y_true)
  return {
    labels: r.y_true.map(v => v.toFixed(2)),
    datasets: [
      {
        label: '预测 vs 真实',
        data: r.y_true.map((_, i) => ({ x: r.y_true[i], y: r.y_pred[i] })),
        borderColor: '#10b981',
        backgroundColor: 'rgba(16, 185, 129, 0.15)',
        borderWidth: 2,
        pointRadius: 0,
        showLine: true,
        tension: 0,
        fill: false
      } as any,
      {
        label: '理想线 y=x',
        data: [{ x: min, y: min }, { x: max, y: max }],
        borderColor: 'rgba(156, 163, 175, 0.4)',
        borderWidth: 1.5,
        borderDash: [6, 4],
        pointRadius: 0,
        fill: false
      } as any
    ]
  }
})

const lossChartOptions: ChartOptions<'line'> = {
  responsive: true,
  maintainAspectRatio: false,
  animation: { duration: 200 },
  plugins: {
    legend: {
      display: true,
      position: 'top' as const,
      labels: { color: '#9ca3af', font: { size: 12 }, boxWidth: 12, boxHeight: 12 }
    },
    tooltip: {
      backgroundColor: 'rgba(30, 30, 30, 0.9)',
      padding: 8,
      cornerRadius: 6,
      callbacks: {
        label(ctx) {
          return `${ctx.dataset.label}: ${Number(ctx.parsed.y).toFixed(6)}`
        }
      }
    }
  },
  scales: {
    x: {
      display: true,
      title: { display: true, text: 'Epoch', color: '#9ca3af', font: { size: 11 } },
      grid: { color: 'rgba(156, 163, 175, 0.08)' },
      ticks: { color: '#9ca3af', font: { size: 10 }, maxTicksLimit: 15 }
    },
    y: {
      display: true,
      title: { display: true, text: 'Loss', color: '#9ca3af', font: { size: 11 } },
      grid: { color: 'rgba(156, 163, 175, 0.08)' },
      ticks: { color: '#9ca3af', font: { size: 10 } },
      beginAtZero: false
    }
  }
}

const scatterChartOptions: ChartOptions<'line'> = {
  responsive: true,
  maintainAspectRatio: false,
  animation: { duration: 300 },
  plugins: {
    legend: {
      display: true,
      position: 'top' as const,
      labels: { color: '#9ca3af', font: { size: 12 }, boxWidth: 12, boxHeight: 12 }
    },
    tooltip: {
      backgroundColor: 'rgba(30, 30, 30, 0.9)',
      padding: 8,
      cornerRadius: 6,
      callbacks: {
        label(ctx) {
          return `真实=${Number(ctx.parsed.x).toFixed(4)}, 预测=${Number(ctx.parsed.y).toFixed(4)}`
        }
      }
    }
  },
  scales: {
    x: {
      display: true,
      title: { display: true, text: '真实值 y_true', color: '#9ca3af', font: { size: 11 } },
      grid: { color: 'rgba(156, 163, 175, 0.08)' },
      ticks: { color: '#9ca3af', font: { size: 10 } }
    },
    y: {
      display: true,
      title: { display: true, text: '预测值 y_pred', color: '#9ca3af', font: { size: 11 } },
      grid: { color: 'rgba(156, 163, 175, 0.08)' },
      ticks: { color: '#9ca3af', font: { size: 10 } }
    }
  }
}

async function handleStep() {
  await mlearn.doStep()
  await mlearn.fetchInference()
}
</script>

<style scoped>
.regression-page {
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
  padding: 12px 20px;
  border-bottom: 1px solid var(--border);
}

.tab-switcher {
  display: flex;
  gap: 4px;
}

.tab-btn {
  padding: 6px 14px;
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid transparent;
  border-radius: 5px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
}

.tab-btn:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

.tab-btn.active {
  color: var(--accent-blue);
  background: rgba(74, 144, 226, 0.08);
  border-color: var(--accent-blue);
}

.chart-body {
  padding: 16px 20px 20px;
  height: 400px;
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