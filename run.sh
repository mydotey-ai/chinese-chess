#!/bin/bash

# Chinese Chess Build and Run Script
# 一键构建和启动中国象棋应用程序的脚本

set -e  # 遇到错误时停止执行

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'  # 无颜色

# 显示消息的函数
show_info() {
    echo -e "${BLUE}INFO: $1${NC}"
}

show_success() {
    echo -e "${GREEN}SUCCESS: $1${NC}"
}

show_warning() {
    echo -e "${YELLOW}WARNING: $1${NC}"
}

show_error() {
    echo -e "${RED}ERROR: $1${NC}"
}

# 清理现有进程的函数
cleanup_processes() {
    show_info "正在清理现有进程..."
    pkill -f "vite" || true
    pkill -f "chinese-chess" || true
    sleep 2  # 等待进程完全结束
}

# 检查依赖项的函数
check_dependencies() {
    show_info "正在检查依赖项..."

    # 检查 npm 是否可用
    if ! command -v npm &> /dev/null; then
        show_error "npm 未找到，请安装 Node.js 和 npm"
        exit 1
    fi

    # 检查 cargo 是否可用
    if ! command -v cargo &> /dev/null; then
        show_error "cargo 未找到，请安装 Rust"
        exit 1
    fi

    show_success "所有依赖项已满足"
}

# 构建前端的函数
build_frontend() {
    show_info "正在构建前端..."

    cd frontend || exit 1

    # 检查 node_modules 是否存在，不存在则安装依赖
    if [ ! -d "node_modules" ]; then
        show_info "node_modules 不存在，正在安装依赖..."
        npm install
        if [ $? -ne 0 ]; then
            show_error "依赖安装失败"
            exit 1
        fi
    fi

    # 构建前端
    npm run build
    if [ $? -ne 0 ]; then
        show_error "前端构建失败"
        exit 1
    fi

    cd .. || exit 1

    show_success "前端构建成功"
}

# 构建 Rust 后端的函数
build_backend() {
    show_info "正在构建 Rust 后端..."

    # 构建开发版本
    cargo build
    if [ $? -ne 0 ]; then
        show_error "Rust 后端构建失败"
        exit 1
    fi

    show_success "Rust 后端构建成功"
}

# 启动开发服务器的函数
start_dev_server() {
    show_info "正在启动开发服务器..."

    # 启动 Vite 开发服务器（后台运行）
    cd frontend || exit 1
    npm run dev &
    VITE_PID=$!
    cd .. || exit 1

    # 等待服务器启动
    show_info "正在等待 Vite 服务器启动..."
    sleep 5

    # 检查服务器是否成功启动
    if ps -p $VITE_PID > /dev/null; then
        show_success "Vite 服务器已成功启动"
    else
        show_error "Vite 服务器启动失败"
        exit 1
    fi
}

# 启动 Tauri 应用程序的函数
start_tauri_app() {
    show_info "正在启动 Tauri 应用程序..."
    
    # 使用 Tauri 开发模式（会自动启动前端和后端）
    cd frontend || exit 1
    npm run tauri:dev
}

# 主函数
main() {
    show_info "开始构建和启动中国象棋应用程序"
    
    # 检查依赖项
    check_dependencies
    
    # 清理现有进程
    cleanup_processes
    
    # 安装前端依赖（如果需要）
    cd frontend || exit 1
    if [ ! -d "node_modules" ]; then
        show_info "node_modules 不存在，正在安装依赖..."
        npm install
        if [ $? -ne 0 ]; then
            show_error "依赖安装失败"
            exit 1
        fi
    fi
    cd .. || exit 1
    
    # 使用 Tauri 开发模式启动（会自动构建和运行前端和后端）
    start_tauri_app
}

# 脚本入口
if [ "$1" == "clean" ]; then
    # 清理模式：删除所有构建产物和进程
    show_info "进入清理模式..."
    cleanup_processes
    
    # 删除前端构建产物
    if [ -d "frontend/dist" ]; then
        show_info "正在删除前端构建产物..."
        rm -rf frontend/dist
    fi
    
    # 删除 Rust 构建产物
    if [ -d "target" ]; then
        show_info "正在删除 Rust 构建产物..."
        cargo clean
    fi
    
    # 删除 node_modules
    if [ -d "frontend/node_modules" ]; then
        show_info "正在删除前端依赖..."
        rm -rf frontend/node_modules
    fi
    
    show_success "清理完成"
elif [ "$1" == "build" ]; then
    # 只构建模式
    show_info "进入只构建模式..."
    check_dependencies
    cleanup_processes
    build_frontend
    cargo build --release
    
    if [ $? -eq 0 ]; then
        show_success "发布版本构建成功"
        show_info "可执行文件位置: $(pwd)/target/release/chinese-chess"
    else
        show_error "构建失败"
        exit 1
    fi
elif [ "$1" == "run" ]; then
    # 只运行模式（需要先构建）
    if [ ! -f "target/debug/chinese-chess" ]; then
        show_warning "未找到 debug 版本，正在构建..."
        build_frontend
        build_backend
    fi
    
    start_tauri_app
elif [ "$1" == "help" ]; then
    # 帮助信息
    echo "中国象棋应用程序构建和启动脚本"
    echo ""
    echo "用法: $0 [命令]"
    echo ""
    echo "命令:"
    echo "  help    显示此帮助信息"
    echo "  clean   清理所有构建产物和进程"
    echo "  build   构建发布版本"
    echo "  run     运行应用程序（如果需要会先构建）"
    echo "  (无参数) 执行完整流程：检查依赖、清理、构建、启动"
    echo ""
    echo "示例:"
    echo "  $0              # 执行完整流程"
    echo "  $0 build        # 只构建发布版本"
    echo "  $0 clean        # 清理所有构建产物"
    echo "  $0 help         # 显示帮助信息"
else
    # 无参数，执行完整流程
    main
fi