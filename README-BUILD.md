# Chinese Chess Build and Packaging Guide

This document describes how to build and package the Chinese Chess application.

## Quick Start

### Build and run in development mode:
```bash
./run.sh
```

### Build release version:
```bash
./build.sh release
```

### Full build and package pipeline:
```bash
./build.sh all
```

## Build Script Commands

### Main build script (`build.sh`)

The main build script supports the following commands:

```bash
./build.sh help                    # Show help
./build.sh clean                   # Clean all build artifacts
./build.sh build [OPTIONS]         # Build application
./build.sh package [OPTIONS]       # Build and package
./build.sh release                 # Build and package release version
./build.sh all                     # Full pipeline (clean → build → package)
```

### Platform-specific scripts

For convenience, platform-specific scripts are available:

```bash
scripts/build-windows.sh           # Build for Windows
scripts/build-macos.sh             # Build for macOS
scripts/build-linux.sh             # Build for Linux
```

## Build Options

### Build modes:
- `--debug` (default): Development build with debug symbols
- `--release`: Optimized release build

### Target platforms:
- `--target=windows-x86_64`: Windows 64-bit
- `--target=macos-x86_64`: macOS Intel
- `--target=macos-aarch64`: macOS Apple Silicon
- `--target=linux-x86_64`: Linux 64-bit

### Other options:
- `--no-frontend`: Skip frontend build
- `--no-backend`: Skip backend build
- `--verbose`: Enable verbose output

## Output Files

### Development builds:
- Binary: `target/debug/chinese-chess` (or `.exe` on Windows)

### Release builds:
- Binary: `target/release/chinese-chess` (or `.exe` on Windows)

### Packages:
- Linux: `.AppImage`, `.deb`, `.rpm` in `src-tauri/target/release/`
- Windows: `.exe`, `.msi` in `src-tauri/target/release/`
- macOS: `.app`, `.dmg` in `src-tauri/target/release/`

## Requirements

### Development:
- Node.js 18+ and npm
- Rust 1.70+

### Cross-compilation:
- For Windows on Linux/macOS: `rustup target add x86_64-pc-windows-msvc`
- For macOS on Linux: Requires macOS SDK (not available on Linux)

## Advanced Usage

### Build only the backend:
```bash
./build.sh build --no-frontend --release
```

### Package existing build:
```bash
./build.sh package --no-frontend --release
```

### Custom target:
```bash
./build.sh build --target=x86_64-unknown-linux-gnu --release
```

## Troubleshooting

### Common issues:

1. **Missing dependencies**: Run `./build.sh clean all` to clean everything, then rebuild
2. **Tauri build fails**: Make sure you have all required system dependencies for Tauri
3. **Cross-compilation fails**: Install required target with `rustup target add <target>`
