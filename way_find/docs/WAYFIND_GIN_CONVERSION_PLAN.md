# WayFind Wails → Gin REST API 转换实施计划

## 一、项目背景

WayFind 当前使用 Wails 3 框架封装为桌面应用，已有一个对应的 Vue 3 前端（`frontend/`）。需要将 Wails 后端转换为 Gin REST API，使其能独立部署并对接现有的 `show-web` 前端项目。

### 1.1 现状分析

| 组件 | 技术栈 | 说明 |
|------|--------|------|
| 寻路算法 | Go | BFS / DFS / A*，支持 Step 逐步执行 |
| 数据结构 | Go | Map、DistGrid、Point、队列 |
| 存储 | Go 文件系统 | JSON + 文本格式地图存储 |
| 后端框架 | Wails 3 | JSON-RPC IPC 暴露方法给前端 |
| 前端 | Vue 3 + Vite | Pinia 状态管理，Vite Dev Server |

现有前端已经通过 `useWailsSearch` Hook 封装了所有 Wails 调用。转换后前端需改为调用 REST API。

### 1.2 转换目标

```
当前架构:
  [Vue Frontend] ←(Wails IPC)→ [Go Wails Backend]

目标架构:
  [Vue Frontend] ←(HTTP JSON)→ [Go Gin Backend] (8081)
  [show-web] ←(HTTP JSON)→ [Go Gin Backend] (8081)
```

---

## 二、技术方案

### 2.1 技术选型

| 项目 | 选型 | 说明 |
|------|------|------|
| HTTP 框架 | Gin v1.10+ | 高性能、成熟稳定 |
| Go 版本 | 1.21+ | 兼容现有 go.mod |
| 端口 | 8081 | 避免与专家系统 8080 冲突 |
| 数据格式 | JSON | 与现有前端兼容 |
| CORS | 启用 | 允许跨域请求 |
| 日志 | Gin Logger + 自定义 | 请求追踪 |

### 2.2 目录结构

```
way_find/
├── backend/
│   ├── main.go           ← Gin 入口（新建）
│   ├── main_wails.go     ← 保留（可选删除）
│   ├── app.go            ← 核心服务逻辑（保留）
│   ├── handlers.go       ← HTTP 处理器（新建，拆分自 app.go）
│   ├── router.go         ← 路由注册（新建）
│   ├── middleware/
│   │   └── cors.go       ← CORS 中间件（新建）
│   ├── algorithms/       ← 寻路算法（保留不变）
│   ├── map/              ← 地图结构（保留不变）
│   ├── path/             ← 距离网格（保留不变）
│   ├── queue/            ← 队列（保留不变）
│   ├── storage/          ← 存储（保留不变）
│   └── data/             ← 测试数据（保留不变）
├── frontend/             ← 寻路前端（可选保留或删除）
└── show_web/             ← 对接 show-web 前端（新建）
    └── (由 show-web 项目提供)
```

---

## 三、API 设计

### 3.1 路由总览

```
基础路径: /api/v1

地图管理:
  POST   /api/v1/map/create          创建空白地图
  GET    /api/v1/map                 获取当前地图
  POST   /api/v1/map/load            加载地图（JSON body）
  PUT    /api/v1/map/cell            设置单元格
  DELETE /api/v1/map                 清除地图

可视化:
  GET    /api/v1/map/draw            获取当前搜索状态可视化数据
  GET    /api/v1/map/final-draw      获取最终搜索结果可视化

搜索控制:
  POST   /api/v1/search/init         初始化搜索算法
  POST   /api/v1/search/step         执行单步搜索
  GET    /api/v1/search/done         查询是否完成
  GET    /api/v1/search/result       获取搜索结果
  GET    /api/v1/search/path         获取当前路径

地图存储:
  GET    /api/v1/maps                列出所有地图
  GET    /api/v1/maps/:name          按名称加载地图
  POST   /api/v1/maps/:name          保存当前地图
  DELETE /api/v1/maps/:name          删除地图

系统:
  GET    /api/v1/health              健康检查
  GET    /api/v1/algorithms          支持的算法列表
```

### 3.2 请求与响应详情

#### 3.2.1 地图创建

```
POST /api/v1/map/create
Body: { "width": 20, "height": 15 }
Response:
{
  "code": 0,
  "data": {
    "width": 20,
    "height": 15,
    "grid": [[0,0,...], ...],
    "startPoint": null,
    "endPoint": null
  }
}
```

#### 3.2.2 设置单元格

