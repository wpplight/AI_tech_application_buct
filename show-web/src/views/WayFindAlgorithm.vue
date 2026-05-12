<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue'

const currentSlide = ref(0)
const totalSlides = 4
const isAnimating = ref(false)

const slides = [
  { id: 'overview', title: '应用概述', badge: 'OVERVIEW' },
  { id: 'bfs', title: '广度优先搜索', badge: 'BFS' },
  { id: 'dfs', title: '深度优先搜索', badge: 'DFS' },
  { id: 'astar', title: 'A* 搜索算法', badge: 'A*' },
]

function goTo(index: number) {
  if (isAnimating.value || index === currentSlide.value) return
  if (index < 0 || index >= totalSlides) return
  isAnimating.value = true
  currentSlide.value = index
  setTimeout(() => { isAnimating.value = false }, 600)
}

function next() { goTo(currentSlide.value + 1) }
function prev() { goTo(currentSlide.value - 1) }

const touchStartX = ref(0)
const touchEndX = ref(0)

function onTouchStart(e: TouchEvent) {
  touchStartX.value = e.changedTouches[0]!.screenX
}
function onTouchEnd(e: TouchEvent) {
  touchEndX.value = e.changedTouches[0]!.screenX
  const diff = touchStartX.value - touchEndX.value
  if (Math.abs(diff) > 50) {
    diff > 0 ? next() : prev()
  }
}

function onKeyDown(e: KeyboardEvent) {
  if (e.key === 'ArrowRight') next()
  else if (e.key === 'ArrowLeft') prev()
}

onMounted(() => { document.addEventListener('keydown', onKeyDown) })
onUnmounted(() => { document.removeEventListener('keydown', onKeyDown) })

const bfsAnimActive = ref(false)
const dfsAnimActive = ref(false)
const astarAnimActive = ref(false)

function playBfs() {
  bfsAnimActive.value = false
  nextTick(() => { bfsAnimActive.value = true })
}
function playDfs() {
  dfsAnimActive.value = false
  nextTick(() => { dfsAnimActive.value = true })
}
function playAstar() {
  astarAnimActive.value = false
  nextTick(() => { astarAnimActive.value = true })
}
</script>

