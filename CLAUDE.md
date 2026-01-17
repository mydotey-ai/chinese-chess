# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Chinese Chess (Xiangqi) game implementation project. The repository is currently in initial setup phase.

## Technology Stack

- **Language**: Rust
- **Package Manager**: Cargo
- **License**: Apache 2.0

## Development Commands

### Build
```bash
cargo build
```

### Build with release optimizations
```bash
cargo build --release
```

### Run
```bash
cargo run
```

### Run tests
```bash
cargo test
```

### Run a specific test
```bash
cargo test <test_name>
```

### Run tests with output
```bash
cargo test -- --nocapture
```

### Check code without building
```bash
cargo check
```

### Format code
```bash
cargo fmt
```

### Lint
```bash
cargo clippy
```

### Mutation Testing
```bash
cargo mutants
```

## Current Status

The repository was just initialized. Core architecture and dependencies have not been established yet.
