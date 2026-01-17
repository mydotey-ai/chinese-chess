# Chinese Chess (Xiangqi) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a complete Chinese Chess (Xiangqi) game with core game logic, board representation, piece movement, and basic gameplay functionality.

**Architecture:**
- Modular design with separate crates for core game logic, board representation, and piece implementations
- TDD approach with comprehensive unit tests for all game rules and piece movements
- Clean, idiomatic Rust code following best practices
- Extensible architecture to support future features like AI opponents and GUI interfaces

**Tech Stack:**
- **Language:** Rust 1.70+
- **Package Manager:** Cargo
- **Testing:** Built-in Rust test framework with optional mutation testing via cargo-mutants
- **Linting:** Clippy for Rust code quality checks
- **Formatting:** rustfmt for consistent code style

---

## Task 1: Initialize Cargo Project Structure

**Files:**
- Create: `Cargo.toml` - Project manifest
- Create: `src/main.rs` - Main entry point
- Create: `src/lib.rs` - Library crate root

**Step 1: Initialize Cargo project**

Run: `cargo init --name chinese-chess --vcs git`
Expected: Creates initial Cargo project structure

**Step 2: Create basic project structure**

```toml
# Cargo.toml
[package]
name = "chinese-chess"
version = "0.1.0"
edition = "2021"
description = "A Chinese Chess (Xiangqi) game implementation in Rust"
license = "Apache-2.0"
repository = "https://github.com/koqizhao/chinese-chess"

[dependencies]

[dev-dependencies]
```

```rust
// src/main.rs
fn main() {
    println!("Chinese Chess (Xiangqi)");
}
```

```rust
// src/lib.rs
pub mod board;
pub mod pieces;
pub mod game;
```

**Step 3: Commit initial structure**

Run: `git add Cargo.toml src/main.rs src/lib.rs && git commit -m "init: initial project structure"`

---

## Task 2: Implement Board Representation

**Files:**
- Create: `src/board/mod.rs` - Board module
- Create: `src/board/square.rs` - Square type
- Create: `src/board/coordinate.rs` - Coordinate system
- Create: `tests/board.rs` - Board tests

**Step 1: Write failing tests for board basics**

```rust
// tests/board.rs
use chinese_chess::board::{Board, Coordinate, Square};

#[test]
fn test_board_creation() {
    let board = Board::new();
    assert!(!board.is_empty());
}

#[test]
fn test_coordinate_validation() {
    let valid = Coordinate::new(3, 5);
    assert!(valid.is_valid());
    
    let invalid = Coordinate::new(10, 5);
    assert!(!invalid.is_valid());
}

#[test]
fn test_square_access() {
    let board = Board::new();
    let square = board.get_square(Coordinate::new(0, 0)).unwrap();
    assert!(square.is_some());
}
```

**Step 2: Implement board module**

```rust
// src/board/coordinate.rs
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    
    pub fn is_valid(&self) -> bool {
        self.x < 9 && self.y < 10
    }
}

// src/board/square.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Square {
    Empty,
    Occupied(PieceType, Color),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    King,
    Advisor,
    Elephant,
    Horse,
    Chariot,
    Cannon,
    Soldier,
}

// src/board/mod.rs
use crate::pieces::Piece;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Board {
    squares: HashMap<Coordinate, Square>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            squares: HashMap::new(),
        };
        
        // Initialize empty board
        for x in 0..9 {
            for y in 0..10 {
                board.squares.insert(Coordinate::new(x, y), Square::Empty);
            }
        }
        
        board
    }
    
    pub fn get_square(&self, coord: Coordinate) -> Option<&Square> {
        self.squares.get(&coord)
    }
    
    pub fn set_square(&mut self, coord: Coordinate, square: Square) {
        self.squares.insert(coord, square);
    }
    
    pub fn is_empty(&self) -> bool {
        self.squares.values().all(|s| matches!(s, Square::Empty))
    }
}
```

**Step 3: Run tests to verify they pass**

Run: `cargo test tests/board.rs -v`
Expected: All tests pass

**Step 4: Commit board implementation**

Run: `git add src/board tests/board.rs && git commit -m "feat: implement board representation"`

---

## Task 3: Implement Piece Movements

