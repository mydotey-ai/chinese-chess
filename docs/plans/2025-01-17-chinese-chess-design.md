# Chinese Chess Game Design

## Project Overview

A Chinese Chess (Xiangqi) desktop game built with Rust and Tauri, supporting local two-player mode with complete rule validation, check detection, and move history.

## Architecture

### MVC Architecture

**Model Layer (Rust)**
- `Board`: Chess board data structure and operations
- `Piece`: Piece types and colors
- `GameState`: Current game state (board, turn, check status, winner)
- `MoveValidator`: Move validation logic
- `GameStateManager`: History and undo functionality
- Pure Rust logic, independent of Tauri, fully testable

**View Layer (Tauri Frontend - TypeScript)**
- `ChessBoardView`: Board rendering
- `PieceView`: Piece display
- `HistoryPanel`: Move history
- HTML/CSS/JavaScript, board rendered via CSS Grid
- Components use modern framework (React/Vue/Solid)

**Controller Layer (Tauri Commands)**
- `#[tauri::command]` exposed APIs:
  - `make_move(from, to)`: Execute a move
  - `undo_move()`: Undo last move
  - `get_valid_moves(position)`: Get valid target positions
  - `get_game_state()`: Get current game state
- Receives frontend requests, calls Model, returns results

## Core Data Structures

### Board Representation
- 9×10 grid (90 positions) as 2D or 1D array
- Each position: `Option<Piece>` (empty = None)
- `PieceType`: 将/帅, 士/仕, 象/相, 马, 车, 炮, 卒/兵
- `Color`: Red, Black

### Game State
- `GameState` struct: board, current turn, is_in_check, is_ended, winner
- `Board`: encapsulates board data with `get_piece`, `set_piece`, `move_piece` methods

### Move History
- `MoveRecord`: from, to, captured_piece (if any), timestamp
- `History`: `Vec<MoveRecord>` with `push`, `pop`, `peek` operations

### Move Validation
- Each piece type has independent validation logic
- Horse: "日" shape, check for blocked legs
- Elephant: "田" shape, check for blocked eye, cannot cross river
- Cannon: move like rook, capture by jumping over one piece
- Chariot: straight lines, no jumping
- Guard/General: within palace (3×3 area), one step diagonal
- Soldier: forward only, one step; can move sideways after crossing river

## Component Design

### Frontend TypeScript Components
- `ChessBoard`: Main board, renders 90 squares and pieces, handles clicks
- `Piece`: Individual piece, displays Chinese character and color
- `HistoryPanel`: List of move records
- `GameStatus`: Current turn, check status, game result
- `ControlPanel`: "Restart", "Undo" buttons

### Backend Rust Modules
- `board.rs`: Board structure and operations
- `piece.rs`: Piece types and related logic
- `rules.rs`: All move validation rules
- `validator.rs`: Unified validation interface
- `game.rs`: Game state, turn switching, check detection, win/loss
- `history.rs`: History records and undo logic
- `tauri_commands.rs`: All exposed commands

### File Structure
- Rust: `src/` organized by modules
- Frontend: `frontend/` or `src/` (depending on Tauri template)

## Data Flow and Interaction

### Move Flow
1. User clicks square → frontend calls `get_valid_moves(position)`
2. Backend validates piece ownership → returns valid targets
3. Frontend highlights valid positions
4. User clicks target → frontend calls `make_move(from, to)`
5. Backend: updates board, records history, detects check, switches turn
6. Backend returns new state → frontend re-renders

### Undo Flow
1. User clicks "Undo" → frontend calls `undo_move()`
2. Backend pops last move → restores captured piece, board, turn
3. Backend returns restored state → frontend refreshes

### Check Detection
After each move, check if General is under attack:
- Enumerate all opponent pieces
- Check if each can attack the General's position
- If in check with no valid moves → checkmate (loss)

### State Synchronization
Frontend maintains local state copy, syncs after each operation. Reduces Tauri calls, improves responsiveness.

## Error Handling

### Error Types: `ChessError`
- `InvalidMove`: Doesn't follow piece rules
- `OutOfBoard`: Position outside board
- `NotYourPiece`: Not your turn's piece
- `CannotCaptureOwnPiece`: Cannot capture own piece
- `InCheck`: Move leaves you in check
- `NoHistory`: No history to undo
- `GameEnded`: Game over, no more moves

### Error Propagation
- Rust: `Result<T, ChessError>`
- Tauri: `Result<GameState, ChessError>`
- Frontend: Display user-friendly messages

### Edge Cases
- Perpetual check (长将): Warn or disallow
- Repetition: Track positions, threefold repetition = draw
- Stalemate: No valid moves, not in check = loss

## Testing Strategy

### Unit Tests
- `rules.rs`: Each piece's move rules (blocked legs, blocked eyes, etc.)
- `validator.rs`: Combined validation
- `history.rs`: Add, pop, restore operations
- `game.rs`: Turn switching, check detection, win/loss

### Integration Tests
- Full game flow validation
- Edge cases (horse blocked legs, elephant river crossing, palace constraints)
- Check and checkmate scenarios

### Frontend Tests
- UI interaction (clicks, highlighting, state updates)
- Tauri command integration (with mocks)

### Mutation Tests
- Use `cargo mutants` to verify test coverage

## Features

### MVP Scope
- Complete move rules for all pieces
- Check detection
- Move history and undo
- Local two-player mode

### Future (Out of Scope)
- AI opponent
- Online multiplayer
- Rule learning mode
- Sound effects
- Move animations
