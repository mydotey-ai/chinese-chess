use crate::board::Board;
use crate::piece::{Color, PieceType};

pub fn validate_move(
    board: &Board,
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
    current_color: Color,
) -> Result<(), crate::ChessError> {
    // Check if from position is within bounds
    if from_x >= 9 || from_y >= 10 {
        return Err(crate::ChessError::OutOfBoard);
    }

    // Check if to position is within bounds
    if to_x >= 9 || to_y >= 10 {
        return Err(crate::ChessError::OutOfBoard);
    }

    // Check if there is a piece at from position
    let from_piece = match board.get_piece(from_x, from_y) {
        Some(p) => p,
        None => return Err(crate::ChessError::InvalidMove),
    };

    // Check if piece color matches current turn
    if from_piece.color != current_color {
        return Err(crate::ChessError::NotYourPiece);
    }

    // Check if to position has own piece
    if let Some(to_piece) = board.get_piece(to_x, to_y) {
        if to_piece.color == current_color {
            return Err(crate::ChessError::CannotCaptureOwnPiece);
        }
    }

    // Validate specific piece movement rules
    match from_piece.piece_type {
        PieceType::General => validate_general_move(from_x, from_y, to_x, to_y),
        PieceType::Advisor => validate_advisor_move(from_x, from_y, to_x, to_y),
        PieceType::Elephant => validate_elephant_move(from_x, from_y, to_x, to_y, from_piece.color),
        PieceType::Horse => validate_horse_move(board, from_x, from_y, to_x, to_y),
        PieceType::Chariot => validate_chariot_move(board, from_x, from_y, to_x, to_y),
        PieceType::Cannon => validate_cannon_move(board, from_x, from_y, to_x, to_y),
        PieceType::Soldier => validate_soldier_move(from_x, from_y, to_x, to_y, from_piece.color),
    }
}

fn validate_general_move(
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
) -> Result<(), crate::ChessError> {
    // General moves one step in any direction within the palace (3x3 area)
    let palace_x = (3..=5).contains(&from_x) && (3..=5).contains(&to_x);
    let palace_y_red = (0..=2).contains(&from_y) && (0..=2).contains(&to_y);
    let palace_y_black = (7..=9).contains(&from_y) && (7..=9).contains(&to_y);
    let in_palace = palace_x && (palace_y_red || palace_y_black);
    let one_step = (from_x as isize - to_x as isize).abs() <= 1
        && (from_y as isize - to_y as isize).abs() <= 1;

    if in_palace && one_step {
        Ok(())
    } else {
        Err(crate::ChessError::InvalidMove)
    }
}

fn validate_advisor_move(
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
) -> Result<(), crate::ChessError> {
    // Advisor moves one step diagonally within the palace
    let palace_x = (3..=5).contains(&from_x) && (3..=5).contains(&to_x);
    let palace_y_red = (0..=2).contains(&from_y) && (0..=2).contains(&to_y);
    let palace_y_black = (7..=9).contains(&from_y) && (7..=9).contains(&to_y);
    let in_palace = palace_x && (palace_y_red || palace_y_black);
    let diagonal = (from_x as isize - to_x as isize).abs() == 1
        && (from_y as isize - to_y as isize).abs() == 1;

    if in_palace && diagonal {
        Ok(())
    } else {
        Err(crate::ChessError::InvalidMove)
    }
}

fn validate_elephant_move(
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
    color: Color,
) -> Result<(), crate::ChessError> {
    // Elephant moves two steps diagonally, cannot cross river
    let dx = (from_x as isize - to_x as isize).abs();
    let dy = (from_y as isize - to_y as isize).abs();
    let two_steps_diagonal = dx == 2 && dy == 2;

    // Check if crossing river
    let crosses_river = match color {
        Color::Red => to_y >= 5,
        Color::Black => to_y <= 4,
    };

    if two_steps_diagonal && !crosses_river {
        // Check if eye is blocked
        let _eye_x = (from_x + to_x) / 2;
        let _eye_y = (from_y + to_y) / 2;
        Ok(()) // TODO: Check if eye is blocked
    } else {
        Err(crate::ChessError::InvalidMove)
    }
}

