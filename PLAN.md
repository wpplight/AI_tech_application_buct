# AI 作业展示系统 - 项目改造计划

## 📋 项目概述

本计划旨在将三个计算机人工智能应用作业改造为统一的 REST API 后端，并集成到 show-web 展示网站中呈现。

### 三个作业项目现状

| 项目 | 技术栈 | 现有接口 | 改造难度 |
|------|--------|----------|----------|
| **m-learn** | Rust | ❌ 无 REST API（仅命令行示例） | ⭐⭐⭐⭐⭐ 高 |
| **professor** | Python | ✅ 简单 HTTP API（需标准化） | ⭐⭐⭐ 中 |
| **way_find** | Go + Wails | ⚠️ Wails RPC（需转换为 REST） | ⭐⭐⭐⭐ 高 |

---

## 🎯 总体架构设计

```
┌─────────────────────────────────────────────────────────────┐
│                    show-web (Vue 3 前端)                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │  m-learn    │  │  professor  │  │  way_find   │        │
│  │  展示组件    │  │  展示组件    │  │  展示组件   │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ HTTP/REST API
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     API Gateway (可选)                       │
│  统一入口、负载均衡、接口聚合、认证授权                       │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
┌───────────────┐    ┌───────────────┐    ┌───────────────┐
│  m-learn API  │    │ professor API │    │ way_find API  │
│  (Rust/Actix) │    │ (Python/FastAPI)│   │  (Go/Chi)     │
└───────────────┘    └───────────────┘    └───────────────┘
        │                     │                     │
        ▼                     ▼                     ▼
┌───────────────┐    ┌───────────────┐    ┌───────────────┐
│  Rust 核心库  │    │  Python 算法  │    │  Go 算法库    │
│  - 神经网络   │    │  - 专家系统    │    │  - 寻路算法   │
│  - 遗传算法   │    │  - 推理引擎    │    │  - 地图管理   │
│  - 可视化    │    │  - 知识库      │    │               │
└───────────────┘    └───────────────┘    └───────────────┘
```

---

## 📐 REST API 接口规范

### 1. m-learn REST API 设计

#### 1.1 基础信息
- **基础路径**: `/api/ml`
- **端口**: `8081`
- **框架**: Actix-web (Rust)
- **数据格式**: JSON

#### 1.2 接口列表

##### 神经网络模块 (`/api/ml/nn`)

```yaml
POST /api/ml/nn/train:
  description: 训练神经网络模型
  request:
    {
      "input_dim": 1,
      "hidden_dim": 64,
      "output_dim": 1,
      "layers": ["Linear", "ReLU", "Linear"],
      "epochs": 1000,
      "learning_rate": 0.001,
      "train_data": [
        {"input": [0.1], "target": [0.2]},
        {"input": [0.5], "target": [0.8]}
      ]
    }
  response:
    {
      "success": true,
      "model_id": "nn_20260506_001",
      "final_loss": 0.0234,
      "training_time_ms": 1523,
      "metrics": {
        "loss_history": [0.45, 0.32, 0.28, ...]
      }
    }

POST /api/ml/nn/predict:
  description: 使用训练好的模型进行预测
  request:
    {
      "model_id": "nn_20260506_001",
      "inputs": [[0.3], [0.7]]
    }
  response:
    {
      "success": true,
      "predictions": [[0.45], [0.92]]
    }

GET /api/ml/nn/models:
  description: 获取所有已保存的模型列表
  response:
    {
      "models": [
        {
          "id": "nn_20260506_001",
          "architecture": "1-64-1",
          "created_at": "2026-05-06T10:30:00Z",
          "final_loss": 0.0234
        }
      ]
    }
```

##### 遗传算法模块 (`/api/ml/ga`)

```yaml
POST /api/ml/ga/optimize:
  description: 运行遗传算法优化
  request:
    {
      "problem_type": "regression",
      "objective_function": "sphere",
      "dimension": 2,
      "bounds": [-50, 50],
      "population_size": 200,
      "generations": 500,
      "crossover_type": "sbx",
      "mutation_type": "polynomial",
      "elite_protect": true
    }
  response:
    {
      "success": true,
      "job_id": "ga_20260506_001",
      "best_solution": [0.0123, -0.0234],
      "best_fitness": 0.0001,
      "convergence": {
        "generations": 500,
        "fitness_history": [12.5, 10.2, 8.7, ...],
        "execution_time_ms": 2341
      }
    }

GET /api/ml/ga/status/{job_id}:
  description: 查询优化任务状态
  response:
    {
      "job_id": "ga_20260506_001",
      "status": "running|completed|failed",
      "progress": 0.75,
      "current_best": [0.5, -0.3],
      "current_fitness": 0.05
    }
```

