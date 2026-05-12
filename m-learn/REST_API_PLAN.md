# m-learn REST API 服务设计方案

## 一、目标

在 `m-learn/example/` 下新建 `rest-api` 项目，作为 m-learn 库的 HTTP 服务层，将以下 4 个算法封装为 REST API：

| 类型 | 算法 | 来源文件 |
|------|------|----------|
| 拟合 | 线性回归 | `regression-example/src/linear_regression.rs` |
| 拟合 | 正弦回归 | `regression-example/src/sin_regression.rs` |
| 遗传 | Rastrigin 变体函数优化 | `genetic-example/src/rastrigin_variant.rs` |
| 遗传 | Ackley 函数优化 | `genetic-example/src/ackley.rs` |

## 二、核心架构：训练（反向） + 推理（正向）

采用与 way_find 模块相同的 **异步任务 + 状态查询** 模式：

```
┌─────────────┐         ┌─────────────┐
│   前端页面    │         │  m-learn    │
│             │         │  REST API   │
│  1. POST /train ──────→│  创建任务    │
│             │         │  后台训练    │←── 反向传播（更新权重/进化种群）
│  2. GET /train/status ─→│  查询进度    │
│             │         │             │
│  3. GET /inference ────→│  正向传播    │──→ 计算渲染数据
│     (可随时调用)        │  返回曲线点  │
└─────────────┘         └─────────────┘
```

### 反向传播接口（Training）

负责模型训练/优化的后台任务管理。启动后在后台线程中逐步执行，前端可轮询进度。

### 正向传播接口（Inference）

读取当前模型状态（权重/种群），执行正向计算，返回前端渲染所需的数据点。可在训练过程中随时调用，观察模型的实时拟合效果或种群分布。

## 三、REST 框架选型：axum

选择 **axum** 作为 REST 框架：

| 考量 | axum | actix-web | rocket | warp |
|------|------|-----------|--------|------|
| 生态活跃度 | ★★★★★ (tokio 官方) | ★★★★★ | ★★★ | ★★★ |
| 与 tokio 集成 | 原生 | 独立 runtime | 需适配 | 基于 hyper |
| 类型安全 | extract 模式，编译期校验 | 宏较多 | 宏较多 | filter 链较复杂 |
| 学习曲线 | 低 | 中 | 低 | 高 |

**核心理由**：项目 `optim` 和 `nn` crate 已依赖 tokio，axum 基于同一 tokio runtime，零额外开销。

### 依赖清单

```toml
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = { version = "0.6", features = ["cors"] }
uuid = { version = "1", features = ["v4"] }

# m-learn crates
tensor = { path = "../../crates/tensor" }
formula = { path = "../../crates/formula" }
genetic = { path = "../../crates/genetic" }
```

> 不引入 `draw` crate。前端用 JS 图表库渲染，后端只返回 JSON 数据。

## 四、接口设计

### 设计理念：前端驱动的步进式训练

训练由**前端驱动**，每次请求训练 N 个 epoch/generation（称为"步长"），同步等待完成后返回。前端拿到结果后立即请求推理数据刷新图表，再发起下一步训练，如此循环。

```
POST /train → 创建任务，初始化模型
loop:
  POST /train/step { epochs: N } → 同步执行 N 步，返回 loss
  GET /inference → 正向传播，拿渲染数据，刷新图表
  重复直到完成或用户停止
POST /train/stop → 可选，提前终止
```

**步长（epochs）即"速率控制"**：
- `epochs: 1` → 每步训练 1 轮，图表逐帧更新，**动画细腻**（适合展示原理）
- `epochs: 10` → 每步训练 10 轮，图表每 10 帧更新一次，**中等速度**
- `epochs: 100` → 每步训练 100 轮，**快速收敛**（适合展示结果）

### 4.1 训练接口（反向传播）

#### 4.1.1 创建训练任务

