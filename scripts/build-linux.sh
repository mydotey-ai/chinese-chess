#!/bin/bash
# Linux build script for Chinese Chess

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "Building Chinese Chess for Linux..."
./build.sh all --target=linux-x86_64 --release