##### 可视化模块 (`/api/ml/viz`)

```yaml
POST /api/ml/viz/plot:
  description: 生成可视化图表
  request:
    {
      "chart_type": "line|scatter|heatmap|surface",
      "title": "遗传算法收敛曲线",
      "data": {
        "x": [1, 2, 3, ...],
        "y": [0.5, 0.3, 0.1, ...],
        "labels": ["种群1", "种群2"]
      },
      "options": {
        "xlabel": "代数",
        "ylabel": "适应度",
        "export_format": "svg|png"
      }
    }
  response:
    {
      "success": true,
      "chart_id": "viz_20260506_001",
      "image_url": "/api/ml/viz/image/viz_20260506_001.svg",
      "base64_data": "..."
    }

GET /api/ml/viz/image/{chart_id}:
  description: 获取生成的图表文件
  response: 图像文件 (SVG/PNG)
```

---

### 2. professor REST API 设计

#### 2.1 基础信息
- **基础路径**: `/api/expert`
- **端口**: `8082`
- **框架**: FastAPI (Python)
- **数据格式**: JSON

#### 2.2 接口列表

##### 知识库管理 (`/api/expert/knowledge`)

```yaml
GET /api/expert/knowledge/rules:
  description: 获取所有规则列表
  query_params:
    - search: str (可选，搜索关键词)
    - page: int (分页页码)
    - page_size: int (每页数量)
  response:
    {
      "total": 610,
      "page": 1,
      "page_size": 20,
      "rules": [
        {
          "id": 1,
          "conditions": ["毛发", "哺乳动物"],
          "conclusion": "是哺乳动物",
          "confidence": 1.0
        }
      ]
    }

POST /api/expert/knowledge/rules:
  description: 添加新规则
  request:
    {
      "conditions": ["羽毛", "恒温"],
      "conclusion": "是鸟类"
    }
  response:
    {
      "success": true,
      "rule_id": 611,
      "message": "规则添加成功"
    }

DELETE /api/expert/knowledge/rules/{rule_id}:
  description: 删除指定规则
  response:
    {
      "success": true,
      "message": "规则已删除"
    }
```

##### 事实库管理 (`/api/expert/facts`)

```yaml
GET /api/expert/facts:
  description: 获取当前事实库中的所有事实
  response:
    {
      "facts": ["毛发", "恒温", "哺乳动物", ...]
    }

POST /api/expert/facts:
  description: 添加事实到事实库
  request:
    {
      "facts": ["毛发", "恒温", "四肢"]
    }
  response:
    {
      "success": true,
      "added_count": 3,
      "current_facts": ["毛发", "恒温", "四肢", ...]
    }

DELETE /api/expert/facts:
  description: 清空事实库
  response:
    {
      "success": true,
      "message": "事实库已清空"
    }
```

##### 推理引擎 (`/api/expert/inference`)

```yaml
POST /api/expert/inference/forward:
  description: 执行正向推理
  request:
    {
      "algorithm": "fullscan|incremental|rete",
      "facts": ["毛发", "恒温", "四肢", "胎生"],
      "max_iterations": 100
    }
  response:
    {
      "success": true,
      "algorithm": "incremental",
      "inferred_facts": ["哺乳动物", "脊椎动物", ...],
      "inference_steps": [
        {
          "iteration": 1,
          "new_fact": "哺乳动物",
          "triggered_rule_id": 15,
          "reasoning": "因为 [毛发, 恒温, 胎生] 满足规则 #15"
        }
      ],
      "execution_time_ms": 12.5,
      "rules_checked": 610
    }

POST /api/expert/inference/backward:
  description: 执行反向推理（验证目标）
  request:
      {
        "algorithm": "fullscan|incremental|rete",
        "goal": "是猫科动物",
        "facts": ["毛发", "恒温", "四肢", "胎生", "食肉"]
      }
  response:
    {
      "success": true,
      "goal_verified": true,
      "proof_tree": {
        "goal": "是猫科动物",
        "subgoals": ["是食肉目", "有小犬齿"],
        "derived_facts": ["毛发", "恒温", ...]
      },
      "reasoning_path": ["事实[毛发,恒温] → 推导[哺乳动物] → ... → 结论[是猫科动物]"]
    }

GET /api/expert/inference/compare:
  description: 算法性能对比
  query_params:
    - rule_count: int (规则数量，默认610)
    - fact_count: int (事实数量，默认10)
    - iterations: int (测试轮次，默认30)
  response:
    {
      "fullscan": {
        "avg_time_ms": 92.29,
        "min_time_ms": 85.1,
        "max_time_ms": 105.3
      },
      "incremental": {
        "avg_time_ms": 13.43,
        "min_time_ms": 10.2,
        "max_time_ms": 18.7
      },
      "rete": {
        "avg_time_ms": 149.66,
        "min_time_ms": 140.2,
        "max_time_ms": 165.1
      },
      "recommendation": "incremental"
    }
```

