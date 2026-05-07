# Show-Web 前端实现总结

> 本文档总结了为 AI 作业展示系统创建的完整前端项目结构和功能。

---

## 📁 项目结构

```
show-web/
├── src/
│   ├── api/                           # API 服务层
│   │   ├── index.ts                  # API 基础配置
│   │   ├── professor.ts              # 专家系统 API
│   │   ├── wayfind.ts                # 寻路算法 API
│   │   └── mlearn.ts                # 机器学习 API
│   │
│   ├── components/                    # 公共组件
│   │   ├── layout/
│   │   │   └── AppHeader.vue        # 顶部导航栏
│   │   └── common/
│   │       ├── LoadingSpinner.vue   # 加载动画
│   │       ├── ErrorMessage.vue     # 错误提示
│   │       └── StatCard.vue         # 统计卡片
│   │
│   ├── stores/                        # Pinia 状态管理
│   │   ├── professor.ts             # 专家系统状态
│   │   ├── wayfind.ts               # 寻路算法状态
│   │   └── mlearn.ts                # 机器学习状态
│   │
│   ├── views/                         # 页面视图
│   │   ├── HomeView.vue             # 首页/概览
│   │   ├── ProfessorView.vue        # 专家系统页面
│   │   ├── WayFindView.vue          # 寻路算法页面
│   │   ├── MLearnView.vue           # 机器学习页面
│   │   └── NotFoundView.vue         # 404 页面
│   │
│   ├── router/
│   │   └── index.ts                 # 路由配置
│   │
│   ├── App.vue                       # 根组件
│   └── main.ts                       # 入口文件
│
├── ARCHITECTURE.md                   # 架构设计文档
└── package.json                      # 依赖配置
```

---

## 🎨 设计特色

### 1. 深色主题 + 高端美学
- **背景色**: `#09090b` (Zinc-950)
- **卡片色**: `#18181b` (Zinc-900)
- **边框色**: `#27272a` (Zinc-800)
- **主色调**: `#3b82f6` (蓝色)
- **成功色**: `#10b981` (绿色)
- **警告色**: `#f59e0b` (橙色)
- **危险色**: `#f43f5e` (红色)

### 2. 响应式设计
- 桌面端: 多列布局
- 平板端: 两列布局
- 移动端: 单列布局

### 3. 流畅动画
- 按钮悬停效果
- 卡片浮起效果
- 加载动画
- 过渡动画

---

## 🔌 API 服务层

### 专家系统 API (`professor.ts`)
```typescript
// 主要方法
- getServerInfo()           // 获取服务器信息
- getAlgorithmsStatus()     // 获取所有算法状态
- getRules(algorithm)       // 获取知识库规则
- addRule(conditions, conclusion, algorithm)  // 添加规则
- getFacts(algorithm)       // 获取当前事实
- addFact(fact, algorithm)  // 添加事实
- forwardInference(algorithm)  // 正向推理
- backwardInference(goal, algorithm)  // 反向推理
- getNetworkStats()         // Rete 网络统计
```

### 寻路算法 API (`wayfind.ts`)
```typescript
// 主要方法
- createMap(width, height)    // 创建地图
- updateCell(x, y, type)      // 更新单元格
- initSearch(algorithm)       // 初始化搜索
- stepSearch()                 // 单步执行
- runSearch()                  // 完整执行
- getSearchHistory()           // 获取搜索历史
```

### 机器学习 API (`mlearn.ts`)
```typescript
// 神经网络
- trainNN(config)              // 训练模型
- predictNN(modelId, inputs)   // 模型预测
- getModels()                  // 获取模型列表

// 遗传算法
- optimizeGA(config)           // 运行优化
- getGAStatus(jobId)           // 查询任务状态

// 可视化
- plot(chartConfig)            // 生成图表
```

---

## 📱 页面功能

