# Rete 网络算法 (Rete Network)

> 动物识别专家系统的 Rete 网络推理算法实现，配有完整的 Web 交互界面。
> 通过构建 Alpha/Beta/Terminal 节点图实现事实的增量传播，是经典的人工智能推理算法。

---

## 概述

Rete 是由 Charles Forgy 于 1979 年提出的模式匹配算法，是大多数现代规则引擎（如 CLIPS、Jess、Drools）的核心算法。其核心思想是：将规则编译成一个网络结构（Rete 网络），事实沿网络路径传播，通过增量更新避免重复计算。

本项目在 Python 中实现了完整的 Rete 网络，包含 AlphaNode（条件过滤）、BetaNode（AND 合并）和 TerminalNode（触发推导）三种节点类型。虽然在 Python 环境下因对象开销较大而性能不如增量触发，但该实现完整展示了 Rete 算法的核心思想和网络结构。

---

## 快速开始

```bash
cd rete_py

# 启动 Web 服务（端口 8082）
python3 web_server.py
```

启动后在浏览器中访问 [http://localhost:8082](http://localhost:8082)

---

## 项目结构

```
rete_py/
├── web_server.py           # HTTP 服务器 + 嵌入式前端（HTML/JS/CSS）
├── rete_network.py         # Rete 网络核心（Alpha/Beta/Terminal 节点）
├── rete_runner.py          # Rete 推理执行器（正向 + 反向推理）
├── knowledge_base.py       # 知识库管理（SQLite CRUD，指向 knowledge/rules.db）
└── README.md               # 本文件
```

---

## Rete 网络原理

### 网络结构

Rete 网络将每条规则分解为一个节点链，事实从入口沿路径传播：

```
事实输入
    ↓
┌─────────────────────────────────────────────────────────┐
│  Alpha 层（条件过滤）                                      │
│                                                          │
│  [有羽毛] → AlphaNode(有羽毛) ──┐                         │
│                                  ├──→ BetaNode(是鸟)      │
│  [恒温]   → AlphaNode(恒温)   ──┘        │               │
│                                          ↓               │
│  [无龙骨突] → AlphaNode(无龙骨突) ──┐                    │
│                                      ├──→ BetaNode       │
│  [不能飞]  → AlphaNode(不能飞)   ──┘        │            │
│                                              ↓            │
│  [体型大]  → AlphaNode(体型大) ──────────────┤            │
│                                              ↓            │
│  ...更多条件...                               ↓            │
│                                              ↓            │
│                                      TerminalNode         │
│                                      → 推导"鸵鸟"         │
└─────────────────────────────────────────────────────────┘
```

### 三种节点类型

| 节点类型 | 作用 | 数据结构 |
|----------|------|----------|
| **AlphaNode** | 按单个条件过滤事实，维护 Alpha Memory | `{condition, children, facts}` |
| **BetaNode** | AND 连接多个条件，累积部分匹配，维护 Beta Memory | `{left, right, children, tokens}` |
| **TerminalNode** | 条件全部满足时触发推导，产生新事实 | `{rule, conclusion}` |

### 事实传播过程

```
新事实 "有羽毛" 到达
    ↓
AlphaNode(有羽毛) 激活 → 存入 Alpha Memory
    ↓
传播到所有连接的 BetaNode
    ↓
BetaNode 检查左/右输入是否都有匹配
    ↓
左输入"有羽毛"已有 + 右输入"恒温"已有 → BetaNode 激活
    ↓
传播到 TerminalNode
    ↓
TerminalNode 触发 → 推导新事实"鸟类"
    ↓
新事实"鸟类"重新进入网络 → 可能触发更多规则
```

### 网络构建过程

初始化时，每条规则被编译为节点链：

```python
def build(self, rules):
    for rule in rules:
        alpha_nodes = []
        for condition in rule.conditions:
            alpha = self._get_or_create_alpha(condition)
            alpha_nodes.append(alpha)

        # 将 Alpha 节点两两组合为 Beta 节点
        current = alpha_nodes[0]
        for i in range(1, len(alpha_nodes)):
            beta = BetaNode(current, alpha_nodes[i])
            current = beta

        # 最终连接 Terminal 节点
        terminal = TerminalNode(rule)
        current.children.append(terminal)
```

---

## Web 界面功能

界面采用深色主题设计，主色调为琥珀色（`#f59e0b`）。

### 功能模块

| 模块 | 说明 |
|------|------|
| **知识库管理** | 查看全部规则、搜索、添加新规则、修改、删除 |
| **事实库管理** | 添加事实、删除事实、清空全部、批量输入 |
| **正向推理** | 输入已知事实，展示 Rete 网络传播过程，高亮激活的节点 |
| **反向推理** | 输入目标结论，验证是否可推导，展示推理路径 |
| **Rete 网络统计** | 查看网络结构（Alpha/Beta/Terminal 节点数量） |
| **快速示例** | 内置鸵鸟、老虎、豹等识别案例，一键加载 |
| **导航切换** | 顶部导航栏可跳转至全扫描（8080）和增量触发（8081） |

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
| `GET` | `/api/network` | 获取 Rete 网络统计信息 |

---

## 模块详解

### rete_network.py — Rete 网络核心

这是 Rete 算法的核心实现，包含三种节点类型和网络构建逻辑：

- **Fact 类**：事实对象，使用 `__slots__` 优化内存
- **AlphaNode 类**：条件过滤节点，维护匹配的事实集合（Alpha Memory）
- **BetaNode 类**：AND 连接节点，累积左右输入的部分匹配（Beta Memory / Tokens）
- **TerminalNode 类**：终端节点，条件全部满足时触发推导
- **ReteNetwork 类**：网络管理器，负责构建网络、传播事实、收集推导结果

### rete_runner.py — 推理执行器

封装 Rete 网络为完整的推理引擎：

- **Rule 类**：规则数据结构（id, conditions, conclusion）
- **ReteInferenceEngine 类**：
  - `add_rule()` — 添加规则
  - `build_network()` — 编译规则为 Rete 网络
  - `forward()` — 正向推理，事实注入网络并收集推导结果
  - `backward()` — 反向推理，递归目标分解
  - `reset_steps()` / `get_steps()` — 推理步骤记录

### knowledge_base.py — 知识库管理

- **存储后端**：SQLite 数据库，路径指向 `../knowledge/rules.db`
- **数据结构**：`rules` 表（id, conclusion）+ `rule_conditions` 表（rule_id, condition_text）
- **CRUD 操作**：支持规则的增删改查，含重复检测

---

## 性能特征

| 指标 | 说明 |
|------|------|
| 初始化开销 | ~3041 μs（构建 Rete 网络，610 条规则产生数千个节点） |
| 单次推理（610 条规则） | ~150 μs |
| 优势场景 | 编译型语言（C++/Java）+ 大规模规则 + 增量推理 |
| 劣势场景 | Python 环境，单次推理，规则频繁变更 |
| 对比详情 | 参见 [benchmark/compare.py](../benchmark/compare.py) |

### 为什么 Rete 在 Python 中性能较低？

1. **网络构建开销极大**：610 条规则 × 平均 2-3 个条件 = 数千个 Python 对象（AlphaNode + BetaNode + TerminalNode），每个对象包含字典、列表等复杂数据结构
2. **单次推理无法复用部分匹配**：Rete 的 O(1) 优势建立在"网络只建一次，事实逐条追加"的前提上。基准测试中每次 run 都重建网络并一次性灌入全部事实
3. **Python 对象调用开销**：Beta 节点匹配涉及大量 Python 方法调用、dict 查找、list 操作，而全扫描的核心循环是纯 C 层面的 list iteration + set membership

### Rete 的理想使用场景

| 条件 | 说明 |
|------|------|
| 编译型语言 | C++/Java 无 GC 和对象开销 |
| 网络复用 | 规则编译一次，多次推理复用 |
| 增量事实 | 事实逐条到达，利用部分匹配 |
| 大规模规则 | 5000+ 条规则时网络复用收益显著 |

---

## 知识库内容

基于 Wikipedia 真实生物分类学数据，Web 界面内置 **610 条规则**，覆盖完整的脊索动物分类体系：

```
脊索动物 → 脊椎动物
    ├── 鸟类（有羽毛、恒温）
    │   ├── 古颚下纲（无龙骨突）→ 鸵鸟目/鹤鸵目/无翼目/䳍形目/几维目
    │   └── 今颚下纲（有龙骨突）
    │       ├── 雁形目 → 鸭科/雁科
    │       ├── 鸡形目 → 雉科/吐绶鸡科
    │       └── 隼形目 → 鹰科/隼科/鸱鸮科
    ├── 哺乳动物（有毛发、乳腺、恒温）
    │   ├── 原兽亚纲（卵生）→ 鸭嘴兽科/针鼹科
    │   ├── 后兽亚纲（有育儿袋）→ 袋鼠科/考拉
    │   └── 真兽亚纲（有胎盘）
    │       ├── 食肉目 → 猫科/犬科/熊科
    │       ├── 偶蹄目 → 牛科/鹿科
    │       └── 鲸目 → 须鲸科/海豚科
    └── 爬行动物（变温、鳞片）
        ├── 龟鳖目 → 海龟科/泽龟科/陆龟科
        ├── 有鳞目 → 蛇亚目/蜥蜴亚目
        └── 鳄目 → 鳄科
```

---

## 使用示例

### 识别鸵鸟

输入事实：`有脊椎骨, 有羽毛, 恒温, 无龙骨突, 不能飞, 体型大, 有长颈, 有长腿, 黑白二色`

Rete 网络传播路径：
```
Alpha(有脊椎骨) + Alpha(有羽毛) + Alpha(恒温)
    → Beta(脊椎动物+鸟类) → Terminal(鸟类)
        → 新事实"鸟类"注入网络
Alpha(无龙骨突) + Alpha(鸟类)
    → Beta(古颚下纲) → Terminal(古颚下纲)
        → ...继续传播... → Terminal(鸵鸟)
```

### 识别老虎

输入事实：`有脊椎骨, 有毛发, 乳腺, 恒温, 食肉, 有利爪, 有斑纹(条纹), 体型大`

传播路径：脊椎动物 → 哺乳动物 → 真兽亚纲 → 食肉目 → 猫科 → 豹属 → 虎

---

## 相关文档

- [根目录 README](../README.md) — 项目总览与算法对比
- [全扫描算法](../fullscan_py/README.md) — 全扫描算法详细文档
- [增量触发算法](../incremental_py/README.md) — 增量触发算法详细文档