---

### 3. way_find REST API 设计

#### 3.1 基础信息
- **基础路径**: `/api/pathfinding`
- **端口**: `8083`
- **框架**: Chi (Go) 或独立的 HTTP 服务
- **数据格式**: JSON

#### 3.2 接口列表

##### 地图管理 (`/api/pathfinding/maps`)

```yaml
POST /api/pathfinding/maps:
  description: 创建新地图
  request:
    {
      "width": 20,
      "height": 15,
      "obstacles": [[3,3], [3,4], [3,5]]
    }
  response:
    {
      "success": true,
      "map_id": "map_20260506_001",
      "map_data": {
        "width": 20,
        "height": 15,
        "start": {"x": 1, "y": 1},
        "end": {"x": 18, "y": 13},
        "grid": [[0,0,0,...], ...]
      }
    }

GET /api/pathfinding/maps/{map_id}:
  description: 获取指定地图
  response:
    {
      "map_id": "map_20260506_001",
      "map_data": {...}
    }

PUT /api/pathfinding/maps/{map_id}/cells:
  description: 更新地图单元格
  request:
    {
      "cells": [
        {"x": 5, "y": 5, "type": "wall"},
        {"x": 10, "y": 10, "type": "start"}
      ]
    }
  response:
    {
      "success": true,
      "updated_count": 2
    }
```

##### 路径搜索 (`/api/pathfinding/search`)

```yaml
POST /api/pathfinding/search:
  description: 初始化搜索算法
  request:
    {
      "map_id": "map_20260506_001",
      "algorithm": "bfs|dfs|astar"
    }
  response:
    {
      "success": true,
      "search_id": "search_20260506_001",
      "algorithm": "astar",
      "initialized": true
    }

POST /api/pathfinding/search/{search_id}/step:
  description: 执行单步搜索
  response:
    {
      "success": true,
      "state": "running|found|not_found",
      "current": {"x": 5, "y": 5},
      "visited": [{"x": 1, "y": 1}, {"x": 2, "y": 1}, ...],
      "neighbors": [{"x": 3, "y": 1}, {"x": 2, "y": 2}],
      "added": [{"x": 3, "y": 1}],
      "expanded_count": 15,
      "distance": 8
    }

POST /api/pathfinding/search/{search_id}/run:
  description: 执行完整搜索（一步到位）
  response:
    {
      "success": true,
      "found": true,
      "path": [{"x": 1, "y": 1}, {"x": 2, "y": 1}, ...],
      "distance": 28,
      "expanded_count": 156,
      "execution_time_ms": 2.3,
      "algorithm": "astar"
    }

GET /api/pathfinding/search/{search_id}/history:
  description: 获取搜索过程历史
  response:
    {
      "search_id": "search_20260506_001",
      "algorithm": "astar",
      "total_steps": 156,
      "history": [
        {
          "step": 1,
          "state": "running",
          "current": {"x": 1, "y": 1},
          "visited_count": 1,
          "distance": 0
        },
        ...
      ]
    }
```

##### 算法对比 (`/api/pathfinding/compare`)

```yaml
POST /api/pathfinding/compare:
  description: 对比三种算法在同一地图上的表现
  request:
    {
      "map_id": "map_20260506_001"
    }
  response:
    {
      "map_id": "map_20260506_001",
      "results": {
        "dfs": {
          "found": true,
          "path_length": 45,
          "expanded_count": 67,
          "execution_time_ms": 1.2,
          "optimal": false
        },
        "bfs": {
          "found": true,
          "path_length": 28,
          "expanded_count": 156,
          "execution_time_ms": 2.3,
          "optimal": true
        },
        "astar": {
          "found": true,
          "path_length": 28,
          "expanded_count": 89,
          "execution_time_ms": 1.8,
          "optimal": true
        }
      },
      "recommendation": "astar (最短路径 + 最少扩展节点)"
    }
```

