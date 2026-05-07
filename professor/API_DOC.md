# 动物识别专家系统 - REST API 文档

> 本文档描述了专家系统统一服务器的 REST API 接口，供 show-web 展示网站对接使用。
>
> **版本更新**: v2.0 - 新增分页、搜索、规则详情等功能

---

## 📡 基础信息

### 服务器地址

**统一服务器（推荐）**: `http://localhost:8080`

| 算法参数 | 算法名称 | 说明 |
|----------|----------|------|
| `fullscan` | 全扫描 | 遍历所有规则检查条件 |
| `incremental` | 增量触发 | 条件索引优化 |
| `rete` | Rete 网络 | Alpha/Beta 节点网络 |

### API 使用方式

所有 API 端点都通过 `?algo=<algorithm>` 参数指定使用的算法：

```bash
# 全扫描算法
curl 'http://localhost:8080/api/rules?algo=fullscan'

# 增量触发算法
curl 'http://localhost:8080/api/rules?algo=incremental'

# Rete 网络算法
curl 'http://localhost:8080/api/rules?algo=rete'
```

---

## 🆕 增强功能 (v2.0)

### 1. 分页获取规则

**接口**: `GET /api/rules?algo=<algorithm>&page=1&limit=50`

**参数**:
- `page`: 页码（默认: 1）
- `limit`: 每页数量（默认: 50）

**示例请求**:
```bash
curl 'http://localhost:8080/api/rules?algo=incremental&page=1&limit=10'
```

**响应示例**:
```json
{
  "rules": [...],
  "algorithm": "incremental",
  "pagination": {
    "page": 1,
    "limit": 10,
    "total": 610,
    "total_pages": 61
  }
}
```

---

### 2. 模糊搜索规则

**接口**: `GET /api/rules/search?algo=<algorithm>&q=<关键词>`

**示例请求**:
```bash
curl 'http://localhost:8080/api/rules/search?algo=incremental&q=mammal'
```

**响应示例**:
```json
{
  "rules": [...],
  "algorithm": "incremental",
  "total": 15,
  "query": "mammal"
}
```

---

### 3. 获取规则详情

**接口**: `GET /api/rules/<rule_id>?algo=<algorithm>`

**描述**: 获取指定规则的详细信息，包括该规则结论可以推导出的其他规则

**示例请求**:
```bash
curl 'http://localhost:8080/api/rules/1?algo=incremental'
```

**响应示例**:
```json
{
  "rule": {
    "id": 1,
    "conditions": ["有脊索"],
    "conclusion": "脊索动物"
  },
  "algorithm": "incremental",
  "related_rules": [
    {
      "id": 2,
      "conditions": ["脊索动物"],
      "conclusion": "脊椎动物"
    }
  ]
}
```

---

### 4. 模糊搜索事实

**接口**: `GET /api/facts?algo=<algorithm>&search=<关键词>`

**示例请求**:
```bash
curl 'http://localhost:8080/api/facts?algo=incremental&search=毛发'
```

**响应示例**:
```json
{
  "facts": ["身上有毛发"],
  "algorithm": "incremental",
  "total": 1,
  "search": "毛发"
}
```

---

### 5. 根据事实查询相关规则

**接口**: `GET /api/rules?algo=<algorithm>&fact=<事实名称>`

**描述**: 获取所有使用该事实作为条件的规则

**示例请求**:
```bash
curl 'http://localhost:8080/api/rules?algo=incremental&fact=体温恒定'
```

**响应示例**:
```json
{
  "rules": [
    {
      "id": 5,
      "conditions": ["体温恒定", "身上有毛发"],
      "conclusion": "是哺乳动物"
    }
  ],
  "algorithm": "incremental",
  "pagination": {
    "page": 1,
    "limit": 50,
    "total": 1,
    "total_pages": 1
  }
}
```

---

## 📚 知识库管理 API

### 1. 获取所有规则（支持分页和搜索）

**接口**: `GET /api/rules?algo=<algorithm>`

**参数**:
- `page`: 页码（默认: 1）
- `limit`: 每页数量（默认: 50）
- `search`: 搜索关键词（可选）
- `fact`: 事实名称（可选，用于过滤相关规则）