```
PUT /api/v1/map/cell
Body: { "x": 5, "y": 3, "cellType": 1 }
CellType: 0=道路 1=墙壁 2=起点 3=终点
Response: { "code": 0, "message": "ok" }
```

#### 3.2.3 初始化搜索

```
POST /api/v1/search/init
Body: { "algorithm": "astar" }
算法可选值: "bfs" | "dfs" | "astar"
Response:
{
  "code": 0,
  "data": {
    "algorithm": "astar",
    "mapValid": true,
    "hasStart": true,
    "hasEnd": true
  }
}
```

#### 3.2.4 执行单步搜索

```
POST /api/v1/search/step
Response:
{
  "code": 0,
  "data": {
    "state": 1,           // 0=就绪 1=运行中 2=已找到 3=未找到
    "current": {"x": 0, "y": 0},
    "neighbors": [...],
    "added": [...],
    "pruned": [...],
    "path": [...],
    "visited": [...],
    "distance": 5,
    "expanded": 12,
    "stepsTaken": 12
  }
}
```

#### 3.2.5 获取搜索结果

```
GET /api/v1/search/result
Response:
{
  "code": 0,
  "data": {
    "found": true,
    "distance": 25,
    "path": [...],
    "algorithm": "astar",
    "expanded": 45
  }
}
```

#### 3.2.6 获取可视化数据

```
GET /api/v1/map/draw
Response:
{
  "code": 0,
  "data": {
    "width": 20,
    "height": 15,
    "cells": [[0,0,1,0,...], ...]
    // cells: 0=路 1=墙 2=起点 3=终点 4=已访问 5=当前 6=路径
  }
}
```

### 3.3 错误响应格式

```json
{
  "code": 40001,
  "message": "地图尚未创建",
  "data": null
}
```

错误码约定：
| 范围 | 说明 |
|------|------|
| 0 | 成功 |
| 40001-40099 | 参数错误 |
| 40101-40199 | 地图未创建/无效 |
| 40201-40299 | 搜索未初始化 |
| 50001-50099 | 内部错误 |

---

## 四、核心代码设计

### 4.1 Gin 入口 (main.go)

```go
package main

import (
    "log"
    "github.com/gin-gonic/gin"
    "wayfind/backend"
)

func main() {
    // 设置 Gin 模式
    gin.SetMode(gin.ReleaseMode)

    r := gin.New()
    r.Use(gin.Logger())
    r.Use(gin.Recovery())

    // 启用 CORS
    setupCORS(r)

    // 创建服务实例（复用 app.go 的核心逻辑）
    service := backend.NewWayFindService()

    // 注册路由
    backend.RegisterRoutes(r, service)

    log.Println("WayFind API server starting on :8081")
    if err := r.Run(":8081"); err != nil {
        log.Fatal("Failed to start server:", err)
    }
}
```

### 4.2 HTTP 处理器 (handlers.go)

将 `app.go` 中的 Wails 方法转换为 HTTP 处理器：

