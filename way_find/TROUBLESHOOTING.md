# Wails 依赖问题解决

## 问题描述

运行 `wails dev` 时出现以下错误：

```
Package gtk+-3.0 was not found in the pkg-config search path.
Package 'webkit2gtk-4.0', required by 'virtual:world', not found
```

## 原因

系统缺少 Wails 桌面应用运行所需的依赖库：
- GTK 3.0
- WebKit2GTK 4.0
- Gio Unix 2.0

## 解决方案

### Ubuntu/Debian

```bash
sudo apt update
sudo apt install libgtk-3-dev libwebkit2gtk-4.0-dev
```

### Fedora/RHEL

```bash
sudo dnf install gtk3-devel webkit2gtk4.0-devel
```

### Arch Linux

```bash
sudo pacman -S gtk3 webkit2gtk
```

### macOS

WebKit 已在系统中预装，无需额外安装。

### Windows

安装 MinGW 或使用 MSYS2：

```bash
pacman -S mingw-w64-x86_64-gtk3 mingw-w64-x86_64-webkit2gtk-4.0
```

## 安装后验证

运行以下命令验证依赖是否安装成功：

```bash
cd /home/wpp/homework/way_find/backend
wails doctor
```

应该看到所有依赖项都标记为 "Installed"。

## 临时解决方案

如果暂时无法安装依赖，可以：

1. **在浏览器中开发**（推荐）：
   - Wails 已启动开发服务器
   - 访问 http://localhost:34115
   - 可以测试大部分前端功能
   - Go 后端调用会失败（因为没有运行）

2. **继续前端开发**：
   ```bash
   cd /home/wpp/homework/way_find/frontend
   npm run dev
   ```
   - 这会启动纯前端开发服务器
   - 所有 Wails 调用会失败
   - 但可以开发 UI 和逻辑

3. **安装依赖后**：
   ```bash
   cd /home/wpp/homework/way_find/backend
   wails dev
   ```

## 当前状态

- ✅ Go 代码编译成功
- ✅ 前端构建成功
- ✅ Vite 开发服务器运行正常
- ❌ 桌面应用缺少系统库
- ✅ 可以在浏览器中访问前端

## 下一步

1. 安装系统依赖（推荐）
2. 重新运行 `wails dev`
3. 测试完整的 Wails 集成

## 参考链接

- [Wails 官方文档 - Linux Requirements](https://wails.io/docs/gettingstarted/installation#linux-requirements)
- [GTK3 安装指南](https://www.gtk.org/docs/installations/linux/)
- [WebKitGTK 文档](https://webkitgtk.org/)
