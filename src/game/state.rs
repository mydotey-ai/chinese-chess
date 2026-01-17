#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Playing,
    RedWins,
    BlackWins,
    Draw,
}