### 1. 首页 (`HomeView.vue`)
- **Hero Section**: 大标题 + 副标题
- **项目卡片 Grid**: Bento 风格布局
- **四个项目入口**: 寻路算法、专家系统、神经网络、遗传算法
- **Hover 动画**: 卡片浮起效果

### 2. 专家系统页面 (`ProfessorView.vue`)
- **算法选择器**: Fullscan / Incremental / Rete
- **事实库管理**: 添加/删除事实
- **推理控制**: 正向推理、反向推理
- **推理步骤时间线**: 实时显示推理过程
- **统计信息**: 事实数、规则数、推理步数

### 3. 寻路算法页面 (`WayFindView.vue`)
- **地图编辑器**: 可点击编辑迷宫
- **画笔工具**: 道路、墙壁、起点、终点
- **算法选择**: BFS / DFS / A*
- **搜索控制**: 初始化、单步、运行、暂停
- **实时可视化**: 显示搜索过程和路径
- **统计信息**: 访问节点、路径长度、执行时间

### 4. 机器学习页面 (`MLearnView.vue`)
**神经网络 Tab:**
- **网络配置**: 输入/隐藏/输出维度、训练轮次、学习率
- **训练按钮**: 开始训练
- **收敛曲线**: Canvas 绘制
- **网络架构图**: 可视化显示

**遗传算法 Tab:**
- **优化配置**: 基准函数、维度、种群大小、迭代次数
- **优化按钮**: 开始优化
- **收敛曲线**: Canvas 绘制
- **算法参数**: 显示当前配置

---

## 🧩 组件说明

### 公共组件
| 组件 | 说明 |
|------|------|
| `LoadingSpinner` | 三环加载动画 |
| `ErrorMessage` | 错误提示框（可关闭） |
| `StatCard` | 统计数据卡片（标签+数值+单位） |

### 布局组件
| 组件 | 说明 |
|------|------|
| `AppHeader` | 顶部导航栏（桌面+移动端适配） |

---

## 🚀 运行方式

### 1. 安装依赖
```bash
cd show-web
npm install
```

### 2. 启动开发服务器
```bash
npm run dev
```

### 3. 构建生产版本
```bash
npm run build
```

---

## 🔗 依赖关系

### 需要启动的后端服务
1. **专家系统**: `cd professor && make start`
2. **寻路算法**: `cd way_find && make start` (待实现)
3. **机器学习**: `cd m-learn && make start` (待实现)

### 前端依赖
- `vue` (^3.0.0-beta)
- `vue-router` (^5.0.4)
- `pinia` (^3.0.4)
- `axios` (已安装，用于 HTTP 请求)

---

## 📝 后续工作

### 1. 后端 API 对接
- [ ] 启动寻路算法后端服务
- [ ] 启动机器学习后端服务
- [ ] 配置正确的 API 地址

### 2. 功能完善
- [ ] 添加更多的可视化图表
- [ ] 实现算法对比功能
- [ ] 添加数据持久化
- [ ] 实现用户交互反馈

### 3. 性能优化
- [ ] 添加虚拟滚动（大量数据）
- [ ] 优化 Canvas 渲染性能
- [ ] 添加骨架屏加载
- [ ] 图片懒加载

### 4. 测试
- [ ] 添加单元测试
- [ ] 添加 E2E 测试
- [ ] 性能测试

---

## 🎯 设计原则

遵循 `design-taste-frontend` skill 的设计原则：

1. **Deterministic Typography**: 清晰的字体层级
2. **Color Calibration**: 单色系 + 有限强调色
3. **Layout Diversification**: 非中心化布局
4. **Materiality & Shadows**: 适度使用阴影
5. **Interactive UI States**: 完整的交互状态
6. **Data & Form Patterns**: 标准表单设计
7. **Performance Guardrails**: 硬件加速动画

---

## 📄 文档

- [架构设计文档](./ARCHITECTURE.md) - 详细的技术架构说明
- [API 文档](../professor/API_DOC.md) - 专家系统 API 说明

---

**创建日期**: 2026-05-06  
**最后更新**: 2026-05-06
