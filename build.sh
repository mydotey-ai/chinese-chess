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

# Build frontend
build_frontend() {
    show_info "Building frontend..."
    
    cd frontend || exit 1
    
    # Check and install dependencies if needed
    if [ ! -d "node_modules" ]; then
        show_info "Installing frontend dependencies..."
        npm install
        if [ $? -ne 0 ]; then
            show_error "Failed to install frontend dependencies"
            return 1
        fi
    fi
    
    # Build frontend
    show_info "Running frontend build..."
    npm run build
    if [ $? -ne 0 ]; then
        show_error "Frontend build failed"
        return 1
    fi
    
    cd .. || exit 1
    
    show_success "Frontend build completed"
    return 0
}

# Build backend (Rust)
build_backend() {
    local build_mode="$1"
    local target="$2"
    
    show_info "Building backend ($build_mode)..."
    
    # Build command
    local build_cmd="cargo build"
    
    if [ "$build_mode" = "release" ]; then
        build_cmd="$build_cmd --release"
    fi
    
    if [ -n "$target" ]; then
        build_cmd="$build_cmd --target=$target"
    fi
    
    show_debug "Running: $build_cmd"
    
    # Execute build
    eval $build_cmd
    if [ $? -ne 0 ]; then
        show_error "Backend build failed"
        return 1
    fi
    
    show_success "Backend build completed"
    return 0
}

# Build help function
show_build_help() {
    echo "Build Command Usage: $0 build [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --release             Build release version"
    echo "  --debug               Build debug version (default)"
    echo "  --target=<target>     Specify target platform"
    echo "  --no-frontend         Skip frontend build"
    echo "  --no-backend          Skip backend build"
    echo "  --help                Show this help"
    echo ""
    echo "Example:"
    echo "  $0 build --release --target=x86_64-unknown-linux-gnu"
}

# Main build function
build() {
    local build_mode="debug"
    local target=""
    local skip_frontend=false
    local skip_backend=false
    
    # Parse arguments
    while [ $# -gt 0 ]; do
        case "$1" in
            --release)
                build_mode="release"
                ;;
            --debug)
                build_mode="debug"
                ;;
            --target=*)
                target="${1#*=}"
                ;;
            --no-frontend)
                skip_frontend=true
                ;;
            --no-backend)
                skip_backend=true
                ;;
            --help)
                show_build_help
                return 0
                ;;
            *)
                show_error "Unknown option: $1"
                show_build_help
                return 1
                ;;
        esac
        shift
    done
    
    show_info "Starting build ($build_mode mode)..."
    
    # Build frontend
    if [ "$skip_frontend" = false ]; then
        build_frontend
        if [ $? -ne 0 ]; then
            return 1
        fi
    fi
    
    # Build backend
    if [ "$skip_backend" = false ]; then
        build_backend "$build_mode" "$target"
        if [ $? -ne 0 ]; then
            return 1
        fi
    fi
    
    # Show build info
    local output_dir="target/$build_mode"
    if [ -n "$target" ]; then
        output_dir="target/$target/$build_mode"
    fi
    
    if [ -f "$output_dir/chinese-chess" ]; then
        show_success "Build completed: $output_dir/chinese-chess"
        echo "  Size: $(du -h "$output_dir/chinese-chess" | cut -f1)"
        echo "  Type: $(file "$output_dir/chinese-chess" | cut -d: -f2-)"
    elif [ -f "$output_dir/chinese-chess.exe" ]; then
        show_success "Build completed: $output_dir/chinese-chess.exe"
        echo "  Size: $(du -h "$output_dir/chinese-chess.exe" | cut -f1)"
        echo "  Type: $(file "$output_dir/chinese-chess.exe" | cut -d: -f2-)"
    fi
}

