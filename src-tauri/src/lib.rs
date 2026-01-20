use std::sync::Mutex;
use tauri::command;

// 从主 crate 导入
use crate::game::GameState;
use crate::game::GameStateManager;
use crate::ChessError;

#[command]
pub fn make_move(
    manager: tauri::State<'_, Mutex<GameStateManager>>,
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
) -> Result<GameState, ChessError> {
    let mut manager = manager.lock().unwrap();
    manager.make_move(from_x, from_y, to_x, to_y)?;
    Ok(manager.state.clone())
}

#[command]
pub fn undo_move(
    manager: tauri::State<'_, Mutex<GameStateManager>>,
) -> Result<GameState, ChessError> {
    let mut manager = manager.lock().unwrap();
    manager.undo_move()?;
    Ok(manager.state.clone())
}

#[command]
pub fn get_valid_moves(
    manager: tauri::State<'_, Mutex<GameStateManager>>,
    x: usize,
    y: usize,
) -> Vec<(usize, usize)> {
    manager.lock().unwrap().get_valid_moves(x, y)
}

#[command]
pub fn get_game_state(manager: tauri::State<'_, Mutex<GameStateManager>>) -> GameState {
    manager.lock().unwrap().state.clone()
}

#[command]
pub fn new_game(manager: tauri::State<'_, Mutex<GameStateManager>>) -> GameState {
    let mut manager = manager.lock().unwrap();
    manager.state = crate::game::GameState::new();
    manager.history.clear();
    manager.state.clone()
}
