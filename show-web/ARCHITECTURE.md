# AI 作业展示系统 - 架构设计文档

> 本文档描述了 show-web 前端项目的技术架构和设计规范。

---

## 📋 项目概述

**项目名称**: AI Showcase  
**项目类型**: Vue 3 前端展示应用  
**核心功能**: 展示四个 AI 作业项目（寻路算法、专家系统、神经网络、遗传算法）  
**技术栈**: Vue 3 + Vite + TypeScript + Pinia + Vue Router 5

---

## 🎨 设计规范

### 设计风格
- **美学**: 高端、极简、功能性强
- **灵感**: Vercel 核心 + Dribbble 干净风格
- **Motion**: 活跃但不夸张，60fps 流畅动画

### 色彩系统
```css
--background: #09090b      /* Zinc-950, Off-Black */
--surface: #18181b         /* Zinc-900 */
--card: #27272a           /* Zinc-800 */
--border: #3f3f46         /* Zinc-700 */
--text-primary: #fafafa   /* Zinc-50 */
--text-secondary: #a1a1aa  /* Zinc-400 */
--accent-blue: #3b82f6    /* Blue-500 */
--accent-emerald: #10b981 /* Emerald-500 */
--accent-amber: #f59e0b   /* Amber-500 */
--accent-rose: #f43f5e    /* Rose-500 */
```

### 字体系统
- **Display/Headlines**: Geist, Satoshi, Cabinet Grotesk
- **Body**: Geist, Inter alternative
- **Monospace**: JetBrains Mono (用于代码和数字)

### 间距系统
- 基础单位: 4px
- 卡片圆角: 2.5rem (40px)
- 内边距: 2rem-2.5rem (32-40px)
- 卡片阴影: `0 20px 40px -15px rgba(0,0,0,0.3)`

---

## 🏗️ 项目架构

```
show-web/
├── src/
│   ├── api/                    # API 服务层
│   │   ├── index.ts           # API 基础配置
│   │   ├── professor.ts       # 专家系统 API
│   │   ├── wayfind.ts         # 寻路算法 API
│   │   └── mlearn.ts          # 机器学习 API (NN + GA)
│   │
│   ├── components/             # 公共组件
│   │   ├── layout/
│   │   │   ├── AppHeader.vue  # 顶部导航
│   │   │   ├── AppSidebar.vue # 侧边栏
│   │   │   └── AppFooter.vue  # 底部
│   │   ├── common/
│   │   │   ├── LoadingSpinner.vue
│   │   │   ├── ErrorMessage.vue
│   │   │   ├── EmptyState.vue
│   │   │   └── StatCard.vue
│   │   └── icons/
│   │       └── (复用现有 icons)
│   │
│   ├── composables/            # 组合式函数
│   │   ├── useApi.ts          # API 请求封装
│   │   ├── useLoading.ts      # 加载状态
│   │   └── useError.ts        # 错误处理
│   │
│   ├── stores/                 # Pinia 状态管理
│   │   ├── professor.ts       # 专家系统状态
│   │   ├── wayfind.ts         # 寻路算法状态
│   │   ├── mlearn.ts          # 机器学习状态
│   │   └── app.ts             # 全局应用状态
│   │
│   ├── views/                  # 页面视图
│   │   ├── HomeView.vue        # 首页/概览
│   │   ├── ProfessorView.vue   # 专家系统
│   │   ├── WayFindView.vue     # 寻路算法
│   │   ├── MLearnView.vue      # 机器学习 (NN + GA)
│   │   └── NotFoundView.vue    # 404 页面
│   │
│   ├── router/
│   │   └── index.ts           # 路由配置
│   │
│   └── assets/
│       └── main.css            # 全局样式
```

---

## 🔌 API 服务设计

### 后端 API 地址

| 服务 | 端口 | 基础 URL |
|------|------|----------|
| 专家系统 | 8080 | `http://localhost:8080` |
| 寻路算法 | 8083 | `http://localhost:8083` |
| 机器学习 | 8081 | `http://localhost:8081` |