# Package help function
show_package_help() {
    echo "Package Command Usage: $0 package [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --release             Build release version package"
    echo "  --debug               Build debug version package (default)"
    echo "  --target=<target>     Specify target platform"
    echo "  --no-frontend         Skip frontend build (use existing dist)"
    echo "  --binary-only         Only build binary, skip Tauri packaging"
    echo "  --help                Show this help"
    echo ""
    echo "Target platforms:"
    echo "  windows-x86_64        Windows 64-bit (.exe)"
    echo "  macos-x86_64          macOS Intel (.app, .dmg)"
    echo "  macos-aarch64         macOS Apple Silicon (.app, .dmg)"
    echo "  linux-x86_64          Linux 64-bit (.AppImage, .deb, .rpm)"
    echo ""
    echo "Example:"
    echo "  $0 package --release --target=linux-x86_64"
}

# Package using Tauri
package() {
    local build_mode="debug"
    local target=""
    local skip_frontend=false
    local binary_only=false
    
    # Parse arguments
    while [ $# -gt 0 ]; do
        case "$1" in
            --release)
                build_mode="release"
                ;;
            --debug)
                build_mode="debug"
                ;;
            --target=*)
                target="${1#*=}"
                ;;
            --no-frontend)
                skip_frontend=true
                ;;
            --binary-only)
                binary_only=true
                ;;
            --help)
                show_package_help
                return 0
                ;;
            *)
                show_error "Unknown option: $1"
                show_package_help
                return 1
                ;;
        esac
        shift
    done
    
    # If binary-only mode, just build without Tauri packaging
    if [ "$binary_only" = true ]; then
        show_info "Binary-only mode: building executable only, skipping Tauri packaging"
        build --$build_mode $([ -n "$target" ] && echo "--target=$target") --no-frontend
        return $?
    fi
    
    show_info "Starting package ($build_mode mode)..."
    
    # Check if Tauri CLI is available
    if ! command -v npx &> /dev/null; then
        show_error "npx is required for Tauri packaging"
        show_info "Installing Tauri CLI..."
        npm install -g @tauri-apps/cli
        if [ $? -ne 0 ]; then
            show_error "Failed to install Tauri CLI"
            return 1
        fi
    fi
    
    # Check if linuxdeploy is installed (for Linux packaging)
    if [[ "$target" == linux* ]] || [[ -z "$target" && "$(uname)" == "Linux" ]]; then
        if ! command -v linuxdeploy &> /dev/null; then
            show_warning "linuxdeploy is required for Linux packaging"
            show_info "Attempting to install linuxdeploy..."
            
            # Try to install linuxdeploy
            if command -v wget &> /dev/null; then
                wget -q https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage -O /tmp/linuxdeploy.AppImage
                chmod +x /tmp/linuxdeploy.AppImage
                sudo mv /tmp/linuxdeploy.AppImage /usr/local/bin/linuxdeploy
            elif command -v curl &> /dev/null; then
                curl -sL https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage -o /tmp/linuxdeploy.AppImage
                chmod +x /tmp/linuxdeploy.AppImage
                sudo mv /tmp/linuxdeploy.AppImage /usr/local/bin/linuxdeploy
            else
                show_error "Cannot install linuxdeploy: neither wget nor curl found"
                show_info "Please install linuxdeploy manually:"
                echo "  wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage"
                echo "  chmod +x linuxdeploy-x86_64.AppImage"
                echo "  sudo mv linuxdeploy-x86_64.AppImage /usr/local/bin/linuxdeploy"
                return 1
            fi
            
            if [ $? -eq 0 ]; then
                show_success "linuxdeploy installed successfully"
            else
                show_error "Failed to install linuxdeploy"
                return 1
            fi
        fi
    fi
    
    # Build frontend first
    if [ "$skip_frontend" = false ]; then
        build_frontend
        if [ $? -ne 0 ]; then
            return 1
        fi
    fi
    
    # Prepare Tauri build command
    local tauri_cmd="npx tauri build"
    
    if [ "$build_mode" = "release" ]; then
        tauri_cmd="$tauri_cmd --release"
    fi
    
    if [ -n "$target" ]; then
        # Convert simplified target names to Tauri targets
        case "$target" in
            windows*)
                tauri_cmd="$tauri_cmd --target=x86_64-pc-windows-msvc"
                ;;
            macos-x86_64)
                tauri_cmd="$tauri_cmd --target=x86_64-apple-darwin"
                ;;
            macos-aarch64)
                tauri_cmd="$tauri_cmd --target=aarch64-apple-darwin"
                ;;
            linux*)
                tauri_cmd="$tauri_cmd --target=x86_64-unknown-linux-gnu"
                ;;
            *)
                tauri_cmd="$tauri_cmd --target=$target"
                ;;
        esac
    fi
    
    show_debug "Running: $tauri_cmd"
    
    # Execute Tauri build
    eval $tauri_cmd
    local tauri_result=$?
    
    # Check if any packages were created even if some failed
    local packages_found=0
    local package_dirs=("target/$build_mode/bundle" "src-tauri/target/$build_mode/bundle")
    
    for pkg_dir in "${package_dirs[@]}"; do
        if [ -d "$pkg_dir" ]; then
            local count=$(find "$pkg_dir" -type f \( -name "*.deb" -o -name "*.rpm" -o -name "*.AppImage" -o -name "*.exe" -o -name "*.dmg" -o -name "*.msi" \) 2>/dev/null | wc -l)
            if [ "$count" -gt 0 ]; then
                packages_found=1
                break
            fi
        fi
    done
    
    if [ $tauri_result -ne 0 ]; then
        if [ $packages_found -eq 1 ]; then
            show_warning "Tauri packaging encountered errors, but some packages were created"
        else
            show_error "Tauri packaging failed"
            return 1
        fi
    fi
    
    # Show package info
    local package_dir="src-tauri/target/$build_mode"
    if [ -n "$target" ]; then
        package_dir="src-tauri/target/$target/$build_mode"
    fi
    
    show_success "Packaging completed"
    
    # List generated packages
    if [ -d "$package_dir" ]; then
        show_info "Generated packages:"
        find "$package_dir" -name "*.deb" -o -name "*.rpm" -o -name "*.msi" -o -name "*.app" -o -name "*.exe" -o -name "*.AppImage" -o -name "*.dmg" 2>/dev/null | while read -r pkg; do
            echo "  - $(basename "$pkg") ($(du -h "$pkg" | cut -f1))"
        done
    fi
}

