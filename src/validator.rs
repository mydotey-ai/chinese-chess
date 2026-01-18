use crate::board::Board;
use crate::piece::Color;
use crate::rules;

pub struct MoveValidator;

impl MoveValidator {
    pub fn validate(
        board: &Board,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
        current_color: Color,
    ) -> Result<(), crate::ChessError> {
        rules::validate_move(board, from_x, from_y, to_x, to_y, current_color)
    }

    pub fn get_valid_moves(
        board: &Board,
        x: usize,
        y: usize,
        current_color: Color,
    ) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();

        if let Some(piece) = board.get_piece(x, y) {
            if piece.color == current_color {
                for to_x in 0..9 {
                    for to_y in 0..10 {
                        if (to_x, to_y) != (x, y) {
                            if let Ok(()) =
                                rules::validate_move(board, x, y, to_x, to_y, current_color)
                            {
                                valid_moves.push((to_x, to_y));
                            }
                        }
                    }
                }
            }
        }

        valid_moves
    }
}