---

## 🔄 后端改造详细计划

### 阶段一：m-learn REST API 开发（4-5天）

#### 目标
为 Rust 机器学习库开发独立的 REST API 服务，不修改原有核心库代码。

#### 技术选型
- **HTTP 框架**: Actix-web 4.x
- **序列化**: Serde JSON
- **异步运行时**: Tokio
- **构建工具**: Cargo

#### 目录结构设计
```
m-learn-api/
├── Cargo.toml
├── src/
│   ├── main.rs              # 应用入口
│   ├── api/
│   │   ├── mod.rs
│   │   ├── nn.rs           # 神经网络 API 路由
│   │   ├── ga.rs           # 遗传算法 API 路由
│   │   └── viz.rs          # 可视化 API 路由
│   ├── services/
│   │   ├── mod.rs
│   │   ├── nn_service.rs   # 神经网络服务
│   │   ├── ga_service.rs   # 遗传算法服务
│   │   └── viz_service.rs  # 可视化服务
│   ├── models/
│   │   ├── mod.rs
│   │   ├── nn.rs           # NN 数据模型
│   │   ├── ga.rs           # GA 数据模型
│   │   └── viz.rs          # 可视化数据模型
│   └── core/
│       └── wrapper.rs      # Rust 核心库封装层
├── src/bin/
│   └── server.rs           # 可执行服务器
└── Dockerfile
```

#### 关键实现点

1. **核心库封装层**
   - 创建 Rust 核心库的 wrapper 模块
   - 处理所有权和生命周期问题
   - 提供线程安全的调用接口

2. **神经网络服务**
   - 实现模型训练 API
   - 实现模型预测 API
   - 管理模型生命周期（保存/加载）
   - 返回训练指标和收敛曲线

3. **遗传算法服务**
   - 实现优化任务提交
   - 实现异步任务状态查询
   - 支持多种基准测试函数
   - 返回优化结果和收敛数据

4. **可视化服务**
   - 集成现有的 draw crate
   - 支持图表生成和导出
   - 提供 Base64 编码的图片数据

#### 改造任务清单

- [ ] 创建 m-learn-api 项目结构
- [ ] 配置 Cargo.toml 依赖
- [ ] 实现核心库封装层
- [ ] 开发神经网络 REST API
- [ ] 开发遗传算法 REST API
- [ ] 开发可视化 REST API
- [ ] 编写 API 文档（OpenAPI/Swagger）
- [ ] 添加单元测试和集成测试
- [ ] 编写 Dockerfile

---

### 阶段二：professor API 标准化（2-3天）

#### 目标
将现有的简单 HTTP API 升级为标准化的 REST API，使用 FastAPI 框架重构。

#### 技术选型
- **HTTP 框架**: FastAPI
- **数据验证**: Pydantic
- **文档生成**: 自动 OpenAPI
- **ASGI 服务器**: Uvicorn

#### 目录结构设计
```
professor-api/
├── main.py                  # FastAPI 应用入口
├── routers/
│   ├── __init__.py
│   ├── knowledge.py         # 知识库路由
│   ├── facts.py             # 事实库路由
│   └── inference.py         # 推理引擎路由
├── services/
│   ├── __init__.py
│   ├── knowledge_service.py
│   ├── facts_service.py
│   └── inference_service.py
├── models/
│   ├── __init__.py
│   ├── rule.py
│   ├── fact.py
│   └── inference.py
├── algorithms/
│   ├── fullscan.py          # 引用原 fullscan_py
│   ├── incremental.py       # 引用原 incremental_py
│   └── rete.py              # 引用原 rete_py
├── requirements.txt
└── Dockerfile
```

#### 改造任务清单

- [ ] 创建 FastAPI 项目结构
- [ ] 定义 Pydantic 数据模型
- [ ] 重构知识库管理 API（GET/POST/DELETE）
- [ ] 重构事实库管理 API
- [ ] 重构推理引擎 API（正向/反向）
- [ ] 添加算法性能对比 API
- [ ] 保留原有算法实现（不做修改）
- [ ] 添加 API 版本控制（v1）
- [ ] 自动生成 OpenAPI 文档
- [ ] 编写 Dockerfile

