# WayFind Wails 集成指南

## 架构概览

WayFind 使用 **Wails** 框架将 **Go 后端**与 **Vue 3 前端**紧密集成，提供高性能的路径搜索算法执行。

```
┌─────────────────────────────────────────┐
│         Vue 3 Frontend                  │
│  ┌─────────────────────────────────┐   │
│  │  Composables / Stores           │   │
│  │  - useWailsSearch               │   │
│  │  - useExecutionStore            │   │
│  └─────────────────────────────────┘   │
│  ┌─────────────────────────────────┐   │
│  │  Services                       │   │
│  │  - wails.ts (Wails Bindings)    │   │
│  └─────────────────────────────────┘   │
└──────────────┬──────────────────────────┘
               │ Wails IPC (JSON-RPC)
┌──────────────▼──────────────────────────┐
│         Go Backend                     │
│  ┌─────────────────────────────────┐   │
│  │  cmd/wails/                     │   │
│  │  - main.go (入口)                │   │
│  │  - app.go (Wails 绑定)           │   │
│  └─────────────────────────────────┘   │
│  ┌─────────────────────────────────┐   │
│  │  algorithms/                    │   │
│  │  - bfs.go                       │   │
│  │  - dfs.go                       │   │
│  │  - astar.go                     │   │
│  └─────────────────────────────────┘   │
│  ┌─────────────────────────────────┐   │
│  │  map/                           │   │
│  │  - map.go (地图数据结构)         │   │
│  └─────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

## 文件结构

```
way_find/
├── backend/
│   ├── cmd/
│   │   └── wails/
│   │       ├── main.go          # Wails 入口点
│   │       └── app.go           # Wails 绑定方法
│   ├── algorithms/
│   │   ├── bfs.go               # BFS 算法实现
│   │   ├── dfs.go               # DFS 算法实现
│   │   ├── astar.go             # A* 算法实现
│   │   └── step.go              # 步骤接口定义
│   └── map/
│       └── map.go               # 地图数据结构
│
└── frontend/
    └── src/
        ├── services/
        │   └── wails.ts         # Wails TypeScript 绑定
        └── composables/
            └── useWailsSearch.ts # Wails 封装 Hook
```

## Go 后端 API

### App 结构体方法

所有方法都通过 Wails 自动暴露给前端。

#### 地图操作

##### `CreateMap(width, height int) -> *MapData`
创建一个新的空白地图。

```go
mapData, err := app.CreateMap(20, 15)
```

**参数：**
- `width`: 地图宽度 (5-50)
- `height`: 地图高度 (5-50)

**返回值：**
- `MapData`: 包含地图网格、起点、终点信息

##### `LoadMap(mapData *MapData) -> error`
加载已有地图。

```go
err := app.LoadMap(&MapData{
    Width:  20,
    Height: 15,
    Grid:   grid,
    StartPoint: Point{X: 1, Y: 1},
    EndPoint:   Point{X: 18, Y: 13},
})
```

##### `SetCell(x, y, cellType int) -> error`
设置单个单元格类型。

```go
// cellType: 0=空, 1=墙, 2=起点, 3=终点
err := app.SetCell(5, 5, 1) // 设置为墙
```

##### `GetMap() -> *MapData`
获取当前地图数据。

```go
mapData, err := app.GetMap()
```

#### 搜索操作

##### `InitializeSearch(algorithm string) -> error`
初始化搜索算法。

```go
// algorithm: "bfs", "dfs", 或 "astar"
err := app.InitializeSearch("bfs")
```

##### `SearchStep() -> *StepData`
执行单个搜索步骤。

```go
step, err := app.SearchStep()
```

**StepData 结构：**
```go
type StepData struct {
    State      int         // 0=就绪, 1=运行, 2=找到, 3=未找到
    Current    Point       // 当前探索点
    Neighbors  []Point     // 邻居节点
    Added      []Point     // 新增节点
    Pruned     []Point     // 剪枝节点
    Path       []Point     // 路径
    Visited    []Point     // 已访问节点
    Distance   int         // 距离
    Expanded   int         // 扩展节点数
    StepsTaken int         // 已执行步数
}
```

##### `IsSearchDone() -> bool`
检查搜索是否完成。

```go
if app.IsSearchDone() {
    // 搜索完成
}
```

##### `GetSearchResult() -> *SearchResultData`
获取最终搜索结果。

```go
result, err := app.GetSearchResult()
```

**SearchResultData 结构：**
```go
type SearchResultData struct {
    Found     bool     // 是否找到路径
    Distance  int      // 路径长度
    Path      []Point  // 最终路径
    Algorithm string   // 算法名称
}
```

##### `GetCurrentPath() -> []Point`
获取当前路径（用于实时显示）。

```go
path, err := app.GetCurrentPath()
```

## 前端使用

### 方式一：使用 Composable（推荐）

```typescript
import { useWailsSearch } from '@/composables/useWailsSearch'

export default {
  setup() {
    const {
      map,
      grid,
      algorithm,
      execution,
      createMap,
      loadMap,
      initializeSearch,
      step,
      runToEnd,
      reset
    } = useWailsSearch()

    // 创建地图
    async function handleCreateMap() {
      await createMap(20, 15)
    }

    // 初始化并运行
    async function handleSearch() {
      await initializeSearch('bfs')
      await runToEnd()
    }

    // 单步执行
    async function handleStep() {
      if (!execution.isDone) {
        await step()
      }
    }

    return {
      grid,
      execution,
      handleCreateMap,
      handleSearch,
      handleStep,
      reset
    }
  }
}
```

### 方式二：直接调用 Wails 服务

```typescript
import { wailsService } from '@/services/wails'

