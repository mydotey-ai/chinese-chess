#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Square {
    Empty,
    Occupied(PieceType, Color),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    King,
    Advisor,
    Elephant,
    Horse,
    Chariot,
    Cannon,
    Soldier,
}
