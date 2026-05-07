# 条件集（ConditionSet）设计文档

## 一、问题背景

### 1.1 当前设计的问题

现有的推理流程存在三个问题：

**状态混淆**：工作内存（facts）和知识库规则混在一起，前端每次添加一个事实就调一次 API，推理时又依赖后端工作内存的状态，导致：
- 无法独立保存"一组条件"
- 推理结果受历史残留 facts 影响
- 无法在多个场景间切换条件组合

**Rete 优势被浪费**：Rete 网络的优势在于增量匹配（相同 facts 不重复计算），但现有实现中每次推理前都清空 facts，优势完全无法发挥。

**批量操作缺抽象**：用户无法"保存"一组条件为模板，每次都要手动一个个添加。

### 1.2 目标

设计一个**条件集（ConditionSet）**抽象，实现：

1. 持久化存储条件组合，支持增删改查
2. 推理时无状态调用，传入条件集 ID 或 facts 列表
3. Rete 可以按条件集缓存增量计算结果

---

## 二、核心概念

### 2.1 条件集（ConditionSet）

```
ConditionSet {
    id: int              // 唯一标识
    name: str            // 用户自定义名称
    facts: List[str]     // 该条件集包含的事实列表
    created_at: datetime
    updated_at: datetime
}
```

### 2.2 与现有事实库的关系

```
知识库（Knowledge Base）
├── 规则库（Rules）         ← 推理的"逻辑"，持久化不变
└── 条件集（ConditionSets） ← 推理的"输入"，用户可增删改查

工作内存（Working Memory）
└── 推理时临时使用，不持久化
```

**关键区分**：
- **事实库（Facts）**：规则库中所有可用的条件（只读）
- **条件集（ConditionSets）**：用户自定义的条件组合（可写）

---

## 三、数据层设计

### 3.1 数据库 Schema

建议复用现有的 SQLite 数据库，新增一张表：

```sql
CREATE TABLE IF NOT EXISTS condition_sets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    facts TEXT NOT NULL,          -- JSON 序列化的字符串列表
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_cs_name ON condition_sets(name);
```

### 3.2 Rete 缓存策略

```python
class ReteCache:
    """
    Rete 增量缓存，按条件集 ID 索引
    结构：condition_set_id -> {
        'facts': frozenset,          # 条件集的事实快照
        'network_snapshot': bytes,    # Alpha/Beta 节点匹配结果序列化（可选）
        'matched_rules': List[int],  # 匹配到的规则 ID
        'deduced_facts': List[str],  # 从该条件集推导出的新事实
    }
    """
    def __init__(self):
        self._cache: Dict[int, CacheEntry] = {}

    def get(self, condition_set_id: int, facts: List[str]) -> Optional[CacheEntry]:
        entry = self._cache.get(condition_set_id)
        if entry and entry.facts == frozenset(facts):
            return entry  # 命中：facts 未变，直接复用
        return None

    def invalidate(self, condition_set_id: int):
        """条件集被编辑时调用"""
        self._cache.pop(condition_set_id, None)

    def set(self, condition_set_id: int, facts: List[str], entry: CacheEntry):
        self._cache[condition_set_id] = entry
```

### 3.3 缓存失效规则

| 事件 | Rete 缓存行为 |
|------|-------------|
| 条件集被编辑 | `invalidate(condition_set_id)` |
| 规则库被修改（增/删/改规则） | 清空所有缓存（全局失效） |
| 推理时 facts 与缓存不符 | 重新计算并更新缓存 |

---

## 四、API 设计

### 4.1 条件集 CRUD

#### 创建条件集
```
POST /api/condition-sets
Body: { "name": "我的条件集", "facts": ["有毛发", "会哺乳"] }
Response: {
    "id": 1,
    "name": "我的条件集",
    "facts": ["有毛发", "会哺乳"],
    "created_at": "2025-07-01T10:00:00Z"
}
```

#### 列出所有条件集
```
GET /api/condition-sets?page=1&limit=20
Response: {
    "condition_sets": [...],
    "pagination": { "page": 1, "limit": 20, "total": 5, "total_pages": 1 }
}
```

#### 获取单个条件集
```
GET /api/condition-sets/{id}
Response: {
    "id": 1,
    "name": "我的条件集",
    "facts": ["有毛发", "会哺乳"],
    "created_at": "...",
    "updated_at": "..."
}
```

#### 更新条件集
```
PUT /api/condition-sets/{id}
Body: { "name": "新名字", "facts": ["有毛发", "有犬齿"] }
Response: { "id": 1, "name": "新名字", "facts": [...], ... }
副作用：Rete 引擎 invalidate 该 condition_set_id 的缓存
```

#### 删除条件集
```
DELETE /api/condition-sets/{id}
Response: { "success": true }
副作用：Rete 引擎 invalidate 该 condition_set_id 的缓存
```

### 4.2 推理接口（无状态）

#### 前向推理
```
POST /api/inference/forward
Body: {
    "facts": ["有毛发", "会哺乳"]        // 方式一：直接传 facts（兼容旧接口）
    "condition_set_id": 1               // 方式二：传条件集 ID（优先）
}
优先级：condition_set_id > facts

Response: {
    "success": true,
    "condition_set_id": 1,             // 透传，用于前端关联
    "input_facts": ["有毛发", "会哺乳"],
    "new_facts": ["是哺乳动物", "是肉食动物"],  // 推理得出的新事实
    "all_facts": ["有毛发", "会哺乳", "是哺乳动物", "是肉食动物"],
    "steps": 3,
    "cache_hit": true,                  // Rete 特有：是否命中缓存
    "algorithm": "rete"
}
```

