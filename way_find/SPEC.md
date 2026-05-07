# WayFind - 迷宫寻路算法可视化系统

## 1. 项目概述

### 项目名称
WayFind - 寻路算法可视化系统

### 项目类型
桌面应用程序（使用 Wails 框架）

### 核心功能
实现三种经典寻路算法（DFS、BFS、A*）的可视化界面，支持地图编辑、路径搜索、结果展示

### 目标用户
计算机科学学生、算法学习者、教育工作者

---

## 2. 架构设计

### 2.1 核心架构原则

本项目遵循**显示逻辑与数据存储分离**的原则，采用了简化的架构设计：

- **算法层（algorithms/）**：使用统一的 Step 模式展示搜索过程
- **Map 层（map/）**：负责地图数据存储和邻居查询
- **Path 层（path/）**：使用 DistGrid 记录距离信息
- **Queue 层（queue/）**：BFS 使用的队列数据结构

### 2.2 目录结构

```
backend/
├── main.go                    # Wails 应用入口
├── algorithms/                # 寻路算法实现
│   ├── step.go               # 统一接口定义
│   ├── result.go             # 搜索结果结构
│   ├── dfs.go                # 深度优先搜索
│   ├── bfs.go                # 广度优先搜索
│   ├── astar.go              # A* 算法
│   ├── search_test.go        # 算法测试
│   └── step_test.go          # Step 模式测试
├── map/                       # 地图数据结构
│   ├── map.go                # 地图定义和操作
│   └── map_test.go           # 地图测试
├── path/                      # 距离网格
│   └── path.go               # DistGrid 实现
├── queue/                     # 队列数据结构
│   └── queue.go              # BFS 队列
├── cmd/                       # 命令行工具
│   └── interactive.go        # 交互式演示
└── data/                      # 地图数据文件
    └── test_map.txt          # 测试地图

frontend/                      # Vue 3 前端
├── src/
│   ├── components/            # Vue 组件
│   ├── stores/                # Pinia 状态
│   └── ...
└── ...
```

### 2.3 架构图

```
┌──────────────────────────────────────────────────────────────┐
│                    Searcher Interface                        │
│         (Initialize / Step / IsDone / GetResult)             │
└──────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ↓                     ↓                     ↓
┌───────────────┐    ┌───────────────┐    ┌───────────────┐
│  DFSAlgorithm │    │  BFSAlgorithm │    │  AStarAlgorithm│
│  ─────────────│    │  ─────────────│    │  ─────────────│
│  stack[]      │    │  queue[]      │    │  heap[]       │
│  distGrid     │    │  distGrid     │    │  distGrid     │
│  visited[]    │    │  visited[]    │    │  visited[]    │
└───────────────┘    └───────────────┘    └───────────────┘
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              ↓
┌──────────────────────────────────────────────────────────────┐
│                      DistGrid (path/)                        │
│            记录每个点到起点的距离，用于剪枝和回溯              │
└──────────────────────────────────────────────────────────────┘
                              │
                              ↓
┌──────────────────────────────────────────────────────────────┐
│                        Map (map/)                             │
│              Grid[][] + Start + End + GetNeighbors()         │
└──────────────────────────────────────────────────────────────┘
```

---

## 3. 核心接口定义

### 3.1 Searcher 接口

所有算法实现统一的 Step 模式接口：

```go
type SearchState int

const (
    StateReady    SearchState = iota  // 就绪
    StateRunning                      // 运行中
    StateFound                        // 已找到
    StateNotFound                     // 未找到
)

type StepResult struct {
    State      SearchState      // 当前状态
    Current    Point             // 当前探索位置
    Neighbors  []Point           // 邻居节点
    Added      []Point           // 新增节点
    Pruned     []Point           // 剪枝节点
    Path       []Point           // 最佳路径
    Visited    []Point           // 已访问节点
    Distance   int               // 当前最短距离
    Expanded   int               // 扩展节点数
    StepsTaken int               // 已执行步数
}

type Searcher interface {
    Initialize(m *Map)           // 初始化算法
    Step() *StepResult           // 执行一步
    IsDone() bool                // 判断是否完成
    GetResult() *SearchResult    // 获取最终结果
    GetCurrentPath() []Point     // 获取当前探索路径
}
```

### 3.2 搜索结果

```go
type SearchResult struct {
    Path      []Point  // 找到的路径
    Expanded  int      // 扩展节点数
    Time      int64    // 执行时间（纳秒）
    Distance  int      // 路径长度
    Algorithm string   // 算法名称
    Found     bool     // 是否找到路径
}
```

