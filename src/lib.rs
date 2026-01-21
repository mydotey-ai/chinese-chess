pub mod board;
pub mod game;
pub mod game_with_history;
pub mod history;
pub mod piece;
pub mod rules;
pub mod tauri_commands;
pub mod validator;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChessError {
    InvalidMove,
    OutOfBoard,
    NotYourPiece,
    CannotCaptureOwnPiece,
    InCheck,
    NoHistory,
    GameEnded,
}

impl std::fmt::Display for ChessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChessError::InvalidMove => write!(f, "Invalid move"),
            ChessError::OutOfBoard => write!(f, "Position out of board"),
            ChessError::NotYourPiece => write!(f, "Not your piece"),
            ChessError::CannotCaptureOwnPiece => write!(f, "Cannot capture your own piece"),
            ChessError::InCheck => write!(f, "You are in check"),
            ChessError::NoHistory => write!(f, "No move history"),
            ChessError::GameEnded => write!(f, "Game has ended"),
        }
    }
}

impl std::error::Error for ChessError {}
