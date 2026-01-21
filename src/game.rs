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
        // Black pieces (top, y=0)
        board.set_piece(0, 0, Some(Piece::new(PieceType::Chariot, Color::Black)));
        board.set_piece(1, 0, Some(Piece::new(PieceType::Horse, Color::Black)));
        board.set_piece(2, 0, Some(Piece::new(PieceType::Elephant, Color::Black)));
        board.set_piece(3, 0, Some(Piece::new(PieceType::Advisor, Color::Black)));
        board.set_piece(4, 0, Some(Piece::new(PieceType::General, Color::Black)));
        board.set_piece(5, 0, Some(Piece::new(PieceType::Advisor, Color::Black)));
        board.set_piece(6, 0, Some(Piece::new(PieceType::Elephant, Color::Black)));
        board.set_piece(7, 0, Some(Piece::new(PieceType::Horse, Color::Black)));
        board.set_piece(8, 0, Some(Piece::new(PieceType::Chariot, Color::Black)));

        board.set_piece(1, 2, Some(Piece::new(PieceType::Cannon, Color::Black)));
        board.set_piece(7, 2, Some(Piece::new(PieceType::Cannon, Color::Black)));

        board.set_piece(0, 3, Some(Piece::new(PieceType::Soldier, Color::Black)));
        board.set_piece(2, 3, Some(Piece::new(PieceType::Soldier, Color::Black)));
        board.set_piece(4, 3, Some(Piece::new(PieceType::Soldier, Color::Black)));
        board.set_piece(6, 3, Some(Piece::new(PieceType::Soldier, Color::Black)));
        board.set_piece(8, 3, Some(Piece::new(PieceType::Soldier, Color::Black)));

        // Red pieces (bottom, y=9)
        board.set_piece(0, 9, Some(Piece::new(PieceType::Chariot, Color::Red)));
        board.set_piece(1, 9, Some(Piece::new(PieceType::Horse, Color::Red)));
        board.set_piece(2, 9, Some(Piece::new(PieceType::Elephant, Color::Red)));
        board.set_piece(3, 9, Some(Piece::new(PieceType::Advisor, Color::Red)));
        board.set_piece(4, 9, Some(Piece::new(PieceType::General, Color::Red)));
        board.set_piece(5, 9, Some(Piece::new(PieceType::Advisor, Color::Red)));
        board.set_piece(6, 9, Some(Piece::new(PieceType::Elephant, Color::Red)));
        board.set_piece(7, 9, Some(Piece::new(PieceType::Horse, Color::Red)));
        board.set_piece(8, 9, Some(Piece::new(PieceType::Chariot, Color::Red)));

        board.set_piece(1, 7, Some(Piece::new(PieceType::Cannon, Color::Red)));
        board.set_piece(7, 7, Some(Piece::new(PieceType::Cannon, Color::Red)));

        board.set_piece(0, 6, Some(Piece::new(PieceType::Soldier, Color::Red)));
        board.set_piece(2, 6, Some(Piece::new(PieceType::Soldier, Color::Red)));
        board.set_piece(4, 6, Some(Piece::new(PieceType::Soldier, Color::Red)));
        board.set_piece(6, 6, Some(Piece::new(PieceType::Soldier, Color::Red)));
        board.set_piece(8, 6, Some(Piece::new(PieceType::Soldier, Color::Red)));
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct GameStateManager {
    pub state: GameState,
    pub history: History,
}

