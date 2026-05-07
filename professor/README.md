# 动物识别专家系统

> Python 环境下三种前向推理算法（全扫描、增量触发、Rete）的完整实现与性能对比实验。
> 基于 Wikipedia 真实生物分类学数据（610 条规则），通过控制变量法进行严格的算法性能评估。

---

## 项目概述

本项目实现了一个完整的动物识别专家系统，涵盖知识库管理、事实库管理、推理引擎三大核心模块。系统提供三种不同复杂度的推理算法实现，每种算法均配有独立的 Web 应用界面，支持正向推理（Forward Chaining）和反向推理（Backward Chaining）。

### 核心特性

- **三种推理算法**：全扫描（Full Scan）、增量触发（Incremental Triggering）、Rete 网络
- **共享知识库**：610 条 Wikipedia 生物分类规则，SQLite 持久化存储
- **Web 交互界面**：每个算法独立部署，深色主题 UI，支持推理过程可视化
- **性能基准测试**：控制变量法对比，自动生成图表
- **正向 + 反向推理**：支持数据驱动和目标驱动两种推理模式

---

## 项目结构

```
Expert_System/
├── knowledge/                       # 共享知识库
│   ├── wikipedia_rules.py          # Wikipedia 610 条规则（纯 Python 数据源，供 benchmark 使用）
│   ├── rules.db                    # 共享 SQLite 数据库（三个 Web 应用共用）
│   └── __init__.py
│
├── algorithms/                      # 三种算法薄封装（供 benchmark 调用）
│   ├── fullscan.py                 # 全扫描算法封装
│   ├── incremental.py              # 增量触发算法封装
│   ├── rete.py                     # Rete 网络算法封装
│   └── __init__.py
│
├── benchmark/                       # 性能对比
│   ├── compare.py                  # 控制变量法对比三种算法
│   ├── chart_generator.py          # 图表生成器（matplotlib）
│   └── charts/                     # 生成的对比图表
│       ├── line.png                # 折线图
│       ├── bar.png                 # 柱状图
│       └── summary.png             # 汇总图
│
├── fullscan_py/                     # 全扫描算法 Web 应用（端口 8080）
│   ├── web_server.py               # HTTP 服务器 + 嵌入式前端（HTML/JS/CSS）
│   ├── inference_engine.py         # 推理引擎（正向 + 反向推理）
│   ├── knowledge_base.py           # 知识库管理（SQLite CRUD，指向 knowledge/rules.db）
│   ├── fact_base.py                # 事实库（内存 set）
│   └── README.md
│
├── incremental_py/                  # 增量触发算法 Web 应用（端口 8081）
│   ├── web_server.py               # HTTP 服务器 + 嵌入式前端
│   ├── inference_engine.py         # 推理引擎（条件索引 + 增量触发）
│   ├── knowledge_base.py           # 知识库管理（SQLite，指向 knowledge/rules.db）
│   ├── fact_base.py                # 事实库（内存 set）
│   └── README.md
│
├── rete_py/                         # Rete 网络算法 Web 应用（端口 8082）
│   ├── web_server.py               # HTTP 服务器 + 嵌入式前端
│   ├── rete_network.py             # Rete 网络核心（Alpha/Beta/Terminal 节点）
│   ├── rete_runner.py              # Rete 推理执行器（正向 + 反向推理）
│   ├── knowledge_base.py           # 知识库管理（SQLite，指向 knowledge/rules.db）
│   └── README.md
│
└── README.md                        # 本文件
```

---

## 快速开始

### 环境要求

- Python 3.8+（无第三方依赖，仅使用标准库）
- matplotlib（仅图表生成需要）

### 启动 Web 应用

```bash
# 全扫描算法（端口 8080）
cd fullscan_py && python3 web_server.py

# 增量触发算法（端口 8081）
cd incremental_py && python3 web_server.py

# Rete 网络算法（端口 8082）
cd rete_py && python3 web_server.py
```

启动后在浏览器中访问对应端口即可。

### 运行性能对比

```bash
# 三种算法性能对比（控制变量法）
python3 benchmark/compare.py

# 生成可视化图表
python3 benchmark/chart_generator.py
```

