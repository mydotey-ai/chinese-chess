use chinese_chess::board::{coordinate::Coordinate, square::Square, Board};

#[test]
fn test_board_creation() {
    let board = Board::new();
    assert!(board.is_empty());
}

#[test]
fn test_coordinate_validation() {
    let valid = Coordinate::new(3, 5);
    assert!(valid.is_valid());

    let invalid = Coordinate::new(10, 5);
    assert!(!invalid.is_valid());
}

#[test]
fn test_square_access() {
    let board = Board::new();
    let square = board.get_square(Coordinate::new(0, 0)).unwrap();
    assert!(matches!(square, Square::Empty));
}
