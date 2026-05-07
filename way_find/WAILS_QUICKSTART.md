# WayFind Wails 快速入门

## 🚀 快速开始

### 1. 安装 Wails CLI

```bash
# 使用 Go 安装
go install github.com/wailsapp/wails/v2/cmd/wails@latest

# 或使用脚本
curl https://wails.io/install.sh | bash
```

### 2. 初始化 Wails 项目（如果需要）

```bash
cd backend
wails init
```

### 3. 运行开发模式

```bash
cd backend
wails dev
```

这将：
- 启动 Go 后端服务
- 启动 Vue 前端开发服务器
- 自动打开桌面应用程序窗口
- 监听文件变化并热重载

### 4. 构建生产版本

```bash
cd backend
wails build
```

生成的可执行文件将在 `build/bin/` 目录中。

## 📁 项目文件说明

### Go 后端

```
backend/
├── cmd/wails/
│   ├── main.go      # Wails 应用入口
│   └── app.go       # Wails 绑定方法（暴露给前端）
├── algorithms/      # 路径搜索算法
│   ├── bfs.go       # 广度优先搜索
│   ├── dfs.go       # 深度优先搜索
│   └── astar.go     # A* 算法
├── map/              # 地图数据结构
│   └── map.go
└── wails.json       # Wails 配置
```

### Vue 前端

```
frontend/
├── src/
│   ├── services/
│   │   └── wails.ts           # Wails TypeScript 绑定
│   ├── composables/
│   │   └── useWailsSearch.ts  # Wails 封装 Hook
│   ├── stores/
│   │   └── execution.ts      # 执行状态管理
│   └── views/
│       └── DisplayView.vue    # 算法可视化视图
└── wails.json
```

## 🔧 核心 API 快速参考

### 创建和加载地图

```typescript
import { wailsService } from '@/services/wails'

// 创建新地图
const map = await wailsService.createMap(20, 15)

// 加载已有地图
await wailsService.loadMap(mapData)

// 设置单元格 (0=空, 1=墙, 2=起点, 3=终点)
await wailsService.setCell(5, 5, 1)
```

### 执行搜索

```typescript
// 初始化搜索算法 ('bfs', 'dfs', 'astar')
await wailsService.initializeSearch('bfs')

// 单步执行
const step = await wailsService.searchStep()

// 检查是否完成
const done = await wailsService.isSearchDone()

// 获取最终结果
const result = await wailsService.getSearchResult()
```

## 💡 使用示例

### 基础用法

```vue
<template>
  <div>
    <button @click="runBFS">运行 BFS</button>
    <div class="grid">
      <div v-for="(row, y) in grid" :key="y" class="row">
        <div v-for="(cell, x) in row" :key="x" :class="cell.type" />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { wailsService } from '@/services/wails'

const grid = ref([])

async function runBFS() {
  // 1. 创建地图
  await wailsService.createMap(20, 15)
  
  // 2. 初始化 BFS
  await wailsService.initializeSearch('bfs')
  
  // 3. 执行到结束
  while (!await wailsService.isSearchDone()) {
    await wailsService.searchStep()
  }
  
  // 4. 获取结果
  const result = await wailsService.getSearchResult()
  console.log('路径:', result.path)
}
</script>
```

### 逐步可视化

```vue
<template>
  <div>
    <button @click="initialize">初始化</button>
    <button @click="step">下一步</button>
    <button @click="runAll">全部执行</button>
    <div>{{ stepData?.stepsTaken }} / {{ result?.distance }}</div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { wailsService } from '@/services/wails'

const stepData = ref(null)
const result = ref(null)

async function initialize() {
  await wailsService.createMap(20, 15)
  await wailsService.initializeSearch('bfs')
}

async function step() {
  stepData.value = await wailsService.searchStep()
}

async function runAll() {
  while (!await wailsService.isSearchDone()) {
    await wailsService.searchStep()
  }
  result.value = await wailsService.getSearchResult()
}
</script>
```

## 🎯 推荐的工作流程

### 1. 开发模式
```bash
cd backend
wails dev
```

**优势：**
- 🔥 热重载 - 修改前端代码自动刷新
- 🐛 调试友好 - 浏览器开发者工具可用
- ⚡ 快速迭代 - 实时查看更改

### 2. 生产构建
```bash
cd backend
wails build
```

**优势：**
- 📦 独立可执行文件
- 🚀 性能优化
- 🎨 资源打包

## 📊 数据流

```
用户点击"开始"
    ↓
Vue 组件调用 wailsService.initializeSearch('bfs')
    ↓
Wails IPC (JSON-RPC over WebSocket)
    ↓
Go 后端 App.InitializeSearch()
    ↓
调用 BFSAlgorithm.Initialize()
    ↓
返回 StepData 给前端
    ↓
Vue 更新 grid 状态
    ↓
界面重新渲染
```

## 🛠️ 调试技巧

### 1. 查看 Wails 方法

打开浏览器控制台：
```javascript
console.log(window.go_main_App_CreateMap)
// 输出: ƒ CreateMap() { [native code] }
```

### 2. 打印数据

```typescript
const step = await wailsService.searchStep()
console.log(JSON.stringify(step, null, 2))
```

### 3. 性能监控

```typescript
const start = Date.now()
await wailsService.initializeSearch('bfs')
console.log(`初始化耗时: ${Date.now() - start}ms`)
```

## ❓ 常见问题

**Q: 如何更新现有地图？**
```typescript
// 重新加载地图会重置所有状态
await wailsService.loadMap(newMapData)
```

**Q: 如何实现实时动画？**
```typescript
async function animate() {
  await wailsService.initializeSearch('bfs')
  while (!await wailsService.isSearchDone()) {
    await wailsService.searchStep()
    await new Promise(r => setTimeout(r, 100)) // 100ms 延迟
    // 更新 UI
  }
}
```

**Q: 如何添加新算法？**
1. 在 `backend/algorithms/` 中实现新算法
2. 实现 `Searcher` 接口
3. 在 `backend/cmd/wails/app.go` 中添加分支

## 📚 深入学习

- [完整集成文档](./WAILS_INTEGRATION.md)
- [Wails 官方文档](https://wails.io/docs/gettingstarted/installation)
- [Vue 3 文档](https://vuejs.org/guide/introduction.html)
- [TypeScript 手册](https://www.typescriptlang.org/docs/)

## 🎉 下一步

1. **运行演示** - 执行 `wails dev` 查看可视化效果
2. **扩展功能** - 添加新算法或地图编辑功能
3. **性能优化** - 优化大批量数据的处理
4. **构建发布** - 使用 `wails build` 创建可分发版本

祝编码愉快！ 🚀
