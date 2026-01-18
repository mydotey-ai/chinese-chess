use crate::board::Board;
use crate::history::{History, MoveRecord};
use crate::piece::{Color, Piece, PieceType};
use crate::validator::MoveValidator;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    pub board: Board,
    pub current_turn: Color,
    pub is_in_check: bool,
    pub is_ended: bool,
    pub winner: Option<Color>,
}

impl GameState {
    pub fn new() -> Self {
        let mut board = Board::new();
        Self::initialize_pieces(&mut board);

        Self {
            board,
            current_turn: Color::Red,
            is_in_check: false,
            is_ended: false,
            winner: None,
        }
    }

    fn initialize_pieces(board: &mut Board) {
        // Red pieces (bottom, y=0)
        board.set_piece(0, 0, Some(Piece::new(PieceType::Chariot, Color::Red)));
        board.set_piece(1, 0, Some(Piece::new(PieceType::Horse, Color::Red)));
        board.set_piece(2, 0, Some(Piece::new(PieceType::Elephant, Color::Red)));
        board.set_piece(3, 0, Some(Piece::new(PieceType::Advisor, Color::Red)));
        board.set_piece(4, 0, Some(Piece::new(PieceType::General, Color::Red)));
        board.set_piece(5, 0, Some(Piece::new(PieceType::Advisor, Color::Red)));
        board.set_piece(6, 0, Some(Piece::new(PieceType::Elephant, Color::Red)));
        board.set_piece(7, 0, Some(Piece::new(PieceType::Horse, Color::Red)));
        board.set_piece(8, 0, Some(Piece::new(PieceType::Chariot, Color::Red)));

        board.set_piece(1, 2, Some(Piece::new(PieceType::Cannon, Color::Red)));
        board.set_piece(7, 2, Some(Piece::new(PieceType::Cannon, Color::Red)));

        board.set_piece(0, 3, Some(Piece::new(PieceType::Soldier, Color::Red)));
        board.set_piece(2, 3, Some(Piece::new(PieceType::Soldier, Color::Red)));
        board.set_piece(4, 3, Some(Piece::new(PieceType::Soldier, Color::Red)));
        board.set_piece(6, 3, Some(Piece::new(PieceType::Soldier, Color::Red)));
        board.set_piece(8, 3, Some(Piece::new(PieceType::Soldier, Color::Red)));

        // Black pieces (top, y=9)
        board.set_piece(0, 9, Some(Piece::new(PieceType::Chariot, Color::Black)));
        board.set_piece(1, 9, Some(Piece::new(PieceType::Horse, Color::Black)));
        board.set_piece(2, 9, Some(Piece::new(PieceType::Elephant, Color::Black)));
        board.set_piece(3, 9, Some(Piece::new(PieceType::Advisor, Color::Black)));
        board.set_piece(4, 9, Some(Piece::new(PieceType::General, Color::Black)));
        board.set_piece(5, 9, Some(Piece::new(PieceType::Advisor, Color::Black)));
        board.set_piece(6, 9, Some(Piece::new(PieceType::Elephant, Color::Black)));
        board.set_piece(7, 9, Some(Piece::new(PieceType::Horse, Color::Black)));
        board.set_piece(8, 9, Some(Piece::new(PieceType::Chariot, Color::Black)));

        board.set_piece(1, 7, Some(Piece::new(PieceType::Cannon, Color::Black)));
        board.set_piece(7, 7, Some(Piece::new(PieceType::Cannon, Color::Black)));

        board.set_piece(0, 6, Some(Piece::new(PieceType::Soldier, Color::Black)));
        board.set_piece(2, 6, Some(Piece::new(PieceType::Soldier, Color::Black)));
        board.set_piece(4, 6, Some(Piece::new(PieceType::Soldier, Color::Black)));
        board.set_piece(6, 6, Some(Piece::new(PieceType::Soldier, Color::Black)));
        board.set_piece(8, 6, Some(Piece::new(PieceType::Soldier, Color::Black)));
    }
}

#[derive(Debug, Clone)]
pub struct GameStateManager {
    pub state: GameState,
    pub history: History,
}