初始化模型和训练环境，不执行训练。

```
POST /api/v1/train
```

**拟合类请求体**：
```json
{
  "algorithm": "linear_regression",
  "data": {
    "x": [1, 2, 3, 4, 5],
    "y": [3, 5, 7, 9, 11]
  },
  "params": {
    "learning_rate": 0.01
  }
}
```

**遗传类请求体**：
```json
{
  "algorithm": "rastrigin",
  "params": {
    "population_size": 200,
    "mutation_rate": 0.05,
    "tournament_size": 5,
    "bounds": [0.0, 4.5]
  }
}
```

`algorithm` 可选值：`linear_regression`、`sin_regression`、`rastrigin`、`ackley`

**响应体**：
```json
{
  "taskId": "a1b2c3d4-...",
  "state": "initialized",
  "algorithm": "linear_regression"
}
```

> 拟合类需要传入 `data`（训练数据点）。遗传类不需要训练数据，只需 `params`。

#### 4.1.2 执行训练步

同步接口，执行指定步数的训练后立即返回。这是**反向传播的核心接口**。

```
POST /api/v1/train/step
```

**请求体**：
```json
{
  "taskId": "a1b2c3d4-...",
  "epochs": 5
}
```

- `epochs`：本次训练步数。拟合类为梯度下降迭代次数，遗传类为进化代数。

**响应体（拟合类）**：
```json
{
  "taskId": "a1b2c3d4-...",
  "state": "running",
  "trained_epochs": 5,
  "total_epochs": 5,
  "current_loss": 12.34,
  "convergence": [
    {"epoch": 1, "loss": 45.0},
    {"epoch": 2, "loss": 32.1},
    {"epoch": 3, "loss": 22.5},
    {"epoch": 4, "loss": 16.8},
    {"epoch": 5, "loss": 12.34}
  ]
}
```

**响应体（遗传类）**：
```json
{
  "taskId": "a1b2c3d4-...",
  "state": "running",
  "trained_generations": 5,
  "total_generations": 5,
  "best_fitness": -27.05,
  "convergence": [
    {"generation": 1, "best_fitness": -15.2, "avg_fitness": -8.3},
    {"generation": 2, "best_fitness": -18.7, "avg_fitness": -10.1},
    {"generation": 3, "best_fitness": -21.3, "avg_fitness": -12.4},
    {"generation": 4, "best_fitness": -24.1, "avg_fitness": -14.0},
    {"generation": 5, "best_fitness": -27.05, "avg_fitness": -15.8}
  ]
}
```

- `state`：`running`（还可继续训练）或 `completed`（已达内部最大限制）
- `convergence`：本步内每一轮的 loss/fitness 记录
- 接口同步阻塞直到本步所有 epoch 完成才返回

#### 4.1.3 查询训练状态

获取任务的整体训练历史（跨多次 step 的累积数据）。

```
GET /api/v1/train/status?taskId=a1b2c3d4-...
```

**响应体**：
```json
{
  "taskId": "a1b2c3d4-...",
  "algorithm": "linear_regression",
  "state": "running",
  "total_trained_epochs": 25,
  "latest_loss": 0.0032,
  "convergence": [
    {"epoch": 1, "loss": 45.0},
    {"epoch": 2, "loss": 32.1},
    ...
    {"epoch": 25, "loss": 0.0032}
  ]
}
```

- `convergence` 为全量历史，前端可用于绘制完整的 loss 曲线图

#### 4.1.4 停止训练

```
POST /api/v1/train/stop?taskId=a1b2c3d4-...
```

**响应体**：
```json
{
  "taskId": "a1b2c3d4-...",
  "state": "stopped",
  "total_trained_epochs": 25,
  "message": "训练已停止，保留当前模型状态"
}
```

停止后仍可调用 `/inference` 获取当前模型的渲染数据（查看停止时的拟合效果或种群分布）。

### 4.2 推理接口（正向传播）

