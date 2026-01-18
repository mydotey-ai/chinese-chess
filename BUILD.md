# 中国象棋应用程序 - 构建和启动说明

## 项目介绍

这是一个基于 Rust 和 Tauri 构建的中国象棋（Xiangqi）桌面应用程序。

## 快速开始

### 首次运行（完整流程）

```bash
# 使用 Bash 脚本（适用于 Linux/macOS）
./run.sh

# 使用批处理脚本（适用于 Windows）
run.bat
```

## 脚本命令说明

### 1. 完整流程（无参数）

```bash
./run.sh        # Linux/macOS
run.bat         # Windows
```

执行以下步骤：
- 检查 npm 和 Cargo 依赖
- 清理现有进程
- 安装前端依赖（如果 node_modules 不存在）
- 构建前端代码
- 构建 Rust 后端
- 启动 Vite 开发服务器
- 启动 Tauri 应用程序

### 2. 只运行应用程序

```bash
./run.sh run    # Linux/macOS
run.bat run     # Windows
```

如果应用程序尚未构建，会自动先进行构建。

### 3. 只构建发布版本

```bash
./run.sh build  # Linux/macOS
run.bat build   # Windows
```

构建发布版本，生成优化后的可执行文件。

### 4. 清理构建产物

```bash
./run.sh clean  # Linux/macOS
run.bat clean   # Windows
```

清理所有构建产物：
- 删除前端构建产物（frontend/dist）
- 删除 Rust 构建产物（target）
- 删除前端依赖（node_modules）
- 停止所有相关进程

### 5. 显示帮助信息

```bash
./run.sh help   # Linux/macOS
run.bat help    # Windows
```

显示所有可用命令和说明。

## 手动构建和运行

如果您需要手动执行步骤，可以使用以下命令：

### 1. 安装依赖

```bash
cd frontend
npm install
cd ..
```

### 2. 构建前端

```bash
cd frontend
npm run build
cd ..
```

### 3. 构建 Rust 后端

```bash
cargo build
```

### 4. 运行应用程序

```bash
cargo run
```

### 5. 构建发布版本

```bash
cargo build --release
```

## 应用程序功能

- 完整的中国象棋游戏逻辑
- 图形化界面
- 移动历史记录
- 撤销功能
- 检查和将死检测
- 支持 Red 和 Black 双方

## 技术栈

- **前端**: React + TypeScript + Vite
- **后端**: Rust + Tauri
- **构建工具**: Cargo, npm

## 系统要求

- Node.js 16 或更高版本
- Rust 1.56 或更高版本
- Tauri 环境

## 故障排除

如果遇到端口被占用的问题，可以：

1. 使用 `clean` 命令停止所有进程
2. 或者手动检查和释放端口：

```bash
# 检查端口 5173 是否被占用
lsof -ti :5173 | xargs kill -9

# 或者直接执行
pkill -f vite
pkill -f chinese-chess
```