---

## 4. 数据结构

### 4.1 地图结构 (map/)

```go
type CellType uint8

const (
    CELL_ROAD  CellType = 0  // 道路
    CELL_WALL  CellType = 1  // 墙壁
    CELL_START CellType = 2  // 起点
    CELL_END   CellType = 3  // 终点
)

type Point struct {
    X int
    Y int
}

func (p Point) Equals(other Point) bool
func (p Point) String() string

type Map struct {
    Width  int
    Height int
    Grid   [][]CellType
    Start  Point
    End    Point
}

func NewMap(width, height int) *Map
func (m *Map) IsValid(x, y int) bool
func (m *Map) IsWall(x, y int) bool
func (m *Map) GetNeighbors(p Point) []Point
func (m *Map) SetCell(x, y int, cellType CellType) error
func (m *Map) Validate() error
```

### 4.2 距离网格 (path/)

DistGrid 用于记录每个点到起点的最短距离，配合各算法实现剪枝：

```go
type DistGrid struct {
    grid   [][]int  // 距离网格，初始化为 MaxInt
    width  int
    height int
}

func NewDistGrid(width, height int) *DistGrid
func (dg *DistGrid) Get(p Point) int          // 获取距离
func (dg *DistGrid) Set(p Point, dist int)     // 设置距离
func (dg *DistGrid) IsBetter(p Point, newDist int) bool  // 判断是否更好
func (dg *DistGrid) IsUnvisited(p Point) bool            // 是否未访问
```

### 4.3 队列 (queue/)

BFS 使用简单的切片实现队列：

```go
type BFSItem struct {
    Point Point
    Step  int
}
```

---

## 5. 算法实现

### 5.1 深度优先搜索 (DFS)

**数据结构组合**：`栈切片 + DistGrid + 最短路径剪枝`

```go
type DFSItem struct {
    Point mappkg.Point
    Step  int
}

type DFSAlgorithm struct {
    m          *mappkg.Map
    stack      []DFSItem      // 栈（LIFO）
    distGrid   *pathpkg.DistGrid
    visited    []mappkg.Point
    state      SearchState
    current    mappkg.Point   // 当前探索位置
    bestDist   int            // 当前最短距离
    bestPath   []mappkg.Point // 当前最佳路径
}
```

**算法流程**：
1. 初始化：起点入栈，distGrid[起点] = 0
2. 循环：
   - 弹栈获取当前节点
   - 更新 distGrid
   - 如果到达终点且距离更短，记录路径
   - 剪枝：如果新距离 >= bestDist，跳过
   - 否则将有效邻居入栈
3. 重复直到栈为空或找到终点

**特点**：
- 栈实现深度优先
- DistGrid 记录最短距离，实现剪枝优化
- 不保证找到最短路径（但能找到一条路径）

### 5.2 广度优先搜索 (BFS)

**数据结构组合**：`队列切片 + DistGrid`

```go
type BFSAlgorithm struct {
    m        *mappkg.Map
    queue    []queue.BFSItem  // 队列（FIFO）
    distGrid *pathpkg.DistGrid
    visited  []mappkg.Point
    state    SearchState
    current  mappkg.Point
    bestDist int
    bestPath []mappkg.Point
}
```

**算法流程**：
1. 初始化：起点入队，distGrid[起点] = 0
2. 循环：
   - 出队获取当前节点
   - 更新 distGrid
   - 如果到达终点，记录路径，返回结果
   - 将未访问的邻居入队
3. 重复直到队列为空

**特点**：
- 队列实现广度优先
- 保证找到最短路径（无权图）
- DistGrid 用于路径回溯

### 5.3 A* 算法

**数据结构组合**：`最小堆 + DistGrid + 曼哈顿距离启发式`

```go
type AStarItem struct {
    Point mappkg.Point
    G     int  // 从起点到当前的实际代价
    H     int  // 从当前到终点的估计代价（曼哈顿距离）
    F     int  // G + H
    Index int  // 堆中索引
}

type AStarHeap []AStarItem  // 使用 container/heap

type AStarAlgorithm struct {
    m        *mappkg.Map
    heap     AStarHeap
    distGrid *pathpkg.DistGrid
    visited  []mappkg.Point
    state    SearchState
    current  mappkg.Point
    bestDist int
    bestPath []mappkg.Point
}

func ManhattanDistance(p1, p2 mappkg.Point) int {
    return int(math.Abs(float64(p1.X-p2.X)) + math.Abs(float64(p1.Y-p2.Y)))
}
```