impl Clone for GameStateManager {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            history: self.history.clone(),
        }
    }
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

        // Check if captured the opponent's general (game ends immediately)
        if let Some(piece) = captured_piece {
            if piece.piece_type == PieceType::General {
                self.state.is_ended = true;
                self.state.winner = Some(self.state.current_turn);
                // Record move
                self.history.push_with_color(
                    MoveRecord {
                        from_x,
                        from_y,
                        to_x,
                        to_y,
                        captured_piece,
                    },
                    self.state.current_turn,
                );
                return Ok(());
            }
        }

        // Record move
        self.history.push_with_color(
            MoveRecord {
                from_x,
                from_y,
                to_x,
                to_y,
                captured_piece,
            },
            self.state.current_turn,
        );

        // Switch turn
        self.state.current_turn = match self.state.current_turn {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };

        // Check if in check
        self.state.is_in_check = self.is_in_check(self.state.current_turn);

        // Check if game ended (checkmate)
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

        let (last_move, move_color) = self.history.pop().unwrap();

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

        // Set turn to the color of the move that was undone
        self.state.current_turn = move_color;

        // Check if in check for the current player
        self.state.is_in_check = self.is_in_check(self.state.current_turn);

        // Reset game ended state since we undid a move
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
                        if piece.color == opponent_color
                            && MoveValidator::validate(
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

impl Default for GameStateManager {
    fn default() -> Self {
        Self::new()
    }
}

// Implement AsRef for Tauri State compatibility
impl AsRef<GameState> for GameStateManager {
    fn as_ref(&self) -> &GameState {
        &self.state
    }
}

// Implement AsMut for Tauri State compatibility
impl AsMut<GameState> for GameStateManager {
    fn as_mut(&mut self) -> &mut GameState {
        &mut self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::piece::{Color, Piece, PieceType};

    #[test]
    fn test_capture_general_ends_game() {
        println!("=== 测试吃将结束游戏 ===\n");

        // 创建一个简单的测试棋局
        let mut board = Board::new();

        // 在 (4, 0) 放置黑方将
        board.set_piece(4, 0, Some(Piece::new(PieceType::General, Color::Black)));

        // 在 (4, 1) 放置红方车
        board.set_piece(4, 1, Some(Piece::new(PieceType::Chariot, Color::Red)));

        // 创建游戏状态
        let state = GameState {
            board,
            current_turn: Color::Red,
            is_in_check: false,
            is_ended: false,
            winner: None,
        };

        let mut manager = GameStateManager {
            state,
            history: History::new(),
        };

        println!("初始状态: 红方回合，黑将在 (4,0)，红车在 (4,1)");

        // 红方车吃掉黑方将
        let result = manager.make_move(4, 1, 4, 0);
        assert!(result.is_ok());

        println!("移动成功后:");
        println!("游戏是否结束: {}", manager.state.is_ended);
        println!("胜利者: {:?}", manager.state.winner);

        // 验证游戏是否结束并且红方获胜
        assert!(manager.state.is_ended);
        assert_eq!(manager.state.winner, Some(Color::Red));

        println!("\n=== 测试成功！吃将后游戏立即结束 ===\n");
    }

    #[test]
    fn test_undo_move_correct_turn() {
        println!("=== 测试撤销移动后的正确回合 ===\n");

        // 创建一个简单的测试棋局：红方兵，黑方卒
        let mut board = Board::new();
        // 红方兵在 (0,6)，可以向前移动
        board.set_piece(0, 6, Some(Piece::new(PieceType::Soldier, Color::Red)));
        // 黑方卒在 (8,3)，可以向前移动
        board.set_piece(8, 3, Some(Piece::new(PieceType::Soldier, Color::Black)));

        let state = GameState {
            board,
            current_turn: Color::Red,
            is_in_check: false,
            is_ended: false,
            winner: None,
        };

        let mut manager = GameStateManager {
            state,
            history: History::new(),
        };

        println!("初始状态: 红方回合");
        assert_eq!(manager.state.current_turn, Color::Red);

        // 红方移动：兵向前移动一步 (0,6) -> (0,5)
        println!("\n1. 红方移动 (0,6) -> (0,5)");
        let result = manager.make_move(0, 6, 0, 5);
        println!("   移动结果: {:?}", result);
        assert!(result.is_ok(), "红方移动失败: {:?}", result);
        assert_eq!(
            manager.state.current_turn,
            Color::Black,
            "红方移动后应该是黑方回合"
        );

        // 黑方移动：卒向前移动一步 (8,3) -> (8,4)
        println!("2. 黑方移动 (8,3) -> (8,4)");
        let result = manager.make_move(8, 3, 8, 4);
        println!("   移动结果: {:?}", result);
        assert!(result.is_ok(), "黑方移动失败: {:?}", result);
        assert_eq!(
            manager.state.current_turn,
            Color::Red,
            "黑方移动后应该是红方回合"
        );

        println!("\n--- 撤销测试 ---");
        println!("当前历史长度（轮次数）: {}", manager.history.len());

        // 测试弹出第一个移动（黑方移动）
        let (popped_move, popped_color) = manager.history.peek().unwrap();
        println!(
            "当前栈顶移动: 颜色={:?}, from=({},{})",
            popped_color, popped_move.from_x, popped_move.from_y
        );
        assert_eq!(popped_color, Color::Black, "栈顶应该是黑方移动");

        // 撤销黑方移动
        println!("\n3. 撤销黑方移动");
        let undo_result = manager.undo_move();
        println!("   撤销结果: {:?}", undo_result);
        assert!(undo_result.is_ok());

        // 验证撤销后的回合：撤销黑方移动后应该轮到黑方
        println!("   撤销后回合: {:?}", manager.state.current_turn);
        println!("   [期望] 撤销黑方移动后应该轮到黑方");
        println!("   [实际] 回合: {:?}", manager.state.current_turn);

        // 关键测试：撤销黑方移动后应该轮到黑方
        assert_eq!(
            manager.state.current_turn,
            Color::Black,
            "撤销黑方移动后应该是黑方回合"
        );

        // 检查历史状态
        println!("   撤销后历史长度: {}", manager.history.len());

        // 测试弹出第二个移动（红方移动）
        if let Some((peeked_move, peeked_color)) = manager.history.peek() {
            println!(
                "   当前栈顶移动: 颜色={:?}, from=({},{})",
                peeked_color, peeked_move.from_x, peeked_move.from_y
            );
            assert_eq!(peeked_color, Color::Red, "栈顶应该是红方移动");
        }

        // 撤销红方移动
        println!("\n4. 撤销红方移动");
        let undo_result = manager.undo_move();
        println!("   撤销结果: {:?}", undo_result);
        assert!(undo_result.is_ok());

        // 验证撤销后的回合：撤销红方移动后应该轮到红方
        println!("   撤销后回合: {:?}", manager.state.current_turn);
        println!("   [期望] 撤销红方移动后应该轮到红方");
        println!("   [实际] 回合: {:?}", manager.state.current_turn);

        // 撤销红方移动后，应该轮到红方
        assert_eq!(
            manager.state.current_turn,
            Color::Red,
            "撤销红方移动后应该是红方回合"
        );

        // 检查历史是否为空
        println!("   最终历史长度: {}", manager.history.len());
        assert!(manager.history.is_empty(), "历史应该为空");

        println!("\n=== 测试成功！撤销后回合正确 ===\n");
    }

    #[test]
    fn test_undo_single_red_move() {
        println!("=== 测试撤销单个红方移动 ===\n");

        // 创建一个简单的测试棋局：只有红方兵
        let mut board = Board::new();
        board.set_piece(0, 6, Some(Piece::new(PieceType::Soldier, Color::Red)));

        let state = GameState {
            board,
            current_turn: Color::Red,
            is_in_check: false,
            is_ended: false,
            winner: None,
        };

        let mut manager = GameStateManager {
            state,
            history: History::new(),
        };

        println!("初始状态: 红方回合");
        assert_eq!(manager.state.current_turn, Color::Red);

        // 红方移动
        println!("\n1. 红方移动 (0,6) -> (0,5)");
        let result = manager.make_move(0, 6, 0, 5);
        println!("   移动结果: {:?}", result);
        assert!(result.is_ok(), "红方移动失败: {:?}", result);
        assert_eq!(
            manager.state.current_turn,
            Color::Black,
            "红方移动后应该是黑方回合"
        );

        println!("\n--- 撤销测试 ---");
        println!("当前历史长度: {}", manager.history.len());

        // 验证栈顶移动
        let (peeked_move, peeked_color) = manager.history.peek().unwrap();
        println!(
            "当前栈顶移动: 颜色={:?}, from=({},{})",
            peeked_color, peeked_move.from_x, peeked_move.from_y
        );
        assert_eq!(peeked_color, Color::Red, "栈顶应该是红方移动");

        // 撤销红方移动
        println!("\n2. 撤销红方移动");
        let undo_result = manager.undo_move();
        println!("   撤销结果: {:?}", undo_result);
        assert!(undo_result.is_ok());

        // 关键测试：撤销红方移动后应该轮到红方
        println!("   撤销后回合: {:?}", manager.state.current_turn);
        println!("   [期望] 撤销红方移动后应该轮到红方");
        println!("   [实际] 回合: {:?}", manager.state.current_turn);

        // 检查结果：撤销红方移动后应该轮到红方
        assert_eq!(
            manager.state.current_turn,
            Color::Red,
            "撤销红方移动后应该是红方回合"
        );

        // 检查历史是否为空
        println!("   最终历史长度: {}", manager.history.len());
        assert!(manager.history.is_empty(), "历史应该为空");

        println!("\n=== 测试成功！单个红方移动撤销正确 ===\n");
    }

    #[test]
    fn test_undo_move_with_color_logic() {
        println!("=== 测试撤销移动的颜色逻辑 ===\n");

        // 这个测试验证撤销逻辑是否正确使用颜色信息
        // 我们创建一个简单的自定义棋盘，避免将军

        let mut board = Board::new();

        // 放置一些简单棋子用于测试
        // 红方马在 (1,9)，红方兵在 (0,6)
        board.set_piece(1, 9, Some(Piece::new(PieceType::Horse, Color::Red)));
        board.set_piece(0, 6, Some(Piece::new(PieceType::Soldier, Color::Red)));

        // 黑方马在 (7,0)，黑方卒在 (8,3)
        board.set_piece(7, 0, Some(Piece::new(PieceType::Horse, Color::Black)));
        board.set_piece(8, 3, Some(Piece::new(PieceType::Soldier, Color::Black)));

        let state = GameState {
            board,
            current_turn: Color::Red,
            is_in_check: false,
            is_ended: false,
            winner: None,
        };

        let mut manager = GameStateManager {
            state,
            history: History::new(),
        };

        println!("初始状态: 红方回合");
        assert_eq!(manager.state.current_turn, Color::Red);

        // 移动1：红方马移动 (1,9) -> (2,7) - 马走日
        println!("\n移动1: 红方马 (1,9) -> (2,7)");
        let result = manager.make_move(1, 9, 2, 7);
        println!("   移动结果: {:?}", result);
        assert!(result.is_ok(), "红方马移动失败: {:?}", result);
        assert_eq!(
            manager.state.current_turn,
            Color::Black,
            "红方移动后应该是黑方回合"
        );

        // 移动2：黑方马移动 (7,0) -> (5,1) - 马走日
        println!("移动2: 黑方马 (7,0) -> (5,1)");
        let result = manager.make_move(7, 0, 5, 1);
        println!("   移动结果: {:?}", result);
        assert!(result.is_ok(), "黑方马移动失败: {:?}", result);
        assert_eq!(
            manager.state.current_turn,
            Color::Red,
            "黑方移动后应该是红方回合"
        );

        // 移动3：红方兵向前移动 (0,6) -> (0,5)
        println!("移动3: 红方兵 (0,6) -> (0,5)");
        let result = manager.make_move(0, 6, 0, 5);
        println!("   移动结果: {:?}", result);
        assert!(result.is_ok(), "红方兵移动失败: {:?}", result);
        assert_eq!(
            manager.state.current_turn,
            Color::Black,
            "红方移动后应该是黑方回合"
        );

        println!("\n--- 复杂撤销场景 ---");
        println!("当前回合: {:?} (应该是黑方)", manager.state.current_turn);
        assert_eq!(
            manager.state.current_turn,
            Color::Black,
            "当前应该是黑方回合"
        );

        // 现在我们有3个移动：红(马)、黑(马)、红(兵)

        // 撤销移动3（红方兵）
        println!("\n1. 撤销移动3（红方兵）");
        let undo_result = manager.undo_move();
        println!("   撤销结果: {:?}", undo_result);
        assert!(undo_result.is_ok(), "撤销移动3失败: {:?}", undo_result);

        // 修复后：撤销红方移动后应该轮到红方
        println!("   撤销后回合: {:?}", manager.state.current_turn);
        println!("   [修复后期望] 撤销红方移动后应该轮到红方");
        println!("   [实际] 回合: {:?}", manager.state.current_turn);
        assert_eq!(
            manager.state.current_turn,
            Color::Red,
            "撤销红方移动后应该是红方回合"
        );

        // 撤销移动2（黑方马）
        println!("\n2. 撤销移动2（黑方马）");
        let undo_result = manager.undo_move();
        println!("   撤销结果: {:?}", undo_result);
        assert!(undo_result.is_ok(), "撤销移动2失败: {:?}", undo_result);

        // 修复后：撤销黑方移动后应该轮到黑方
        println!("   撤销后回合: {:?}", manager.state.current_turn);
        println!("   [修复后期望] 撤销黑方移动后应该轮到黑方");
        println!("   [实际] 回合: {:?}", manager.state.current_turn);
        assert_eq!(
            manager.state.current_turn,
            Color::Black,
            "撤销黑方移动后应该是黑方回合"
        );

        // 撤销移动1（红方马）
        println!("\n3. 撤销移动1（红方马）");
        let undo_result = manager.undo_move();
        println!("   撤销结果: {:?}", undo_result);
        assert!(undo_result.is_ok(), "撤销移动1失败: {:?}", undo_result);

        // 修复后：撤销红方移动后应该轮到红方
        println!("   撤销后回合: {:?}", manager.state.current_turn);
        println!("   [修复后期望] 撤销红方移动后应该轮到红方");
        println!("   [实际] 回合: {:?}", manager.state.current_turn);
        assert_eq!(
            manager.state.current_turn,
            Color::Red,
            "撤销红方移动后应该是红方回合"
        );

        println!("\n=== 测试成功！复杂撤销场景正确 ===\n");
    }

    #[test]
    fn test_undo_move_fixed_logic() {
        println!("=== 测试修复后的撤销逻辑 ===\n");

        // 这个测试验证修复后的撤销逻辑是否正确使用颜色信息
        // 修复后的代码应该基于移动的颜色设置当前回合

        let mut manager = GameStateManager::new();

        println!("初始回合: {:?}", manager.state.current_turn);
        assert_eq!(manager.state.current_turn, Color::Red);

        // 红方移动：兵 (0,6) -> (0,5)
        println!("\n1. 红方移动 (0,6) -> (0,5)");
        let result = manager.make_move(0, 6, 0, 5);
        assert!(result.is_ok());
        assert_eq!(manager.state.current_turn, Color::Black);

        // 黑方移动：卒 (8,3) -> (8,4)
        println!("2. 黑方移动 (8,3) -> (8,4)");
        let result = manager.make_move(8, 3, 8, 4);
        assert!(result.is_ok());
        assert_eq!(manager.state.current_turn, Color::Red);

        println!("\n--- 验证修复后的逻辑 ---");

        // 场景1：撤销黑方移动
        println!("\n3. 撤销黑方移动");
        let undo_result = manager.undo_move();
        assert!(undo_result.is_ok());

        // 修复后：撤销黑方移动后应该轮到黑方
        println!("   当前回合: {:?}", manager.state.current_turn);
        println!("   [修复后期望] 撤销黑方移动后应该轮到黑方");
        println!("   [实际] 回合: {:?}", manager.state.current_turn);
        assert_eq!(
            manager.state.current_turn,
            Color::Black,
            "撤销黑方移动后应该是黑方回合"
        );

        // 场景2：撤销红方移动
        println!("\n4. 撤销红方移动");
        let undo_result = manager.undo_move();
        assert!(undo_result.is_ok());

        // 修复后：撤销红方移动后应该轮到红方
        println!("   当前回合: {:?}", manager.state.current_turn);
        println!("   [修复后期望] 撤销红方移动后应该轮到红方");
        println!("   [实际] 回合: {:?}", manager.state.current_turn);
        assert_eq!(
            manager.state.current_turn,
            Color::Red,
            "撤销红方移动后应该是红方回合"
        );

        println!("\n=== 测试成功！修复后的撤销逻辑正确 ===\n");
    }
}