读取当前模型权重/种群状态，执行正向计算，返回前端渲染所需的数据点。

```
GET /api/v1/inference?taskId=a1b2c3d4-...
```

**拟合类响应体**（linear_regression / sin_regression）：
```json
{
  "taskId": "a1b2c3d4-...",
  "algorithm": "linear_regression",
  "state": "running",
  "trained_epochs": 25,
  "original_data": {
    "x": [1, 2, 3, 4, 5],
    "y": [3, 5, 7, 9, 11]
  },
  "fitted_curve": {
    "x": [1.0, 1.1, 1.2, ..., 5.0],
    "y": [3.0, 3.2, 3.4, ..., 11.0]
  },
  "weights": [2.0, 1.0],
  "formula": "y = 2.0000 * x + 1.0000"
}
```

| 字段 | 说明 |
|------|------|
| `original_data` | 原始训练数据散点，前端画散点图 |
| `fitted_curve` | 当前权重下的拟合曲线（100 个密集采样点），前端画折线 |
| `weights` | 当前模型权重 |
| `formula` | 当前拟合公式文本 |

**遗传算法响应体**（rastrigin / ackley）：
```json
{
  "taskId": "a1b2c3d4-...",
  "algorithm": "rastrigin",
  "state": "running",
  "trained_generations": 25,
  "function_landscape": {
    "x": [0.0, 0.1, 0.2, ..., 4.5],
    "y": [28.5, 25.3, 22.1, ..., 15.2]
  },
  "population": [1.2, 3.4, 0.8, 2.1, 4.3, ...],
  "best_solution": {
    "x": 4.5203,
    "fitness": -27.0575
  },
  "bounds": [0.0, 4.5]
}
```

| 字段 | 说明 |
|------|------|
| `function_landscape` | 目标函数在搜索范围内的曲线（密集采样），前端画函数图像 |
| `population` | 当前种群所有个体的 x 值，前端画散点分布（展示种群聚集过程） |
| `best_solution` | 当前最优解的位置和适应度 |
| `bounds` | 搜索范围边界 |

### 4.3 公共接口

```
GET  /api/v1/health       → { "status": "ok", "service": "m-learn", "version": "0.1.0" }
GET  /api/v1/algorithms   → 返回所有可用算法列表、描述、默认参数
```

**algorithms 响应**：
```json
{
  "algorithms": [
    {
      "id": "linear_regression",
      "name": "线性回归",
      "type": "regression",
      "description": "使用梯度下降法拟合 y = wx + b",
      "default_params": { "learning_rate": 0.01 },
      "data_format": { "x": "f32[]", "y": "f32[]" }
    },
    {
      "id": "sin_regression",
      "name": "正弦回归",
      "type": "regression",
      "description": "拟合 y = A·sin(ωx + φ) + b",
      "default_params": { "learning_rate": 0.01 },
      "data_format": { "x": "f32[]", "y": "f32[]" }
    },
    {
      "id": "rastrigin",
      "name": "Rastrigin 变体函数优化",
      "type": "genetic",
      "description": "遗传算法优化 f(x) = x² - 10cos(2πx-5) + 10",
      "default_params": { "population_size": 200, "mutation_rate": 0.05, "tournament_size": 5, "bounds": [0.0, 4.5] }
    },
    {
      "id": "ackley",
      "name": "Ackley 函数优化",
      "type": "genetic",
      "description": "遗传算法优化 f(x) = x² - 10cos(2πx) + 10",
      "default_params": { "population_size": 200, "mutation_rate": 0.01, "tournament_size": 5, "bounds": [-5.12, 5.12] }
    }
  ]
}
```

### 4.4 接口总览

