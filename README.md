# Session Context Manager

桌面端应用，用于整理和组织 Claude Code 与 OpenCode 的完整对话上下文。

![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue)
![License](https://img.shields.io/badge/License-MIT-green)

## 功能特性

### 核心功能
- **自动扫描** - 自动检测并读取 Claude Code 和 OpenCode 的会话文件
- **项目分组** - 按项目自动整理会话，方便管理
- **时间排序** - 所有会话按最近更新时间排序
- **全文搜索** - 快速搜索所有会话内容
- **Markdown 导出** - 将会话导出为 Markdown 格式，便于分享和回顾

### 跨平台支持
- **Windows + WSL2** - 自动检测 Windows 上的 WSL2 Ubuntu 安装，读取其中的会话
- **原生 Linux/macOS** - 直接读取本地会话文件
- **独立运行** - 生成可执行的安装包，无需开发环境

## 支持的会话来源

| 来源 | 会话路径 | 文件格式 |
|------|----------|----------|
| Claude Code (WSL2) | `\\wsl$\Ubuntu\home\{user}\.claude\projects\` | `.jsonl` |
| Claude Code (Linux) | `~/.claude/projects/` | `.jsonl` |
| OpenCode (WSL2) | `\\wsl$\Ubuntu\home\{user}\.config\opencode\sessions\` | `.json` |
| OpenCode (Linux) | `~/.config/opencode/sessions/` | `.json` |

## 安装

### 下载安装包

从 [GitHub Releases](https://github.com/HunterXing/session-context-manager/releases) 下载对应平台的安装包：

| 平台 | 安装包 | 说明 |
|------|--------|------|
| Windows | `.exe` (NSIS) / `.msi` | 推荐使用 NSIS 安装包 |
| macOS | `.dmg` | 需要 macOS 10.15+ |
| Linux | `.AppImage` / `.deb` | Ubuntu/Debian 系使用 .deb |

### 从源码构建

```bash
# 克隆项目
git clone https://github.com/HunterXing/session-context-manager.git
cd session-context-manager

# 安装依赖
npm install

# 开发模式运行
npm run tauri dev

# 构建安装包
npm run tauri build
```

## 使用方式

### 1. 启动应用
安装完成后，运行 "Session Context Manager"

### 2. 自动扫描
应用启动时会自动扫描以下位置：
- Claude Code 会话（按项目分组）
- OpenCode 会话

### 3. 浏览会话
- **左侧边栏** - 显示所有项目列表
- **中间列表** - 显示选中项目的会话，按时间排序
- **右侧预览** - 显示选中会话的完整内容

### 4. 搜索
在顶部搜索框输入关键词，实时搜索所有会话内容

### 5. 导出
- 选择单个会话，点击 "Export" 导出为 Markdown
- 选择项目后，可导出整个项目的会话

### 6. 设置
点击右上角设置图标可查看：
- 检测到的源路径
- 修改导出目录

## 项目结构

```
session-context-manager/
├── src/                      # React 前端
│   ├── components/           # UI 组件
│   │   ├── Sidebar.tsx      # 项目侧边栏
│   │   ├── SessionList.tsx   # 会话列表
│   │   ├── Preview.tsx       # 会话预览
│   │   ├── Toolbar.tsx      # 工具栏
│   │   └── Settings.tsx      # 设置弹窗
│   ├── hooks/
│   │   └── useSessions.ts    # Zustand 状态管理
│   ├── types/
│   │   └── index.ts         # TypeScript 类型定义
│   └── App.tsx              # 主应用组件
├── src-tauri/               # Rust 后端
│   └── src/
│       ├── commands/        # Tauri 命令
│       │   ├── scanner.rs    # 会话扫描
│       │   ├── parser.rs     # JSON/JSONL 解析
│       │   ├── search.rs     # 全文搜索
│       │   └── exporter.rs   # Markdown 导出
│       ├── models/           # 数据模型
│       └── lib.rs            # 命令入口
└── .github/
    └── workflows/            # CI/CD 自动构建
```

## 技术栈

| 层级 | 技术 | 说明 |
|------|------|------|
| 前端框架 | React 19 | UI 渲染 |
| 状态管理 | Zustand | 会话状态管理 |
| 样式 | Tailwind CSS | 现代化样式 |
| Markdown | react-markdown | 内容渲染 |
| 桌面框架 | Tauri 2.x | 跨平台桌面应用 |
| 后端语言 | Rust | 高性能后端 |
| 构建系统 | Cargo | Rust 编译 |

## WSL2 支持说明

在 Windows 上使用时，应用会自动检测 WSL2 Ubuntu：

1. 检测 `\\wsl$\Ubuntu` 是否存在
2. 查找 Ubuntu home 目录中的用户名
3. 自动扫描以下路径：
   - `~/.claude/projects/` - Claude Code 项目
   - `~/.config/opencode/sessions/` - OpenCode 会话

无需手动配置，会话路径会在设置中自动显示。

## 开发相关

### 环境要求
- Node.js 18+
- Rust 1.70+
- Windows: Visual Studio Build Tools
- Linux: pkg-config, libgtk-3-dev
- macOS: Xcode

### 运行测试
```bash
# 前端类型检查
npm run build

# Rust 测试
cargo test --workspace
```

## License

MIT License - 详见 [LICENSE](LICENSE) 文件

## Contributing

欢迎提交 Issue 和 Pull Request！