# Release build function
release() {
    show_info "Starting release build and package..."
    
    # Build with release mode
    build --release
    if [ $? -ne 0 ]; then
        return 1
    fi
    
    # Package with release mode
    package --release --binary-only
    if [ $? -ne 0 ]; then
        show_warning "Tauri packaging failed, but binary build succeeded"
        return 0
    fi
    
    show_success "Release build and package completed"
}

# All-in-one function
all() {
    show_info "Starting full build and package pipeline..."
    
    # Clean first
    clean
    if [ $? -ne 0 ]; then
        return 1
    fi
    
    # Check dependencies
    check_dependencies
    if [ $? -ne 0 ]; then
        return 1
    fi
    
    # Build and package with release mode
    release
    if [ $? -ne 0 ]; then
        return 1
    fi
    
    show_success "Full pipeline completed"
}

# Main function
main() {
    # Parse main command
    local command="${1:-help}"
    shift 2>/dev/null || true
    
    # Execute command
    case "$command" in
        help)
            show_help
            ;;
        clean)
            clean "$@"
            ;;
        build)
            build "$@"
            ;;
        package)
            package "$@"
            ;;
        release)
            release "$@"
            ;;
        all)
            all "$@"
            ;;
        *)
            show_error "Unknown command: $command"
            show_help
            return 1
            ;;
    esac
}

# Script entry point
if [[ "${BASH_SOURCE[0]}" = "${0}" ]]; then
    main "$@"
fi