| 接口 | 方法 | 角色 | 说明 |
|------|------|------|------|
| `/api/v1/train` | POST | 反向传播 | 创建训练任务，初始化模型 |
| `/api/v1/train/step` | POST | 反向传播 | 执行 N 步训练（同步），返回 loss |
| `/api/v1/train/status` | GET | 反向传播 | 查询累积训练历史 |
| `/api/v1/train/stop` | POST | 反向传播 | 停止训练，保留模型状态 |
| `/api/v1/inference` | GET | 正向传播 | 读取当前模型，计算渲染数据 |
| `/api/v1/health` | GET | 公共 | 健康检查 |
| `/api/v1/algorithms` | GET | 公共 | 算法列表及默认参数 |

### 4.5 前端完整调用流程

以线性回归为例，前端交互循环：

```
① 用户输入数据 [1,2,3,4,5] → [3,5,7,9,11]，设置 step_epochs=5
② POST /train → { taskId: "abc", state: "initialized" }
③ 第1轮: POST /train/step { taskId: "abc", epochs: 5 }  → loss 从 45.0 降到 12.3
④         GET /inference?taskId=abc → 拿到拟合曲线 → 绘图
⑤ 第2轮: POST /train/step { taskId: "abc", epochs: 5 }  → loss 从 12.3 降到 3.1
⑥         GET /inference?taskId=abc → 拿到拟合曲线 → 绘图（曲线更贴合数据点）
⑦ ...
⑧ 用户点"停止" → POST /train/stop
⑨ 最后 GET /inference → 展示最终拟合效果
```

**动画效果控制**：
- `step_epochs=1`：每轮只训练 1 步，拟合曲线逐帧"爬"向数据点，**动画丝滑**
- `step_epochs=20`：每轮训练 20 步，拟合曲线快速跳变，**快速到达结果**
- 前端可提供滑块让用户实时调节步长

## 五、项目结构

```
m-learn/example/rest-api/
├── Cargo.toml
└── src/
    ├── main.rs              # axum 服务启动、路由注册、CORS 配置
    ├── task_manager.rs      # 训练任务管理器（创建、步进、停止、查询）
    ├── routes/
    │   ├── mod.rs
    │   ├── train.rs         # 训练接口 handler（POST /train, POST /step, GET /status, POST /stop）
    │   └── inference.rs     # 推理接口 handler（GET /inference）
    └── models.rs            # 请求/响应结构体定义、算法枚举
```

### 各模块职责

| 文件 | 职责 |
|------|------|
| `main.rs` | 创建 Router，注册路由，配置 CORS，创建共享 TaskManager，绑定端口启动 |
| `task_manager.rs` | 核心模块。管理任务生命周期：创建模型 → 步进训练 → 查询状态 → 停止。持有 `Arc<Mutex<HashMap<String, TaskEntry>>>` 存储每个任务的模型状态和累积训练历史 |
| `models.rs` | 定义 AlgorithmType 枚举、TrainRequest、StepRequest、StepResponse、InferenceResponse 等 |
| `routes/train.rs` | 解析请求 → 调用 TaskManager 创建任务 / 执行步进 / 查询状态 / 停止 |
| `routes/inference.rs` | 根据 taskId 查找任务 → 读取当前模型状态 → 正向计算渲染数据 → 返回 JSON |

### TaskManager 设计

```rust
pub struct TaskManager {
    tasks: Arc<Mutex<HashMap<String, TaskEntry>>>,
}

struct TaskEntry {
    algorithm: AlgorithmType,
    state: TaskState,                    // Initialized / Running / Completed / Stopped
    model: ModelState,                   // 拟合: weights; 遗传: GA 实例
    training_data: Option<TrainingData>, // 拟合类的原始数据
    convergence: Vec<ConvergencePoint>,  // 累积训练历史（跨多次 step）
    total_trained: usize,               // 累积已训练步数
}

enum ModelState {
    Regression {
        weights: Tensor,
        gradient: Tensor,
        input: Tensor,
        target: Tensor,
    },
    Genetic {
        ga: GeneticAlgorithm,
        best_genes: Vec<f32>,
    },
}

impl TaskManager {
    fn create(&mut self, req: TrainRequest) -> String;           // 创建任务，返回 taskId
    fn step(&mut self, task_id: &str, epochs: usize) -> StepResult;  // 执行 N 步训练，同步返回
    fn get_status(&self, task_id: &str) -> TrainStatus;          // 查询累积历史
    fn get_inference(&self, task_id: &str) -> InferenceData;     // 正向计算渲染数据
    fn stop(&mut self, task_id: &str) -> Result<()>;             // 停止任务
}
```

