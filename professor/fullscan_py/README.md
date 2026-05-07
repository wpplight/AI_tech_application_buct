# 全扫描算法 (Full Scan)

> 动物识别专家系统的全扫描推理算法实现，配有完整的 Web 交互界面。
> 每轮遍历知识库中的全部规则，检查条件是否满足，是最基础、最直观的推理策略。

---

## 概述

全扫描（Full Scan）是本项目三种推理算法中最简单的一种。其核心思想是：在每一轮推理中，遍历知识库中的所有规则，逐一检查每条规则的条件是否全部满足于当前事实库。如果满足且结论尚未存在于事实库中，则将结论加入事实库。重复此过程直到没有新事实产生为止。

该算法实现简单、逻辑清晰，非常适合用于教学演示和小规模知识库场景。

---

## 快速开始

```bash
cd fullscan_py

# 启动 Web 服务（端口 8080）
python3 web_server.py
```

启动后在浏览器中访问 [http://localhost:8080](http://localhost:8080)

---

## 项目结构

```
fullscan_py/
├── web_server.py           # HTTP 服务器 + 嵌入式前端（HTML/JS/CSS）
├── inference_engine.py     # 推理引擎（正向推理 + 反向推理）
├── knowledge_base.py       # 知识库管理（SQLite CRUD，指向 knowledge/rules.db）
├── fact_base.py            # 事实库（内存 set）
└── README.md               # 本文件
```

---

## 算法原理

### 正向推理 (Forward Chaining)

正向推理采用数据驱动策略，从已知事实出发，逐步推导新事实：

```
输入事实 → 加入事实库
    ↓
遍历全部规则
    ↓
规则条件全部满足？ ──否──→ 跳过
    │是
    ↓
结论已在事实库？ ──是──→ 跳过
    │否
    ↓
结论加入事实库
    ↓
本轮有新事实？ ──是──→ 重新遍历全部规则
    │否
    ↓
推理结束
```

核心代码逻辑：

```python
while iteration < max_iterations:
    added = False
    for rule in all_rules:                          # 遍历全部规则
        if all(fb.contains(c) for c in rule.conditions):  # 检查所有条件
            if not fb.contains(rule.conclusion):    # 结论是新的
                fb.add_fact(rule.conclusion)         # 加入事实库
                added = True
    if not added:
        break                                       # 无新事实，终止
```

**时间复杂度**：O(R × C × I)，其中 R = 规则数，C = 平均条件数，I = 迭代轮数。

### 反向推理 (Backward Chaining)

反向推理采用目标驱动策略，从目标出发，逆向查找推导路径：

```
输入目标
    ↓
目标已在事实库？ ──是──→ 返回 True
    │否
    ↓
查找结论为该目标的规则
    ↓
对每条候选规则：
    递归证明其每个条件是否成立
    ↓
所有条件均成立？ ──是──→ 返回 True
    │否
    ↓
无规则可推导 → 返回 False
```

反向推理会记录完整的推理路径，便于用户理解推导过程。

---

## Web 界面功能

界面采用深色主题设计，主色调为紫色（`#7c3aed`）。

### 功能模块

| 模块 | 说明 |
|------|------|
| **知识库管理** | 查看全部规则、搜索、添加新规则、修改、删除 |
| **事实库管理** | 添加事实、删除事实、清空全部、批量输入 |
| **正向推理** | 输入已知事实，逐步展示推理过程，高亮触发的规则 |
| **反向推理** | 输入目标结论，验证是否可推导，展示推理路径 |
| **快速示例** | 内置鸵鸟、老虎、豹等识别案例，一键加载 |
| **导航切换** | 顶部导航栏可跳转至增量触发（8081）和 Rete（8082） |

### API 接口

| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/` | 返回 Web 页面 |
| `GET` | `/api/rules` | 获取全部规则 |
| `POST` | `/api/rules` | 添加规则（`{conditions, conclusion}`） |
| `PUT` | `/api/rules/{id}` | 修改规则 |
| `DELETE` | `/api/rules/{id}` | 删除规则 |
| `GET` | `/api/facts` | 获取全部事实 |
| `POST` | `/api/facts` | 添加事实（`{fact}`） |
| `DELETE` | `/api/facts` | 删除事实（`{fact}`） |
| `POST` | `/api/forward` | 正向推理 |
| `POST` | `/api/backward` | 反向推理（`{goal}`） |
| `POST` | `/api/reset` | 重置事实库 |

---

## 模块详解

### knowledge_base.py — 知识库管理

- **存储后端**：SQLite 数据库，路径指向 `../knowledge/rules.db`
- **数据结构**：`rules` 表（id, conclusion）+ `rule_conditions` 表（rule_id, condition_text）
- **条件索引**：构建 `condition_index` 字典，将每个条件映射到包含该条件的规则列表
- **CRUD 操作**：支持规则的增删改查，含重复检测

### fact_base.py — 事实库

- **存储方式**：内存中的 Python `set`
- **操作**：添加、删除、包含检查、批量添加、清空
- **特点**：轻量高效，适合单次会话使用

### inference_engine.py — 推理引擎

- **正向推理**：`forward_chain()` 方法，遍历全部规则直到无新事实
- **反向推理**：`backward_chain()` 方法，递归目标分解，含循环依赖检测
- **步骤记录**：记录每一步推理的详细信息（触发规则、新增事实、匹配状态等）

---

## 性能特征

| 指标 | 说明 |
|------|------|
| 初始化开销 | 接近零（无需构建额外数据结构） |
| 单次推理（610 条规则） | ~92 μs |
| 优势场景 | 小规模知识库（<100 条规则） |
| 劣势场景 | 大规模知识库，每轮需遍历全部规则 |
| 对比详情 | 参见 [benchmark/compare.py](../benchmark/compare.py) |

---

## 使用示例

### 识别鸵鸟

输入事实：`有脊椎骨, 有羽毛, 恒温, 无龙骨突, 不能飞, 体型大, 有长颈, 有长腿, 黑白二色`

推理路径：脊椎动物 → 鸟类 → 古颚下纲 → 鸵鸟目 → 鸵鸟科 → 鸵鸟

### 识别老虎

输入事实：`有脊椎骨, 有毛发, 乳腺, 恒温, 食肉, 有利爪, 有斑纹(条纹), 体型大`

推理路径：脊椎动物 → 哺乳动物 → 真兽亚纲 → 食肉目 → 猫科 → 豹属 → 虎

---

## 相关文档

- [根目录 README](../README.md) — 项目总览与算法对比
- [增量触发算法](../incremental_py/README.md) — 增量触发算法详细文档
- [Rete 网络算法](../rete_py/README.md) — Rete 网络算法详细文档