**算法流程**：
1. 初始化：
   - 起点入堆，G=0，H=曼哈顿距离
   - distGrid[起点] = 0
2. 循环：
   - 出堆获取 F 值最小的节点
   - 更新 distGrid
   - 如果到达终点，记录路径，返回结果
   - 将有效邻居入堆
3. 重复直到堆为空

**特点**：
- 最小堆按 F = G + H 排序
- 保证找到最短路径
- 曼哈顿距离是 admissible 的启发式函数

---

## 6. Step 模式可视化

### 6.1 GetCurrentPath 机制

每种算法都实现了 `GetCurrentPath()` 方法，通过 distGrid 回溯当前探索路径：

```go
func (d *DFSAlgorithm) GetCurrentPath() []mappkg.Point {
    if d.current.X == 0 && d.current.Y == 0 && !d.current.Equals(d.m.Start) {
        return nil
    }
    return d.getPathFromPoint(d.current)
}

func (d *DFSAlgorithm) getPathFromPoint(p mappkg.Point) []mappkg.Point {
    path := make([]mappkg.Point, 0)
    current := p
    for {
        path = append(path, current)
        if current.Equals(d.m.Start) {
            break
        }
        step := d.distGrid.Get(current)
        if step == 0 {
            break
        }
        neighbors := d.m.GetNeighbors(current)
        found := false
        for _, n := range neighbors {
            if d.distGrid.Get(n) == step-1 {
                current = n
                found = true
                break
            }
        }
        if !found {
            break
        }
    }
    for i, j := 0, len(path)-1; i < j; i, j = i+1, j-1 {
        path[i], path[j] = path[j], path[i]
    }
    return path
}
```

### 6.2 可视化符号

交互式演示使用以下符号：

| 符号 | 含义 | 颜色 |
|------|------|------|
| `S` | 起点 | 绿色 |
| `E` | 终点 | 红色 |
| `#` | 墙壁 | 深灰色 |
| `.` | 未探索 | 白色 |
| `*` | 已访问 | 蓝色 |
| `+` | 最佳路径 | 高亮 |
| `@` | 当前探索路径 | 高亮 |

### 6.3 交互流程

```
┌─────────────────────────────────────────────┐
│  迷宫路径查找 - 交互式演示                      │
├─────────────────────────────────────────────┤
│  选择地图文件                                 │
│  选择算法 (BFS/DFS/A*)                        │
│  初始化算法                                   │
├─────────────────────────────────────────────┤
│  按回车执行一步 (q 退出)                       │
│                                             │
│  ===== 第 N 步 =====                          │
│  状态: 运行中/已找到/未找到                    │
│  当前位置: (x, y)                            │
│  已扩展节点: N                                │
│  路径长度: N                                  │
│                                             │
│  地图显示:                                    │
│  S @ @ # * # * * # *                          │
│  # . @ # * # * * # *                          │
│  ...                                         │
│                                             │
│  图例: S=起点, E=终点, #=墙,                   │
│        .=未探索, *=已访问, +=最短路径,         │
│        @=当前路径                             │
└─────────────────────────────────────────────┘
```

---

## 7. 算法对比

| 特性 | DFS | BFS | A* |
|------|-----|-----|-----|
| **数据结构** | 栈 (LIFO) | 队列 (FIFO) | 最小堆 |
| **最短路径** | ❌ 不保证 | ✅ 保证 | ✅ 保证 |
| **时间复杂度** | O(b^m) | O(b^d) | O(b^d) |
| **空间复杂度** | O(bd) | O(b^d) | O(b^d) |
| **剪枝优化** | ✅ 基于已知最短路径 | ❌ | ❌ |
| **启发式** | ❌ | ❌ | ✅ 曼哈顿距离 |

其中：
- b = 分支因子（每个节点的平均邻居数）
- d = 最短路径深度
- m = 最大深度

---

## 8. 统一剪枝策略

所有算法使用统一的剪枝策略，通过 DistGrid 实现：

```go
// 剪枝条件
if d.distGrid.IsBetter(neighbor, newStep) && newStep < d.bestDist {
    d.distGrid.Set(neighbor, newStep)
    // 入栈/入队
}
```

**剪枝规则**：
1. 如果邻居已被访问且距离更短，跳过
2. 如果当前深度已达到已知最短路径，跳过（DFS）
3. 只访问未被访问或距离更短的节点