**示例请求**:
```bash
# 分页获取
curl 'http://localhost:8080/api/rules?algo=incremental&page=2&limit=20'

# 搜索
curl 'http://localhost:8080/api/rules?algo=incremental&search=动物'

# 事实相关规则
curl 'http://localhost:8080/api/rules?algo=incremental&fact=有毛发'
```

**响应示例**:
```json
{
  "rules": [
    {
      "id": 1,
      "conditions": ["体温恒定", "身上有毛发"],
      "conclusion": "是哺乳动物"
    }
  ],
  "algorithm": "incremental",
  "pagination": {
    "page": 1,
    "limit": 50,
    "total": 610,
    "total_pages": 13
  }
}
```

---

### 2. 添加规则

**接口**: `POST /api/rules/add?algo=<algorithm>`

**请求体**:
```json
{
  "conditions": ["条件1", "条件2"],
  "conclusion": "结论"
}
```

**示例请求**:
```bash
curl -X POST 'http://localhost:8080/api/rules/add?algo=fullscan' \
  -H 'Content-Type: application/json' \
  -d '{"conditions": ["会飞", "有羽毛"], "conclusion": "是鸟类"}'
```

**成功响应**:
```json
{
  "success": true,
  "rule_id": 611,
  "algorithm": "fullscan"
}
```

---

### 3. 删除规则

**接口**: `POST /api/rules/delete?algo=<algorithm>`

**示例请求**:
```bash
curl -X POST 'http://localhost:8080/api/rules/delete?algo=incremental' \
  -H 'Content-Type: application/json' \
  -d '{"rule_id": 15}'
```

---

### 4. 修改规则

**接口**: `POST /api/rules/modify?algo=<algorithm>`

**示例请求**:
```bash
curl -X POST 'http://localhost:8080/api/rules/modify?algo=rete' \
  -H 'Content-Type: application/json' \
  -d '{"rule_id": 15, "conditions": ["新条件1", "新条件2"]}'
```

---

## 📝 事实库管理 API

### 1. 获取当前事实

**接口**: `GET /api/facts?algo=<algorithm>`

**参数**:
- `search`: 搜索关键词（可选）

**示例请求**:
```bash
curl 'http://localhost:8080/api/facts?algo=fullscan'
```

**响应示例**:
```json
{
  "facts": ["体温恒定", "身上有毛发"],
  "algorithm": "fullscan",
  "total": 2,
  "search": null
}
```

---

### 2. 添加事实

**接口**: `POST /api/facts/add?algo=<algorithm>`

**示例请求**:
```bash
curl -X POST 'http://localhost:8080/api/facts/add?algo=incremental' \
  -H 'Content-Type: application/json' \
  -d '{"fact": "身上有毛发"}'
```

---

### 3. 删除事实

**接口**: `POST /api/facts/delete?algo=<algorithm>`

**示例请求**:
```bash
curl -X POST 'http://localhost:8080/api/facts/delete?algo=rete' \
  -H 'Content-Type: application/json' \
  -d '{"fact": "身上有毛发"}'
```

---

### 4. 清空事实库

**接口**: `POST /api/facts/clear?algo=<algorithm>`

---

## 🧠 推理引擎 API

### 1. 正向推理

**接口**: `POST /api/inference/forward?algo=<algorithm>`

**描述**: 执行正向链式推理，从已知事实推导出新的结论

**示例请求**:
```bash
curl -X POST 'http://localhost:8080/api/inference/forward?algo=incremental' \
  -H 'Content-Type: application/json'
```

**响应示例**:
```json
{
  "success": true,
  "new_facts": ["是哺乳动物", "是脊椎动物"],
  "all_facts": ["体温恒定", "身上有毛发", "会哺乳", "是哺乳动物", "是脊椎动物"],
  "steps": 2,
  "algorithm": "incremental"
}
```

---

### 2. 反向推理

**接口**: `POST /api/inference/backward?algo=<algorithm>`

**描述**: 验证某个目标结论是否能由已知事实推导出来

**示例请求**:
```bash
curl -X POST 'http://localhost:8080/api/inference/backward?algo=fullscan' \
  -H 'Content-Type: application/json' \
  -d '{"goal": "是猫科动物"}'
```