---

## 架构设计

### 知识库架构

```
knowledge/wikipedia_rules.py    ← 纯 Python 列表（610 条规则）
        ↓ 导入
knowledge/rules.db              ← SQLite 数据库（三个 Web 应用共用）
        ↑ 读写
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│  fullscan_py │  │incremental_py│  │   rete_py    │
│  :8080       │  │  :8081       │  │   :8082      │
└──────────────┘  └──────────────┘  └──────────────┘
```

- **`wikipedia_rules.py`**：原始数据源，纯 Python 列表格式，供 `benchmark/compare.py` 直接使用
- **`rules.db`**：SQLite 持久化数据库，由首次启动时从 `wikipedia_rules.py` 自动导入，三个 Web 应用共享读写
- **`knowledge_base.py`**：每个 Web 应用各有一份，统一指向 `../knowledge/rules.db`，提供规则 CRUD 和条件索引

### Web 应用架构

每个 Web 应用采用**单文件架构**，Python `http.server` 同时承担 HTTP 服务和前端页面渲染：

```
浏览器 ←→ HTTP Server (Python)
              │
              ├── GET  /          → 返回嵌入式 HTML 页面
              ├── POST /api/*     → JSON API（知识库/事实库/推理）
              └── HEAD /          → 健康检查（返回 200）
```

前端页面内嵌在 Python 文件的 `HTML_PAGE` 变量中，包含完整的 HTML + CSS + JavaScript，无需额外的静态文件。

---

## 算法对比

### 算法原理

| 算法 | 核心思想 | 时间复杂度 | 空间复杂度 |
|------|----------|-----------|-----------|
| **全扫描** | 每轮遍历全部规则，检查条件是否满足 | O(R × C × I) | O(R) |
| **增量触发** | 条件索引，只检查被新事实触发的规则 | O(R_triggered × C) | O(R + F) |
| **Rete 网络** | 构建 Alpha/Beta 节点图，事实沿路径增量传播 | O(F × N_depth) | O(N_nodes) |

> R = 规则数, C = 平均条件数, I = 迭代次数, F = 事实数, N_depth = 网络深度, N_nodes = 节点数

### 性能测试结果

控制变量法：同一 Wikipedia 生物分类规则集，同一组输入事实，每组 30 次取中位数。

| 规则数 | 全扫描 (μs) | 增量触发 (μs) | Rete (μs) | 最快 |
|--------|-------------|---------------|-----------|------|
| 20 | 9.01 | **2.25** | 24.09 | 增量触发 |
| 50 | 23.27 | **12.49** | 67.50 | 增量触发 |
| 100 | 47.34 | **4.55** | 68.89 | 增量触发 |
| 200 | 54.58 | **8.68** | 76.30 | 增量触发 |
| 300 | 138.05 | **13.80** | 142.48 | 增量触发 |
| 400 | 73.83 | **8.19** | 123.16 | 增量触发 |
| 500 | 77.48 | **8.84** | 122.44 | 增量触发 |
| 610 | 92.29 | **13.43** | 149.66 | 增量触发 |

**结论：增量触发在 Python 环境下 8/8 全线胜出。**

### 610 条规则下初始化与推理开销拆解

| 组件 | 全扫描 (μs) | 增量触发 (μs) | Rete (μs) |
|------|-------------|---------------|-----------|
| 初始化 (`__init__`) | ~0 | 213 | **3041** |
| 单次推理 (`forward`) | 92 | **13** | 150 |

### 深度分析：为什么 Rete 在 Python 中未体现理论优势？

**1. 网络构建开销极大**

Rete 需要为每条规则创建 AlphaNode + BetaNode + TerminalNode，610 条规则产生数千个 Python 对象。而增量触发仅需构建一个 `condition_index` 字典。

**2. 单次推理场景为 Rete 的最差工况**

Rete 的 O(1) 优势建立在"网络只建一次，事实逐条追加"的前提上。基准测试中每次 run 都重建网络并一次性灌入全部事实，无法复用部分匹配状态。

**3. Python 对象开销吞噬理论收益**