async function search() {
  // 1. 创建地图
  const mapData = await wailsService.createMap(20, 15)
  
  // 2. 设置墙壁
  await wailsService.setCell(5, 5, 1)
  await wailsService.setCell(6, 5, 1)
  
  // 3. 初始化搜索
  await wailsService.initializeSearch('bfs')
  
  // 4. 单步执行
  while (!await wailsService.isSearchDone()) {
    const step = await wailsService.searchStep()
    console.log('Current step:', step)
  }
  
  // 5. 获取结果
  const result = await wailsService.getSearchResult()
  console.log('Path found:', result.path)
}
```

## 数据模型映射

### Go -> TypeScript

| Go 类型 | TypeScript 类型 | 说明 |
|---------|----------------|------|
| `Point{X, Y}` | `{x: number, y: number}` | 坐标点 |
| `MapData` | `MapData` | 地图数据 |
| `StepData` | `StepData` | 步骤数据 |
| `SearchResultData` | `SearchResult` | 搜索结果 |
| `CellType` (0-3) | `number` (0-3) | 单元格类型 |

### 单元格类型映射

| Go 常量 | 值 | TypeScript |
|---------|---|------------|
| `CELL_ROAD` | 0 | 0 - 空道路 |
| `CELL_WALL` | 1 | 1 - 墙壁 |
| `CELL_START` | 2 | 2 - 起点 |
| `CELL_END` | 3 | 3 - 终点 |

## 示例：完整的搜索流程

### Go 后端实现

```go
// 初始化搜索
await app.InitializeSearch("bfs")

// 执行到结束
for !await app.IsSearchDone() {
    await app.SearchStep()
}

// 获取结果
result := await app.GetSearchResult()
```

### Vue 前端实现

```vue
<template>
  <div class="search-view">
    <div class="grid">
      <div 
        v-for="(row, y) in grid.cells" 
        :key="y"
        class="row"
      >
        <div
          v-for="(cell, x) in row"
          :key="x"
          class="cell"
          :class="cell.type"
        />
      </div>
    </div>
    
    <div class="controls">
      <button @click="startSearch">开始搜索</button>
      <button @click="stepSearch">单步</button>
      <button @click="resetSearch">重置</button>
    </div>
    
    <div class="stats">
      <div>访问节点: {{ execution.visitedCells.length }}</div>
      <div>路径长度: {{ execution.path.length }}</div>
      <div>状态: {{ execution.status }}</div>
    </div>
  </div>
</template>

<script>
import { useWailsSearch } from '@/composables/useWailsSearch'

export default {
  setup() {
    const { grid, execution, createMap, initializeSearch, step, runToEnd, reset } = useWailsSearch()
    
    onMounted(async () => {
      await createMap(20, 15)
    })
    
    async function startSearch() {
      await initializeSearch('bfs')
      await runToEnd()
    }
    
    async function stepSearch() {
      await step()
    }
    
    return {
      grid,
      execution,
      startSearch,
      stepSearch,
      reset
    }
  }
}
</script>
```

## 性能优化

### 1. 批量操作
```go
// ❌ 慢 - 每个单元格单独调用
for _, wall := range walls {
    await app.SetCell(wall.X, wall.Y, 1)
}

// ✅ 快 - 一次加载整个地图
await app.LoadMap(&MapData{
    Grid: grid,
    ...
})
```

### 2. 异步执行
```typescript
// ✅ 使用 async/await
async function search() {
  await initializeSearch('bfs')
  await runToEnd() // 在后台执行
}
```

### 3. 减少 DOM 更新
```typescript
// ✅ 批量更新状态
watch(steps, (newSteps) => {
  updateGridState() // 一次更新所有变化
})
```

## 错误处理

```typescript
try {
  await wailsService.createMap(20, 15)
} catch (error) {
  console.error('地图创建失败:', error)
  // 显示错误消息给用户
}
```

### 常见错误

| 错误 | 原因 | 解决方案 |
|------|------|----------|
| `no map loaded` | 未创建或加载地图 | 先调用 `CreateMap` 或 `LoadMap` |
| `search not initialized` | 未初始化搜索 | 先调用 `InitializeSearch` |
| `search already completed` | 搜索已完成 | 调用 `Reset` 重置 |
| `invalid map dimensions` | 地图尺寸无效 | 使用 5-50 范围内的尺寸 |

## 调试技巧

### 1. 检查 Wails 连接
```typescript
console.log(window.go_main_App_CreateMap) // 确认方法存在
```

### 2. 打印数据流
```typescript
const result = await wailsService.searchStep()
console.log('Step result:', JSON.stringify(result, null, 2))
```

### 3. 监控性能
```typescript
const start = performance.now()
await runToEnd()
console.log(`执行时间: ${performance.now() - start}ms`)
```

## 下一步

1. **测试当前实现** - 运行 Wails 应用测试前后端通信
2. **更新 UI** - 将 `executionStore` 替换为 `useWailsSearch`
3. **添加更多功能** - 实现更多的算法或地图编辑功能
4. **性能优化** - 优化大批量地图的加载性能

## 常见问题

**Q: Wails 服务未定义？**
A: 确保在 `main.ts` 中导入了生成的 Wails 绑定文件

**Q: 如何查看 Wails 日志？**
A: 在开发者工具的 Console 中查看，或在终端运行 Wails 时查看

**Q: 如何处理大地图？**
A: 使用 `LoadMap` 一次性加载，而不是逐个设置单元格

**Q: 如何实现实时可视化？**
A: 使用 `SearchStep` 逐步执行，并在每步后更新 UI
