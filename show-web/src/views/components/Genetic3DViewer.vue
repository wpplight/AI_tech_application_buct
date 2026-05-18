<template>
  <div class="viewer-wrapper">
    <div ref="container" class="three-container"></div>
    <div class="zoom-controls">
      <button class="zoom-btn" @click="zoomIn" title="放大">+</button>
      <button class="zoom-btn" @click="zoomOut" title="缩小">−</button>
      <button class="zoom-btn" @click="resetView" title="重置视角">⟲</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, computed } from 'vue'
import * as THREE from 'three'
import { useMLearnStore } from '../../stores/mlearn'

const mlearn = useMLearnStore()
const container = ref<HTMLDivElement | null>(null)

const chart2D = computed(() => mlearn.genetic2DData)

let scene: THREE.Scene
let camera: THREE.PerspectiveCamera
let renderer: THREE.WebGLRenderer
let mesh: THREE.Mesh
let animationId: number

const HEIGHT_SCALE = 8

interface GenePoint {
  x: number
  y: number
  isBest: boolean
}

function createSurfaceWithGenes(
  xGrid: number[],
  yGrid: number[],
  fitnessGrid: number[],
  genes: GenePoint[]
) {
  const totalPoints = fitnessGrid.length
  const size = Math.round(Math.sqrt(totalPoints))

  const vertices: number[] = []
  const colors: number[] = []
  const indices: number[] = []

  const minVal = Math.min(...fitnessGrid)
  const maxVal = Math.max(...fitnessGrid)
  const range = maxVal - minVal || 1

  const baseColors: number[][] = []
  for (let i = 0; i < size; i++) {
    for (let j = 0; j < size; j++) {
      const idx = i * size + j
      if (idx >= totalPoints) continue

      const px = xGrid[idx] ?? (i / (size - 1) * 100 - 50)
      const py = yGrid[idx] ?? (j / (size - 1) * 100 - 50)
      const pz = (fitnessGrid[idx] ?? 0) * HEIGHT_SCALE

      vertices.push(px, pz, py)

      const t = Math.max(0, Math.min(1, (pz - minVal * HEIGHT_SCALE) / (range * HEIGHT_SCALE)))
      const r = 100 + 120 * t
      const g = 50 * (1 - Math.abs(t - 0.5) * 2)
      const b = 255 * (1 - t)
      baseColors.push([r / 255, g / 255, b / 255])
    }
  }

  const influenceRadius = 8
  const finalColors: number[][] = baseColors.map(c => [...c])

  for (const gene of genes) {
    for (let i = 0; i < size; i++) {
      for (let j = 0; j < size; j++) {
        const idx = i * size + j
        if (idx >= totalPoints) continue

        const px = xGrid[idx] ?? (i / (size - 1) * 100 - 50)
        const py = yGrid[idx] ?? (j / (size - 1) * 100 - 50)

        const dist = Math.sqrt((px - gene.x) ** 2 + (py - gene.y) ** 2)

        if (dist < influenceRadius) {
          const influence = Math.max(0, 1 - dist / influenceRadius)
          const blendFactor = influence * influence * (gene.isBest ? 0.9 : 0.5)

          const targetColor: [number, number, number] = gene.isBest
            ? [0.0, 1.0, 0.2]
            : [1.0, 0.85, 0.0]

          const colorArr = finalColors[idx]
          if (colorArr && colorArr.length >= 3) {
            const r = colorArr[0] ?? 0
            const g = colorArr[1] ?? 0
            const b = colorArr[2] ?? 0
            colorArr[0] = r * (1 - blendFactor) + targetColor[0] * blendFactor
            colorArr[1] = g * (1 - blendFactor) + targetColor[1] * blendFactor
            colorArr[2] = b * (1 - blendFactor) + targetColor[2] * blendFactor
          }
        }
      }
    }
  }

  for (let i = 0; i < size; i++) {
    for (let j = 0; j < size; j++) {
      const color = finalColors[i * size + j]
      if (color) {
        colors.push(color[0] as number, color[1] as number, color[2] as number)
      }
    }
  }

  for (let i = 0; i < size - 1; i++) {
    for (let j = 0; j < size - 1; j++) {
      const a = i * size + j
      const b = (i + 1) * size + j
      const c = (i + 1) * size + (j + 1)
      const d = i * size + (j + 1)
      indices.push(a, b, c)
      indices.push(a, c, d)
    }
  }

  const geometry = new THREE.BufferGeometry()
  geometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3))
  geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3))
  geometry.setIndex(indices)
  geometry.computeVertexNormals()

  const material = new THREE.MeshPhongMaterial({
    vertexColors: true,
    side: THREE.DoubleSide,
    shininess: 80,
    flatShading: false,
    transparent: true,
    opacity: 0.9
  })

  return new THREE.Mesh(geometry, material)
}