#### 反向推理
```
POST /api/inference/backward
Body: {
    "goal": "能飞行",                   // 目标结论
    "condition_set_id": 1,              // 可选：提供前提条件集
}
Response: {
    "success": true,
    "goal": "能飞行",
    "result": "succeed" | "failed" | "unknown",
    "condition_set_id": 1,
    "cache_hit": false,
    "steps": 5
}
```

---

## 五、推理执行流程

### 5.1 前向推理（以 Rete 为例）

```
1. 解析请求
   if condition_set_id:
       facts = load_condition_set(id)  # 从 DB 加载条件集
   else:
       facts = body.facts              # 直接使用传入 facts

2. 检查缓存
   cache_entry = rete_cache.get(condition_set_id, facts)
   if cache_entry:
       return { ..., cache_hit: true, ... }

3. 执行推理
   new_facts = rete.forward(facts)

4. 更新缓存
   if condition_set_id:
       rete_cache.set(condition_set_id, facts, {
           'facts': frozenset(facts),
           'deduced_facts': new_facts,
           ...
       })

5. 返回结果
```

### 5.2 条件集编辑后的缓存失效

```
前端 PUT /api/condition-sets/1
       ↓
后端 DB 更新成功
       ↓
engine.invalidate_condition_set_cache(1)   # Rete 缓存失效
       ↓
前端切换到推理页面
       ↓
POST /api/inference/forward { condition_set_id: 1 }
       ↓
Rete 检查缓存 → 未命中 → 重新计算 → 更新缓存
```

---

## 六、前端交互设计

### 6.1 条件集管理面板（新增 Tab）

```
┌─────────────────────────────────────────────────────┐
│  条件集管理                              [+ 新建条件集] │
├─────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────┐  │
│  │ 名称：哺乳动物条件        操作：[编辑] [删除]     │  │
│  │ 事实：有毛发、有犬齿、会哺乳                   │  │
│  │ 创建：2025-07-01 10:00                       │  │
│  └───────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────┐  │
│  │ 名称：鸟类条件              操作：[编辑] [删除]     │  │
│  │ 事实：有羽毛、是卵生                          │  │
│  └───────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
```

### 6.2 推理控制面板（改造）

```
┌─────────────────────────────────────────────────────┐
│  推理控制                                            │
├─────────────────────────────────────────────────────┤
│  算法：[全扫描] [增量触发] [Rete网络]                  │
├─────────────────────────────────────────────────────┤
│  条件输入模式：                                        │
│  ○ 直接输入事实  →  输入框 + 快速添加（现有逻辑）          │
│  ● 选择条件集    →  下拉选择已保存的条件集                │
│                                                     │
│  选择条件集：[▼ 哺乳动物条件        ]                   │
│                                                     │
│  快速添加：[有毛发] [有犬齿] [会哺乳] [有羽毛]           │
├─────────────────────────────────────────────────────┤
│                    [▶ 开始推理]                       │
└─────────────────────────────────────────────────────┘
```

### 6.3 状态管理（Store）

```typescript
// stores/conditionSet.ts

interface ConditionSet {
  id: number
  name: string
  facts: string[]
  created_at: string
  updated_at: string
}

// 新增 store
const useConditionSetStore = defineStore('conditionSet', () => {
  const sets = ref<ConditionSet[]>([])
  const selectedSetId = ref<number | null>(null)

  const selectedSet = computed(() =>
    sets.value.find(s => s.id === selectedSetId.value) ?? null
  )

  async function loadSets() { /* GET /api/condition-sets */ }
  async function createSet(name: string, facts: string[]) { /* POST */ }
  async function updateSet(id: number, data: Partial<ConditionSet>) { /* PUT */ }
  async function deleteSet(id: number) { /* DELETE */ }

  return { sets, selectedSetId, selectedSet, loadSets, createSet, updateSet, deleteSet }
})
```

---

## 七、实现计划

### Phase 1：后端条件集 CRUD（基础）
- [ ] 新增 `condition_sets` 表
- [ ] `AlgorithmEngine` 添加 `condition_sets` 管理方法
- [ ] 新增 REST API：POST/GET/PUT/DELETE `/api/condition-sets`
- [ ] 条件集编辑时触发 Rete 缓存失效

### Phase 2：推理接口改造（无状态化）
- [ ] 改造 `forward_chain()` 支持接收 facts 参数
- [ ] Rete 引擎实现按条件集 ID 的缓存层
- [ ] 推理 API 支持 `condition_set_id` 优先于直接 facts

### Phase 3：前端集成
- [ ] 新增条件集管理 Tab（增删改查）
- [ ] 推理控制面板支持选择条件集模式
- [ ] 推理结果关联条件集 ID 显示
- [ ] 条件集编辑后自动刷新推理结果

### Phase 4：优化
- [ ] 缓存序列化（支持重启后预热）
- [ ] 规则库变更时全局缓存失效
- [ ] 条件集克隆功能

---

## 八、扩展思考

### 8.1 条件集版本化
未来可支持条件集的历史版本，保存每次编辑的快照，方便回溯。

### 8.2 条件集导入/导出
支持 JSON 格式的导入导出，方便用户备份和分享条件集。

### 8.3 条件集对比
推理前对比两个条件集的推理结果差异，可视化展示哪些事实因条件不同而变化。

---

## 九、总结

| 维度 | 改动前 | 改动后 |
|------|--------|--------|
| 事实管理 | 工作内存，推理后残留 | 条件集，持久化 |
| 推理方式 | 有状态（依赖工作内存） | 无状态（每次传 facts 或条件集 ID） |
| Rete 缓存 | 无 | 按条件集 ID 缓存，增量匹配 |
| 用户体验 | 每次手动添加事实 | 保存条件集，一键推理 |
| API 兼容性 | 旧接口保留 | 新接口优先读 condition_set_id |