> **关键**：`step()` 是同步方法，直接在 axum handler 中调用（或用 `spawn_blocking` 包裹），执行完 N 步后返回。不需要后台线程和 CancellationToken，因为训练节奏完全由前端控制。

## 六、关键技术决策

### 6.1 不引入 draw crate

原示例用 `draw` 渲染 SVG。REST API 不使用它：
- `draw` 依赖 `minifb`（桌面窗口）和 `plotters`，引入不必要的桌面依赖
- 前端已有图表组件，数据可视化由前端完成更灵活
- 后端只返回结构化 JSON，前后端分离

### 6.2 步进式训练架构

训练由前端控制节奏，后端 `step()` 是同步方法：

```rust
// TaskManager::step 的核心逻辑
fn step(&mut self, task_id: &str, epochs: usize) -> StepResult {
    let entry = self.tasks.get_mut(task_id).unwrap();
    let mut step_convergence = Vec::new();

    match &mut entry.model {
        ModelState::Regression { weights, gradient, input, target } => {
            for epoch in 0..epochs {
                // 反向传播一步
                let y_pred = formula::linear(input, weights);
                let loss = formula::loss::mse(&y_pred, target);
                *gradient = formula::loss::mse_gradient(&y_pred, target);
                formula::gradient_descent(weights, gradient, 0.01);
                step_convergence.push(ConvergencePoint { epoch: entry.total_trained + epoch + 1, loss });
            }
        }
        ModelState::Genetic { ga, best_genes } => {
            for gen in 0..epochs {
                // 遗传算法进化一代
                ga.step();
                let fitness = ga.best_fitness();
                *best_genes = ga.best_chromosome().to_vec();
                step_convergence.push(ConvergencePoint {
                    generation: entry.total_trained + gen + 1,
                    best_fitness: fitness,
                });
            }
        }
    }

    entry.total_trained += epochs;
    entry.convergence.extend(step_convergence.iter().cloned());
    StepResult { convergence: step_convergence, total_trained: entry.total_trained }
}
```

前端发起 step 请求 → 后端同步执行 N 步 → 返回 loss 数据 → 前端调 inference 拿渲染数据 → 绘图 → 再发 step → 循环。

**不需要** `CancellationToken` 和后台线程，因为训练完全由前端驱动。

### 6.3 spawn_blocking 包裹

虽然 step 是同步的，但步数较多时（如 `epochs: 1000`）会阻塞较久。用 `spawn_blocking` 包裹以不阻塞 tokio worker：

```rust
async fn step_handler(
    State(state): State<AppState>,
    Json(req): Json<StepRequest>,
) -> Result<Json<StepResponse>, AppError> {
    let mgr = state.task_manager.clone();
    let task_id = req.task_id.clone();
    let epochs = req.epochs;

    let result = tokio::task::spawn_blocking(move || {
        mgr.lock().unwrap().step(&task_id, epochs)
    }).await??;

    Ok(Json(result))
}
```

### 6.4 遗传算法的集成

`GeneticAlgorithm` 不支持中途获取种群快照。需要在每代 `step()` 后手动提取：