<template>
  <div class="algo-view" @touchstart="onTouchStart" @touchend="onTouchEnd">
    <div class="algo-header">
      <div class="header-left">
        <span class="page-badge">ALGORITHM</span>
        <h1 class="page-title">算法讲解</h1>
        <p class="page-desc">理解三种经典寻路算法的核心思想</p>
      </div>
      <div class="slide-nav">
        <button
          v-for="(s, i) in slides"
          :key="s.id"
          class="nav-dot"
          :class="{ active: currentSlide === i }"
          @click="goTo(i)"
        >
          <span class="dot-label">{{ s.badge }}</span>
        </button>
      </div>
    </div>

    <div class="slides-viewport">
      <div
        class="slides-track"
        :style="{ transform: `translateX(-${currentSlide * 100}%)` }"
      >
        <!-- Slide 0: Overview -->
        <div class="slide">
          <div class="slide-content-box overview-slide">
          <div class="overview-text">
            <span class="slide-badge">OVERVIEW</span>
            <h2 class="slide-title">寻路算法可视化平台</h2>
            <p class="slide-body">
              寻路算法是图论和人工智能中的经典问题，目标是在一个带障碍物的网格地图中，
              找到从<strong>起点</strong>到<strong>终点</strong>的最优路径。
            </p>
            <p class="slide-body">
              本平台实现了三种经典算法，每种算法都有不同的搜索策略和适用场景。
              通过可视化的逐步演示，你可以直观地观察它们的探索过程和差异。
            </p>
            <div class="overview-cards">
              <div class="algo-card" @click="goTo(1)">
                <div class="algo-card-icon bfs-icon">BFS</div>
                <div class="algo-card-info">
                  <span class="algo-card-name">广度优先搜索</span>
                  <span class="algo-card-hint">层层扩展 · 最短路径</span>
                </div>
                <svg class="algo-card-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
              </div>
              <div class="algo-card" @click="goTo(2)">
                <div class="algo-card-icon dfs-icon">DFS</div>
                <div class="algo-card-info">
                  <span class="algo-card-name">深度优先搜索</span>
                  <span class="algo-card-hint">深入探索 · 回溯机制</span>
                </div>
                <svg class="algo-card-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
              </div>
              <div class="algo-card" @click="goTo(3)">
                <div class="algo-card-icon astar-icon">A*</div>
                <div class="algo-card-info">
                  <span class="algo-card-name">A* 启发式搜索</span>
                  <span class="algo-card-hint">启发评估 · 高效最优</span>
                </div>
                <svg class="algo-card-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
              </div>
            </div>
          </div>
          <div class="overview-visual">
            <svg viewBox="0 0 320 240" class="overview-svg">
              <defs>
                <linearGradient id="pathGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                  <stop offset="0%" stop-color="#a855f7"/>
                  <stop offset="100%" stop-color="#3b82f6"/>
                </linearGradient>
              </defs>
              <rect x="20" y="20" width="280" height="200" rx="16" fill="rgba(24,24,27,0.5)" stroke="rgba(63,63,70,0.4)" stroke-width="1"/>
              <g v-for="row in 5" :key="'or'+row">
                <rect v-for="col in 7" :key="'oc'+row+col"
                  :x="30 + (col-1)*38" :y="30 + (row-1)*38" width="34" height="34" rx="6"
                  :fill="(row===1 && col===1) ? '#10b981' : (row===5 && col===7) ? '#f43f5e' : (row===3 && col>=3 && col<=5) || (row===4 && col===3) ? '#3f3f46' : 'rgba(63,63,70,0.2)'"
                  :stroke="(row===1 && col===1) ? '#10b981' : (row===5 && col===7) ? '#f43f5e' : 'rgba(63,63,70,0.3)'"
                  stroke-width="0.5"
                />
              </g>
              <path d="M47 47 L85 47 L85 85 L123 85 L123 123 L161 123 L161 161 L199 161 L237 161 L275 199"
                fill="none" stroke="url(#pathGrad)" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"
                stroke-dasharray="600" stroke-dashoffset="600" class="overview-path-anim"/>
              <circle cx="47" cy="47" r="6" fill="#10b981" class="pulse-dot"/>
              <circle cx="275" cy="199" r="6" fill="#f43f5e" class="pulse-dot" style="animation-delay:0.5s"/>
            </svg>
          </div>
          </div>
        </div>

        <!-- Slide 1: BFS -->
        <div class="slide">
          <div class="slide-content-box algo-slide">
          <div class="algo-content">
            <span class="slide-badge bfs-badge">BFS</span>
            <h2 class="slide-title">广度优先搜索</h2>
            <p class="slide-subtitle">Breadth-First Search</p>
            <div class="algo-desc">
              <p>
                BFS 从起点开始，<strong>逐层</strong>向外扩展。它先访问所有距离起点为 1 的节点，
                再访问距离为 2 的节点，依此类推，像水波纹一样扩散。
              </p>
              <div class="key-points">
                <div class="point">
                  <span class="point-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M9 12l2 2 4-4"/><circle cx="12" cy="12" r="10"/></svg>
                  </span>
                  <span>使用<strong>队列 (FIFO)</strong> 存储待访问节点</span>
                </div>
                <div class="point">
                  <span class="point-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M9 12l2 2 4-4"/><circle cx="12" cy="12" r="10"/></svg>
                  </span>
                  <span>保证找到的路径是<strong>最短路径</strong>（等权图）</span>
                </div>
                <div class="point">
                  <span class="point-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M9 12l2 2 4-4"/><circle cx="12" cy="12" r="10"/></svg>
                  </span>
                  <span>时间复杂度 <strong>O(V+E)</strong>，空间复杂度较高</span>
                </div>
              </div>
              <div class="complexity-bar">
                <div class="comp-item">
                  <span class="comp-label">最优性</span>
                  <div class="comp-track"><div class="comp-fill" style="width:100%"></div></div>
                </div>
                <div class="comp-item">
                  <span class="comp-label">速度</span>
                  <div class="comp-track"><div class="comp-fill" style="width:60%"></div></div>
                </div>
                <div class="comp-item">
                  <span class="comp-label">空间</span>
                  <div class="comp-track"><div class="comp-fill bfs-fill" style="width:80%"></div></div>
                </div>
              </div>
            </div>
          </div>
          <div class="algo-visual">
            <div class="anim-card">
              <div class="anim-header">
                <span class="anim-title">BFS 逐层扩展演示</span>
                <button class="anim-btn" @click="playBfs">
                  <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
                  播放
                </button>
              </div>
              <div class="anim-body">
                <svg viewBox="0 0 260 260" class="bfs-svg">
                  <defs>
                    <filter id="bfsGlow"><feGaussianBlur stdDeviation="3" result="blur"/><feMerge><feMergeNode in="blur"/><feMergeNode in="SourceGraphic"/></feMerge></filter>
                  </defs>
                  <g v-for="r in 6" :key="'br'+r">
                    <rect v-for="c in 6" :key="'bc'+r+c"
                      :x="10+(c-1)*40" :y="10+(r-1)*40" width="36" height="36" rx="8"
                      :fill="(r===1&&c===1)?'#10b981':(r===6&&c===6)?'#f43f5e':(r===3&&c>=3&&c<=4)||(r===4&&c===3)?'#3f3f46':'rgba(63,63,70,0.15)'"
                      :stroke="(r===1&&c===1)?'#10b981':(r===6&&c===6)?'#f43f5e':'rgba(63,63,70,0.25)'"
                      stroke-width="0.5"
                    />
                  </g>
                  <template v-if="bfsAnimActive">
                    <rect v-for="(cell, idx) in [
                      [1,1],[1,2],[2,1],[2,2],[1,3],[3,1],[3,2],[2,3],[3,3],
                      [1,4],[4,1],[4,2],[2,4],[4,3],[3,4],[4,4],
                      [1,5],[5,1],[5,2],[2,5],[5,3],[3,5],[5,4],[4,5],[5,5],
                      [1,6],[6,1],[6,2],[2,6],[6,3],[3,6],[6,4],[4,6],[5,6],[6,5],[6,6]
                    ]" :key="'ba'+idx"
                      :x="10+((cell[1]??0)-1)*40" :y="10+((cell[0]??0)-1)*40" width="36" height="36" rx="8"
                      fill="rgba(59,130,246,0.3)" stroke="#3b82f6" stroke-width="1.5"
                      filter="url(#bfsGlow)"
                      :style="{ animationDelay: `${idx * 0.12}s` }"
                      class="bfs-wave"
                    />
                    <rect x="10" y="10" width="36" height="36" rx="8" fill="rgba(16,185,129,0.3)" stroke="#10b981" stroke-width="2" class="start-pulse"/>
                    <rect x="210" y="210" width="36" height="36" rx="8" fill="rgba(244,63,94,0.3)" stroke="#f43f5e" stroke-width="2" class="end-pulse"/>
                  </template>
                  <circle cx="28" cy="28" r="5" fill="#10b981"/>
                  <circle cx="228" cy="228" r="5" fill="#f43f5e"/>
                </svg>
              </div>
              <div class="anim-caption">像水波纹一样，从起点逐层向外扩展，直到覆盖终点</div>
            </div>
          </div>
          </div>
        </div>

        <!-- Slide 2: DFS -->
        <div class="slide">
          <div class="slide-content-box algo-slide">
          <div class="algo-content">
            <span class="slide-badge dfs-badge">DFS</span>
            <h2 class="slide-title">深度优先搜索</h2>
            <p class="slide-subtitle">Depth-First Search</p>
            <div class="algo-desc">
              <p>
                DFS 沿着一条路径<strong>尽可能深</strong>地探索，直到遇到死路才回溯到上一个分叉点，
                尝试其他方向。像走迷宫时一直贴着右边墙壁走。
              </p>
              <div class="key-points">
                <div class="point">
                  <span class="point-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M9 12l2 2 4-4"/><circle cx="12" cy="12" r="10"/></svg>
                  </span>
                  <span>使用<strong>栈 (LIFO)</strong> 或递归实现</span>
                </div>
                <div class="point">
                  <span class="point-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M9 12l2 2 4-4"/><circle cx="12" cy="12" r="10"/></svg>
                  </span>
                  <span><strong>不保证</strong>找到最短路径，但空间效率高</span>
                </div>
                <div class="point">
                  <span class="point-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M9 12l2 2 4-4"/><circle cx="12" cy="12" r="10"/></svg>
                  </span>
                  <span>适合探索连通性、拓扑排序等场景</span>
                </div>
              </div>
              <div class="complexity-bar">
                <div class="comp-item">
                  <span class="comp-label">最优性</span>
                  <div class="comp-track"><div class="comp-fill dfs-fill" style="width:30%"></div></div>
                </div>
                <div class="comp-item">
                  <span class="comp-label">速度</span>
                  <div class="comp-track"><div class="comp-fill" style="width:70%"></div></div>
                </div>
                <div class="comp-item">
                  <span class="comp-label">空间</span>
                  <div class="comp-track"><div class="comp-fill" style="width:40%"></div></div>
                </div>
              </div>
            </div>
          </div>
          <div class="algo-visual">
            <div class="anim-card">
              <div class="anim-header">
                <span class="anim-title">DFS 深度探索演示</span>
                <button class="anim-btn" @click="playDfs">
                  <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
                  播放
                </button>
              </div>
              <div class="anim-body">
                <svg viewBox="0 0 260 260" class="dfs-svg">
                  <g v-for="r in 6" :key="'dr'+r">
                    <rect v-for="c in 6" :key="'dc'+r+c"
                      :x="10+(c-1)*40" :y="10+(r-1)*40" width="36" height="36" rx="8"
                      :fill="(r===1&&c===1)?'#10b981':(r===6&&c===6)?'#f43f5e':(r===3&&c>=3&&c<=4)||(r===4&&c===3)?'#3f3f46':'rgba(63,63,70,0.15)'"
                      :stroke="(r===1&&c===1)?'#10b981':(r===6&&c===6)?'#f43f5e':'rgba(63,63,70,0.25)'"
                      stroke-width="0.5"
                    />
                  </g>
                  <template v-if="dfsAnimActive">
                    <path
                      d="M28 28 L68 28 L108 28 L148 28 L188 28 L228 28
                         L228 68 L188 68 L148 68 L108 68 L68 68 L28 68
                         L28 108 L68 108
                         L68 148 L28 148
                         L28 188 L68 188 L108 188 L148 188 L188 188 L228 188
                         L228 228"
                      fill="none" stroke="#a855f7" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"
                      stroke-dasharray="1600" stroke-dashoffset="1600"
                      class="dfs-path-line"
                    />
                    <rect v-for="(cell, idx) in [
                      [1,1],[1,2],[1,3],[1,4],[1,5],[1,6],
                      [2,6],[2,5],[2,4],[2,3],[2,2],[2,1],
                      [3,1],[3,2],
                      [4,2],[4,1],
                      [5,1],[5,2],[5,3],[5,4],[5,5],[5,6],
                      [6,6]
                    ]" :key="'da'+idx"
                      :x="10+((cell[1]??0)-1)*40" :y="10+((cell[0]??0)-1)*40" width="36" height="36" rx="8"
                      fill="rgba(168,85,247,0.25)" stroke="#a855f7" stroke-width="1.5"
                      :style="{ animationDelay: `${idx * 0.12}s` }"
                      class="dfs-dive"
                    />
                    <rect x="10" y="10" width="36" height="36" rx="8" fill="rgba(16,185,129,0.3)" stroke="#10b981" stroke-width="2" class="start-pulse"/>
                    <rect x="210" y="210" width="36" height="36" rx="8" fill="rgba(244,63,94,0.3)" stroke="#f43f5e" stroke-width="2" class="end-pulse"/>
                  </template>
                  <circle cx="28" cy="28" r="5" fill="#10b981"/>
                  <circle cx="228" cy="228" r="5" fill="#f43f5e"/>
                </svg>
              </div>
              <div class="anim-caption">沿一条路深入到底，遇阻则回溯，尝试其他分支</div>
            </div>
          </div>
          </div>
        </div>

        <!-- Slide 3: A* -->
        <div class="slide">
          <div class="slide-content-box algo-slide astar-slide">
          <div class="algo-content">
            <span class="slide-badge astar-badge">A*</span>
            <h2 class="slide-title">A* 启发式搜索</h2>
            <p class="slide-subtitle">A-Star Heuristic Search</p>
            <div class="algo-desc">
              <p>
                A* 算法是 Dijkstra 算法的优化版本，通过引入<strong>启发函数</strong>来引导搜索方向，
                从而大幅减少不必要的探索。它是寻路算法中最常用的选择。
              </p>
              <div class="formula-card">
                <div class="formula">f(n) = g(n) + h(n)</div>
                <div class="formula-items">
                  <div class="formula-item">
                    <span class="f-var g-var">g(n)</span>
                    <span class="f-desc">从<strong>起点</strong>到当前节点的<strong>实际代价</strong></span>
                  </div>
                  <div class="formula-item">
                    <span class="f-var h-var">h(n)</span>
                    <span class="f-desc">从当前节点到<strong>终点</strong>的<strong>估计代价</strong>（启发值）</span>
                  </div>
                  <div class="formula-item">
                    <span class="f-var f-var-color">f(n)</span>
                    <span class="f-desc">经过当前节点到达终点的<strong>总估计代价</strong></span>
                  </div>
                </div>
              </div>
              <div class="key-points">
                <div class="point">
                  <span class="point-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M9 12l2 2 4-4"/><circle cx="12" cy="12" r="10"/></svg>
                  </span>
                  <span>使用<strong>最小堆（Min-Heap）</strong>维护待扩展节点，堆顶始终是 f 值最小的节点，弹出复杂度 O(log n)</span>
                </div>
                <div class="point">
                  <span class="point-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M9 12l2 2 4-4"/><circle cx="12" cy="12" r="10"/></svg>
                  </span>
                  <span>启发函数可采纳时保证<strong>最优解</strong></span>
                </div>
              </div>
            </div>
          </div>
          <div class="algo-visual">
            <div class="anim-card">
              <div class="anim-header">
                <span class="anim-title">A* 启发式搜索演示</span>
                <button class="anim-btn" @click="playAstar">
                  <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
                  播放
                </button>
              </div>
              <div class="anim-body">
                <svg viewBox="0 0 260 260" class="astar-svg">
                  <defs>
                    <marker id="arrowHead" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
                      <polygon points="0 0, 8 3, 0 6" fill="rgba(16,185,129,0.5)"/>
                    </marker>
                  </defs>
                  <g v-for="r in 6" :key="'ar'+r">
                    <rect v-for="c in 6" :key="'ac'+r+c"
                      :x="10+(c-1)*40" :y="10+(r-1)*40" width="36" height="36" rx="8"
                      :fill="(r===1&&c===1)?'#10b981':(r===6&&c===6)?'#f43f5e':(r===3&&c>=3&&c<=4)||(r===4&&c===3)?'#3f3f46':'rgba(63,63,70,0.15)'"
                      :stroke="(r===1&&c===1)?'#10b981':(r===6&&c===6)?'#f43f5e':'rgba(63,63,70,0.25)'"
                      stroke-width="0.5"
                    />
                  </g>
                  <template v-if="astarAnimActive">
                    <rect v-for="(cell, idx) in [
                      [1,1],[1,2],[2,2],[2,3],[2,4],[2,5],[2,6],
                      [3,6],[4,6],[4,5],[5,5],[5,6],[6,6]
                    ]" :key="'aa'+idx"
                      :x="10+((cell[1]??0)-1)*40" :y="10+((cell[0]??0)-1)*40" width="36" height="36" rx="8"
                      fill="rgba(16,185,129,0.15)" stroke="#10b981" stroke-width="1.5"
                      :style="{ animationDelay: `${idx * 0.18}s` }"
                      class="astar-guided"
                    />
                    <path
                      d="M28 28 L68 28 L68 68 L108 68 L148 68 L188 68 L228 68 L228 108 L228 148 L188 148 L188 188 L228 188 L228 228"
                      fill="none" stroke="#10b981" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"
                      stroke-dasharray="1200" stroke-dashoffset="1200"
                      class="astar-path-line"
                    />
                    <path
                      d="M55 15 L200 85" fill="none" stroke="rgba(16,185,129,0.4)" stroke-width="1.5"
                      stroke-dasharray="4 4" marker-end="url(#arrowHead)"
                      class="heuristic-arrow"
                    />
                    <rect x="10" y="10" width="36" height="36" rx="8" fill="rgba(16,185,129,0.3)" stroke="#10b981" stroke-width="2" class="start-pulse"/>
                    <rect x="210" y="210" width="36" height="36" rx="8" fill="rgba(244,63,94,0.3)" stroke="#f43f5e" stroke-width="2" class="end-pulse"/>
                    <text v-for="(label, idx) in [
                      {x:28,y:28,t:'S'},{x:228,y:228,t:'E'},
                    ]" :key="'at'+idx"
                      :x="label.x" :y="label.y+1" text-anchor="middle" dominant-baseline="middle"
                      fill="white" font-size="11" font-weight="700"
                    >{{ label.t }}</text>
                  </template>
                  <circle cx="28" cy="28" r="5" fill="#10b981"/>
                  <circle cx="228" cy="228" r="5" fill="#f43f5e"/>
                </svg>
              </div>
              <div class="anim-caption">启发函数引导搜索方向，优先探索最有希望的路径</div>
            </div>
          </div>
          </div>
        </div>
      </div>
    </div>

    <div class="slide-controls">
      <button class="ctrl-btn" :disabled="currentSlide === 0" @click="prev">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M15 19l-7-7 7-7"/></svg>
      </button>
      <span class="slide-counter">{{ currentSlide + 1 }} / {{ totalSlides }}</span>
      <button class="ctrl-btn" :disabled="currentSlide === totalSlides - 1" @click="next">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 5l7 7-7 7"/></svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.algo-view {
  max-width: 960px;
  display: flex;
  flex-direction: column;
  gap: 20px;
  height: calc(100vh - 48px);
  overflow: hidden;
  user-select: none;
}