```go
package backend

import (
    "github.com/gin-gonic/gin"
)

type Handler struct {
    service *WayFindService
}

func NewHandler(service *WayFindService) *Handler {
    return &Handler{service: service}
}

func (h *Handler) CreateMap(c *gin.Context) {
    var req CreateMapRequest
    if err := c.ShouldBindJSON(&req); err != nil {
        respondError(c, 40001, "参数错误: "+err.Error())
        return
    }
    result, err := h.service.CreateMap(req.Width, req.Height)
    if err != nil {
        respondError(c, 50001, err.Error())
        return
    }
    respondOK(c, result)
}

func (h *Handler) SetCell(c *gin.Context) {
    var req SetCellRequest
    if err := c.ShouldBindJSON(&req); err != nil {
        respondError(c, 40001, "参数错误: "+err.Error())
        return
    }
    if err := h.service.SetCell(req.X, req.Y, req.CellType); err != nil {
        respondError(c, 40101, err.Error())
        return
    }
    respondOK(c, nil)
}

func (h *Handler) InitializeSearch(c *gin.Context) {
    var req InitSearchRequest
    if err := c.ShouldBindJSON(&req); err != nil {
        respondError(c, 40001, "参数错误")
        return
    }
    if err := h.service.InitializeSearch(req.Algorithm); err != nil {
        respondError(c, 40101, err.Error())
        return
    }
    respondOK(c, gin.H{
        "algorithm": req.Algorithm,
        "mapValid":  h.service.currentMap != nil,
        "hasStart":  h.service.currentMap != nil && h.service.currentMap.HasStart(),
        "hasEnd":    h.service.currentMap != nil && h.service.currentMap.HasEnd(),
    })
}

func (h *Handler) SearchStep(c *gin.Context) {
    step, err := h.service.SearchStep()
    if err != nil {
        respondError(c, 50001, err.Error())
        return
    }
    respondOK(c, step)
}

func (h *Handler) GetSearchDone(c *gin.Context) {
    respondOK(c, gin.H{"done": h.service.IsSearchDone()})
}

func (h *Handler) GetSearchResult(c *gin.Context) {
    result, err := h.service.GetSearchResult()
    if err != nil {
        respondError(c, 40201, err.Error())
        return
    }
    respondOK(c, result)
}

func (h *Handler) GetCurrentPath(c *gin.Context) {
    path, err := h.service.GetCurrentPath()
    if err != nil {
        respondError(c, 40201, err.Error())
        return
    }
    respondOK(c, path)
}

func (h *Handler) GetDraw(c *gin.Context) {
    draw, err := h.service.GetDraw()
    if err != nil {
        respondError(c, 50001, err.Error())
        return
    }
    respondOK(c, draw)
}

func (h *Handler) GetFinalDraw(c *gin.Context) {
    draw, err := h.service.GetFinalDraw()
    if err != nil {
        respondError(c, 50001, err.Error())
        return
    }
    respondOK(c, draw)
}

func (h *Handler) GetMap(c *gin.Context) {
    m, err := h.service.GetMap()
    if err != nil {
        respondError(c, 40101, err.Error())
        return
    }
    respondOK(c, m)
}

func (h *Handler) SaveMap(c *gin.Context) {
    name := c.Param("name")
    if name == "" {
        respondError(c, 40001, "地图名称不能为空")
        return
    }
    if err := h.service.SaveMap(name); err != nil {
        respondError(c, 50001, err.Error())
        return
    }
    respondOK(c, gin.H{"name": name})
}

func (h *Handler) LoadMapByName(c *gin.Context) {
    name := c.Param("name")
    mapData, err := h.service.LoadMapByName(name)
    if err != nil {
        respondError(c, 40101, err.Error())
        return
    }
    respondOK(c, mapData)
}

func (h *Handler) DeleteMap(c *gin.Context) {
    name := c.Param("name")
    if err := h.service.DeleteMap(name); err != nil {
        respondError(c, 50001, err.Error())
        return
    }
    respondOK(c, nil)
}

func (h *Handler) ListMaps(c *gin.Context) {
    maps, err := h.service.ListMaps()
    if err != nil {
        respondError(c, 50001, err.Error())
        return
    }
    respondOK(c, maps)
}

func (h *Handler) GetAlgorithms(c *gin.Context) {
    respondOK(c, gin.H{
        "algorithms": []string{"bfs", "dfs", "astar"},
        "default": "astar"
    })
}

func (h *Handler) HealthCheck(c *gin.Context) {
    respondOK(c, gin.H{"status": "ok"})
}

// 统一响应格式
func respondOK(c *gin.Context, data interface{}) {
    c.JSON(200, gin.H{"code": 0, "data": data})
}

func respondError(c *gin.Context, code int, message string) {
    c.JSON(200, gin.H{"code": code, "message": message})
}
```

### 4.3 路由注册 (router.go)

```go
package backend

import "github.com/gin-gonic/gin"

func RegisterRoutes(r *gin.Engine, service *WayFindService) {
    h := NewHandler(service)

    v1 := r.Group("/api/v1")
    {
        // 健康检查
        v1.GET("/health", h.HealthCheck)
        v1.GET("/algorithms", h.GetAlgorithms)

        // 地图管理
        v1.POST("/map/create", h.CreateMap)
        v1.GET("/map", h.GetMap)
        v1.PUT("/map/cell", h.SetCell)

        // 可视化
        v1.GET("/map/draw", h.GetDraw)
        v1.GET("/map/final-draw", h.GetFinalDraw)

        // 搜索控制
        v1.POST("/search/init", h.InitializeSearch)
        v1.POST("/search/step", h.SearchStep)
        v1.GET("/search/done", h.GetSearchDone)
        v1.GET("/search/result", h.GetSearchResult)
        v1.GET("/search/path", h.GetCurrentPath)

        // 地图存储
        v1.GET("/maps", h.ListMaps)
        v1.GET("/maps/:name", h.LoadMapByName)
        v1.POST("/maps/:name", h.SaveMap)
        v1.DELETE("/maps/:name", h.DeleteMap)
    }
}
```

### 4.4 CORS 中间件 (middleware/cors.go)