**Files:**
- Create: `src/pieces/mod.rs` - Pieces module
- Create: `src/pieces/king.rs` - King piece implementation
- Create: `src/pieces/advisor.rs` - Advisor piece implementation
- Create: `src/pieces/elephant.rs` - Elephant piece implementation
- Create: `src/pieces/horse.rs` - Horse piece implementation
- Create: `src/pieces/chariot.rs` - Chariot piece implementation
- Create: `src/pieces/cannon.rs` - Cannon piece implementation
- Create: `src/pieces/soldier.rs` - Soldier piece implementation
- Create: `tests/pieces.rs` - Piece movement tests

**Step 1: Write failing tests for piece movements**

```rust
// tests/pieces.rs
use chinese_chess::board::{Board, Coordinate, Color, PieceType, Square};
use chinese_chess::pieces::{King, Advisor, Elephant, Horse, Chariot, Cannon, Soldier};

#[test]
fn test_king_movement() {
    let board = Board::new();
    let king = King::new(Color::Red);
    let start = Coordinate::new(4, 0);
    
    let valid_moves = king.valid_moves(start, &board);
    assert!(valid_moves.contains(&Coordinate::new(4, 1)));
    assert!(valid_moves.contains(&Coordinate::new(3, 0)));
    assert!(!valid_moves.contains(&Coordinate::new(5, 1)));
}

#[test]
fn test_chariot_movement() {
    let mut board = Board::new();
    let chariot = Chariot::new(Color::Black);
    let start = Coordinate::new(0, 9);
    
    let valid_moves = chariot.valid_moves(start, &board);
    assert!(valid_moves.contains(&Coordinate::new(0, 8)));
    assert!(valid_moves.contains(&Coordinate::new(1, 9)));
}
```

**Step 2: Implement piece movement traits**

```rust
// src/pieces/mod.rs
use crate::board::{Board, Coordinate, Color};

pub trait Piece {
    fn new(color: Color) -> Self;
    fn color(&self) -> Color;
    fn valid_moves(&self, from: Coordinate, board: &Board) -> Vec<Coordinate>;
}

pub mod king;
pub mod advisor;
pub mod elephant;
pub mod horse;
pub mod chariot;
pub mod cannon;
pub mod soldier;
```

**Step 3: Implement individual piece movements**

```rust
// src/pieces/king.rs
use crate::board::{Board, Coordinate, Color};
use crate::pieces::Piece;

#[derive(Debug, Clone)]
pub struct King {
    color: Color,
}

impl Piece for King {
    fn new(color: Color) -> Self {
        Self { color }
    }
    
    fn color(&self) -> Color {
        self.color
    }
    
    fn valid_moves(&self, from: Coordinate, board: &Board) -> Vec<Coordinate> {
        let mut moves = Vec::new();
        
        // King moves one square in any direction within palace
        let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        
        for (dx, dy) in delta.iter() {
            let new_x = (from.x as isize + dx) as usize;
            let new_y = (from.y as isize + dy) as usize;
            let coord = Coordinate::new(new_x, new_y);
            
            if Self::is_in_palace(coord, self.color) && coord.is_valid() {
                moves.push(coord);
            }
        }
        
        moves
    }
}

impl King {
    fn is_in_palace(coord: Coordinate, color: Color) -> bool {
        let (x_valid, y_valid) = match color {
            Color::Red => (3 <= coord.x && coord.x <= 5, 0 <= coord.y && coord.y <= 2),
            Color::Black => (3 <= coord.x && coord.x <= 5, 7 <= coord.y && coord.y <= 9),
        };
        
        x_valid && y_valid
    }
}

// src/pieces/chariot.rs
use crate::board::{Board, Coordinate, Color};
use crate::pieces::Piece;

#[derive(Debug, Clone)]
pub struct Chariot {
    color: Color,
}

impl Piece for Chariot {
    fn new(color: Color) -> Self {
        Self { color }
    }
    
    fn color(&self) -> Color {
        self.color
    }
    
    fn valid_moves(&self, from: Coordinate, board: &Board) -> Vec<Coordinate> {
        let mut moves = Vec::new();
        
        // Chariot moves horizontally and vertically
        moves.extend(Self::valid_horizontal_moves(from, board));
        moves.extend(Self::valid_vertical_moves(from, board));
        
        moves
    }
}

impl Chariot {
    fn valid_horizontal_moves(from: Coordinate, board: &Board) -> Vec<Coordinate> {
        let mut moves = Vec::new();
        
        // Left
        for x in (0..from.x).rev() {
            let coord = Coordinate::new(x, from.y);
            moves.push(coord);
            if !matches!(board.get_square(coord), Some(Square::Empty)) {
                break;
            }
        }
        
        // Right
        for x in from.x + 1..9 {
            let coord = Coordinate::new(x, from.y);
            moves.push(coord);
            if !matches!(board.get_square(coord), Some(Square::Empty)) {
                break;
            }
        }
        
        moves
    }
    
    fn valid_vertical_moves(from: Coordinate, board: &Board) -> Vec<Coordinate> {
        let mut moves = Vec::new();
        
        // Up
        for y in (0..from.y).rev() {
            let coord = Coordinate::new(from.x, y);
            moves.push(coord);
            if !matches!(board.get_square(coord), Some(Square::Empty)) {
                break;
            }
        }
        
        // Down
        for y in from.y + 1..10 {
            let coord = Coordinate::new(from.x, y);
            moves.push(coord);
            if !matches!(board.get_square(coord), Some(Square::Empty)) {
                break;
            }
        }
        
        moves
    }
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test tests/pieces.rs -v`
Expected: All tests pass

