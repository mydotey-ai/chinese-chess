use crate::board::{coordinate::Coordinate, square::Color, square::Square, Board};

pub fn validate_move(
    board: &Board,
    from: Coordinate,
    to: Coordinate,
    current_color: Color,
) -> bool {
    // Check coordinates are valid
    if !from.is_valid() || !to.is_valid() {
        return false;
    }

    // Check piece exists and matches current color
    let from_square = board.get_square(from);
    if let Some(Square::Occupied(_, color)) = from_square {
        if *color != current_color {
            return false;
        }
    } else {
        return false;
    }

    // Check destination square
    let to_square = board.get_square(to);
    if let Some(Square::Occupied(_, color)) = to_square {
        if *color == current_color {
            return false;
        }
    }

    true
}
