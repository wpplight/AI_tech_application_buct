# WayFind 架构设计

## 系统概述

WayFind 是一个路径查找可视化工具，采用模块化设计，支持地图编辑和算法执行展示。

## 核心模块

### 1. Map Editor (地图编辑器)

**功能：**
- 创建新地图
- 编辑现有地图
- 设置起点 (Start Point)
- 设置终点 (End Point)
- 绘制墙壁 (Wall Drawing)
- 清除墙壁
- 保存地图

**数据结构：**
```typescript
interface Map {
  id: string
  name: string
  width: number
  height: number
  startPoint: Point
  endPoint: Point
  walls: Point[]
  createdAt: Date
  updatedAt: Date
}
```

**组件：**
- `MapEditorView.vue` - 主编辑器页面
- `EditorGrid.vue` - 可编辑的网格
- `EditorToolbar.vue` - 工具栏（画笔、橡皮擦、设置起点/终点）
- `MapSettings.vue` - 地图设置（名称、尺寸）

### 2. Display (可视化展示)

**功能：**
- 选择算法 (BFS / DFS / A*)
- 选择地图
- 按步执行 (Step by Step)
- 直接显示结果 (Show Final)
- 控制播放 (播放/暂停/停止/重置)
- 调整速度

**数据结构：**
```typescript
interface Display {
  id: string
  name: string
  algorithm: AlgorithmType
  mapId: string
  executionMode: 'step' | 'instant'
  speed: number
  createdAt: Date
}

interface ExecutionState {
  step: number
  totalSteps: number
  currentCells: Cell[]
  visitedCells: Cell[]
  path: Cell[]
  status: 'idle' | 'running' | 'paused' | 'completed'
  found: boolean
}
```

**组件：**
- `DisplayView.vue` - 主展示页面
- `DisplayGrid.vue` - 可视化网格
- `DisplayControls.vue` - 控制面板
- `StepControl.vue` - 步进控制
- `AlgorithmSelector.vue` - 算法选择

### 3. Dashboard (仪表板)

**功能：**
- 查看所有地图
- 查看所有展示
- 快速创建新地图
- 快速创建新展示
- 删除/编辑地图和展示

**组件：**
- `DashboardView.vue` - 主仪表板
- `MapList.vue` - 地图列表
- `DisplayList.vue` - 展示列表
- `QuickCreateCard.vue` - 快速创建卡片

## 路由设计

```
/                     -> Dashboard
/maps                -> 地图列表
/maps/new            -> 创建新地图
/maps/:id/edit       -> 编辑地图
/displays            -> 展示列表
/displays/new        -> 创建新展示
/displays/:id        -> 查看展示
```

## Pinia Stores

### 1. mapStore
- 管理所有地图
- CRUD 操作
- 当前编辑的地图

### 2. displayStore
- 管理所有展示
- CRUD 操作
- 当前展示的配置

### 3. executionStore
- 管理算法执行状态
- 步进逻辑
- 动画控制

### 4. editorStore
- 当前编辑状态
- 工具选择
- 绘制模式

## 页面流程

```
Dashboard
├── [创建地图] -> Map Editor -> 保存 -> Dashboard
├── [编辑地图] -> Map Editor -> 保存 -> Dashboard
├── [创建展示] -> Display Setup -> Display View -> Dashboard
└── [查看展示] -> Display View -> Dashboard
```

## UI/UX 设计

### 风格
- 深色主题
- 玻璃态效果 (Glassmorphism)
- 现代渐变
- 流畅动画

### 配色
- 主色: Emerald (#10b981)
- 强调色: Cyan (#22d3ee)
- 背景: #0f1419
- 卡片: rgba(255, 255, 255, 0.03)
- 边框: rgba(255, 255, 255, 0.08)

### 字体
- 主字体: Inter
- 代码字体: JetBrains Mono

## 技术栈

- Vue 3 (Composition API)
- TypeScript
- Pinia (状态管理)
- Vue Router
- 原生 CSS
- Vite (构建工具)

## 组件层次

```
App.vue
├── AppLayout.vue
│   ├── AppHeader.vue
│   └── RouterView
│       ├── DashboardView.vue
│       ├── MapEditorView.vue
│       └── DisplayView.vue
```

## 待实现功能

1. ✅ 地图编辑器基础功能
2. ✅ Display 展示功能
3. ✅ 按步执行逻辑
4. ⏳ 地图持久化 (localStorage)
5. ⏳ 展示持久化 (localStorage)
6. ⏳ 预设地图系统
7. ⏳ 地图导入/导出
8. ⏳ 性能优化
