use chinese_chess::board::{coordinate::Coordinate, square::Color, Board};
use chinese_chess::pieces::chariot::Chariot;
use chinese_chess::pieces::king::King;
use chinese_chess::pieces::Piece;

#[test]
fn test_king_movement() {
    let board = Board::new();
    let king = King::new(Color::Red);
    let start = Coordinate::new(4, 0);

    let valid_moves = king.valid_moves(start, &board);
    assert!(valid_moves.contains(&Coordinate::new(4, 1)));
    assert!(valid_moves.contains(&Coordinate::new(3, 0)));
    assert!(!valid_moves.contains(&Coordinate::new(5, 1)));
}

#[test]
fn test_chariot_movement() {
    let board = Board::new();
    let chariot = Chariot::new(Color::Black);
    let start = Coordinate::new(0, 9);

    let valid_moves = chariot.valid_moves(start, &board);
    assert!(valid_moves.contains(&Coordinate::new(0, 8)));
    assert!(valid_moves.contains(&Coordinate::new(1, 9)));
}
