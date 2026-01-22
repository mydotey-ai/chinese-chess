#!/bin/bash

# Chinese Chess Build and Package Script
# 中国象棋项目构建打包脚本

set -e  # Exit on error

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'  # No Color

# Display functions
show_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

show_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

show_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

show_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

show_debug() {
    if [ "$DEBUG" = "true" ]; then
        echo -e "${CYAN}[DEBUG]${NC} $1"
    fi
}

# Help function
show_help() {
    echo "Chinese Chess (Xiangqi) Build and Package Script"
    echo ""
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  help                  Show this help message"
    echo "  clean                 Clean all build artifacts"
    echo "  build                 Build the application (default: debug)"
    echo "  package               Build and package into executable"
    echo "  release               Build release version and package"
    echo "  all                   Full build and package pipeline"
    echo ""
    echo "Options for build/package commands:"
    echo "  --debug               Build debug version (default)"
    echo "  --release             Build release version"
    echo "  --target=<target>     Specify target platform"
    echo "  --no-frontend         Skip frontend build"
    echo "  --no-backend          Skip backend build"
    echo "  --verbose             Enable verbose output"
    echo "  --help                Show help for specific command"
    echo ""
    echo "Target platforms:"
    echo "  windows-x86_64        Windows 64-bit"
    echo "  macos-x86_64          macOS Intel"
    echo "  macos-aarch64         macOS Apple Silicon"
    echo "  linux-x86_64          Linux 64-bit"
    echo ""
    echo "Examples:"
    echo "  $0 clean                    # Clean all build artifacts"
    echo "  $0 build --release          # Build release version"
    echo "  $0 package --target=linux   # Package for Linux"
    echo "  $0 all                      # Full pipeline"
}

# Check dependencies
check_dependencies() {
    show_info "Checking dependencies..."
    
    local missing_deps=()
    
    # Check npm
    if ! command -v npm &> /dev/null; then
        missing_deps+=("npm (Node.js)")
    fi
    
    # Check cargo
    if ! command -v cargo &> /dev/null; then
        missing_deps+=("cargo (Rust)")
    fi
    
    # Check tauri-cli (optional, will install if missing)
    if ! command -v npx &> /dev/null; then
        missing_deps+=("npx (npm)")
    fi
    
    if [ ${#missing_deps[@]} -gt 0 ]; then
        show_error "Missing dependencies:"
        for dep in "${missing_deps[@]}"; do
            echo "  - $dep"
        done
        return 1
    fi
    
    show_success "All dependencies satisfied"
    return 0
}

# Clean function
clean() {
    show_info "Cleaning build artifacts..."
    
    # Clean frontend
    if [ -d "frontend/dist" ]; then
        show_info "Removing frontend dist..."
        rm -rf frontend/dist
    fi
    
    # Clean node_modules (optional)
    if [ "$1" = "all" ]; then
        if [ -d "frontend/node_modules" ]; then
            show_info "Removing frontend node_modules..."
            rm -rf frontend/node_modules
        fi
    fi
    
    # Clean Rust artifacts
    if [ -d "target" ]; then
        show_info "Cleaning Rust artifacts..."
        cargo clean
    fi
    
    # Clean Tauri artifacts
    if [ -d "src-tauri/target" ]; then
        show_info "Cleaning Tauri artifacts..."
        rm -rf src-tauri/target
    fi
    
    show_success "Clean completed"
}
