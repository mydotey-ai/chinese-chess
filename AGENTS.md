# AGENTS.md

This file provides guidance to agentic coding assistants working in this Chinese Chess (Xiangqi) repository.

## Development Commands

### Build
```bash
cargo build
```

### Release Build
```bash
cargo build --release
```

### Run Application
```bash
cargo run
```

### Run CLI
```bash
cargo run --bin chinese-chess-cli
```

### Testing
```bash
# Run all tests
cargo test

# Run a specific test
cargo test <test_name>

# Run tests with output
cargo test -- --nocapture

# Run tests with verbose output
cargo test -- --nocapture --verbose
```

### Code Quality
```bash
# Check code without building
cargo check

# Format code
cargo fmt

# Lint with Clippy
cargo clippy

# Mutation testing
cargo mutants
```

## Code Style Guidelines

### Rust Edition
This project uses Rust 2021 edition. Follow modern Rust conventions and idioms.

### Imports
- Group imports logically: std crates, external crates, local modules
- Use `use crate::module::Type` for local imports
- Keep imports at the top of files, organized in this order:
  1. Standard library
  2. External dependencies
  3. Local crate modules (crate::)
- Prefer specific imports over glob imports (`use item` over `use *`)

### Formatting
- Use `cargo fmt` before committing
- Maximum line length: 100 characters
- Use 4-space indentation (Rust default)
- Place opening braces on the same line for structs, enums, functions
- Use trailing commas in multi-line struct/enum definitions for better diffs

### Types
- Use `#[derive(Debug, Clone, PartialEq, Eq)]` for data types
- Add `#[derive(Serialize, Deserialize)]` for types used with serde
- Use `Copy` for small, trivial types (enums, simple structs)
- Prefer `Result<T, ChessError>` over `Option<T>` for operations that can fail
- Use `pub` visibility for types needed across modules
- Keep structs small and focused (SRP)

### Naming Conventions
- **Types**: `PascalCase` (e.g., `GameState`, `MoveValidator`)
- **Functions/Methods**: `snake_case` (e.g., `validate_move`, `make_move`)
- **Constants**: `SCREAMING_SNAKE_CASE` (rare in idiomatic Rust)
- **Enum Variants**: `PascalCase` (e.g., `Color::Red`, `PieceType::Horse`)
- **Fields**: `snake_case` (e.g., `piece_type`, `current_turn`)
- **Modules**: `snake_case` (e.g., `board`, `piece`, `rules`)
- Boolean fields: prefix with `is_`, `has_`, `can_` (e.g., `is_ended`, `is_in_check`)

### Error Handling
- Use the custom `ChessError` enum from `crate::ChessError` for domain errors
- Return `Result<T, ChessError>` for fallible operations
- Use `?` operator for early returns on errors
- Implement `std::fmt::Display` and `std::error::Error` for custom errors
- Avoid `unwrap()` and `expect()` in production code
- Use `unwrap()` only in tests where failure indicates a bug

### Structs and Enums
- Derive `Default` for types with sensible defaults
- Use builder pattern for complex construction if needed
- Document public types with `///` doc comments
- Mark fields as `pub` only when necessary
- Use tuple structs for simple wrappers, regular structs for data holders

### Functions and Methods
- Keep functions focused and small (< 50 lines ideally)
- Use `self` for read-only methods, `mut self` for consuming methods
- Use `&mut self` for in-place mutations
- Prefer returning values over mutable references where possible
- Use helper functions (non-pub) to reduce code duplication

### Comments and Documentation
- Use `///` for public API documentation
- Use `//!` for module-level documentation
- Keep inline comments minimal; let code be self-documenting
- Document non-obvious behavior and invariants
- Include examples in doc comments for complex types/functions

### Board Coordinate System
- Board size: 9 columns (x: 0-8) × 10 rows (y: 0-9)
- (0, 0) is top-left corner
- Black pieces start at y=0-4 (top)
- Red pieces starts at y=5-9 (bottom)
- River is between y=4 and y=5
- Palace (九宫格) for Black: x=3-5, y=0-2
- Palace (九宫格) for Red: x=3-5, y=7-9

### Game State Management
- `GameState` holds immutable game data (board, turn, status)
- `GameStateManager` manages mutable state and history
- All state changes go through `GameStateManager`
- History is tracked via `MoveRecord` for undo functionality

### Tauri Integration
- Use `tauri_commands` module for Tauri command handlers
- Implement `AsRef<GameState>` and `AsMut<GameState>` for Tauri state compatibility
- Keep Tauri commands separate from core game logic

### Testing
- Write unit tests in the same file as the code (mod tests)
- Use `cargo test <test_name>` to run specific tests
- Test edge cases (board boundaries, piece movement rules)
- Test error conditions (invalid moves, out of bounds)
- Use descriptive test names with `test_<what>_<when>_<expected>` pattern

### Performance Considerations
- Use `Copy` types for frequently moved small values
- Prefer references over cloning large structs
- Consider using `usize` for coordinates (fits with array indexing)
- Board operations should be O(1) or O(n) where n is board dimension
