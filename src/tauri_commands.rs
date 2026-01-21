use crate::game::GameState;
use crate::game::GameStateManager;
use crate::game_with_history::GameStateWithHistory;
use crate::ChessError;
use std::sync::Mutex;
use tauri::command;

#[command(rename_all = "camelCase")]
pub fn make_move(
    manager: tauri::State<'_, Mutex<GameStateManager>>,
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
) -> Result<GameStateWithHistory, ChessError> {
    let mut manager = manager.lock().unwrap();
    manager.make_move(from_x, from_y, to_x, to_y)?;
    Ok(GameStateWithHistory::new(
        manager.state.clone(),
        manager.history.clone(),
    ))
}

#[command(rename_all = "camelCase")]
pub fn undo_move(
    manager: tauri::State<'_, Mutex<GameStateManager>>,
) -> Result<GameStateWithHistory, ChessError> {
    let mut manager = manager.lock().unwrap();
    manager.undo_move()?;
    Ok(GameStateWithHistory::new(
        manager.state.clone(),
        manager.history.clone(),
    ))
}

#[command(rename_all = "camelCase")]
pub fn get_valid_moves(
    manager: tauri::State<'_, Mutex<GameStateManager>>,
    x: usize,
    y: usize,
) -> Vec<(usize, usize)> {
    manager.lock().unwrap().get_valid_moves(x, y)
}

#[command(rename_all = "camelCase")]
pub fn get_game_state(manager: tauri::State<'_, Mutex<GameStateManager>>) -> GameStateWithHistory {
    let manager = manager.lock().unwrap();
    GameStateWithHistory::new(manager.state.clone(), manager.history.clone())
}

#[command(rename_all = "camelCase")]
pub fn new_game(manager: tauri::State<'_, Mutex<GameStateManager>>) -> GameStateWithHistory {
    let mut manager = manager.lock().unwrap();
    manager.state = crate::game::GameState::new();
    manager.history.clear();
    GameStateWithHistory::new(manager.state.clone(), manager.history.clone())
}
