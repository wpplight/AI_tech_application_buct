# Wails 对接代码总结

## ✅ 完成的工作

### 1. Go 后端结构

#### 1.1 Wails 入口点
- **文件**: [backend/cmd/wails/main.go](file:///home/wpp/homework/way_find/backend/cmd/wails/main.go)
- **功能**: 
  - Wails 应用初始化
  - 嵌入前端资源（`../../frontend/dist`）
  - 配置应用选项（窗口大小、标题等）

#### 1.2 Wails 绑定方法
- **文件**: [backend/cmd/wails/app.go](file:///home/wpp/homework/way_find/backend/cmd/wails/app.go)
- **核心功能**:
  - `CreateMap()` - 创建新地图
  - `LoadMap()` - 加载地图
  - `SetCell()` - 设置单元格
  - `InitializeSearch()` - 初始化搜索算法
  - `SearchStep()` - 单步执行搜索
  - `IsSearchDone()` - 检查搜索状态
  - `GetSearchResult()` - 获取最终结果
  - `GetCurrentPath()` - 获取当前路径
  - `GetMap()` - 获取地图数据

### 2. 前端结构

#### 2.1 Wails 服务层
- **文件**: [frontend/src/services/wails.ts](file:///home/wpp/homework/way_find/frontend/src/services/wails.ts)
- **功能**:
  - TypeScript 类型定义
  - Wails 全局函数声明
  - 响应式服务对象封装

#### 2.2 Composable Hook
- **文件**: [frontend/src/composables/useWailsSearch.ts](file:///home/wpp/homework/way_find/frontend/src/composables/useWailsSearch.ts)
- **功能**:
  - 封装 Wails 调用逻辑
  - 状态管理（地图、搜索、结果）
  - 网格状态更新
  - 搜索流程控制

### 3. 配置文件

#### 3.1 Wails 配置
- **文件**: [backend/wails.json](file:///home/wpp/homework/way_find/backend/wails.json)
- **包含**:
  - 应用名称和版本
  - 前后端构建命令
  - 窗口配置
  - 平台特定设置

### 4. 文档

#### 4.1 完整集成文档
- **文件**: [WAILS_INTEGRATION.md](file:///home/wpp/homework/way_find/WAILS_INTEGRATION.md)
- **内容**:
  - 架构概览
  - API 参考
  - 数据模型映射
  - 使用示例
  - 性能优化建议

#### 4.2 快速入门指南
- **文件**: [WAILS_QUICKSTART.md](file:///home/wpp/homework/way_find/WAILS_QUICKSTART.md)
- **内容**:
  - 安装和运行
  - 快速参考
  - 实用示例
  - 常见问题

## 📐 架构设计

```
┌─────────────────────────────────────────┐
│         Vue 3 Frontend                  │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │  Composables                      │  │
│  │  useWailsSearch.ts                │  │
│  │  - 封装 Wails 调用逻辑            │  │
│  │  - 状态管理                        │  │
│  │  - 搜索流程控制                    │  │
│  └───────────────────────────────────┘  │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │  Services                          │  │
│  │  wails.ts                         │  │
│  │  - TypeScript 绑定                 │  │
│  │  - 类型定义                        │  │
│  │  - 响应式封装                      │  │
│  └───────────────────────────────────┘  │
└──────────────┬──────────────────────────┘
               │ Wails IPC
┌──────────────▼──────────────────────────┐
│         Go Backend                      │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │  cmd/wails/                       │  │
│  │  - main.go (入口)                 │  │
│  │  - app.go (绑定方法)              │  │
│  └───────────────────────────────────┘  │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │  algorithms/                       │  │
│  │  - bfs.go (BFS 实现)              │  │
│  │  - dfs.go (DFS 实现)              │  │
│  │  - astar.go (A* 实现)             │  │
│  │  - step.go (接口定义)             │  │
│  └───────────────────────────────────┘  │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │  map/                             │  │
│  │  - map.go (数据结构)              │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

## 🔑 核心设计决策

### 1. 数据模型分离
- **Go 端**: 使用原生结构体（`Point`, `MapData`, `StepData`）
- **前端**: TypeScript 接口（保持一致）
- **通信**: JSON-RPC 自动序列化/反序列化

### 2. 状态管理
- **前端状态**: 使用 Vue 3 `reactive` 和 `ref`
- **后端状态**: 在 `App` 结构体中管理
- **同步策略**: 通过 Wails IPC 调用更新状态

### 3. 错误处理
- **Go 端**: 返回 `error` 类型
- **前端**: 使用 `try/catch` 捕获
- **用户反馈**: 显示友好的错误消息

### 4. 性能优化
- **批量操作**: `LoadMap` 一次性加载
- **按需执行**: `SearchStep` 单步执行
- **异步处理**: 所有调用都是异步的

## 📝 API 一览表

### 地图操作

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `CreateMap` | `width, height` | `MapData` | 创建新地图 |
| `LoadMap` | `MapData` | `error` | 加载地图 |
| `SetCell` | `x, y, cellType` | `error` | 设置单元格 |
| `GetMap` | - | `MapData` | 获取地图 |

### 搜索操作

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `InitializeSearch` | `algorithm` | `error` | 初始化算法 |
| `SearchStep` | - | `StepData` | 单步执行 |
| `IsSearchDone` | - | `bool` | 检查状态 |
| `GetSearchResult` | - | `SearchResult` | 获取结果 |
| `GetCurrentPath` | - | `[]Point` | 获取路径 |

### 数据类型

| 类型 | 字段 | 说明 |
|------|------|------|
| `MapData` | `width, height, grid, startPoint, endPoint` | 地图数据 |
| `Point` | `x, y` | 坐标点 |
| `StepData` | `state, current, neighbors, added, pruned, path, visited, distance, expanded, stepsTaken` | 步骤数据 |
| `SearchResult` | `found, distance, path, algorithm` | 搜索结果 |

## 🚀 使用方法

### 1. 初始化 Wails 项目

```bash
cd backend
wails init
```

### 2. 运行开发模式

```bash
wails dev
```

### 3. 前端调用示例

```typescript
import { wailsService } from '@/services/wails'

async function example() {
  // 创建地图
  await wailsService.createMap(20, 15)
  
  // 初始化搜索
  await wailsService.initializeSearch('bfs')
  
  // 执行搜索
  while (!await wailsService.isSearchDone()) {
    await wailsService.searchStep()
  }
  
  // 获取结果
  const result = await wailsService.getSearchResult()
  console.log('Path:', result.path)
}
```

## 📊 数据流示例

```
1. 用户创建地图 (20x15)
   ↓
2. Frontend: wailsService.createMap(20, 15)
   ↓
3. Wails IPC: go_main_App_CreateMap(20, 15)
   ↓
4. Backend: App.CreateMap()
   ↓
5. 创建 Map{width:20, height:15, ...}
   ↓
6. 返回 MapData JSON
   ↓
7. Frontend: 接收并更新状态
```

## 🎯 下一步建议

1. **测试当前实现**
   - 运行 `wails dev`
   - 测试地图创建
   - 测试搜索算法

2. **集成到 UI**
   - 替换现有 `executionStore`
   - 使用 `useWailsSearch` composable
   - 实现实时可视化

3. **扩展功能**
   - 添加更多算法
   - 实现地图保存/加载
   - 添加性能统计

4. **优化性能**
   - 批量操作优化
   - 异步加载优化
   - 渲染性能优化

## 📚 相关资源

- [Wails 官方文档](https://wails.io/)
- [Vue 3 文档](https://vuejs.org/)
- [TypeScript 文档](https://www.typescriptlang.org/)

## ✨ 总结

Wails 对接代码已经完整实现，包括：
- ✅ Go 后端 Wails 绑定
- ✅ TypeScript 前端服务层
- ✅ Composable Hook 封装
- ✅ 完整的数据类型映射
- ✅ 详细的文档和示例

现在可以：
1. 启动 `wails dev` 进行开发
2. 使用提供的 API 构建功能
3. 参考文档进行扩展

有任何问题请参考 `WAILS_INTEGRATION.md` 和 `WAILS_QUICKSTART.md`！
