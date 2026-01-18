use crate::game::GameState;
use crate::game::GameStateManager;
use crate::ChessError;
use tauri::command;

#[command]
pub fn make_move(
    manager: tauri::State<GameStateManager>,
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
) -> Result<GameState, ChessError> {
    let mut manager = manager.inner().clone();
    manager.make_move(from_x, from_y, to_x, to_y)?;
    Ok(manager.state)
}

#[command]
pub fn undo_move(manager: tauri::State<GameStateManager>) -> Result<GameState, ChessError> {
    let mut manager = manager.inner().clone();
    manager.undo_move()?;
    Ok(manager.state)
}

#[command]
pub fn get_valid_moves(
    manager: tauri::State<GameStateManager>,
    x: usize,
    y: usize,
) -> Vec<(usize, usize)> {
    manager.inner().get_valid_moves(x, y)
}

#[command]
pub fn get_game_state(manager: tauri::State<GameStateManager>) -> GameState {
    manager.inner().state.clone()
}

#[command]
pub fn new_game() -> GameState {
    GameStateManager::new().state
}