**Step 5: Commit piece implementations**

Run: `git add src/pieces tests/pieces.rs && git commit -m "feat: implement piece movement logic"`

---

## Task 4: Implement Game Logic

**Files:**
- Create: `src/game/mod.rs` - Game module
- Create: `src/game/state.rs` - Game state management
- Create: `src/game/rules.rs` - Game rules and validation
- Create: `tests/game.rs` - Game logic tests

**Step 1: Write failing tests for game logic**

```rust
// tests/game.rs
use chinese_chess::game::{Game, GameState};
use chinese_chess::board::{Coordinate, Color};

#[test]
fn test_game_initialization() {
    let mut game = Game::new();
    assert_eq!(game.state(), GameState::Playing);
    assert_eq!(game.current_turn(), Color::Red);
}

#[test]
fn test_valid_move() {
    let mut game = Game::new();
    let start = Coordinate::new(1, 0); // Red Horse
    let end = Coordinate::new(2, 2);
    
    assert!(game.is_valid_move(start, end));
}
```

**Step 2: Implement game logic**

```rust
// src/game/state.rs
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Playing,
    RedWins,
    BlackWins,
    Draw,
}

// src/game/rules.rs
use crate::board::{Board, Coordinate, Color, Square};
use crate::pieces::Piece;

pub fn validate_move(board: &Board, from: Coordinate, to: Coordinate, current_color: Color) -> bool {
    // Check coordinates are valid
    if !from.is_valid() || !to.is_valid() {
        return false;
    }
    
    // Check piece exists and matches current color
    let from_square = board.get_square(from);
    if let Some(Square::Occupied(_, color)) = from_square {
        if color != current_color {
            return false;
        }
    } else {
        return false;
    }
    
    // Check destination square
    let to_square = board.get_square(to);
    if let Some(Square::Occupied(_, color)) = to_square {
        if color == current_color {
            return false;
        }
    }
    
    true
}

// src/game/mod.rs
use crate::board::{Board, Coordinate, Color, PieceType, Square};
use crate::game::state::GameState;
use crate::game::rules::validate_move;
use crate::pieces::{King, Advisor, Elephant, Horse, Chariot, Cannon, Soldier};

#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
    current_turn: Color,
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        let mut board = Board::new();
        Self::initialize_pieces(&mut board);
        
        Self {
            board,
            current_turn: Color::Red,
            state: GameState::Playing,
        }
    }
    
    fn initialize_pieces(board: &mut Board) {
        // Red pieces (bottom of board)
        board.set_square(Coordinate::new(0, 0), Square::Occupied(PieceType::Chariot, Color::Red));
        board.set_square(Coordinate::new(1, 0), Square::Occupied(PieceType::Horse, Color::Red));
        board.set_square(Coordinate::new(2, 0), Square::Occupied(PieceType::Elephant, Color::Red));
        board.set_square(Coordinate::new(3, 0), Square::Occupied(PieceType::Advisor, Color::Red));
        board.set_square(Coordinate::new(4, 0), Square::Occupied(PieceType::King, Color::Red));
        board.set_square(Coordinate::new(5, 0), Square::Occupied(PieceType::Advisor, Color::Red));
        board.set_square(Coordinate::new(6, 0), Square::Occupied(PieceType::Elephant, Color::Red));
        board.set_square(Coordinate::new(7, 0), Square::Occupied(PieceType::Horse, Color::Red));
        board.set_square(Coordinate::new(8, 0), Square::Occupied(PieceType::Chariot, Color::Red));
        
        board.set_square(Coordinate::new(1, 2), Square::Occupied(PieceType::Cannon, Color::Red));
        board.set_square(Coordinate::new(7, 2), Square::Occupied(PieceType::Cannon, Color::Red));
        
        board.set_square(Coordinate::new(0, 3), Square::Occupied(PieceType::Soldier, Color::Red));
        board.set_square(Coordinate::new(2, 3), Square::Occupied(PieceType::Soldier, Color::Red));
        board.set_square(Coordinate::new(4, 3), Square::Occupied(PieceType::Soldier, Color::Red));
        board.set_square(Coordinate::new(6, 3), Square::Occupied(PieceType::Soldier, Color::Red));
        board.set_square(Coordinate::new(8, 3), Square::Occupied(PieceType::Soldier, Color::Red));
        
        // Black pieces (top of board)
        board.set_square(Coordinate::new(0, 9), Square::Occupied(PieceType::Chariot, Color::Black));
        board.set_square(Coordinate::new(1, 9), Square::Occupied(PieceType::Horse, Color::Black));
        board.set_square(Coordinate::new(2, 9), Square::Occupied(PieceType::Elephant, Color::Black));
        board.set_square(Coordinate::new(3, 9), Square::Occupied(PieceType::Advisor, Color::Black));
        board.set_square(Coordinate::new(4, 9), Square::Occupied(PieceType::King, Color::Black));
        board.set_square(Coordinate::new(5, 9), Square::Occupied(PieceType::Advisor, Color::Black));
        board.set_square(Coordinate::new(6, 9), Square::Occupied(PieceType::Elephant, Color::Black));
        board.set_square(Coordinate::new(7, 9), Square::Occupied(PieceType::Horse, Color::Black));
        board.set_square(Coordinate::new(8, 9), Square::Occupied(PieceType::Chariot, Color::Black));
        
        board.set_square(Coordinate::new(1, 7), Square::Occupied(PieceType::Cannon, Color::Black));
        board.set_square(Coordinate::new(7, 7), Square::Occupied(PieceType::Cannon, Color::Black));
        
        board.set_square(Coordinate::new(0, 6), Square::Occupied(PieceType::Soldier, Color::Black));
        board.set_square(Coordinate::new(2, 6), Square::Occupied(PieceType::Soldier, Color::Black));
        board.set_square(Coordinate::new(4, 6), Square::Occupied(PieceType::Soldier, Color::Black));
        board.set_square(Coordinate::new(6, 6), Square::Occupied(PieceType::Soldier, Color::Black));
        board.set_square(Coordinate::new(8, 6), Square::Occupied(PieceType::Soldier, Color::Black));
    }
    
    pub fn state(&self) -> GameState {
        self.state
    }
    
    pub fn current_turn(&self) -> Color {
        self.current_turn
    }
    
    pub fn is_valid_move(&self, from: Coordinate, to: Coordinate) -> bool {
        validate_move(&self.board, from, to, self.current_turn)
    }
    
    pub fn make_move(&mut self, from: Coordinate, to: Coordinate) -> bool {
        if !self.is_valid_move(from, to) {
            return false;
        }
        
        // Make move
        let piece = self.board.get_square(from).cloned();
        if let Some(piece) = piece {
            self.board.set_square(to, piece);
            self.board.set_square(from, Square::Empty);
            
            // Switch turn
            self.current_turn = match self.current_turn {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            };
            
            true
        } else {
            false
        }
    }
}
```

