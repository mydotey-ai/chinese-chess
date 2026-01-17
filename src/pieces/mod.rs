use crate::board::coordinate::Coordinate;
use crate::board::square::Color;

pub trait Piece {
    fn new(color: Color) -> Self;
    fn color(&self) -> Color;
    fn valid_moves(&self, from: Coordinate, board: &crate::board::Board) -> Vec<Coordinate>;
}

pub mod king;
// pub mod advisor;
// pub mod elephant;
// pub mod horse;
pub mod chariot;
// pub mod cannon;
// pub mod soldier;