```rust
for gen in 0..generations {
    if cancel_token.is_cancelled() { break; }
    ga.step();
    let best = ga.best_chromosome();
    let best_fitness = ga.best_fitness();
    // 提取种群快照（取部分个体用于前端散点展示）
    let population_snapshot = extract_population_sample(&ga, 50);
    task_mgr.update_genetic_progress(task_id, gen, best, best_fitness, population_snapshot);
}
```

> 注：`GeneticAlgorithm` 的种群存储在内部 `Vec<f32>` 中，通过 `best_chromosome()` 和 `best_fitness()` 可以获取最优信息。种群快照需要通过公开方法或新增接口获取。

### 6.5 CORS 配置

```rust
use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);
```

### 6.6 端口分配

| 服务 | 端口 |
|------|------|
| professor (Python) | 8080 |
| way_find (Go) | 8081 |
| **m-learn (Rust)** | **8082** |
| show-web (Vite) | 5173 |

## 七、Makefile 集成

```makefile
# start 中新增：
@cd m-learn && cargo run --bin rest-api --release -- --port 8082 > /dev/null 2>&1 &

# stop 中新增：
@fuser -k 8082/tcp 2>/dev/null; echo ""

# status 中新增：
@(lsof -i :8082 >/dev/null 2>&1 && echo "  [RUNNING] m-learn   :8082" || echo "  [STOPPED] m-learn   :8082")
```

## 八、实现顺序

1. **创建项目骨架**：`Cargo.toml`、`main.rs`（空路由 + health + algorithms）
2. **定义数据模型**：`models.rs` 中所有 Request/Response 结构体、AlgorithmType 枚举
3. **实现 TaskManager**：任务创建（create）、步进训练（step）、状态查询（get_status）、推理（get_inference）、停止（stop）
4. **实现训练接口**：`routes/train.rs`（POST /train、POST /train/step、GET /train/status、POST /train/stop）
5. **实现推理接口**：`routes/inference.rs`（拟合曲线正向计算、函数景观正向计算）
6. **注册路由到 main.rs**，配置 CORS
7. **编译测试**：`cargo build`，用 curl 验证完整流程（创建任务 → 步进训练 → 获取推理数据 → 循环）
8. **更新 Makefile**
9. **前端对接**：在 show-web 中添加 m-learn 模块

## 九、前端对接思路（后续）

### 拟合页面交互流程

1. 用户输入数据点（手动输入 / 上传 CSV / 使用预设数据集）
2. 选择算法（线性回归 / 正弦回归），设置学习率
3. 调节**步长滑块**（step_epochs）：1 = 逐帧细腻动画，100 = 快速收敛
4. 点击"开始训练" → `POST /train` 获取 taskId
5. 循环：
   - `POST /train/step { epochs: step_epochs }` → 等待同步返回
   - `GET /inference` → 获取拟合曲线数据 → 更新图表（原始散点 + 拟合曲线 + loss 曲线）
6. 用户可随时调整步长（下一轮 step 使用新值）
7. 训练完成或用户点"停止" → 展示最终公式和权重

### 遗传算法页面交互流程

1. 选择目标函数（Rastrigin / Ackley），设置种群大小、变异率
2. 调节**步长滑块**：控制每轮进化几代
3. 点击"开始优化" → `POST /train` 获取 taskId
4. 循环：
   - `POST /train/step { epochs: step_epochs }` → 等待同步返回
   - `GET /inference` → 获取种群分布 + 函数景观 → 更新图表
5. 用户可直观观察种群逐步聚集到最优解的过程
6. 优化完成或用户点"停止" → 展示最终最优解

### 图表渲染方案

| 数据 | 图表类型 | 前端库 |
|------|----------|--------|
| 拟合散点 + 拟合曲线 | 散点 + 折线 | ECharts / Chart.js |
| loss 收敛曲线 | 折线图 | ECharts / Chart.js |
| 函数景观 + 种群散点 | 曲线 + 散点叠加 | ECharts / Chart.js |
| 适应度收敛曲线 | 折线图（best + avg） | ECharts / Chart.js |
