# WayFind 前端测试指南

## 当前状态

✅ **已完成：**
- Go 后端代码编译成功
- Vue 前端构建成功
- Wails 对接代码已实现

❌ **待解决：**
- 系统缺少 GTK 3 和 WebKit2 依赖（桌面应用）

## 测试选项

### 选项 1：浏览器中测试前端（推荐用于开发）

即使没有系统依赖，也可以在浏览器中测试大部分功能：

#### 1.1 启动纯前端开发服务器

```bash
cd /home/wpp/homework/way_find/frontend
npm run dev
```

访问 http://localhost:5173 （或其他可用端口）

#### 1.2 可以测试的功能

- ✅ UI 布局和样式
- ✅ Vue Router 导航
- ✅ Pinia Store 状态管理
- ✅ 组件交互
- ✅ 本地存储（LocalStorage）

#### 1.3 无法测试的功能

- ❌ Wails Go 调用（会报错）
- ❌ 算法执行（因为在后端）
- ❌ 路径搜索（因为在后端）

### 选项 2：安装依赖后测试完整功能

#### 步骤 1：安装系统依赖

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libgtk-3-dev libwebkit2gtk-4.0-dev
```

#### 步骤 2：验证安装

```bash
cd /home/wpp/homework/way_find/backend
wails doctor
```

应该看到：
```
✓ libgtk-3: Installed
✓ libwebkit2gtk-4.0: Installed
```

#### 步骤 3：启动 Wails 开发模式

```bash
cd /home/wpp/homework/way_find/backend
wails dev
```

#### 步骤 4：测试完整功能

- ✅ 所有前端功能
- ✅ Wails 前后端通信
- ✅ Go 算法执行
- ✅ 桌面应用体验

## 快速测试清单

### 前端测试（浏览器）

- [ ] 访问 http://localhost:5173
- [ ] 主页（Dashboard）正常显示
- [ ] 导航到地图列表（Maps）
- [ ] 导航到 Display 列表（Displays）
- [ ] 地图编辑器 UI 正常
- [ ] Display 设置向导 UI 正常
- [ ] CSS 样式正常（深色主题）
- [ ] 响应式布局正常

### 完整测试（需要安装依赖）

- [ ] Wails 应用启动
- [ ] 创建新地图
- [ ] 编辑地图（添加墙壁）
- [ ] 设置起点和终点
- [ ] 初始化搜索算法
- [ ] 执行 BFS 搜索
- [ ] 执行 DFS 搜索
- [ ] 执行 A* 搜索
- [ ] 单步执行功能
- [ ] 即时执行功能
- [ ] 路径可视化
- [ ] 统计数据显示

## Wails API 测试

如果前端运行正常但 Wails 调用失败，检查浏览器控制台：

```javascript
// 在浏览器控制台中执行：
console.log(window.go_main_App_CreateMap)

// 应该输出：
// ƒ CreateMap() { [native code] }

// 如果输出 undefined，说明 Wails 未正确绑定
```

## 常见问题

### Q: 前端构建成功但页面空白

检查：
1. 浏览器控制台错误
2. 确保运行的是 `npm run dev`（开发）不是 `npm run build`（生产）
3. 检查是否有 TypeScript 错误

### Q: Wails 调用报错

预期错误（如果没有运行完整 Wails）：
```
Error: Wails runtime not available
```

这是正常的，因为没有启动 Wails 后端。

### Q: 端口被占用

```bash
# 查找占用端口的进程
lsof -i :5173

# 或者使用不同的端口
npm run dev -- --port 5174
```

## 下一步

1. **优先**：安装系统依赖（如果可以）
2. **其次**：先在浏览器中测试前端 UI
3. **之后**：测试完整的 Wails 集成

## 文档

- [WAILS_INTEGRATION.md](WAILS_INTEGRATION.md) - 完整的 Wails 集成文档
- [WAILS_QUICKSTART.md](WAILS_QUICKSTART.md) - 快速入门指南
- [TROUBLESHOOTING.md](TROUBLESHOOTING.md) - 故障排除指南

## 技术栈

- **前端**：Vue 3 + TypeScript + Vite
- **后端**：Go + Wails v2
- **状态管理**：Pinia
- **路由**：Vue Router
- **样式**：原生 CSS（无 Tailwind）
- **算法**：BFS, DFS, A*

## 总结

你现在有两种测试方式：

1. **快速方式**：在浏览器中测试前端 UI（立即可用）
2. **完整方式**：安装依赖后测试完整应用（推荐）

建议先测试前端 UI，然后安装依赖进行完整测试。
