<template>
  <div class="genetic-page">
    <h2 class="page-title">遗传算法</h2>

    <div class="info-bar" v-if="mlearn.hasTask && mlearn.isGenetic">
      <span class="info-item">Epochs: {{ mlearn.totalEpochs }}</span>
      <span class="info-item">Best Fitness: {{ mlearn.bestFitness?.toFixed(6) ?? '-' }}</span>
      <button class="btn-sm" @click="handleStep" :disabled="mlearn.isTraining">
        {{ mlearn.isTraining ? '进化中...' : '进化一步' }}
      </button>
      <button class="btn-sm" @click="handleMultiStep(10)" :disabled="mlearn.isTraining">+10步</button>
      <button class="btn-sm" @click="handleMultiStep(50)" :disabled="mlearn.isTraining">+50步</button>
    </div>

    <div class="chart-container" v-if="chart1D">
      <h3 class="chart-title">Ackley 函数 (1D)</h3>
      <svg :viewBox="`0 0 ${svgW} ${svgH}`" class="chart-svg">
        <line :x1="padL" :y1="padT" :x2="padL" :y2="svgH - padB" stroke="var(--border)" stroke-width="1" />
        <line :x1="padL" :y1="svgH - padB" :x2="svgW - padR" :y2="svgH - padB" stroke="var(--border)" stroke-width="1" />

        <g v-for="(tick, i) in xTicks1D" :key="'xt' + i">
          <line :x1="tick.px" :y1="svgH - padB" :x2="tick.px" :y2="svgH - padB + 4" stroke="var(--border)" />
          <text :x="tick.px" :y="svgH - padB + 16" text-anchor="middle" class="tick-text">{{ tick.label }}</text>
        </g>
        <g v-for="(tick, i) in yTicks1D" :key="'yt' + i">
          <line :x1="padL - 4" :y1="tick.py" :x2="padL" :y2="tick.py" stroke="var(--border)" />
          <text :x="padL - 8" :y="tick.py + 4" text-anchor="end" class="tick-text">{{ tick.label }}</text>
        </g>

        <polyline :points="funcLine1D" fill="none" stroke="rgba(74,144,226,0.4)" stroke-width="1.5" />

        <circle :cx="bestPt1D.px" :cy="bestPt1D.py" r="6" fill="var(--accent-green)" stroke="white" stroke-width="2" />
        <line :x1="bestPt1D.px" :y1="padT" :x2="bestPt1D.px" :y2="svgH - padB" stroke="var(--accent-green)" stroke-width="1" stroke-dasharray="4,3" opacity="0.5" />
      </svg>
      <div class="chart-legend">
        <span class="legend-item"><span class="dot green"></span> 最优解 x={{ chart1D.best_gene.toFixed(4) }}</span>
        <span class="legend-item"><span class="dot blue"></span> 目标函数值 {{ chart1D.best_fitness.toFixed(4) }}</span>
      </div>
    </div>

    <div class="chart-container" v-if="chart2D">
      <h3 class="chart-title">Rastrigin 变体 (2D)</h3>
      <div class="heatmap-wrap">
        <svg :viewBox="`0 0 ${heatW} ${heatH}`" class="chart-svg">
          <image v-if="heatmapUrl" :href="heatmapUrl" :x="padL" :y="padT" :width="heatW - padL - padR" :height="heatH - padT - padB" preserveAspectRatio="none" />

          <circle :cx="bestPt2D.px" :cy="bestPt2D.py" r="7" fill="var(--accent-green)" stroke="white" stroke-width="2" />
        </svg>
      </div>
      <div class="chart-legend">
        <span class="legend-item"><span class="dot green"></span> 最优基因 ({{ chart2D.best_gene_x.toFixed(2) }}, {{ chart2D.best_gene_y.toFixed(2) }})</span>
        <span class="legend-item">Fitness: {{ chart2D.best_fitness.toFixed(4) }}</span>
      </div>
    </div>

    <div class="empty-state" v-if="!chart1D && !chart2D">
      <p>请先在「创建任务」中创建一个遗传算法任务，然后进化几步后查看结果。</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useMLearnStore } from '../stores/mlearn'

const mlearn = useMLearnStore()

const svgW = 700
const svgH = 350
const padL = 50
const padR = 20
const padT = 20
const padB = 40
const chartW = svgW - padL - padR
const chartH = svgH - padT - padB

const heatW = 500
const heatH = 500