function init() {
  if (!container.value) return

  const width = container.value.clientWidth
  const height = container.value.clientHeight

  scene = new THREE.Scene()
  scene.background = new THREE.Color(0x111827)

  camera = new THREE.PerspectiveCamera(60, width / height, 0.1, 1000)
  camera.position.set(80, 60, 80)
  camera.lookAt(0, 0, 0)

  renderer = new THREE.WebGLRenderer({ antialias: true })
  renderer.setSize(width, height)
  renderer.setPixelRatio(window.devicePixelRatio)
  container.value.appendChild(renderer.domElement)

  const ambientLight = new THREE.AmbientLight(0xffffff, 0.6)
  scene.add(ambientLight)

  const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8)
  directionalLight.position.set(50, 100, 50)
  scene.add(directionalLight)

  const pointLight = new THREE.PointLight(0xffffff, 0.4)
  pointLight.position.set(-30, 50, -30)
  scene.add(pointLight)

  updateSurface()
  animate()
  setupControls()
}

function updateSurface() {
  if (mesh) {
    scene.remove(mesh)
    mesh.geometry.dispose()
    ;(mesh.material as THREE.Material).dispose()
  }

  if (!chart2D.value) return

  const xGrid = chart2D.value.x_grid
  const yGrid = chart2D.value.y_grid
  const fitnessGrid = chart2D.value.fitness_grid
  const popX = chart2D.value.population_x
  const popY = chart2D.value.population_y
  const bestX = chart2D.value.best_gene_x
  const bestY = chart2D.value.best_gene_y

  const genes: GenePoint[] = []

  const n = Math.min(popX.length, popY.length)
  for (let i = 0; i < n; i++) {
    genes.push({ x: popX[i] as number, y: popY[i] as number, isBest: false })
  }

  genes.push({ x: bestX, y: bestY, isBest: true })

  mesh = createSurfaceWithGenes(xGrid, yGrid, fitnessGrid, genes)
  scene.add(mesh)
}

function animate() {
  animationId = requestAnimationFrame(animate)
  renderer.render(scene, camera)
}

let isDragging = false
let lastTouchDist = 0
let lastTouchCenter = { x: 0, y: 0 }
let spherical = { theta: Math.PI / 4, phi: Math.PI / 3, radius: 120 }

function getTouchDistance(touches: TouchList): number {
  if (touches.length < 2) return 0
  const t0 = touches[0]
  const t1 = touches[1]
  if (!t0 || !t1) return 0
  const dx = t1.clientX - t0.clientX
  const dy = t1.clientY - t0.clientY
  return Math.sqrt(dx * dx + dy * dy)
}

function getTouchCenter(touches: TouchList): { x: number; y: number } {
  if (touches.length < 2) return { x: 0, y: 0 }
  const t0 = touches[0]
  const t1 = touches[1]
  if (!t0 || !t1) return { x: 0, y: 0 }
  return {
    x: (t0.clientX + t1.clientX) / 2,
    y: (t0.clientY + t1.clientY) / 2
  }
}

