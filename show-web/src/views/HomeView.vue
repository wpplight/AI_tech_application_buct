<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()

const projects = [
  {
    id: 'wayfind',
    title: '寻路算法',
    description: '迷宫寻路可视化系统，支持 DFS、BFS、A* 三种经典算法',
    tags: ['DFS', 'BFS', 'A*', '最短路径'],
    icon: 'map',
    color: '#3b82f6',
    path: '/wayfind'
  },
  {
    id: 'professor',
    title: '专家系统',
    description: '动物识别推理系统，三种前向推理算法对比',
    tags: ['正向推理', '反向推理', 'Rete 网络'],
    icon: 'brain',
    color: '#10b981',
    path: '/professor'
  },
  {
    id: 'nn',
    title: '神经网络',
    description: 'Rust 实现的 MLP 神经网络，从零训练模型',
    tags: ['MLP', '反向传播', '训练可视化'],
    icon: 'cpu',
    color: '#f59e0b',
    path: '/mlearn?tab=nn'
  },
  {
    id: 'ga',
    title: '遗传算法',
    description: '多目标优化算法，实时展示收敛过程',
    tags: ['SBX', '多项式变异', '收敛曲线'],
    icon: 'dna',
    color: '#f43f5e',
    path: '/mlearn?tab=ga'
  }
]

function navigateTo(path: string) {
  router.push(path)
}
</script>

<template>
  <div class="home">
    <section class="hero">
      <div class="hero-content">
        <h1 class="hero-title">
          <span class="title-line">AI</span>
          <span class="title-line accent"> Showcase</span>
        </h1>
        <p class="hero-subtitle">
          计算机人工智能应用作业展示平台
        </p>
      </div>
    </section>

    <section class="projects">
      <div class="projects-header">
        <h2 class="section-title">项目展示</h2>
        <p class="section-subtitle">点击卡片进入详细页面</p>
      </div>

      <div class="projects-grid">
        <div
          v-for="project in projects"
          :key="project.id"
          class="project-card"
          @click="navigateTo(project.path)"
        >
          <div class="card-header" :style="{ borderColor: project.color }">
            <div class="card-icon" :style="{ backgroundColor: project.color + '20' }">
              <svg v-if="project.icon === 'map'" :style="{ color: project.color }" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M3 7l6-3 6 3 6-3v13l-6 3-6-3-6 3V7z"/>
                <path d="M9 4v13"/>
                <path d="M15 7v13"/>
              </svg>
              <svg v-if="project.icon === 'brain'" :style="{ color: project.color }" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 4.5a2.5 2.5 0 0 0-4.96-.46 2.5 2.5 0 0 0-1.98 3 2.5 2.5 0 0 0 1.32 4.24 3 3 0 0 0 .34 5.58 2.5 2.5 0 0 0 5.51.63A2.5 2.5 0 0 0 12 18.5"/>
                <path d="M12 4.5a2.5 2.5 0 0 1 4.96-.46 2.5 2.5 0 0 1 1.98 3 2.5 2.5 0 0 1-1.32 4.24 3 3 0 0 1-.34 5.58 2.5 2.5 0 0 1-5.51.63"/>
                <path d="M12 4.5V18"/>
              </svg>
              <svg v-if="project.icon === 'cpu'" :style="{ color: project.color }" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
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
              <svg v-if="project.icon === 'dna'" :style="{ color: project.color }" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M2 15c6.667-6 13.333 0 20-6"/>
                <path d="M9 22c1.798-1.998 2.518-3.995 2.807-5.993"/>
                <path d="M15 2c-1.798 1.998-2.518 3.995-2.807 5.993"/>
                <path d="M17 6l-2.5-2.5"/>
                <path d="M14 8l-1.5-1.5"/>
                <path d="M7 18l2.5 2.5"/>
                <path d="M10 16l1.5 1.5"/>
                <path d="M3 21c0-2.5 4-3 6-3"/>
                <path d="M21 3c-2.5 0-5 2.5-5 5"/>
              </svg>
            </div>
            <h3 class="card-title">{{ project.title }}</h3>
          </div>

          <div class="card-body">
            <p class="card-description">{{ project.description }}</p>
            <div class="card-tags">
              <span
                v-for="tag in project.tags"
                :key="tag"
                class="tag"
              >
                {{ tag }}
              </span>
            </div>
          </div>

          <div class="card-footer">
            <span class="card-action">
              探索
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="5" y1="12" x2="19" y2="12"/>
                <polyline points="12 5 19 12 12 19"/>
              </svg>
            </span>
          </div>
        </div>
      </div>
    </section>

    <footer class="footer">
      <p class="footer-text">
        计算机人工智能应用作业展示
      </p>
    </footer>
  </div>
</template>

<style scoped>
.home {
  width: 100%;
  padding: 0 16px;
  max-width: 1400px;
  margin: 0 auto;
}