### 专家系统 API (`professor.ts`)

```typescript
// 知识库管理
GET /api/rules?algo=<algorithm>     // 获取所有规则
POST /api/rules/add                  // 添加规则
POST /api/rules/delete               // 删除规则

// 事实库管理
GET /api/facts?algo=<algorithm>      // 获取当前事实
POST /api/facts/add                  // 添加事实
POST /api/facts/delete               // 删除事实
POST /api/facts/clear                // 清空事实库

// 推理引擎
POST /api/inference/forward           // 正向推理
POST /api/inference/backward         // 反向推理
GET /api/inference/steps             // 获取推理步骤

// Rete 特有
GET /api/network/stats               // 网络统计
GET /api/network/trace               // 执行追踪
```

### 寻路算法 API (`wayfind.ts`)

```typescript
// 地图管理
POST /api/pathfinding/maps           // 创建地图
GET /api/pathfinding/maps/:id       // 获取地图
PUT /api/pathfinding/maps/:id/cells // 更新单元格

// 搜索操作
POST /api/pathfinding/search         // 初始化搜索
POST /api/pathfinding/search/:id/step // 单步执行
POST /api/pathfinding/search/:id/run  // 完整搜索
GET /api/pathfinding/search/:id/history // 搜索历史

// 算法对比
POST /api/pathfinding/compare        // 算法对比
```

### 机器学习 API (`mlearn.ts`)

```typescript
// 神经网络
POST /api/ml/nn/train                // 训练模型
POST /api/ml/nn/predict              // 预测
GET /api/ml/nn/models                // 获取模型列表

// 遗传算法
POST /api/ml/ga/optimize             // 运行优化
GET /api/ml/ga/status/:job_id        // 查询状态

// 可视化
POST /api/ml/viz/plot                // 生成图表
GET /api/ml/viz/image/:chart_id      // 获取图表
```

---

## 📱 页面设计

### 1. 首页 (HomeView)

**设计目标**: 展示四个项目的概览卡片，引导用户进入具体页面

**布局**: 
- Hero Section: 标题 + 简介
- 四个项目卡片: Bento Grid 布局（不对称网格）
- 每个卡片包含: 项目图标、名称、简介、快速入口按钮

**卡片信息**:
1. **寻路算法**: 迷宫寻路可视化（DFS/BFS/A*）
2. **专家系统**: 动物识别推理系统（三种算法）
3. **神经网络**: Rust 实现的 MLP 训练
4. **遗传算法**: 多目标优化可视化

### 2. 专家系统页面 (ProfessorView)

**设计目标**: 交互式推理系统演示

**布局**:
- 左侧: 算法选择 + 知识库/事实库管理
- 中央: 推理控制面板
- 右侧: 推理步骤可视化

**功能**:
- 切换三种算法（Fullscan/Incremental/Rete）
- 添加/删除事实
- 执行正向/反向推理
- 实时显示推理步骤
- 性能对比图表

### 3. 寻路算法页面 (WayFindView)

**设计目标**: 交互式迷宫编辑和算法可视化

**布局**:
- 顶部: 控制面板（算法选择、速度控制）
- 中央: 迷宫网格可视化
- 底部: 搜索结果统计

**功能**:
- 创建/编辑迷宫地图
- 设置起点和终点
- 选择搜索算法（DFS/BFS/A*）
- 单步执行/完整执行
- 实时显示搜索过程
- 算法性能对比

### 4. 机器学习页面 (MLearnView)

**设计目标**: 神经网络训练和遗传算法优化演示

**布局**:
- 顶部 Tab 切换: 神经网络 | 遗传算法
- 主区域: 参数配置 + 结果展示
- 侧边: 实时图表

**神经网络功能**:
- 配置网络结构（层数、神经元数）
- 配置训练参数（学习率、轮次）
- 执行训练并显示收敛曲线
- 模型预测

**遗传算法功能**:
- 选择基准测试函数
- 配置 GA 参数（种群大小、迭代次数）
- 执行优化并显示收敛过程
- 结果可视化

---

## 🎭 组件设计