impl GameStateManager {
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
            history: History::new(),
        }
    }

    pub fn make_move(
        &mut self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
    ) -> Result<(), crate::ChessError> {
        if self.state.is_ended {
            return Err(crate::ChessError::GameEnded);
        }

        // Validate move
        MoveValidator::validate(
            &self.state.board,
            from_x,
            from_y,
            to_x,
            to_y,
            self.state.current_turn,
        )?;

        // Make move
        let captured_piece = self.state.board.move_piece(from_x, from_y, to_x, to_y);

        // Record move
        self.history.push(MoveRecord {
            from_x,
            from_y,
            to_x,
            to_y,
            captured_piece,
        });

        // Switch turn
        self.state.current_turn = match self.state.current_turn {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };

        // Check if in check
        self.state.is_in_check = self.is_in_check(self.state.current_turn);

        // Check if game ended
        if self.is_checkmate(self.state.current_turn) {
            self.state.is_ended = true;
            self.state.winner = Some(match self.state.current_turn {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            });
        }

        Ok(())
    }

    pub fn undo_move(&mut self) -> Result<(), crate::ChessError> {
        if self.history.is_empty() {
            return Err(crate::ChessError::NoHistory);
        }

        let last_move = self.history.pop().unwrap();

        // Restore piece
        let piece = self
            .state
            .board
            .get_piece(last_move.to_x, last_move.to_y)
            .unwrap();
        self.state
            .board
            .set_piece(last_move.from_x, last_move.from_y, Some(piece));
        self.state
            .board
            .set_piece(last_move.to_x, last_move.to_y, last_move.captured_piece);

        // Switch turn back
        self.state.current_turn = match self.state.current_turn {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };

        // Check if in check
        self.state.is_in_check = self.is_in_check(self.state.current_turn);

        self.state.is_ended = false;
        self.state.winner = None;

        Ok(())
    }

    pub fn get_valid_moves(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        MoveValidator::get_valid_moves(&self.state.board, x, y, self.state.current_turn)
    }

    fn is_in_check(&self, color: Color) -> bool {
        // Find general position
        let mut general_pos = None;
        for x in 0..9 {
            for y in 0..10 {
                if let Some(piece) = self.state.board.get_piece(x, y) {
                    if piece.piece_type == PieceType::General && piece.color == color {
                        general_pos = Some((x, y));
                        break;
                    }
                }
            }
        }

        if let Some((g_x, g_y)) = general_pos {
            // Check if any opponent piece can attack general
            let opponent_color = match color {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            };

            for x in 0..9 {
                for y in 0..10 {
                    if let Some(piece) = self.state.board.get_piece(x, y) {
                        if piece.color == opponent_color {
                            if MoveValidator::validate(
                                &self.state.board,
                                x,
                                y,
                                g_x,
                                g_y,
                                opponent_color,
                            )
                            .is_ok()
                            {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }

    fn is_checkmate(&self, color: Color) -> bool {
        // If not in check, not checkmate
        if !self.is_in_check(color) {
            return false;
        }

        // Check if any piece has valid move that gets out of check
        for from_x in 0..9 {
            for from_y in 0..10 {
                if let Some(piece) = self.state.board.get_piece(from_x, from_y) {
                    if piece.color == color {
                        let valid_moves = MoveValidator::get_valid_moves(
                            &self.state.board,
                            from_x,
                            from_y,
                            color,
                        );
                        for (to_x, to_y) in valid_moves {
                            // Simulate move to check if still in check
                            let mut temp_board = self.state.board.clone();
                            temp_board.move_piece(from_x, from_y, to_x, to_y);

                            let temp_manager = GameStateManager {
                                state: GameState {
                                    board: temp_board,
                                    current_turn: color,
                                    is_in_check: false,
                                    is_ended: false,
                                    winner: None,
                                },
                                history: self.history.clone(),
                            };

                            if !temp_manager.is_in_check(color) {
                                return false;
                            }
                        }
                    }
                }
            }
        }

        true
    }
}
