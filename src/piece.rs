use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceType {
    General,  // 将/帅
    Advisor,  // 士/仕
    Elephant, // 象/相
    Horse,    // 马
    Chariot,  // 车
    Cannon,   // 炮
    Soldier,  // 卒/兵
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }
}