fn validate_horse_move(
    board: &Board,
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
) -> Result<(), crate::ChessError> {
    // Horse moves in "日" shape: (±2, ±1) or (±1, ±2)
    let dx = (from_x as isize - to_x as isize).abs();
    let dy = (from_y as isize - to_y as isize).abs();
    let valid_shape = (dx == 2 && dy == 1) || (dx == 1 && dy == 2);

    if valid_shape {
        // Check if leg is blocked
        let leg_x = if dx == 2 { (from_x + to_x) / 2 } else { from_x };
        let leg_y = if dy == 2 { (from_y + to_y) / 2 } else { from_y };
        if board.get_piece(leg_x, leg_y).is_none() {
            Ok(())
        } else {
            Err(crate::ChessError::InvalidMove)
        }
    } else {
        Err(crate::ChessError::InvalidMove)
    }
}

fn validate_chariot_move(
    board: &Board,
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
) -> Result<(), crate::ChessError> {
    // Chariot moves straight line
    let straight_line = from_x == to_x || from_y == to_y;

    if straight_line {
        // Check if path is clear
        if from_x == to_x {
            // Vertical move
            let (start, end) = if from_y < to_y {
                (from_y + 1, to_y)
            } else {
                (to_y + 1, from_y)
            };
            for y in start..end {
                if board.get_piece(from_x, y).is_some() {
                    return Err(crate::ChessError::InvalidMove);
                }
            }
        } else {
            // Horizontal move
            let (start, end) = if from_x < to_x {
                (from_x + 1, to_x)
            } else {
                (to_x + 1, from_x)
            };
            for x in start..end {
                if board.get_piece(x, from_y).is_some() {
                    return Err(crate::ChessError::InvalidMove);
                }
            }
        }
        Ok(())
    } else {
        Err(crate::ChessError::InvalidMove)
    }
}

fn validate_cannon_move(
    board: &Board,
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
) -> Result<(), crate::ChessError> {
    // Cannon moves straight line, captures by jumping over one piece
    let straight_line = from_x == to_x || from_y == to_y;

    if straight_line {
        // Count pieces in path
        let mut pieces_in_path = 0;

        if from_x == to_x {
            // Vertical move
            let (start, end) = if from_y < to_y {
                (from_y + 1, to_y)
            } else {
                (to_y + 1, from_y)
            };
            for y in start..end {
                if board.get_piece(from_x, y).is_some() {
                    pieces_in_path += 1;
                }
            }
        } else {
            // Horizontal move
            let (start, end) = if from_x < to_x {
                (from_x + 1, to_x)
            } else {
                (to_x + 1, from_x)
            };
            for x in start..end {
                if board.get_piece(x, from_y).is_some() {
                    pieces_in_path += 1;
                }
            }
        }

        let to_piece = board.get_piece(to_x, to_y);

        let valid_move = (to_piece.is_none() && pieces_in_path == 0)
            || (to_piece.is_some() && pieces_in_path == 1);

        if valid_move {
            Ok(())
        } else {
            Err(crate::ChessError::InvalidMove)
        }
    } else {
        Err(crate::ChessError::InvalidMove)
    }
}

fn validate_soldier_move(
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
    color: Color,
) -> Result<(), crate::ChessError> {
    // Soldier moves forward one step, can move sideways after crossing river
    let dx = (from_x as isize - to_x as isize).abs();
    let dy = (from_y as isize - to_y as isize).abs();

    let forward = match color {
        Color::Red => to_y > from_y,   // Red moves down (y increases)
        Color::Black => to_y < from_y, // Black moves up (y decreases)
    };

    let crossed_river = match color {
        Color::Red => from_y >= 5,
        Color::Black => from_y <= 4,
    };

    let valid_move = (dx == 0 && dy == 1 && forward) || (crossed_river && dy == 0 && dx == 1);

    if valid_move {
        Ok(())
    } else {
        Err(crate::ChessError::InvalidMove)
    }
}