#### 现有接口映射

| 现有接口 | 新接口 | 变化 |
|----------|--------|------|
| GET /api/rules | GET /api/v1/expert/knowledge/rules | 标准化响应格式 |
| POST /api/rules/add | POST /api/v1/expert/knowledge/rules | 使用 Pydantic 验证 |
| GET /api/facts | GET /api/v1/expert/facts | 标准化响应格式 |
| POST /api/inference/start | POST /api/v1/expert/inference/forward | 区分 forward/backward |

---

### 阶段三：way_find REST API 开发（3-4天）

#### 目标
将 Wails RPC 接口转换为独立的 REST API，保持 Go 核心算法不变。

#### 技术选型
- **HTTP 框架**: Chi（轻量级）
- **序列化**: encoding/json
- **路由**: Chi Router
- **CORS**: 自定义中间件

#### 目录结构设计
```
wayfind-api/
├── main.go                  # 应用入口
├── handlers/
│   ├── map_handler.go       # 地图管理
│   ├── search_handler.go    # 搜索操作
│   └── compare_handler.go   # 算法对比
├── services/
│   ├── map_service.go
│   ├── search_service.go
│   └── compare_service.go
├── models/
│   ├── map.go
│   ├── search.go
│   └── result.go
├── algorithms/              # 引用原 backend/algorithms
├── map/                     # 引用原 backend/map
├── path/                    # 引用原 backend/path
├── queue/                   # 引用原 backend/queue
├── go.mod
├── go.sum
└── Dockerfile
```

#### 改造任务清单

- [ ] 创建 Go HTTP API 项目
- [ ] 迁移算法模块（从 backend/algorithms）
- [ ] 迁移数据结构（从 backend/map, path, queue）
- [ ] 实现地图管理 REST API
- [ ] 实现搜索操作 REST API
- [ ] 实现算法对比 REST API
- [ ] 添加请求验证
- [ ] 添加 CORS 支持
- [ ] 编写单元测试
- [ ] 编写 Dockerfile

#### Wails RPC 到 REST 的映射

| Wails 方法 | REST 接口 | HTTP 方法 |
|------------|-----------|-----------|
| CreateMap() | POST /api/pathfinding/maps | POST |
| LoadMap() | PUT /api/pathfinding/maps/{id} | PUT |
| SetCell() | PUT /api/pathfinding/maps/{id}/cells | PUT |
| InitializeSearch() | POST /api/pathfinding/search | POST |
| Step() | POST /api/pathfinding/search/{id}/step | POST |
| GetResult() | GET /api/pathfinding/search/{id}/result | GET |

---

## 🌐 show-web 展示网站开发计划

### 技术栈
- **前端框架**: Vue 3 (Composition API)
- **构建工具**: Vite
- **路由**: Vue Router 5
- **状态管理**: Pinia
- **HTTP 客户端**: Axios
- **样式**: CSS3 + CSS Variables

### 页面结构设计

```
show-web/
├── src/
│   ├── views/
│   │   ├── HomeView.vue           # 首页/概览
│   │   ├── MLearnView.vue          # m-learn 展示页
│   │   ├── ProfessorView.vue      # professor 展示页
│   │   └── WayFindView.vue         # way_find 展示页
│   ├── components/
│   │   ├── common/
│   │   │   ├── AppHeader.vue       # 公共头部
│   │   │   ├── LoadingSpinner.vue  # 加载组件
│   │   │   └── ErrorMessage.vue    # 错误提示
│   │   ├── mlearn/
│   │   │   ├── NNTrainingPanel.vue     # 神经网络训练面板
│   │   │   ├── GAParametersForm.vue    # 遗传算法参数表单
│   │   │   └── ConvergenceChart.vue   # 收敛曲线图表
│   │   ├── professor/
│   │   │   ├── KnowledgeBasePanel.vue   # 知识库管理面板
│   │   │   ├── FactBasePanel.vue       # 事实库管理面板
│   │   │   ├── InferencePanel.vue      # 推理引擎面板
│   │   │   └── RuleVisualizer.vue      # 规则可视化
│   │   └── wayfind/
│   │       ├── MapEditor.vue           # 地图编辑器
│   │       ├── SearchVisualizer.vue     # 搜索可视化
│   │       └── AlgorithmCompare.vue    # 算法对比
│   ├── services/
│   │   ├── api.js                   # API 基础配置
│   │   ├── mlearn.js                # m-learn API 调用
│   │   ├── professor.js            # professor API 调用
│   │   └── wayfind.js              # way_find API 调用
│   ├── stores/
│   │   ├── mlearn.js               # m-learn 状态管理
│   │   ├── professor.js            # professor 状态管理
│   │   └── wayfind.js              # way_find 状态管理
│   └── router/
│       └── index.ts                # 路由配置
```

