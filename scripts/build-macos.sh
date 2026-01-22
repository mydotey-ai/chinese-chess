#!/bin/bash
# macOS build script for Chinese Chess

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "Building Chinese Chess for macOS..."
# Check architecture
ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
    ./build.sh all --target=macos-aarch64 --release
else
    ./build.sh all --target=macos-x86_64 --release
fi