**Step 3: Run tests to verify they pass**

Run: `cargo test tests/game.rs -v`
Expected: All tests pass

**Step 4: Commit game logic implementation**

Run: `git add src/game tests/game.rs && git commit -m "feat: implement core game logic"`

---

## Task 5: Add Command-Line Interface (CLI)

**Files:**
- Modify: `src/main.rs` - Add CLI interface
- Add dependencies: `clap` and `colored` crates in Cargo.toml

**Step 1: Update Cargo.toml with dependencies**

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
colored = "2.0"
```

**Step 2: Implement CLI in main.rs**

```rust
// src/main.rs
use clap::{Parser, Subcommand};
use colored::*;
use chinese_chess::game::Game;
use chinese_chess::board::Coordinate;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a new game
    New,
    /// Make a move (format: from_x,from_y to_x,to_y)
    Move {
        from: String,
        to: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut game = Game::new();
    
    match &cli.command {
        Some(Commands::New) => {
            println!("{}", "New Chinese Chess (Xiangqi) game started!".green());
            println!("{}: {}", "Current Turn".blue(), format!("{:?}", game.current_turn()).red());
        }
        Some(Commands::Move { from, to }) => {
            if let (Some(from_coord), Some(to_coord)) = parse_coordinates(from, to) {
                if game.make_move(from_coord, to_coord) {
                    println!("{}", "Move successful!".green());
                    println!("{}: {}", "Current Turn".blue(), format!("{:?}", game.current_turn()).red());
                } else {
                    println!("{}", "Invalid move!".red());
                }
            } else {
                println!("{}", "Invalid coordinate format! Use x,y format.".red());
            }
        }
        None => {
            println!("{}", "Chinese Chess (Xiangqi)".bold().blue());
            println!("{}", "Use --help for available commands.".yellow());
        }
    }
}