### 各模块展示功能设计

#### 1. m-learn 展示模块

**功能页面**：
- 神经网络训练演示（参数配置 → 训练 → 结果展示）
- 遗传算法优化演示（选择基准函数 → 设置参数 → 优化过程）
- 可视化图表展示（收敛曲线、优化轨迹）

**关键组件**：
- `NNTrainingPanel.vue`: 配置网络结构、训练参数，执行训练
- `GAParametersForm.vue`: 选择优化问题、配置 GA 参数
- `ConvergenceChart.vue`: 实时显示训练/优化收敛曲线
- `VisualizationGallery.vue`: 展示生成的图表

**API 调用示例**：
```javascript
// 训练神经网络
const response = await mlearnService.trainNN({
  input_dim: 1,
  hidden_dim: 64,
  output_dim: 1,
  layers: ['Linear', 'ReLU', 'Linear'],
  epochs: 1000,
  learning_rate: 0.001,
  train_data: trainingData
});

// 显示收敛曲线
convergenceChart.render(response.metrics.loss_history);
```

#### 2. professor 展示模块

**功能页面**：
- 知识库浏览与搜索
- 事实库管理
- 正向/反向推理演示
- 算法性能对比

**关键组件**：
- `KnowledgeBasePanel.vue`: 显示规则列表，支持搜索
- `FactBasePanel.vue`: 添加/删除事实
- `InferencePanel.vue`: 配置推理参数，执行推理
- `RuleVisualizer.vue`: 可视化推理过程和推理树
- `AlgorithmCompare.vue`: 对比三种算法的性能

**API 调用示例**：
```javascript
// 添加事实并执行正向推理
await professorService.addFacts(['毛发', '恒温', '四肢']);
const result = await professorService.forwardInference({
  algorithm: 'incremental',
  max_iterations: 100
});

// 可视化推理步骤
ruleVisualizer.render(result.inference_steps);
```

#### 3. way_find 展示模块

**功能页面**：
- 交互式地图编辑器
- 寻路算法可视化（单步执行/完整执行）
- 算法性能对比

**关键组件**：
- `MapEditor.vue`: 点击编辑地图，设置起点终点
- `SearchVisualizer.vue`: 实时显示搜索过程
- `AlgorithmCompare.vue`: 同时显示三种算法的结果
- `PathStatistics.vue`: 显示路径长度、扩展节点数等统计

**API 调用示例**：
```javascript
// 创建地图
await wayfindService.createMap(20, 15);

// 初始化搜索
await wayfindService.initSearch('astar');

// 单步执行搜索
const step = await wayfindService.step();
searchVisualizer.render(step);

// 完整搜索
const result = await wayfindService.runSearch();
searchVisualizer.renderPath(result.path);
```

### 展示网站改造任务清单

#### 基础架构（1天）
- [ ] 安装必要依赖（axios, chart.js/vue-chartjs）
- [ ] 配置 API 基础服务
- [ ] 设置路由结构
- [ ] 创建 Pinia store

#### 公共组件（1天）
- [ ] AppHeader 组件（导航）
- [ ] LoadingSpinner 组件
- [ ] ErrorMessage 组件
- [ ] API 错误处理封装

#### m-learn 展示页面（2天）
- [ ] MLearnView 主页面布局
- [ ] 神经网络训练面板
- [ ] 遗传算法优化面板
- [ ] 收敛曲线图表组件
- [ ] API 集成测试

#### professor 展示页面（2天）
- [ ] ProfessorView 主页面布局
- [ ] 知识库管理面板
- [ ] 事实库管理面板
- [ ] 推理引擎面板
- [ ] 算法对比面板
- [ ] API 集成测试

#### way_find 展示页面（2天）
- [ ] WayFindView 主页面布局
- [ ] 地图编辑器组件
- [ ] 搜索可视化组件
- [ ] 路径统计组件
- [ ] 算法对比组件
- [ ] API 集成测试

