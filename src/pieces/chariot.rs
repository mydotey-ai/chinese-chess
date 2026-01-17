use crate::board::modl::Board;
use crate::board::coordinate::Coordinate;
use crate::board::square::Color;
use crate::board::square::Square;
use crate::pieces::Piece;

#[derive(Debug, Clone)]
pub struct Chariot {
    color: Color,
}

impl Piece for Chariot {
    fn new(color: Color) -> Self {
        Self { color }
    }
    
    fn color(&self) -> Color {
        self.color
    }
    
    fn valid_moves(&self, from: Coordinate, board: &Board) -> Vec<Coordinate> {
        let mut moves = Vec::new();
        
        // Chariot moves horizontally and vertically
        moves.extend(Self::valid_horizontal_moves(from, board));
        moves.extend(Self::valid_vertical_moves(from, board));
        
        moves
    }
}

impl Chariot {
    fn valid_horizontal_moves(from: Coordinate, board: &Board) -> Vec<Coordinate> {
        let mut moves = Vec::new();
        
        // Left
        for x in (0..from.x).rev() {
            let coord = Coordinate::new(x, from.y);
            moves.push(coord);
            if !matches!(board.get_square(coord), Some(Square::Empty)) {
                break;
            }
        }
        
        // Right
        for x in from.x + 1..9 {
            let coord = Coordinate::new(x, from.y);
            moves.push(coord);
            if !matches!(board.get_square(coord), Some(Square::Empty)) {
                break;
            }
        }
        
        moves
    }
    
    fn valid_vertical_moves(from: Coordinate, board: &Board) -> Vec<Coordinate> {
        let mut moves = Vec::new();
        
        // Up
        for y in (0..from.y).rev() {
            let coord = Coordinate::new(from.x, y);
            moves.push(coord);
            if !matches!(board.get_square(coord), Some(Square::Empty)) {
                break;
            }
        }
        
        // Down
        for y in from.y + 1..10 {
            let coord = Coordinate::new(from.x, y);
            moves.push(coord);
            if !matches!(board.get_square(coord), Some(Square::Empty)) {
                break;
            }
        }
        
        moves
    }
}