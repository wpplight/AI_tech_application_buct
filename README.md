# AI 技术应用展示平台

一个综合性的 AI 算法可视化与演示平台，包含专家系统、寻路算法和机器学习三大模块。

## 项目结构

```
AI_tech_application_buct/
├── professor/          # 专家系统 - 动物识别
├── show-web/          # 前端可视化界面 (Vue 3)
├── way_find/          # 寻路算法可视化
└── m-learn/          # Rust 机器学习库
```

## 模块介绍

### 1. 专家系统 (professor/)

基于 Python 的动物识别专家系统，实现了三种前向推理算法：

- **全扫描 (Full Scan)** - 每次变更遍历所有规则
- **增量触发 (Incremental Triggering)** - 仅触发相关规则
- **Rete 网络** - 基于 Rete 算法的高效模式匹配

**技术栈：** Python + Flask + SQLite + Vue 3

### 2. 寻路算法 (way_find/)

迷宫寻路可视化系统，支持多种搜索算法：

- **BFS** - 广度优先搜索
- **DFS** - 深度优先搜索
- **A\*** - 启发式搜索

**功能特点：**
- 三层架构：地图管理 → 任务管理 → 寻路推导
- 地图编辑器：支持绘制障碍物、起点、终点
- 任务系统：基于地图创建任务，支持独立编辑和搜索
- 逐步可视化：可调节速度的动画演示

**技术栈：** Go + Gin + Vue 3

### 3. 机器学习库 (m-learn/)

Rust 实现的机器学习基础库：

- **Tensor** - 张量运算
- **NN** - 神经网络模块
- **Genetic** - 遗传算法
- **Optimizer** - 优化器

**技术栈：** Rust + Cargo

## 快速开始

### 前端 (show-web)

```bash
cd show-web
npm install
npm run dev
```

访问 `http://localhost:5173`

### 寻路算法后端

```bash
cd way_find/backend
go build -o wayfind-api .
./wayfind-api -port 8081
```

### 专家系统后端

```bash
cd professor
python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate
pip install -r requirements.txt
python app.py
```

### 机器学习库

```bash
cd m-learn
cargo build --release
```

## 技术特性

- **专家系统**：前向/反向推理、知识库持久化、性能对比实验
- **寻路算法**：任务隔离、逐步执行、速度控制、实时统计
- **前端界面**：Vue 3 + TypeScript + Pinia + 组件化设计
- **后端架构**：RESTful API、微服务风格、分层设计
