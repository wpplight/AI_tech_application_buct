# 增量触发算法 (Incremental Triggering)

> 动物识别专家系统的增量触发推理算法实现，配有完整的 Web 交互界面。
> 通过条件索引机制，只检查被新事实触发的规则，避免全量遍历，在 Python 环境下性能最优。

---

## 概述

增量触发（Incremental Triggering）是本项目三种推理算法中性能最优的一种。其核心思想是：构建一个"条件 → 规则"的索引字典，当新事实加入时，只检查与该事实相关的规则，而非遍历全部规则。这种策略大幅减少了不必要的规则匹配操作。

在 610 条 Wikipedia 生物分类规则的基准测试中，增量触发以 **8/8 全线胜出**的成绩击败全扫描和 Rete 算法，单次推理仅需约 20 μs。

---

## 快速开始

```bash
cd incremental_py

# 启动 Web 服务（端口 8081）
python3 web_server.py
```

启动后在浏览器中访问 [http://localhost:8081](http://localhost:8081)

---

## 项目结构

```
incremental_py/
├── web_server.py           # HTTP 服务器 + 嵌入式前端（HTML/JS/CSS）
├── inference_engine.py     # 推理引擎（增量触发 + 反向推理）
├── knowledge_base.py       # 知识库管理（SQLite CRUD，指向 knowledge/rules.db）
├── fact_base.py            # 事实库（内存 set）
└── README.md               # 本文件
```

---

## 算法原理

### 正向推理 — 增量触发

与全扫描不同，增量触发不会在每轮遍历全部规则，而是只检查被新事实触发的规则：

```
输入事实 → 加入事实库
    ↓
通过条件索引查找被触发的规则
    ↓
检查触发规则的全部条件是否满足
    ↓
条件全部满足且结论是新的？ ──否──→ 跳过
    │是
    ↓
结论加入事实库，标记为"新事实"
    ↓
新事实再次触发更多规则？ ──是──→ 继续处理
    │否
    ↓
推理结束
```

核心代码逻辑：

```python
known_facts = set(fb.get_facts())
triggered = collect_triggered_rules(known_facts)  # 条件索引查找
checked = set()

while triggered:
    newly_deduced = []
    for rule in triggered:
        if rule.id in checked:
            continue
        checked.add(rule.id)
        if all(fb.contains(c) for c in rule.conditions):
            if not fb.contains(rule.conclusion):
                fb.add_fact(rule.conclusion)
                newly_deduced.append(rule.conclusion)

    # 用新推导的事实查找下一批触发规则
    triggered = collect_triggered_rules(set(newly_deduced))
```

### 条件索引机制

条件索引是增量触发的核心优化。知识库在初始化时构建一个字典，将每个条件映射到包含该条件的规则列表：

```
condition_index = {
    "有脊椎骨":  [Rule 1, Rule 3, Rule 5, ...],
    "恒温":      [Rule 2, Rule 6, Rule 8, ...],
    "有羽毛":    [Rule 4, Rule 7, ...],
    ...
}
```

当新事实"有羽毛"加入时，直接通过 `condition_index["有羽毛"]` 获取相关规则，无需遍历全部 610 条规则。

### 反向推理 (Backward Chaining)

反向推理采用目标驱动的递归分解策略：

```
输入目标
    ↓
目标已在事实库？ ──是──→ 返回 True（已知事实）
    │否
    ↓
查找结论为该目标的规则
    ↓
对每条候选规则：
    递归证明其每个条件
    ↓
全部条件成立？ ──是──→ 返回 True
    │否（条件不满足 / 循环依赖）
    ↓
无规则可推导 → 返回 False
```

反向推理支持循环依赖检测，避免无限递归。

---

## Web 界面功能

界面采用深色主题设计，主色调为青色/绿色（`#0d9488`）。

### 功能模块

| 模块 | 说明 |
|------|------|
| **知识库管理** | 查看全部规则、搜索、添加新规则、修改、删除 |
| **事实库管理** | 添加事实、删除事实、清空全部、批量输入 |
| **正向推理** | 输入已知事实，逐步展示推理过程，高亮触发的规则，显示触发链 |
| **反向推理** | 输入目标结论，验证是否可推导，展示递归证明路径 |
| **快速示例** | 内置鸵鸟、老虎、豹等识别案例，一键加载 |
| **导航切换** | 顶部导航栏可跳转至全扫描（8080）和 Rete（8082） |

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
- **条件索引**：`build_condition_index()` 方法构建 `condition → [rules]` 映射
- **CRUD 操作**：支持规则的增删改查，含重复检测，增删操作后自动重建索引

### fact_base.py — 事实库

- **存储方式**：内存中的 Python `set`
- **操作**：添加、删除、包含检查、批量添加、清空
- **特点**：轻量高效，O(1) 查找

### inference_engine.py — 推理引擎

- **正向推理**：`forward_chain()` 方法，基于条件索引的增量触发
- **反向推理**：`backward_chain()` 方法，递归目标分解，含循环依赖检测
- **步骤记录**：记录每一步推理的详细信息（触发规则、触发条件、新增事实等）

---

## 性能特征

| 指标 | 说明 |
|------|------|
| 初始化开销 | ~213 μs（构建条件索引） |
| 单次推理（610 条规则） | ~13 μs |
| 优势场景 | Python 环境下的所有规模知识库 |
| 核心优势 | 条件索引避免全量遍历，对象开销小 |
| 对比详情 | 参见 [benchmark/compare.py](../benchmark/compare.py) |

### 为什么增量触发在 Python 中最优？

1. **条件索引简单高效**：一个 `dict` 字典，连续内存布局，Python 原生哈希查找 O(1)
2. **无复杂数据结构开销**：不像 Rete 需要创建数千个节点对象
3. **只检查相关规则**：新事实只触发索引中对应的规则子集，而非全部 610 条
4. **纯 C 层面的集合操作**：`set` 的 `in` 操作在 CPython 中是纯 C 实现

---

## 使用示例

### 识别鸵鸟

输入事实：`有脊椎骨, 有羽毛, 恒温, 无龙骨突, 不能飞, 体型大, 有长颈, 有长腿, 黑白二色`

触发链：
```
有脊椎骨 → 触发"脊椎动物"规则 → 推导"脊椎动物"
有羽毛 + 恒温 + 脊椎动物 → 触发"鸟类"规则 → 推导"鸟类"
无龙骨突 + 鸟类 → 触发"古颚下纲"规则 → 推导"古颚下纲"
... → 最终推导"鸵鸟"
```

### 识别老虎

输入事实：`有脊椎骨, 有毛发, 乳腺, 恒温, 食肉, 有利爪, 有斑纹(条纹), 体型大`

触发链：脊椎动物 → 哺乳动物 → 真兽亚纲 → 食肉目 → 猫科 → 豹属 → 虎

---

## 相关文档

- [根目录 README](../README.md) — 项目总览与算法对比
- [全扫描算法](../fullscan_py/README.md) — 全扫描算法详细文档
- [Rete 网络算法](../rete_py/README.md) — Rete 网络算法详细文档
