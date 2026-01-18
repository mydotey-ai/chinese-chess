use crate::piece::Piece;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoveRecord {
    pub from_x: usize,
    pub from_y: usize,
    pub to_x: usize,
    pub to_y: usize,
    pub captured_piece: Option<Piece>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct History {
    pub moves: Vec<MoveRecord>,
}

impl History {
    pub fn new() -> Self {
        Self { moves: Vec::new() }
    }

    pub fn push(&mut self, move_record: MoveRecord) {
        self.moves.push(move_record);
    }

    pub fn pop(&mut self) -> Option<MoveRecord> {
        self.moves.pop()
    }

    pub fn peek(&self) -> Option<&MoveRecord> {
        self.moves.last()
    }

    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