fn parse_coordinates(from: &str, to: &str) -> (Option<Coordinate>, Option<Coordinate>) {
    let parse = |s: &str| {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() == 2 {
            let x: usize = parts[0].parse().ok()?;
            let y: usize = parts[1].parse().ok()?;
            Some(Coordinate::new(x, y))
        } else {
            None
        }
    };
    
    (parse(from), parse(to))
}
```

**Step 3: Test CLI functionality**

Run: `cargo run -- --help`
Expected: Displays help information

Run: `cargo run -- new`
Expected: Starts new game

Run: `cargo run -- move 1,0 2,2`
Expected: Makes valid move

**Step 4: Commit CLI implementation**

Run: `git add Cargo.toml src/main.rs && git commit -m "feat: add command-line interface"`

---

## Task 6: Run Comprehensive Tests

**Files:**
- All existing files

**Step 1: Run all tests**

Run: `cargo test -v`
Expected: All tests pass

**Step 2: Run mutation testing**

Run: `cargo mutants`
Expected: High mutation score (≥80%)

**Step 3: Check code quality**

Run: `cargo clippy`
Expected: No warnings or errors

**Step 4: Format code**

Run: `cargo fmt`
Expected: Code is properly formatted

**Step 5: Commit final state**

Run: `git add . && git commit -m "test: run comprehensive tests and fix issues"`

---

## Future Enhancements

**Potential features for future development:**

1. **GUI Interface:** Build a graphical user interface using a Rust GUI framework like egui or iced
2. **AI Opponent:** Implement AI players with different difficulty levels using minimax algorithm
3. **Game History:** Track and display move history with undo/redo functionality
4. **Online Multiplayer:** Add networked gameplay support
5. **Tournament Mode:** Support tournament-style gameplay with multiple matches
6. **Advanced Rules:** Implement special rules like "flying general" and "checkmate patterns"

**Architecture improvements:**
- Use bitboards for more efficient board representation
- Implement move generation using Zobrist hashing for performance
- Add support for PGN (Portable Game Notation) for game recording

---

**Plan complete and saved to `docs/plans/2025-01-17-chinese-chess-implementation.md`. Two execution options:**

**1. Subagent-Driven (this session)** - I dispatch fresh subagent per task, review between tasks, fast iteration

**2. Parallel Session (separate)** - Open new session with executing-plans, batch execution with checkpoints

**Which approach?**