# 构建打包脚本使用指南

## 快速开始

### 构建可执行文件（推荐）
```bash
./build.sh build --release
```

这会生成：
- `target/release/chinese-chess` (GUI版本，约8.3M)
- `target/release/chinese-chess-cli` (CLI版本，约1.1M)

### 完整打包（生成安装包）
```bash
./build.sh package --release
```

这会生成以下格式的安装包：
- `.deb` - Debian/Ubuntu系
- `.rpm` - RedHat/CentOS/Fedora系
- `.AppImage` - 通用Linux格式（可能需要额外配置）

## 当前状态

✅ **功能完整**
- 主构建脚本 (`build.sh`) - 完全实现
- 平台特定脚本 - 已创建（Windows/macOS/Linux）
- 文档 - 已更新（AGENTS.md, README-BUILD.md）

✅ **构建成功**
- 前端构建（React + Vite）- 成功
- 后端构建（Rust）- 成功
- 二进制文件 - 已生成

⚠️ **打包部分成功**
- `.deb` 包 - ✅ 成功生成（3.4M）
- `.rpm` 包 - ✅ 成功生成（3.4M）
- `.AppImage` - ❌ 需要额外配置（linuxdeploy）

## 构建脚本命令

```bash
./build.sh help                    # 显示帮助
./build.sh clean                   # 清理构建产物
./build.sh build [OPTIONS]         # 构建应用
./build.sh package [OPTIONS]       # 构建并打包
./build.sh release                 # 发布版本构建和打包
./build.sh all                     # 完整流程（清理→构建→打包）
```

## 选项说明

### 构建模式
- `--debug`（默认）：开发构建，包含调试信息
- `--release`：优化构建，体积更小，性能更好

### 目标平台
- `--target=windows-x86_64`：Windows 64位
- `--target=macos-x86_64`：macOS Intel
- `--target=macos-aarch64`：macOS Apple Silicon
- `--target=linux-x86_64`：Linux 64位

### 其他选项
- `--no-frontend`：跳过前端构建
- `--no-backend`：跳过后端构建
- `--binary-only`：只构建二进制，跳过Tauri打包
- `--verbose`：启用详细输出

## 生成的文件

### 二进制文件
- `target/release/chinese-chess` (Linux/macOS)
- `target/release/chinese-chess.exe` (Windows)
- `target/release/chinese-chess-cli` (CLI版本)

### 安装包
- `target/release/bundle/deb/*.deb` - Debian包
- `target/release/bundle/rpm/*.rpm` - RPM包
- `target/release/bundle/appimage/*.AppImage` - AppImage

## 故障排除

### linuxdeploy错误
如果遇到`failed to run linuxdeploy`错误：
1. 使用`--binary-only`选项只构建二进制
2. 手动安装linuxdeploy：
   ```bash
   wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
   chmod +x linuxdeploy-x86_64.AppImage
   sudo mv linuxdeploy-x86_64.AppImage /usr/local/bin/linuxdeploy
   ```

### Tauri打包部分失败
脚本现在会智能处理部分失败：
- 如果至少有一种包格式成功，会显示警告但继续
- 只有当所有格式都失败时才会返回错误

## Git提交历史

```bash
fb4b1d9 chore: add bundle directories to .gitignore
f662ec2 fix: handle partial Tauri packaging success gracefully
022650d docs: update README-BUILD.md with Linux packaging notes
676709f fix: add linuxdeploy check and binary-only mode for packaging
404dcba docs: update AGENTS.md and .gitignore with build script integration
67b0166 feat: add platform-specific build scripts and documentation
51d478c feat: complete build script with release and all-in-one functions
8f58472 feat: add packaging functionality with Tauri support
f2238f9 feat: add build functionality with frontend/backend support
dab5706 feat: add clean functionality to build script
3413e4a feat: add build script framework with dependency checks and help
```

## 总结

构建打包脚本系统已完全实现并经过测试。虽然AppImage生成需要额外配置，但：
- ✅ 二进制文件构建完全正常
- ✅ .deb和.rpm包可以正常生成
- ✅ 脚本能够优雅处理部分失败情况

对于大多数用例，使用`./build.sh build --release`生成的二进制文件已经足够。如果需要分发，.deb和.rpm包也已成功生成。
