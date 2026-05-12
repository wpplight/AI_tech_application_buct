<template>
  <div class="regression-page">
    <h2 class="page-title">回归拟合</h2>

    <div class="info-bar" v-if="mlearn.hasTask && mlearn.isRegression">
      <span class="info-item">Epochs: {{ mlearn.totalEpochs }}</span>
      <span class="info-item">Loss: {{ mlearn.bestFitness?.toFixed(6) ?? '-' }}</span>
      <button class="btn-sm" @click="handleStep" :disabled="mlearn.isTraining">
        {{ mlearn.isTraining ? '训练中...' : '训练一步' }}
      </button>
      <button class="btn-sm" @click="handleMultiStep(10)" :disabled="mlearn.isTraining">+10步</button>
      <button class="btn-sm" @click="handleMultiStep(50)" :disabled="mlearn.isTraining">+50步</button>
    </div>

    <div class="chart-container" v-if="chartData">
      <svg :viewBox="`0 0 ${svgW} ${svgH}`" class="chart-svg">
        <line :x1="padL" :y1="padT" :x2="padL" :y2="svgH - padB" stroke="var(--border)" stroke-width="1" />
        <line :x1="padL" :y1="svgH - padB" :x2="svgW - padR" :y2="svgH - padB" stroke="var(--border)" stroke-width="1" />

        <g v-for="(tick, i) in xTicks" :key="'xt' + i">
          <line :x1="tick.px" :y1="svgH - padB" :x2="tick.px" :y2="svgH - padB + 4" stroke="var(--border)" />
          <text :x="tick.px" :y="svgH - padB + 16" text-anchor="middle" class="tick-text">{{ tick.label }}</text>
        </g>
        <g v-for="(tick, i) in yTicks" :key="'yt' + i">
          <line :x1="padL - 4" :y1="tick.py" :x2="padL" :y2="tick.py" stroke="var(--border)" />
          <text :x="padL - 8" :y="tick.py + 4" text-anchor="end" class="tick-text">{{ tick.label }}</text>
        </g>

        <circle v-for="(pt, i) in dataPoints" :key="'d' + i"
          :cx="pt.px" :cy="pt.py" r="2.5" fill="rgba(74,144,226,0.5)" />

        <polyline :points="curveLine" fill="none" stroke="var(--accent-green)" stroke-width="2.5" />
      </svg>
    </div>

    <div class="empty-state" v-else>
      <p>请先在「创建任务」中创建一个回归任务，然后训练几步后查看拟合曲线。</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useMLearnStore } from '../stores/mlearn'

const mlearn = useMLearnStore()

const svgW = 700
const svgH = 400
const padL = 50
const padR = 20
const padT = 20
const padB = 40
const chartW = svgW - padL - padR
const chartH = svgH - padT - padB

const chartData = computed(() => mlearn.regressionData)

const xRange = computed(() => {
  if (!chartData.value) return { min: 0, max: 1 }
  const xs = chartData.value.x_data
  return { min: Math.min(...xs), max: Math.max(...xs) }
})

const yRange = computed(() => {
  if (!chartData.value) return { min: 0, max: 1 }
  const all = [...chartData.value.y_data, ...chartData.value.y_curve]
  const min = Math.min(...all)
  const max = Math.max(...all)
  const pad = (max - min) * 0.1 || 1
  return { min: min - pad, max: max + pad }
})

function toSvg(x: number, y: number) {
  const px = padL + ((x - xRange.value.min) / (xRange.value.max - xRange.value.min)) * chartW
  const py = padT + chartH - ((y - yRange.value.min) / (yRange.value.max - yRange.value.min)) * chartH
  return { px, py }
}

const xTicks = computed(() => {
  const n = 5
  const ticks = []
  for (let i = 0; i <= n; i++) {
    const v = xRange.value.min + (xRange.value.max - xRange.value.min) * i / n
    const { px } = toSvg(v, 0)
    ticks.push({ px, py: 0, label: v.toFixed(1) })
  }
  return ticks
})

const yTicks = computed(() => {
  const n = 4
  const ticks = []
  for (let i = 0; i <= n; i++) {
    const v = yRange.value.min + (yRange.value.max - yRange.value.min) * i / n
    const { py } = toSvg(0, v)
    ticks.push({ px: 0, py, label: v.toFixed(1) })
  }
  return ticks
})

const dataPoints = computed(() => {
  if (!chartData.value) return []
  return chartData.value.x_data.map((x, i) => {
    const { px, py } = toSvg(x, chartData.value!.y_data[i])
    return { px, py }
  })
})

const curveLine = computed(() => {
  if (!chartData.value) return ''
  return chartData.value.x_curve.map((x, i) => {
    const { px, py } = toSvg(x, chartData.value!.y_curve[i])
    return `${px},${py}`
  }).join(' ')
})

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
.regression-page {
  max-width: 760px;
}

.page-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 16px 0;
}

.info-bar {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 10px 14px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 6px;
  margin-bottom: 16px;
}

.info-item {
  font-size: 13px;
  color: var(--text-secondary);
  font-weight: 600;
}

.btn-sm {
  padding: 6px 12px;
  background: var(--accent-blue);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: opacity 0.15s;
}

.btn-sm:hover {
  opacity: 0.9;
}

.btn-sm:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.chart-container {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 16px;
}

.chart-svg {
  width: 100%;
  height: auto;
}

.tick-text {
  font-size: 10px;
  fill: var(--text-muted);
}

.empty-state {
  padding: 40px;
  text-align: center;
  color: var(--text-muted);
  font-size: 14px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
}
</style>
