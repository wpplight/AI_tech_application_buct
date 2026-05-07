# WayFind - UI 设计规范

## 1. 设计理念

**主题**: 科学实验室 / 算法可视化
**风格**: 现代科技感、学术严谨、交互友好

## 2. 色彩系统

### 主色调
```css
:root {
  /* 背景色 - 深色主题 */
  --bg-primary: #0f1419;
  --bg-secondary: #1a1f26;
  --bg-tertiary: #242c36;
  --bg-card: #2d3640;

  /* 强调色 - 算法可视化 */
  --accent-blue: #3b82f6;      /* BFS - 广度优先 */
  --accent-green: #10b981;     /* DFS - 深度优先 */
  --accent-amber: #f59e0b;    /* A* - 启发式搜索 */

  /* 状态色 */
  --success: #22c55e;
  --warning: #eab308;
  --error: #ef4444;

  /* 文字色 */
  --text-primary: #f8fafc;
  --text-secondary: #94a3b8;
  --text-muted: #64748b;

  /* 边框 */
  --border: #334155;
  --border-light: #475569;
}
```

### 格子状态颜色
```css
.grid-cell {
  --cell-wall: #374151;        /* 墙壁 */
  --cell-empty: #1f2937;       /* 空地 */
  --cell-start: #10b981;       /* 起点 - 绿色 */
  --cell-end: #ef4444;         /* 终点 - 红色 */
  --cell-current: #f59e0b;      /* 当前探索 */
  --cell-visited: #6366f1;     /* 已访问 */
  --cell-path: #22d3ee;        /* 最短路径 */
  --cell-queue: #8b5cf6;       /* 队列中 */
  --cell-stack: #ec4899;       /* 栈中 */
}
```

## 3. 字体系统

### 主字体
- **标题**: Inter, -apple-system, sans-serif
- **正文**: Inter, -apple-system, sans-serif
- **代码/路径**: JetBrains Mono, Fira Code, monospace

### 字号
```css
--text-xs: 0.75rem;    /* 12px */
--text-sm: 0.875rem;   /* 14px */
--text-base: 1rem;     /* 16px */
--text-lg: 1.125rem;   /* 18px */
--text-xl: 1.25rem;    /* 20px */
--text-2xl: 1.5rem;     /* 24px */
--text-3xl: 1.875rem;   /* 30px */
```

## 4. 布局结构

### 主界面布局
```
┌─────────────────────────────────────────────────────────────┐
│  Header: Logo + 标题 + 控制按钮                              │
├────────────────┬────────────────────────────────────────────┤
│                │                                            │
│   Sidebar      │         Main Content                      │
│   控制面板      │         迷宫网格 + 状态显示                │
│                │                                            │
│   - 算法选择    │                                            │
│   - 速度控制    │                                            │
│   - 地图选择    │                                            │
│   - 操作按钮    │                                            │
│                │                                            │
│   统计信息      │                                            │
│   - 步数       │                                            │
│   - 访问节点    │                                            │
│   - 路径长度    │                                            │
│                │                                            │
└────────────────┴────────────────────────────────────────────┘
```

### 响应式断点
- **Desktop**: >= 1024px (三栏布局)
- **Tablet**: 768px - 1023px (两栏布局)
- **Mobile**: < 768px (单栏堆叠)

## 5. 组件设计

### 5.1 算法选择器
- **样式**: 卡片式单选组
- **状态**:
  - Default: 灰色边框
  - Hover: 边框高亮
  - Selected: 填充色 + 图标
- **动画**: 200ms ease transition

### 5.2 控制按钮
- **播放/暂停**: 主按钮, 圆形, 48px
- **单步执行**: 次要按钮, 40px
- **重置**: 次要按钮, 40px
- **速度滑块**: 0.5x - 10x

### 5.3 迷宫网格
- **格子大小**: 32px (desktop), 24px (tablet), 20px (mobile)
- **圆角**: 4px
- **间距**: 2px
- **动画**: 150ms scale + color transition
- **发光效果**: 选中格子 box-shadow glow

### 5.4 统计面板
- **布局**: 网格布局, 3列
- **卡片**: 圆角矩形, 背景 bg-card
- **数字**: 大字号, monospace 字体
- **标签**: 小字号, text-secondary

## 6. 交互动效

### 6.1 状态变化
- **颜色过渡**: 150ms ease-out
- **缩放**: scale(0.95) → scale(1)
- **发光**: box-shadow 0 0 8px color

### 6.2 按钮交互
- **Hover**: 背景色加深, scale(1.02)
- **Active**: scale(0.98), 背景色最深
- **Disabled**: opacity 0.5, cursor not-allowed

### 6.3 页面过渡
- **淡入淡出**: opacity 0 → 1, 300ms
- **滑入**: translateY(20px) → translateY(0), 400ms

## 7. 功能模块

### 7.1 地图编辑器
- 点击格子切换状态
- 支持拖拽绘制墙壁
- 支持设置起点/终点
- 预设地图选择器

### 7.2 算法控制
- 算法切换
- 播放/暂停
- 单步执行
- 速度控制 (0.5x - 10x)
- 重置

### 7.3 可视化显示
- 实时显示当前探索位置
- 已访问节点标记
- 最短路径高亮
- 队列/栈内容显示 (可选)

### 7.4 统计信息
- 总步数
- 已访问节点数
- 最终路径长度
- 算法运行时间

## 8. 图标系统

使用 Lucide Icons (轻量、现代化):

- **Play**: `▶`
- **Pause**: `⏸`
- **Step Forward**: `⏭`
- **Reset**: `↻`
- **Settings**: `⚙`
- **Map**: `🗺`
- **Clock**: `⏱`
- **Grid**: `⊞`

## 9. 动画帧率

- **VSync**: 60fps (requestAnimationFrame)
- **动画时长**: 150ms - 300ms
- **缓动函数**: cubic-bezier(0.4, 0, 0.2, 1)

## 10. 无障碍设计

- **对比度**: WCAG AA 标准
- **键盘导航**: 支持 Tab, Enter, Space
- **ARIA 标签**: 所有交互元素
- **焦点指示**: 明显的焦点环
- **屏幕阅读器**: 支持语义化 HTML

## 11. 技术栈

### 前端框架
- Vue 3 (Composition API)
- TypeScript
- Pinia (状态管理)
- Vue Router
- Tailwind CSS

### 构建工具
- Vite
- PostCSS
- Autoprefixer

### 桌面集成
- Wails v2 (Go backend)

## 12. 性能优化

- **虚拟列表**: 仅渲染可见区域格子
- **节流**: 搜索输入 300ms debounce
- **记忆化**: computed properties 缓存
- **懒加载**: 路由懒加载
- **CSS 优化**: 避免重排、重绘