@media (min-width: 640px) {
  .home { padding: 0 24px; }
}

@media (min-width: 768px) {
  .home { padding: 0 32px; }
}

@media (min-width: 1024px) {
  .home { padding: 0 48px; }
}

@media (min-width: 1280px) {
  .home { padding: 0 64px; }
}

.hero { padding: 60px 0 40px; }

@media (min-width: 768px) {
  .hero { padding: 80px 0 60px; }
}

.hero-content { max-width: 600px; }

.hero-title {
  font-size: 36px;
  font-weight: 800;
  line-height: 1.1;
  letter-spacing: -0.04em;
  margin-bottom: 12px;
  color: var(--text-primary);
}

@media (min-width: 768px) {
  .hero-title { font-size: 48px; }
}

@media (min-width: 1024px) {
  .hero-title { font-size: 64px; }
}

.title-line { display: block; }

.title-line.accent {
  color: var(--accent-blue);
}

.hero-subtitle {
  font-size: 16px;
  color: var(--text-secondary);
}

@media (min-width: 768px) {
  .hero-subtitle { font-size: 20px; }
}

.projects { padding: 32px 0; }

@media (min-width: 768px) {
  .projects { padding: 40px 0; }
}

.projects-header { margin-bottom: 24px; }

@media (min-width: 768px) {
  .projects-header { margin-bottom: 32px; }
}

.section-title {
  font-size: 24px;
  font-weight: 700;
  letter-spacing: -0.02em;
  margin-bottom: 8px;
  color: var(--text-primary);
}

@media (min-width: 768px) {
  .section-title { font-size: 28px; }
}

@media (min-width: 1024px) {
  .section-title { font-size: 32px; }
}

.section-subtitle {
  color: var(--text-secondary);
  font-size: 14px;
}

@media (min-width: 768px) {
  .section-subtitle { font-size: 16px; }
}

.projects-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 16px;
}

@media (min-width: 640px) {
  .projects-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 20px;
  }
}

@media (min-width: 1024px) {
  .projects-grid { gap: 24px; }
}

.project-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  display: flex;
  flex-direction: column;
}

@media (min-width: 768px) {
  .project-card { border-radius: 20px; }
}

.project-card:hover {
  border-color: var(--text-muted);
  transform: translateY(-4px);
  box-shadow: 0 20px 40px -15px rgba(0, 0, 0, 0.3);
}

.card-header {
  padding: 20px;
  border-bottom: 1px solid;
  display: flex;
  align-items: center;
  gap: 12px;
}

@media (min-width: 768px) {
  .card-header { padding: 24px; gap: 16px; }
}

.card-icon {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

@media (min-width: 768px) {
  .card-icon { width: 48px; height: 48px; border-radius: 14px; }
}

.card-icon svg { width: 22px; height: 22px; }

@media (min-width: 768px) {
  .card-icon svg { width: 24px; height: 24px; }
}

.card-title {
  font-size: 18px;
  font-weight: 700;
  letter-spacing: -0.02em;
  color: var(--text-primary);
}

@media (min-width: 768px) {
  .card-title { font-size: 20px; }
}

@media (min-width: 1024px) {
  .card-title { font-size: 22px; }
}

.card-body { padding: 16px 20px; flex: 1; }

@media (min-width: 768px) {
  .card-body { padding: 20px 24px; }
}

.card-description {
  color: var(--text-secondary);
  font-size: 13px;
  line-height: 1.5;
  margin-bottom: 12px;
}

@media (min-width: 768px) {
  .card-description { font-size: 14px; line-height: 1.6; margin-bottom: 14px; }
}

.card-tags { display: flex; flex-wrap: wrap; gap: 6px; }

@media (min-width: 768px) {
  .card-tags { gap: 8px; }
}

.tag {
  padding: 3px 10px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  font-size: 11px;
  color: var(--text-secondary);
}

@media (min-width: 768px) {
  .tag { padding: 4px 12px; font-size: 12px; }
}

.card-footer { padding: 12px 20px 16px; }

@media (min-width: 768px) {
  .card-footer { padding: 14px 24px 20px; }
}

.card-action {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  color: var(--accent-blue);
  font-size: 13px;
  font-weight: 500;
  transition: gap 0.2s;
}

@media (min-width: 768px) {
  .card-action { gap: 8px; font-size: 14px; }
}

.project-card:hover .card-action { gap: 10px; }

.card-action svg { width: 14px; height: 14px; }

@media (min-width: 768px) {
  .card-action svg { width: 16px; height: 16px; }
}

.footer { padding: 40px 0 32px; text-align: center; }

@media (min-width: 768px) {
  .footer { padding: 48px 0 40px; }
}

.footer-text { color: var(--text-muted); font-size: 13px; }

@media (min-width: 768px) {
  .footer-text { font-size: 14px; }
}
</style>