### 公共组件

| 组件 | 说明 | 状态 |
|------|------|------|
| `AppHeader` | 顶部导航栏 | 必选 |
| `LoadingSpinner` | 加载动画 | 必选 |
| `ErrorMessage` | 错误提示 | 必选 |
| `EmptyState` | 空状态提示 | 必选 |
| `StatCard` | 统计卡片 | 必选 |

### 专家系统组件

| 组件 | 说明 |
|------|------|
| `AlgorithmSelector` | 算法选择器（Fullscan/Incremental/Rete） |
| `KnowledgeBasePanel` | 知识库管理面板 |
| `FactBasePanel` | 事实库管理面板 |
| `InferencePanel` | 推理控制面板 |
| `RuleVisualizer` | 规则可视化 |
| `StepTimeline` | 推理步骤时间线 |

### 寻路算法组件

| 组件 | 说明 |
|------|------|
| `MapEditor` | 迷宫编辑器 |
| `GridVisualizer` | 网格可视化 |
| `AlgorithmControls` | 算法控制面板 |
| `SearchStatistics` | 搜索统计面板 |
| `PathHighlight` | 路径高亮显示 |

### 机器学习组件

| 组件 | 说明 |
|------|------|
| `NNConfigPanel` | 神经网络配置面板 |
| `GAConfigPanel` | 遗传算法配置面板 |
| `ConvergenceChart` | 收敛曲线图表 |
| `ModelMetrics` | 模型评估指标 |
| `OptimizationResult` | 优化结果展示 |

---

## 🛠️ 技术实现

### 状态管理 (Pinia)

每个模块独立的 store：

```typescript
// stores/professor.ts
export const useProfessorStore = defineStore('professor', {
  state: () => ({
    currentAlgorithm: 'fullscan' as 'fullscan' | 'incremental' | 'rete',
    facts: [] as string[],
    rules: [] as Rule[],
    inferenceSteps: [] as InferenceStep[],
    loading: false,
    error: null as string | null
  }),
  actions: {
    async loadFacts() { /* ... */ },
    async addFact(fact: string) { /* ... */ },
    async forwardInference() { /* ... */ },
    // ...
  }
})
```

### API 请求封装

```typescript
// composables/useApi.ts
export function useApi() {
  const request = async <T>(url: string, options?: RequestInit): Promise<T> => {
    const response = await fetch(url, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...options?.headers
      }
    })
    
    if (!response.ok) {
      throw new Error(`API Error: ${response.statusText}`)
    }
    
    return response.json()
  }
  
  return { request }
}
```

---

## 📱 响应式设计

### 断点
- `sm`: 640px
- `md`: 768px
- `lg`: 1024px
- `xl`: 1280px

### 移动端适配
- 所有侧边栏在移动端变为底部标签栏
- Bento Grid 简化为单列布局
- 迷宫网格支持触摸缩放

---

## 🎬 动画规范

### 微交互
- **按钮 Hover**: `scale(1.02)`, `200ms ease-out`
- **按钮 Active**: `scale(0.98)`, `100ms`
- **卡片 Hover**: `translateY(-4px)`, `shadow-lg`

### 加载状态
- 骨架屏动画（Shimmer Effect）
- 禁止使用纯色圆形加载器

### 过渡
- 页面切换: `fade + slide`, `300ms`
- 模态框: `scale + fade`, `200ms`

---

## 📝 开发规范

### 文件命名
- 组件: PascalCase (e.g., `AppHeader.vue`)
- 工具函数: camelCase (e.g., `useApi.ts`)
- 常量: UPPER_SNAKE_CASE

### 代码风格
- 使用 TypeScript 严格模式
- 组件使用 `<script setup lang="ts">`
- 样式使用 Tailwind CSS
- 禁止使用 `!important`

---

## 🚀 部署

### 构建
```bash
npm run build
```

### 预览
```bash
npm run preview
```

---

## 📄 版本历史

| 版本 | 日期 | 说明 |
|------|------|------|
| 1.0 | 2026-05-06 | 初始架构设计 |