Rete 的 Beta 节点匹配涉及大量 Python 方法调用和 dict 查找，而全扫描的核心循环是纯 C 层面的 list iteration + set membership。

### 适用场景建议

| 场景 | 推荐算法 | 理由 |
|------|----------|------|
| Python + 单次推理 | **增量触发** | 条件索引简单高效，初始化快 |
| Python + 逐条增量 | 增量触发 ≈ Rete | Rete 可复用部分匹配，但对象开销仍在 |
| C++ / Java + 大规模增量 | **Rete** | 无 GC/对象开销，网络复用收益显著 |
| 教学演示 / 小规模 (<100 条) | **全扫描** | 实现最简单，逻辑最直观 |

---

## Web 应用功能

三个 Web 应用提供统一的功能集，界面采用深色主题设计，每个算法使用独立的主色调：

| 应用 | 端口 | 主色调 | 算法标识 |
|------|------|--------|----------|
| 全扫描 | `localhost:8080` | 紫色 `#7c3aed` | ⚡ Full Scan |
| 增量触发 | `localhost:8081` | 青色 `#0d9488` | ⚡ Incremental |
| Rete | `localhost:8082` | 琥珀色 `#f59e0b` | ⚡ Rete |

### 功能模块

- **知识库管理**：查看/搜索/添加/修改/删除规则，支持条件-结论的 IF-THEN 结构
- **事实库管理**：添加/删除/清空事实，支持批量输入
- **正向推理**：输入已知事实，自动推导所有可能结论，推理过程逐步可视化
- **反向推理**：输入目标结论，验证是否可由已知事实推导，展示推理路径
- **快速示例**：内置鸵鸟/老虎/豹等动物识别案例，一键加载
- **导航切换**：三个应用之间可通过顶部导航栏互相跳转

---

## 知识库内容

基于 Wikipedia 真实生物分类学数据，共 610 条规则，覆盖完整的脊索动物分类体系：

```
脊索动物
└── 脊椎动物
    ├── 鸟类（有羽毛、恒温）
    │   ├── 古颚下纲（无龙骨突）→ 鸵鸟目/鹤鸵目/无翼目/䳍形目/几维目
    │   └── 今颚下纲（有龙骨突）
    │       ├── 雁形目 → 鸭科/雁科
    │       ├── 鸡形目 → 雉科/吐绶鸡科
    │       ├── 隼形目 → 鹰科/隼科/鸱鸮科
    │       ├── 鹦形目 → 鹦鹉科
    │       ├── 鸽形目 → 鸠鸽科
    │       └── 雀形目 → 鸦科/鹟科/鹡鸰科
    ├── 哺乳动物（有毛发、乳腺、恒温）
    │   ├── 原兽亚纲（卵生）→ 鸭嘴兽科/针鼹科
    │   ├── 后兽亚纲（有育儿袋）→ 袋鼠科/考拉
    │   └── 真兽亚纲（有胎盘）
    │       ├── 食肉目 → 猫科/犬科/熊科/鼬科/浣熊科
    │       ├── 偶蹄目 → 牛科/鹿科/猪科/骆驼科/河马科/长颈鹿科
    │       ├── 奇蹄目 → 马科/犀科/貘科
    │       ├── 灵长目 → 人科/猴科
    │       ├── 啮齿目 → 松鼠科/鼠科
    │       ├── 鲸目 → 须鲸科/海豚科/抹香鲸科
    │       ├── 翼手目 → 蝙蝠
    │       └── 长鼻目 → 象科
    └── 爬行动物（变温、鳞片）
        ├── 龟鳖目 → 海龟科/泽龟科/陆龟科
        ├── 有鳞目 → 蛇亚目（眼镜蛇科/蝰科/蟒科/游蛇科）/蜥蜴亚目
        ├── 鳄目 → 鳄科
        └── 喙头目 → 楔齿蜥
```

---

## 各子项目文档

- [fullscan_py/README.md](fullscan_py/README.md) — 全扫描算法详细文档
- [incremental_py/README.md](incremental_py/README.md) — 增量触发算法详细文档
- [rete_py/README.md](rete_py/README.md) — Rete 网络算法详细文档
