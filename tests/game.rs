use chinese_chess::game::Game;
use chinese_chess::game::state::GameState;
use chinese_chess::board::coordinate::Coordinate;
use chinese_chess::board::square::Color;

#[test]
fn test_game_initialization() {
    let mut game = Game::new();
    assert_eq!(game.state(), GameState::Playing);
    assert_eq!(game.current_turn(), Color::Red);
}

#[test]
fn test_valid_move() {
    let mut game = Game::new();
    let start = Coordinate::new(0, 0); // Red Chariot
    let end = Coordinate::new(0, 1);
    
    assert!(game.is_valid_move(start, end));
}