const chart1D = computed(() => mlearn.genetic1DData)
const chart2D = computed(() => mlearn.genetic2DData)

const xRange1D = computed(() => {
  if (!chart1D.value) return { min: -5.12, max: 5.12 }
  return { min: Math.min(...chart1D.value.x_range), max: Math.max(...chart1D.value.x_range) }
})

const yRange1D = computed(() => {
  if (!chart1D.value) return { min: 0, max: 50 }
  const all = chart1D.value.y_true
  const min = Math.min(...all)
  const max = Math.max(...all)
  const pad = (max - min) * 0.1 || 1
  return { min: min - pad, max: max + pad }
})

function toSvg1D(x: number, y: number) {
  const px = padL + ((x - xRange1D.value.min) / (xRange1D.value.max - xRange1D.value.min)) * chartW
  const py = padT + chartH - ((y - yRange1D.value.min) / (yRange1D.value.max - yRange1D.value.min)) * chartH
  return { px, py }
}

const xTicks1D = computed(() => {
  if (!chart1D.value) return []
  const n = 5
  const ticks = []
  for (let i = 0; i <= n; i++) {
    const v = xRange1D.value.min + (xRange1D.value.max - xRange1D.value.min) * i / n
    const { px } = toSvg1D(v, 0)
    ticks.push({ px, label: v.toFixed(1) })
  }
  return ticks
})

const yTicks1D = computed(() => {
  if (!chart1D.value) return []
  const n = 4
  const ticks = []
  for (let i = 0; i <= n; i++) {
    const v = yRange1D.value.min + (yRange1D.value.max - yRange1D.value.min) * i / n
    const { py } = toSvg1D(0, v)
    ticks.push({ py, label: v.toFixed(1) })
  }
  return ticks
})

const funcLine1D = computed(() => {
  if (!chart1D.value) return ''
  return chart1D.value.x_range.map((x, i) => {
    const { px, py } = toSvg1D(x, chart1D.value!.y_true[i])
    return `${px},${py}`
  }).join(' ')
})

const bestPt1D = computed(() => {
  if (!chart1D.value) return { px: 0, py: 0 }
  return toSvg1D(chart1D.value.best_gene, chart1D.value.best_fitness)
})

const heatmapUrl = ref<string | null>(null)

watch(chart2D, (data) => {
  if (!data) {
    heatmapUrl.value = null
    return
  }
  const steps = Math.round(Math.sqrt(data.x_grid.length))
  const canvas = document.createElement('canvas')
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
      const t = (vals[idx] - min) / range
      const r = Math.round(255 * t)
      const g = Math.round(100 * (1 - Math.abs(t - 0.5) * 2))
      const b = Math.round(255 * (1 - t))
      ctx.fillStyle = `rgb(${r},${g},${b})`
      ctx.fillRect(j, i, 1, 1)
    }
  }

  heatmapUrl.value = canvas.toDataURL()
}, { immediate: true })

const xRange2D = computed(() => {
  if (!chart2D.value) return { min: -50, max: 50 }
  const xs = chart2D.value.x_grid
  return { min: Math.min(...xs), max: Math.max(...xs) }
})

const yRange2D = computed(() => {
  if (!chart2D.value) return { min: -50, max: 50 }
  const ys = chart2D.value.y_grid
  return { min: Math.min(...ys), max: Math.max(...ys) }
})

const bestPt2D = computed(() => {
  if (!chart2D.value) return { px: 0, py: 0 }
  const x = chart2D.value.best_gene_x
  const y = chart2D.value.best_gene_y
  const px = padL + ((x - xRange2D.value.min) / (xRange2D.value.max - xRange2D.value.min)) * (heatW - padL - padR)
  const py = padT + ((y - yRange2D.value.min) / (yRange2D.value.max - yRange2D.value.min)) * (heatH - padT - padB)
  return { px, py }
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
.genetic-page {
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
  margin-bottom: 16px;
}

.chart-title {
  font-size: 14px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 12px 0;
}

.chart-svg {
  width: 100%;
  height: auto;
}

.heatmap-wrap {
  max-width: 500px;
  margin: 0 auto;
}

.tick-text {
  font-size: 10px;
  fill: var(--text-muted);
}

.chart-legend {
  display: flex;
  gap: 16px;
  margin-top: 10px;
  font-size: 12px;
  color: var(--text-secondary);
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 6px;
}

.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  display: inline-block;
}

.dot.green {
  background: var(--accent-green);
}

.dot.blue {
  background: var(--accent-blue);
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
