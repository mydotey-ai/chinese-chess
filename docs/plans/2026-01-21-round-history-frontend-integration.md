# Round History Frontend Integration Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Update frontend App.tsx to support round-based move history by modifying backend Tauri commands to return GameStateWithHistory containing both game state and history data.

**Architecture:** Create a new GameStateWithHistory struct that combines existing GameState with History, modify get_game_state to return this type, and update frontend interfaces and logic to use the round-based history format.

**Tech Stack:** Rust (Tauri), TypeScript, React, serde for serialization

---

## Task 1: Create GameStateWithHistory Struct in Backend

**Files:**
- Create: `src/game_with_history.rs`
- Modify: `src/lib.rs` to include new module

**Step 1: Create the new struct file**

```rust
use serde::{Deserialize, Serialize};
use crate::game::GameState;
use crate::history::History;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameStateWithHistory {
    pub game_state: GameState,
    pub history: History,
}

impl GameStateWithHistory {
    pub fn new(game_state: GameState, history: History) -> Self {
        Self { game_state, history }
    }
}
```

**Step 2: Export the struct from lib.rs**

Modify: `src/lib.rs:4-10`
Add: `pub mod game_with_history;` after other module declarations

**Step 3: Run cargo check to verify**

Run: `cargo check`
Expected: No compilation errors

**Step 4: Commit**

```bash
git add src/game_with_history.rs src/lib.rs
git commit -m "feat: add GameStateWithHistory struct"
```

---

## Task 2: Modify Tauri Commands to Use GameStateWithHistory

**Files:**
- Modify: `src/tauri_commands.rs:1-50`
- Modify: `src-tauri/src/lib.rs:5-50`

**Step 1: Update imports in tauri_commands.rs**

Modify: `src/tauri_commands.rs:1-10`
```rust
use crate::game::GameState;
use crate::game_with_history::GameStateWithHistory;
use crate::history::History;
```

**Step 2: Update get_game_state command**

Modify: `src/tauri_commands.rs:39-41`
```rust
#[command(rename_all = "camelCase")]
pub fn get_game_state(manager: tauri::State<'_, Mutex<GameStateManager>>) -> GameStateWithHistory {
    let manager = manager.lock().unwrap();
    GameStateWithHistory::new(manager.state.clone(), manager.history.clone())
}
```

**Step 3: Update make_move command**

Modify: `src/tauri_commands.rs:14-16`
```rust
) -> Result<GameStateWithHistory, ChessError> {
```

Modify: `src/tauri_commands.rs:29-31`
```rust
) -> Result<GameStateWithHistory, ChessError> {
```

**Step 4: Update new_game command**

Modify: `src/tauri_commands.rs:44-49`
```rust
pub fn new_game(manager: tauri::State<'_, Mutex<GameStateManager>>) -> GameStateWithHistory {
    let mut manager = manager.lock().unwrap();
    manager.state = crate::game::GameState::new();
    manager.history.clear();
    GameStateWithHistory::new(manager.state.clone(), manager.history.clone())
}
```

**Step 5: Update src-tauri/lib.rs similarly**

Apply the same changes to `src-tauri/src/lib.rs`

**Step 6: Run tests to verify**

Run: `cargo test`
Expected: All tests pass

**Step 7: Commit**

```bash
git add src/tauri_commands.rs src-tauri/src/lib.rs
git commit -m "feat: update tauri commands to return GameStateWithHistory"
```

---

## Task 3: Update Frontend TypeScript Interfaces

**Files:**
- Modify: `frontend/src/App.tsx:8-15`

**Step 1: Update GameState interface to include history**

Modify: `frontend/src/App.tsx:8-15`
```typescript
interface GameState {
  board: any;
  current_turn: 'Red' | 'Black';
  is_in_check: boolean;
  is_ended: boolean;
  winner: 'Red' | 'Black' | null;
  history: HistoryData;
}

interface HistoryData {
  rounds: RoundRecord[];
}

interface RoundRecord {
  round_number: number;
  red_move: MoveRecord;
  black_move: MoveRecord | null;
}

interface MoveRecord {
  from_x: number;
  from_y: number;
  to_x: number;
  to_y: number;
  captured_piece: any | null;
}
```

**Step 2: Verify TypeScript compilation**

Run: `cd frontend && npm run build` or `npx tsc --noEmit`
Expected: No type errors

**Step 3: Commit**

```bash
git add frontend/src/App.tsx
git commit -m "feat: update frontend interfaces for round history"
```

---

## Task 4: Update Frontend History Logic

**Files:**
- Modify: `frontend/src/App.tsx:176-181` (addMoveToHistory function)
- Modify: `frontend/src/App.tsx:92-96` (handleMakeMove function)
- Modify: `frontend/src/App.tsx:100-106` (handleUndoMove function)