.algo-header {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: 16px;
  flex-wrap: wrap;
  flex-shrink: 0;
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.page-badge {
  display: inline-block;
  padding: 3px 10px;
  background: rgba(59,130,246,0.1);
  border: 1px solid rgba(59,130,246,0.2);
  border-radius: 20px;
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.15em;
  color: var(--accent-blue);
  width: fit-content;
}

.page-title {
  font-size: 24px;
  font-weight: 700;
  letter-spacing: -0.02em;
  color: var(--text-primary);
}

.page-desc {
  font-size: 13px;
  color: var(--text-secondary);
}

.slide-nav {
  display: flex;
  gap: 4px;
}

.nav-dot {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 14px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 20px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

.nav-dot:hover {
  border-color: var(--accent-blue);
}

.nav-dot.active {
  background: rgba(59,130,246,0.1);
  border-color: var(--accent-blue);
}

.dot-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  letter-spacing: 0.05em;
}

.nav-dot.active .dot-label {
  color: var(--accent-blue);
}

.slides-viewport {
  flex: 1;
  min-height: 0;
  overflow: hidden;
  border-radius: 16px;
}

.slides-track {
  display: flex;
  height: 100%;
  transition: transform 0.6s cubic-bezier(0.32, 0.72, 0, 1);
}

.slide {
  min-width: 100%;
  flex-shrink: 0;
  height: 100%;
  position: relative;
  overflow: hidden;
}

.slide-content-box {
  position: absolute;
  inset: 0;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 24px 28px;
  scrollbar-width: none;
  display: flex;
  gap: 24px;
}

.slide-content-box::-webkit-scrollbar {
  display: none;
}

.slide-badge {
  display: inline-block;
  padding: 3px 10px;
  border-radius: 20px;
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 0.15em;
  background: rgba(161,161,170,0.15);
  color: var(--text-secondary);
  width: fit-content;
  margin-bottom: 8px;
}

.bfs-badge { background: rgba(59,130,246,0.15); color: #3b82f6; }
.dfs-badge { background: rgba(168,85,247,0.15); color: #a855f7; }
.astar-badge { background: rgba(16,185,129,0.15); color: #10b981; }

.slide-title {
  font-size: 28px;
  font-weight: 700;
  letter-spacing: -0.02em;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.slide-subtitle {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 20px;
}

.slide-body {
  font-size: 14px;
  line-height: 1.7;
  color: var(--text-secondary);
  margin-bottom: 12px;
}

.slide-body strong {
  color: var(--text-primary);
  font-weight: 600;
}

.overview-slide {
  flex-direction: column;
}

.overview-slide > .overview-text,
.overview-slide > .overview-visual {
  width: 100%;
}

@media (min-width: 769px) {
  .overview-slide {
    flex-direction: row !important;
    align-items: flex-start;
    gap: 32px;
  }
}

.overview-text {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  min-height: 0;
  overflow-y: auto;
}

.overview-visual {
  flex: 0 0 320px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.overview-svg {
  width: 100%;
  max-width: 320px;
}

.overview-path-anim {
  animation: drawPath 3s ease-out forwards;
  animation-delay: 0.5s;
}

@keyframes drawPath {
  to { stroke-dashoffset: 0; }
}

.pulse-dot {
  animation: pulseGlow 2s ease-in-out infinite;
}

@keyframes pulseGlow {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.overview-cards {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-top: 16px;
}

.algo-card {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 14px 16px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

.algo-card:hover {
  border-color: var(--accent-blue);
  transform: translateX(4px);
}

.algo-card-icon {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 800;
  letter-spacing: -0.03em;
  flex-shrink: 0;
}

.bfs-icon { background: rgba(59,130,246,0.15); color: #3b82f6; }
.dfs-icon { background: rgba(168,85,247,0.15); color: #a855f7; }
.astar-icon { background: rgba(16,185,129,0.15); color: #10b981; }

.algo-card-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.algo-card-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.algo-card-hint {
  font-size: 12px;
  color: var(--text-secondary);
}

.algo-card-arrow {
  width: 18px;
  height: 18px;
  color: var(--text-secondary);
  transition: transform 0.3s;
  flex-shrink: 0;
}

.algo-card:hover .algo-card-arrow {
  transform: translateX(4px);
  color: var(--accent-blue);
}

.algo-slide {
  flex-direction: row !important;
  align-items: flex-start;
}

.algo-content {
  flex: 1;
  min-width: 0;
  min-height: 0;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}

.algo-desc {
  flex: 1;
}

.algo-desc > p {
  font-size: 14px;
  line-height: 1.7;
  color: var(--text-secondary);
  margin-bottom: 16px;
}

.algo-desc > p strong {
  color: var(--text-primary);
  font-weight: 600;
}

.key-points {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-bottom: 20px;
}

.point {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.5;
}

.point strong {
  color: var(--text-primary);
  font-weight: 600;
}

.point-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
  color: var(--accent-green, #10b981);
  margin-top: 1px;
}

.point-icon svg {
  width: 100%;
  height: 100%;
}

.complexity-bar {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 16px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
}

.comp-item {
  display: flex;
  align-items: center;
  gap: 12px;
}

.comp-label {
  font-size: 12px;
  color: var(--text-secondary);
  width: 42px;
  flex-shrink: 0;
}

.comp-track {
  flex: 1;
  height: 6px;
  background: rgba(63,63,70,0.2);
  border-radius: 3px;
  overflow: hidden;
}

.comp-fill {
  height: 100%;
  border-radius: 3px;
  background: var(--accent-blue);
  transition: width 1s cubic-bezier(0.16, 1, 0.3, 1);
}

.bfs-fill { background: #f59e0b; }
.dfs-fill { background: #f59e0b; }

.formula-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 14px;
  padding: 20px;
  margin-bottom: 16px;
}

.formula {
  font-size: 24px;
  font-weight: 700;
  text-align: center;
  color: var(--text-primary);
  margin-bottom: 16px;
  font-family: 'Georgia', 'Times New Roman', serif;
  letter-spacing: 0.02em;
}

.formula-items {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.formula-item {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.5;
}

.formula-item strong {
  color: var(--text-primary);
}

.f-var {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 28px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 700;
  font-family: 'Georgia', serif;
  flex-shrink: 0;
}

.g-var { background: rgba(59,130,246,0.15); color: #3b82f6; }
.h-var { background: rgba(244,63,94,0.15); color: #f43f5e; }
.f-var-color { background: rgba(16,185,129,0.15); color: #10b981; }

.algo-visual {
  flex: 0 0 300px;
  display: flex;
  align-items: flex-start;
}

.anim-card {
  width: 100%;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 14px;
  overflow: hidden;
}

.anim-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-color);
}

.anim-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

.anim-btn {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 5px 14px;
  background: var(--accent-blue);
  border: none;
  border-radius: 8px;
  color: white;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.anim-btn:hover { filter: brightness(1.1); }
.anim-btn:active { transform: scale(0.96); }
.anim-btn svg { width: 12px; height: 12px; }

.anim-body {
  padding: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.bfs-svg, .dfs-svg, .astar-svg {
  width: 100%;
  max-width: 260px;
}

.bfs-wave {
  opacity: 0;
  animation: bfsExpand 0.45s cubic-bezier(0.16, 1, 0.3, 1) forwards;
}

@keyframes bfsExpand {
  0% { opacity: 0; transform: scale(0.5); }
  40% { opacity: 0.8; transform: scale(1.12); }
  100% { opacity: 1; transform: scale(1); }
}

.dfs-dive {
  opacity: 0;
  animation: dfsDive 0.35s cubic-bezier(0.16, 1, 0.3, 1) forwards;
}

@keyframes dfsDive {
  0% { opacity: 0; transform: scale(0.6) translateY(-8px); }
  50% { opacity: 1; transform: scale(1.08); }
  100% { opacity: 1; transform: scale(1) translateY(0); }
}

.dfs-path-line {
  animation: drawPath 4s ease-out forwards;
  animation-delay: 0.1s;
  opacity: 0.7;
}

.astar-guided {
  opacity: 0;
  animation: astarGuide 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
}

@keyframes astarGuide {
  0% { opacity: 0; transform: scale(0.5); filter: brightness(1.5); }
  50% { opacity: 1; transform: scale(1.1); filter: brightness(1.2); }
  100% { opacity: 1; transform: scale(1); filter: brightness(1); }
}

.astar-path-line {
  animation: drawPath 3s ease-out forwards;
  animation-delay: 1.5s;
  opacity: 0.8;
}

.heuristic-arrow {
  opacity: 0;
  animation: arrowFade 1s ease-out forwards;
  animation-delay: 0.3s;
}

@keyframes arrowFade {
  0% { opacity: 0; }
  100% { opacity: 1; }
}

.start-pulse, .end-pulse {
  animation: cellPulse 1.5s ease-in-out infinite;
}

.end-pulse { animation-delay: 0.75s; }

@keyframes cellPulse {
  0%, 100% { opacity: 0.5; }
  50% { opacity: 1; }
}

.anim-caption {
  padding: 10px 16px;
  font-size: 12px;
  color: var(--text-secondary);
  text-align: center;
  border-top: 1px solid var(--border-color);
  line-height: 1.5;
}

.slide-controls {
  position: fixed;
  bottom: 28px;
  right: 28px;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 14px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.15);
  z-index: 100;
  backdrop-filter: blur(12px);
}

.ctrl-btn {
  width: 40px;
  height: 40px;
  border-radius: 12px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s;
  color: var(--text-primary);
}

.ctrl-btn:hover:not(:disabled) {
  border-color: var(--accent-blue);
  color: var(--accent-blue);
}

.ctrl-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.ctrl-btn svg {
  width: 18px;
  height: 18px;
}

.slide-counter {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
  min-width: 50px;
  text-align: center;
}

@media (max-width: 768px) {
  .algo-view {
    height: calc(100vh - 32px);
    gap: 12px;
  }
  .overview-slide,
  .algo-slide {
    flex-direction: column !important;
    padding: 16px;
    gap: 16px;
  }
  .overview-visual {
    flex: none;
    width: 100%;
    order: -1;
  }
  .algo-visual {
    flex: none;
    width: 100%;
  }
  .slide-nav {
    display: none;
  }
  .slide-content-box {
    padding: 16px;
  }
  .slide-controls {
    bottom: 16px;
    right: 16px;
    gap: 8px;
    padding: 6px 10px;
    border-radius: 12px;
  }
  .ctrl-btn {
    width: 34px;
    height: 34px;
  }
}
</style>