---

## 9. 地图格式

### 9.1 地图文件格式 (.txt)

```
width height           # 地图尺寸
startX startY         # 起点坐标
endX endY             # 终点坐标
grid...               # 网格数据（每行对应地图的一行）
```

### 9.2 示例

```
5 5                   # 5x5 地图
0 0                   # 起点 (0, 0)
4 4                   # 终点 (4, 4)
0 0 0 0 0             # 第一行：全道路
0 0 0 0 0             # 第二行：全道路
0 1 1 0 0             # 第三行：中间有墙壁
0 0 0 0 0             # 第四行：全道路
0 0 0 0 0             # 第五行：全道路
```

### 9.3 网格值说明

- `0` = CELL_ROAD（道路，可通过）
- `1` = CELL_WALL（墙壁，不可通过）
- `2` = CELL_START（起点）
- `3` = CELL_END（终点）

---

## 10. UI/UX 规范

### 10.1 布局结构

```
+--------------------------------------------------+
|  标题栏 (系统原生)                                |
+--------------------------------------------------+
|  顶部工具栏 (60px)                                |
|  [新建地图] [打开文件] [保存] | [算法选择] [开始]  |
+--------------------------------------------------+
|                                                   |
|  左侧面板 (250px)      |     主画布区域            |
|  - 节点列表            |     - 网格地图显示         |
|  - 起点/终点设置       |     - 节点编辑             |
|  - 地图设置            |     - 路径可视化           |
|                        |                           |
+--------------------------------------------------+
|  底部信息栏 (50px)                                |
|  [状态信息] [扩展节点数: X] [时间: Xms] [距离: X]  |
+--------------------------------------------------+
```

### 10.2 颜色方案

| 元素 | 颜色代码 | 说明 |
|------|----------|------|
| 主色调 | #2563EB | 蓝色科技感 |
| 起点 | #10B981 | 绿色 |
| 终点 | #EF4444 | 红色 |
| 背景 | #F8FAFC | 浅灰白 |
| 墙壁 | #374151 | 深灰 |
| 已访问 | #93C5FD | 浅蓝 |
| 最短路径 | #2563EB | 蓝色 |
| DFS路径 | #F59E0B | 橙色 |
| BFS路径 | #8B5CF6 | 紫色 |
| A*路径 | #06B6D4 | 青色 |

### 10.3 组件规范

#### 按钮
- 默认：背景 #2563EB，文字白色
- 悬停：背景 #1D4ED8
- 禁用：背景 #CBD5E1，文字灰色

#### 画布
- 网格大小：30x30 像素/格
- 默认地图：20x15 格
- 最小尺寸：5x5
- 最大尺寸：50x50

---

## 11. 测试

### 11.1 测试文件

```
algorithms/
├── search_test.go     # 基础功能测试
└── step_test.go       # Step 模式测试
```

### 11.2 测试用例

```go
// 基础测试
TestBFS()           // BFS 基础功能
TestDFS()           // DFS 基础功能
TestAStar()         // A* 基础功能
TestBFSWithObstacles()  // 有障碍物测试
TestNoPath()        // 无路径测试

// Step 模式测试
TestBFSStep()       // BFS 单步执行
TestDFSStep()       // DFS 单步执行
TestAStarStep()     // A* 单步执行
TestPathOverwrite() // 路径覆盖测试
TestCompareAlgorithms()  // 算法对比测试
```

### 11.3 运行测试

```bash
cd backend
go test ./...           # 运行所有测试
go test -v ./algorithms/...  # 详细输出
```

---

## 12. 技术栈

### 后端
- **语言**：Go 1.21+
- **框架**：Wails v2
- **构建工具**：Wails CLI

### 前端
- **框架**：Vue 3 (Composition API)
- **构建工具**：Vite
- **语言**：TypeScript
- **状态管理**：Pinia

---

## 13. 验收标准

### 功能验收
- [x] 程序能够正常启动
- [x] BFS/DFS/A* 三种算法都能正确执行
- [x] Step 模式逐步展示搜索过程
- [x] GetCurrentPath 显示当前探索路径
- [x] 路径可视化正确显示
- [x] 统计信息准确
- [x] 文件保存和加载功能
- [x] 单元测试全部通过

### 性能要求
- 地图尺寸 ≤ 30x30 时，算法执行 < 1秒
- UI 操作响应时间 < 100ms
- 内存占用 < 200MB
