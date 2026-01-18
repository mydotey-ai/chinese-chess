use crate::piece::Piece;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Board {
    pub cells: [[Option<Piece>; 9]; 10],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[None; 9]; 10],
        }
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<Piece> {
        if x < 9 && y < 10 {
            self.cells[y][x]
        } else {
            None
        }
    }

    pub fn set_piece(&mut self, x: usize, y: usize, piece: Option<Piece>) {
        if x < 9 && y < 10 {
            self.cells[y][x] = piece;
        }
    }

    pub fn move_piece(
        &mut self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
    ) -> Option<Piece> {
        let captured = self.get_piece(to_x, to_y);
        let piece = self.get_piece(from_x, from_y);
        self.set_piece(to_x, to_y, piece);
        self.set_piece(from_x, from_y, None);
        captured
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