#### 整体集成（1天）
- [ ] 首页设计（项目概览卡片）
- [ ] 样式美化与响应式设计
- [ ] 动画与过渡效果
- [ ] 错误处理完善
- [ ] 性能优化

---

## 📅 整体实施时间表

```
Week 1:
├── Day 1-2: m-learn API 架构设计与核心封装
├── Day 3-4: m-learn REST API 实现
└── Day 5: m-learn API 测试与文档

Week 2:
├── Day 1-2: professor API 重构
├── Day 3-4: way_find API 开发
└── Day 5: API 部署与集成测试

Week 3:
├── Day 1-2: show-web 基础架构与公共组件
├── Day 3-4: show-web 各模块页面开发
└── Day 5: show-web 集成与美化

Week 4:
├── Day 1-2: 整体联调与问题修复
├── Day 3-4: 功能完善与性能优化
└── Day 5: 最终测试与交付
```

**总工期**: 约 4 周

---

## 🐳 部署方案

### Docker 容器化

为每个 API 服务和前端创建 Dockerfile：

```dockerfile
# m-learn-api/Dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY m-learn/ ./m-learn/
COPY m-learn-api/ ./m-learn-api/
RUN cd m-learn-api && cargo build --release
EXPOSE 8081

# professor-api/Dockerfile
FROM python:3.11-slim
WORKDIR /app
COPY professor/ ./professor/
COPY professor-api/ ./professor-api/
RUN pip install -r professor-api/requirements.txt
EXPOSE 8082

# wayfind-api/Dockerfile
FROM golang:1.21 as builder
WORKDIR /app
COPY way_find/backend/ ./backend/
COPY wayfind-api/ ./wayfind-api/
RUN cd wayfind-api && go build -o server .
EXPOSE 8083

# show-web/Dockerfile
FROM node:20-alpine as builder
WORKDIR /app
COPY show-web/ ./show-web/
RUN npm install && npm run build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
```

### docker-compose 编排

```yaml
version: '3.8'
services:
  mlearn-api:
    build: ./m-learn-api
    ports:
      - "8081:8081"
    networks:
      - ai-showcase

  professor-api:
    build: ./professor-api
    ports:
      - "8082:8082"
    networks:
      - ai-showcase

  wayfind-api:
    build: ./wayfind-api
    ports:
      - "8083:8083"
    networks:
      - ai-showcase

  show-web:
    build: ./show-web
    ports:
      - "80:80"
    depends_on:
      - mlearn-api
      - professor-api
      - wayfind-api
    networks:
      - ai-showcase

networks:
  ai-showcase:
    driver: bridge
```

---

## ✅ 验收标准

### 功能验收
- [ ] 三个 API 服务都能正常启动
- [ ] 所有 REST API 接口都能正确响应
- [ ] show-web 能成功调用所有后端 API
- [ ] 神经网络训练能正常执行并返回结果
- [ ] 遗传算法优化能正常执行并显示收敛曲线
- [ ] 专家系统推理能正确执行正向/反向推理
- [ ] 寻路算法能在地图上正确找到路径
- [ ] 搜索过程可视化能实时显示

### 性能验收
- [ ] API 响应时间 < 500ms（单次推理/搜索）
- [ ] 神经网络训练 < 10s（1000 轮）
- [ ] 遗传算法 < 5s（500 代）
- [ ] 前端页面加载 < 2s
- [ ] 动画流畅度 60fps

### 代码质量
- [ ] 代码符合各自语言的编码规范
- [ ] API 有完整的文档（OpenAPI）
- [ ] 有单元测试覆盖关键功能
- [ ] 无明显的安全漏洞
- [ ] 错误处理完善

---

## 🎯 后续扩展建议

1. **API Gateway**: 引入 Kong 或 NGINX 作为统一入口
2. **认证授权**: 添加 JWT 认证机制
3. **缓存层**: Redis 缓存常用查询结果
4. **监控**: Prometheus + Grafana 监控 API 性能
5. **日志**: ELK Stack 集中式日志管理
6. **数据库**: 持久化存储训练模型和搜索历史
7. **实时通信**: WebSocket 支持实时搜索可视化

---

## 📞 联系与支持

如有问题或建议，请参考各项目目录下的 README 文件或提交 Issue。

---

**文档版本**: 1.0  
**创建日期**: 2026-05-06  
**最后更新**: 2026-05-06