**Step 1: Remove old addMoveToHistory function**

Modify: `frontend/src/App.tsx:176-181`
Replace the entire addMoveToHistory function with:

```typescript
const updateHistoryFromState = (state: GameState) => {
  if (!state.history || !state.history.rounds) {
    setMoveHistory([]);
    return;
  }
  
  const historyStrings: string[] = [];
  
  state.history.rounds.forEach(round => {
    const redMoveStr = convertMoveRecordToNotation(round.red_move);
    
    if (round.black_move) {
      const blackMoveStr = convertMoveRecordToNotation(round.black_move);
      historyStrings.push(`${round.round_number}. ${redMoveStr} ${blackMoveStr}`);
    } else {
      historyStrings.push(`${round.round_number}. ${redMoveStr}`);
    }
  });
  
  setMoveHistory(historyStrings);
};

const convertMoveRecordToNotation = (move: MoveRecord): string => {
  if (!gameState) return '';
  
  const piece = gameState.board.cells[move.from_y][move.from_x];
  if (!piece) return '';
  
  const pieceName = getPieceName(piece);
  const colNum = piece.color === 'Red' ? (9 - move.from_x) : (move.from_x + 1);
  
  const dx = move.to_x - move.from_x;
  const dy = move.to_y - move.from_y;
  
  let action: string;
  let target: string;
  
  if (dx === 0) {
    action = piece.color === 'Red' ? 
      (dy > 0 ? '退' : '进') : 
      (dy < 0 ? '退' : '进');
    target = Math.abs(dy).toString();
  } else {
    const targetColNum = piece.color === 'Red' ? (9 - move.to_x) : (move.to_x + 1);
    
    if (dy === 0) {
      action = '平';
      target = targetColNum.toString();
    } else if ((piece.color === 'Red' && dy < 0) || (piece.color === 'Black' && dy > 0)) {
      action = '进';
      target = targetColNum.toString();
    } else {
      action = '退';
      target = targetColNum.toString();
    }
  }
  
  return `${pieceName}${colNum}${action}${target}`;
};
```

**Step 2: Update handleMakeMove to use new history**

Modify: `frontend/src/App.tsx:88-96`
```typescript
const state = await invoke<GameState>('make_move', { 
  fromX, fromY, toX, toY 
});
setGameState(state);
updateHistoryFromState(state);
```

**Step 3: Update handleUndoMove to use new history**

Modify: `frontend/src/App.tsx:100-106`
```typescript
const state = await invoke<GameState>('undo_move');
setGameState(state);
updateHistoryFromState(state);
```

**Step 4: Update initGame to use new history**

Modify: `frontend/src/App.tsx:27-30`
```typescript
const state = await invoke<GameState>('new_game');
setGameState(state);
updateHistoryFromState(state);
```

**Step 5: Test the application**

Run: `cargo run`
Expected: App loads, moves can be made, history shows round format

**Step 6: Commit**

```bash
git add frontend/src/App.tsx
git commit -m "feat: implement round history logic in frontend"
```

---

## Task 5: Update HistoryPanel Component

**Files:**
- Modify: `frontend/src/components/HistoryPanel.tsx`

**Step 1: Check HistoryPanel component**

Read `frontend/src/components/HistoryPanel.tsx` to see if it needs updates for round-based display

**Step 2: Update if needed**

If HistoryPanel expects simple string array, no changes needed. If it expects round-based data, update props interface.

**Step 3: Test integration**

Run: `cargo run`
Expected: History panel displays moves in round format (e.g., "1. 炮二平五 马8进7")

**Step 4: Commit**

```bash
git add frontend/src/components/HistoryPanel.tsx
git commit -m "feat: ensure HistoryPanel works with round format"
```

---

## Task 6: Comprehensive Testing

**Files:**
- All modified files

**Step 1: Run backend tests**

Run: `cargo test`
Expected: All tests pass

**Step 2: Run frontend build**

Run: `cd frontend && npm run build`
Expected: Successful build

**Step 3: Manual testing of all features**

Test:
1. New game starts with empty history
2. Red move creates round 1
3. Black move completes round 1
4. Multiple rounds work correctly
5. Undo moves correctly
6. Game end state

**Step 4: Fix any issues found**

**Step 5: Final commit**

```bash
git add -A
git commit -m "test: complete round history integration testing"
```

---

**Plan complete and saved to `docs/plans/2026-01-21-round-history-frontend-integration.md`. Two execution options:**

**1. Subagent-Driven (this session)** - I dispatch fresh subagent per task, review between tasks, fast iteration

**2. Parallel Session (separate)** - Open new session with executing-plans, batch execution with checkpoints

**Which approach?**