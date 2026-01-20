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
            chinese_chess::tauri_commands::make_move,
            chinese_chess::tauri_commands::undo_move,
            chinese_chess::tauri_commands::get_valid_moves,
            chinese_chess::tauri_commands::get_game_state,
            chinese_chess::tauri_commands::new_game
        ]);

    app.run(tauri::generate_context!())
        .expect("failed to run tauri application");
}