function setupControls() {
  if (!container.value) return

  const canvas = renderer.domElement

  canvas.addEventListener('touchstart', (e) => {
    e.preventDefault()
    if (e.touches.length === 1 && e.touches[0]) {
      isDragging = true
      lastTouchCenter = { x: e.touches[0].clientX, y: e.touches[0].clientY }
      lastTouchDist = 0
    } else if (e.touches.length === 2) {
      isDragging = false
      lastTouchDist = getTouchDistance(e.touches)
      lastTouchCenter = getTouchCenter(e.touches)
    }
  }, { passive: false })

  canvas.addEventListener('touchmove', (e) => {
    e.preventDefault()
    if (e.touches.length === 1 && isDragging && e.touches[0]) {
      const deltaX = e.touches[0].clientX - lastTouchCenter.x
      spherical.theta -= deltaX * 0.01
      lastTouchCenter = { x: e.touches[0].clientX, y: e.touches[0].clientY }
      updateCameraPosition()
    } else if (e.touches.length === 2) {
      const dist = getTouchDistance(e.touches)
      const center = getTouchCenter(e.touches)

      const distDelta = Math.abs(dist - lastTouchDist)
      const centerDeltaX = Math.abs(center.x - lastTouchCenter.x)

      if (distDelta > centerDeltaX && distDelta > 5) {
        const scale = lastTouchDist / dist
        spherical.radius = Math.max(30, Math.min(300, spherical.radius * scale))
        lastTouchDist = dist
      } else if (centerDeltaX > 5) {
        const deltaX = center.x - lastTouchCenter.x
        spherical.theta -= deltaX * 0.01
      }

      lastTouchCenter = center
      updateCameraPosition()
    }
  }, { passive: false })

  canvas.addEventListener('touchend', (e) => {
    if (e.touches.length === 0) {
      isDragging = false
      lastTouchDist = 0
    } else if (e.touches.length === 1 && e.touches[0]) {
      isDragging = true
      lastTouchCenter = { x: e.touches[0].clientX, y: e.touches[0].clientY }
    }
  }, false)

  canvas.addEventListener('mousedown', (e) => {
    e.preventDefault()
    isDragging = true
    lastTouchCenter = { x: e.clientX, y: e.clientY }
    lastTouchDist = 0
  })

  canvas.addEventListener('mousemove', (e) => {
    if (!isDragging) return
    e.preventDefault()

    const deltaX = e.clientX - lastTouchCenter.x
    spherical.theta -= deltaX * 0.01
    lastTouchCenter = { x: e.clientX, y: e.clientY }
    updateCameraPosition()
  })

  canvas.addEventListener('mouseup', () => {
    isDragging = false
  })

  canvas.addEventListener('mouseleave', () => {
    isDragging = false
  })

  canvas.addEventListener('wheel', (e) => {
    e.preventDefault()
    if (Math.abs(e.deltaX) > Math.abs(e.deltaY)) {
      spherical.theta -= e.deltaX * 0.005
    } else {
      spherical.phi -= e.deltaY * 0.005
      spherical.phi = Math.max(0.1, Math.min(Math.PI - 0.1, spherical.phi))
    }
    updateCameraPosition()
  }, { passive: false })
}

function updateCameraPosition() {
  const x = spherical.radius * Math.sin(spherical.phi) * Math.cos(spherical.theta)
  const y = spherical.radius * Math.cos(spherical.phi)
  const z = spherical.radius * Math.sin(spherical.phi) * Math.sin(spherical.theta)

  camera.position.set(x, y, z)
  camera.lookAt(0, 0, 0)
}

function zoomIn() {
  spherical.radius = Math.max(30, spherical.radius * 0.8)
  updateCameraPosition()
}

function zoomOut() {
  spherical.radius = Math.min(300, spherical.radius * 1.25)
  updateCameraPosition()
}

function resetView() {
  spherical.theta = Math.PI / 4
  spherical.phi = Math.PI / 3
  spherical.radius = 120
  updateCameraPosition()
}

function handleResize() {
  if (!container.value || !renderer || !camera) return

  const width = container.value.clientWidth
  const height = container.value.clientHeight

  camera.aspect = width / height
  camera.updateProjectionMatrix()
  renderer.setSize(width, height)
}

onMounted(() => {
  init()
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  cancelAnimationFrame(animationId)
  window.removeEventListener('resize', handleResize)

  if (renderer) {
    renderer.dispose()
  }
})

watch(chart2D, () => {
  updateSurface()
}, { deep: true })
</script>

<style scoped>
.viewer-wrapper {
  position: relative;
  width: 100%;
}

.three-container {
  width: 100%;
  height: 400px;
  border-radius: 8px;
  cursor: grab;
  overflow: hidden;
  touch-action: none;
  user-select: none;
}

.three-container:active {
  cursor: grabbing;
}

.zoom-controls {
  position: absolute;
  top: 12px;
  right: 12px;
  display: flex;
  flex-direction: column;
  gap: 6px;
  z-index: 10;
}

.zoom-btn {
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 8px;
  background: rgba(30, 41, 59, 0.9);
  color: #e2e8f0;
  font-size: 18px;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

.zoom-btn:hover {
  background: rgba(59, 130, 246, 0.9);
  transform: scale(1.05);
}

.zoom-btn:active {
  transform: scale(0.95);
}
</style>