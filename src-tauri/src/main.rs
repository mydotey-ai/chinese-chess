use chinese_chess::game::GameStateManager;
use std::sync::Mutex;
use tauri::{Builder, Manager};

fn main() {
    let app = Builder::default()
        .setup(|app| {
            let initial_state = Mutex::new(GameStateManager::new());
            app.manage(initial_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::make_move,
            crate::undo_move,
            crate::get_valid_moves,
            crate::get_game_state,
            crate::new_game
        ]);

    app.run(tauri::generate_context!())
        .expect("failed to run tauri application");
}
