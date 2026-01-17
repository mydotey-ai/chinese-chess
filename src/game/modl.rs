use crate::board::{Board, coordinate::Coordinate, square::Color, square::PieceType, square::Square};
use crate::game::state::GameState;
use crate::game::rules::validate_move;
use crate::pieces::{king::King, chariot::Chariot};

#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
    current_turn: Color,
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        let mut board = Board::new();
        Self::initialize_pieces(&mut board);
        
        Self {
            board,
            current_turn: Color::Red,
            state: GameState::Playing,
        }
    }
    
    fn initialize_pieces(board: &mut Board) {
        // Red pieces (bottom of board)
        board.set_square(Coordinate::new(0, 0), Square::Occupied(PieceType::Chariot, Color::Red));
        board.set_square(Coordinate::new(1, 0), Square::Occupied(PieceType::Horse, Color::Red));
        board.set_square(Coordinate::new(2, 0), Square::Occupied(PieceType::Elephant, Color::Red));
        board.set_square(Coordinate::new(3, 0), Square::Occupied(PieceType::Advisor, Color::Red));
        board.set_square(Coordinate::new(4, 0), Square::Occupied(PieceType::King, Color::Red));
        board.set_square(Coordinate::new(5, 0), Square::Occupied(PieceType::Advisor, Color::Red));
        board.set_square(Coordinate::new(6, 0), Square::Occupied(PieceType::Elephant, Color::Red));
        board.set_square(Coordinate::new(7, 0), Square::Occupied(PieceType::Horse, Color::Red));
        board.set_square(Coordinate::new(8, 0), Square::Occupied(PieceType::Chariot, Color::Red));
        
        board.set_square(Coordinate::new(1, 2), Square::Occupied(PieceType::Cannon, Color::Red));
        board.set_square(Coordinate::new(7, 2), Square::Occupied(PieceType::Cannon, Color::Red));
        
        board.set_square(Coordinate::new(0, 3), Square::Occupied(PieceType::Soldier, Color::Red));
        board.set_square(Coordinate::new(2, 3), Square::Occupied(PieceType::Soldier, Color::Red));
        board.set_square(Coordinate::new(4, 3), Square::Occupied(PieceType::Soldier, Color::Red));
        board.set_square(Coordinate::new(6, 3), Square::Occupied(PieceType::Soldier, Color::Red));
        board.set_square(Coordinate::new(8, 3), Square::Occupied(PieceType::Soldier, Color::Red));
        
        // Black pieces (top of board)
        board.set_square(Coordinate::new(0, 9), Square::Occupied(PieceType::Chariot, Color::Black));
        board.set_square(Coordinate::new(1, 9), Square::Occupied(PieceType::Horse, Color::Black));
        board.set_square(Coordinate::new(2, 9), Square::Occupied(PieceType::Elephant, Color::Black));
        board.set_square(Coordinate::new(3, 9), Square::Occupied(PieceType::Advisor, Color::Black));
        board.set_square(Coordinate::new(4, 9), Square::Occupied(PieceType::King, Color::Black));
        board.set_square(Coordinate::new(5, 9), Square::Occupied(PieceType::Advisor, Color::Black));
        board.set_square(Coordinate::new(6, 9), Square::Occupied(PieceType::Elephant, Color::Black));
        board.set_square(Coordinate::new(7, 9), Square::Occupied(PieceType::Horse, Color::Black));
        board.set_square(Coordinate::new(8, 9), Square::Occupied(PieceType::Chariot, Color::Black));
        
        board.set_square(Coordinate::new(1, 7), Square::Occupied(PieceType::Cannon, Color::Black));
        board.set_square(Coordinate::new(7, 7), Square::Occupied(PieceType::Cannon, Color::Black));
        
        board.set_square(Coordinate::new(0, 6), Square::Occupied(PieceType::Soldier, Color::Black));
        board.set_square(Coordinate::new(2, 6), Square::Occupied(PieceType::Soldier, Color::Black));
        board.set_square(Coordinate::new(4, 6), Square::Occupied(PieceType::Soldier, Color::Black));
        board.set_square(Coordinate::new(6, 6), Square::Occupied(PieceType::Soldier, Color::Black));
        board.set_square(Coordinate::new(8, 6), Square::Occupied(PieceType::Soldier, Color::Black));
    }
    
    pub fn state(&self) -> GameState {
        self.state
    }
    
    pub fn current_turn(&self) -> Color {
        self.current_turn
    }
    
    pub fn is_valid_move(&self, from: Coordinate, to: Coordinate) -> bool {
        validate_move(&self.board, from, to, self.current_turn)
    }
    
    pub fn make_move(&mut self, from: Coordinate, to: Coordinate) -> bool {
        if !self.is_valid_move(from, to) {
            return false;
        }
        
        // Make move
        let piece = self.board.get_square(from).cloned();
        if let Some(piece) = piece {
            self.board.set_square(to, piece);
            self.board.set_square(from, Square::Empty);
            
            // Switch turn
            self.current_turn = match self.current_turn {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            };
            
            true
        } else {
            false
        }
    }
}