```go
package middleware

import (
    "github.com/gin-gonic/gin"
)

func CORS() gin.HandlerFunc {
    return func(c *gin.Context) {
        c.Writer.Header().Set("Access-Control-Allow-Origin", "*")
        c.Writer.Header().Set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
        c.Writer.Header().Set("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With")
        c.Writer.Header().Set("Access-Control-Max-Age", "86400")

        if c.Request.Method == "OPTIONS" {
            c.AbortWithStatus(204)
            return
        }

        c.Next()
    }
}
```

---

## 五、前端对接方案

### 5.1 方案选择

前端有两种对接方式：

| 方案 | 优点 | 缺点 |
|------|------|------|
| **A. 新建 show-web 子路由** (推荐) | 复用现有教授系统布局，统一域名 | 需要新增路由 |
| B. 单独部署 wayfind-vue | 独立管理 | 重复代码，跨域问题 |

推荐**方案 A**：在 `show-web` 中新增 `/wayfind` 路由，调用 Gin API。

### 5.2 前端服务层设计

```typescript
// src/api/wayfind.ts

const API_BASE = import.meta.env.VITE_WAYFIND_API || 'http://localhost:8081/api/v1'

interface ApiResponse<T> {
  code: number
  data: T
  message?: string
}

async function request<T>(
  method: string,
  path: string,
  body?: object
): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`, {
    method,
    headers: { 'Content-Type': 'application/json' },
    body: body ? JSON.stringify(body) : undefined,
  })
  const json: ApiResponse<T> = await res.json()
  if (json.code !== 0) {
    throw new Error(json.message || `API Error ${json.code}`)
  }
  return json.data
}

// 地图管理
export const wayfindMap = {
  create: (width: number, height: number) =>
    request<MapData>('POST', '/map/create', { width, height }),

  get: () => request<MapData>('GET', '/map'),

  setCell: (x: number, y: number, cellType: CellType) =>
    request<void>('PUT', '/map/cell', { x, y, cellType }),

  getDraw: () => request<DrawData>('GET', '/map/draw'),
  getFinalDraw: () => request<DrawData>('GET', '/map/final-draw'),
}

// 搜索控制
export const wayfindSearch = {
  init: (algorithm: AlgorithmType) =>
    request<void>('POST', '/search/init', { algorithm }),

  step: () => request<StepData>('POST', '/search/step'),

  isDone: () => request<{ done: boolean }>('GET', '/search/done'),

  getResult: () => request<SearchResultData>('GET', '/search/result'),

  getPath: () => request<PointData[]>('GET', '/search/path'),
}

// 地图存储
export const wayfindStorage = {
  list: () => request<MapInfoData[]>('GET', '/maps'),

  load: (name: string) => request<MapData>('GET', `/maps/${name}`),

  save: (name: string) => request<void>('POST', `/maps/${name}`),

  delete: (name: string) => request<void>('DELETE', `/maps/${name}`),
}
```

### 5.3 搜索 Hook 封装

```typescript
// src/composables/useWayFindSearch.ts

import { ref, computed } from 'vue'
import { wayfindMap, wayfindSearch, wayfindStorage } from '../api/wayfind'

export type AlgorithmType = 'bfs' | 'dfs' | 'astar'
export type CellType = 0 | 1 | 2 | 3  // 道路|墙壁|起点|终点

