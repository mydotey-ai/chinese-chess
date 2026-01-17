use crate::board::coordinate::Coordinate;
use crate::board::square::{Square, Color, PieceType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Board {
    squares: HashMap<Coordinate, Square>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            squares: HashMap::new(),
        };
        
        // Initialize empty board
        for x in 0..9 {
            for y in 0..10 {
                board.squares.insert(Coordinate::new(x, y), Square::Empty);
            }
        }
        
        board
    }
    
    pub fn get_square(&self, coord: Coordinate) -> Option<&Square> {
        self.squares.get(&coord)
    }
    
    pub fn set_square(&mut self, coord: Coordinate, square: Square) {
        self.squares.insert(coord, square);
    }
    
    pub fn is_empty(&self) -> bool {
        self.squares.values().all(|s| matches!(s, Square::Empty))
    }
}