**成功响应**:
```json
{
  "success": true,
  "goal": "是猫科动物",
  "steps": 5,
  "missing_facts": [],
  "goal_already_known": false,
  "algorithm": "fullscan"
}
```

**失败响应**（缺少必要事实）:
```json
{
  "success": false,
  "goal": "是猫科动物",
  "steps": 3,
  "missing_facts": ["有犬齿", "能爪抓"],
  "goal_already_known": false,
  "algorithm": "fullscan"
}
```

---

### 3. 获取推理步骤

**接口**: `GET /api/inference/steps?algo=<algorithm>`

---

## 🗑️ 系统管理 API

### 1. 重置系统

**接口**: `POST /api/reset?algo=<algorithm>`

---

## 🔬 Rete 算法特有 API

### 1. 获取网络统计

**接口**: `GET /api/network/stats?algo=rete`

**响应示例**:
```json
{
  "alpha_nodes": 150,
  "beta_nodes": 89,
  "terminal_nodes": 50,
  "total_rules": 610
}
```

---

### 2. 获取执行追踪

**接口**: `GET /api/network/trace?algo=rete`

---

## 📋 完整 API 端点列表

| 方法 | 端点 | 说明 | 参数 |
|------|------|------|------|
| GET | `/` | 服务器信息 | 可选 |
| GET | `/api/algorithms` | 所有算法状态 | 否 |
| GET | `/api/rules` | 获取规则列表（分页） | page, limit, search, fact |
| GET | `/api/rules/search` | 搜索规则 | q |
| GET | `/api/rules/<id>` | 获取规则详情 | algorithm |
| POST | `/api/rules/add` | 添加规则 | algorithm |
| POST | `/api/rules/delete` | 删除规则 | algorithm |
| POST | `/api/rules/modify` | 修改规则 | algorithm |
| GET | `/api/facts` | 获取事实列表 | search |
| POST | `/api/facts/add` | 添加事实 | algorithm |
| POST | `/api/facts/delete` | 删除事实 | algorithm |
| POST | `/api/facts/clear` | 清空事实库 | algorithm |
| POST | `/api/inference/forward` | 正向推理 | algorithm |
| POST | `/api/inference/backward` | 反向推理 | algorithm |
| GET | `/api/inference/steps` | 推理步骤 | algorithm |
| POST | `/api/reset` | 重置系统 | algorithm |
| GET | `/api/network/stats` | Rete 网络统计 | `rete` |
| GET | `/api/network/trace` | Rete 执行追踪 | `rete` |

---

## 🎯 快速开始

### 1. 启动服务器

```bash
cd professor
make start
```

### 2. 分页获取规则

```bash
# 获取第1页，每页20条
curl 'http://localhost:8080/api/rules?algo=incremental&page=1&limit=20'
```

### 3. 搜索规则

```bash
# 搜索包含关键词的规则
curl 'http://localhost:8080/api/rules/search?algo=incremental&q=动物'
```

### 4. 获取规则详情和相关信息

```bash
# 查看规则#10的详细信息
curl 'http://localhost:8080/api/rules/10?algo=incremental'
```

### 5. 根据事实查找相关规则

```bash
# 查找所有使用"有毛发"作为条件的规则
curl 'http://localhost:8080/api/rules?algo=incremental&fact=有毛发'
```

### 6. 添加事实并推理

```bash
# 添加事实
curl -X POST 'http://localhost:8080/api/facts/add?algo=incremental' \
  -H 'Content-Type: application/json' \
  -d '{"fact": "体温恒定"}'

curl -X POST 'http://localhost:8080/api/facts/add?algo=incremental' \
  -H 'Content-Type: application/json' \
  -d '{"fact": "身上有毛发"}'

# 执行正向推理
curl -X POST 'http://localhost:8080/api/inference/forward?algo=incremental'
```

---

## 📝 版本历史

| 版本 | 日期 | 说明 |
|------|------|------|
| 2.0 | 2026-05-06 | 新增分页、搜索、规则详情、事实搜索等功能 |
| 1.0 | 2026-05-06 | 初始文档 |