export function useWayFindSearch() {
  const map = ref<MapData | null>(null)
  const algorithm = ref<AlgorithmType>('astar')
  const currentStep = ref<StepData | null>(null)
  const isSearching = ref(false)
  const isDone = ref(false)
  const searchResult = ref<SearchResultData | null>(null)
  const drawData = ref<DrawData | null>(null)

  async function createMap(width: number, height: number) {
    map.value = await wayfindMap.create(width, height)
    isDone.value = false
    searchResult.value = null
    currentStep.value = null
  }

  async function setCell(x: number, y: number, cellType: CellType) {
    await wayfindMap.setCell(x, y, cellType)
  }

  async function initSearch(algo: AlgorithmType) {
    algorithm.value = algo
    await wayfindSearch.init(algo)
    isSearching.value = false
    isDone.value = false
    currentStep.value = null
    searchResult.value = null
  }

  async function step() {
    const done = await wayfindSearch.isDone()
    if (done) return

    currentStep.value = await wayfindSearch.step()
    drawData.value = await wayfindMap.getDraw()

    const done2 = await wayfindSearch.isDone()
    if (done2) {
      isDone.value = true
      searchResult.value = await wayfindSearch.getResult()
    }
  }

  async function runToEnd() {
    while (!isDone.value) {
      await step()
    }
  }

  return {
    map,
    algorithm,
    currentStep,
    isSearching,
    isDone,
    searchResult,
    drawData,
    createMap,
    setCell,
    initSearch,
    step,
    runToEnd,
  }
}
```

---

## 六、实施步骤

### 阶段一：后端改造（第 1-2 天）

1. **新增 Gin 入口**
   - 创建 `main.go`（Gin 入口）
   - 创建 `handlers.go`（HTTP 处理器）
   - 创建 `router.go`（路由注册）
   - 创建 `middleware/cors.go`（跨域中间件）

2. **改造 app.go**
   - 将 Wails 特定代码分离
   - 确保核心逻辑独立于 Wails

3. **添加 go.mod 依赖**
   ```bash
   go get github.com/gin-gonic/gin@v1.10.0
   ```

4. **验证后端独立运行**
   - 启动 `go run main.go`
   - 测试所有 API 端点

### 阶段二：前端对接（第 2-3 天）

5. **show-web 新增路由**
   - 在 `show-web/src/router/` 添加 WayFind 路由
   - 创建 `WayFindView.vue` 页面

6. **创建 API 服务层**
   - `src/api/wayfind.ts` — API 调用封装
   - `src/composables/useWayFindSearch.ts` — Hook

7. **迁移/复用前端组件**
   - 复用车道编辑器组件
   - 复用搜索可视化组件
   - 复用工具栏组件

8. **配置环境变量**
   ```
   VITE_WAYFIND_API=http://localhost:8081/api/v1
   ```

### 阶段三：测试与优化（第 3-4 天）

9. **API 端到端测试**
   - 使用 Postman 或 curl 测试所有接口
   - 验证 BFS/DFS/A* 搜索正确性
   - 验证可视化数据渲染

10. **前后端联调**
    - 处理 CORS 问题
    - 验证地图保存/加载
    - 验证逐步搜索动画

11. **性能测试**
    - 大地图（100x100）搜索性能
    - 并发请求处理

### 阶段四：清理与部署（第 4-5 天）

12. **代码清理**
    - 删除旧的 Wails 相关代码（可选保留 main_wails.go）
    - 清理未使用的导入
    - 添加注释和文档

13. **部署**
    - 编写 Dockerfile
    - 配置 Docker Compose
    - 部署到服务器

---

## 七、Docker 部署

### 7.1 Dockerfile

```dockerfile
FROM golang:1.21-alpine AS builder

WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download

COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -o wayfind-api main.go

FROM alpine:latest
RUN apk --no-cache add ca-certificates
WORKDIR /root/
COPY --from=builder /app/wayfind-api .
COPY --from=builder /app/data ./data
EXPOSE 8081
CMD ["./wayfind-api"]
```

### 7.2 Docker Compose

```yaml
version: '3.8'

services:
  wayfind-api:
    build: .
    ports:
      - "8081:8081"
    volumes:
      - wayfind-data:/root/data
    environment:
      - WAYFIND_PORT=8081
      - WAYFIND_DATA_DIR=/root/data

  show-web:
    image: nginx:alpine
    ports:
      - "3000:80"
    volumes:
      - ./show-web/dist:/usr/share/nginx/html
      - ./nginx.conf:/etc/nginx/conf.d/default.conf
    depends_on:
      - wayfind-api
    environment:
      - VITE_WAYFIND_API=http://localhost:8081/api/v1

volumes:
  wayfind-data:
```

---

## 八、风险与对策

| 风险 | 影响 | 对策 |
|------|------|------|
| Wails 特定代码耦合 | 核心逻辑依赖 Wails 上下文 | 重构 app.go，移除 Wails 依赖 |
| 前端重写工作量 | 需要新写 API 调用层 | 复用现有 TypeScript 类型 |
| 跨域问题 | 前后端分离部署 | 启用 CORS 中间件 |
| 状态管理 | Wails 有状态，Gin 无状态 | 服务器维护单例服务实例 |
| 地图持久化 | 路径需要正确配置 | 使用环境变量配置存储路径 |

---

## 九、验收标准

- [ ] Gin 后端独立启动，无 Wails 依赖错误
- [ ] 所有 17 个 API 端点返回正确数据
- [ ] BFS/DFS/A* 三种算法搜索结果正确
- [ ] 逐步搜索（Step）模式正常工作
- [ ] 地图保存/加载功能正常
- [ ] show-web 前端正确对接并渲染可视化
- [ ] 100x100 大地图搜索性能可接受（< 2s）
- [ ] Docker 部署成功
