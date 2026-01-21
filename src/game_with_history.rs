use crate::game::GameState;
use crate::history::History;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameStateWithHistory {
    pub game_state: GameState,
    pub history: History,
}

impl GameStateWithHistory {
    pub fn new(game_state: GameState, history: History) -> Self {
        Self {
            game_state,
            history,
        }
    }
}
