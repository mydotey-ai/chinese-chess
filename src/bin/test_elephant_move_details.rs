use chinese_chess::game::GameStateManager;
use chinese_chess::piece::{Color, PieceType};
use chinese_chess::rules;

fn main() {
    println!("=== 黑象移动详细测试 ===\n");

    let manager = GameStateManager::new();

    // 检查黑象 (2, 0) 的所有可能移动
    let from_x = 2;
    let from_y = 0;

    println!("检查黑象从 ({}, {}) 的移动：\n", from_x, from_y);

    // 可能的田字格移动
    let possible_moves = [(0, 2), (4, 2)];

    for (to_x, to_y) in possible_moves {
        println!("目标位置: ({}, {})", to_x, to_y);

        // 检查过河规则
        let crosses_river = match Color::Black {
            Color::Red => to_y >= 5,
            Color::Black => to_y <= 4,
        };
        println!("是否过河: {}", crosses_river);

        // 检查象眼位置
        let eye_x = (from_x + to_x) / 2;
        let eye_y = (from_y + to_y) / 2;
        let eye_piece = manager.state.board.get_piece(eye_x, eye_y);
        println!("象眼位置: ({}, {}) 棋子: {:?}", eye_x, eye_y, eye_piece);

        // 验证移动
        let result = rules::validate_move(
            &manager.state.board,
            from_x,
            from_y,
            to_x,
            to_y,
            Color::Black,
        );

        println!("验证结果: {:?}\n", result);
    }

    println!("=== 打印完整棋盘 ===");
    println!("{}", print_board(&manager.state.board));
}

fn print_board(board: &chinese_chess::board::Board) -> String {
    let mut output = String::new();
    output.push_str("  0 1 2 3 4 5 6 7 8\n");
    output.push_str(" -------------------\n");

    for y in 0..10 {
        output.push_str(&format!("{}|", y));

        for x in 0..9 {
            if let Some(piece) = board.get_piece(x, y) {
                let symbol = match (piece.piece_type, piece.color) {
                    (PieceType::General, Color::Black) => "将",
                    (PieceType::General, Color::Red) => "帅",
                    (PieceType::Advisor, Color::Black) => "士",
                    (PieceType::Advisor, Color::Red) => "仕",
                    (PieceType::Elephant, Color::Black) => "象",
                    (PieceType::Elephant, Color::Red) => "相",
                    (PieceType::Horse, Color::Black) => "马",
                    (PieceType::Horse, Color::Red) => "馬",
                    (PieceType::Chariot, Color::Black) => "车",
                    (PieceType::Chariot, Color::Red) => "车",
                    (PieceType::Cannon, Color::Black) => "炮",
                    (PieceType::Cannon, Color::Red) => "炮",
                    (PieceType::Soldier, Color::Black) => "卒",
                    (PieceType::Soldier, Color::Red) => "兵",
                };
                output.push_str(&format!("{} ", symbol));
            } else {
                output.push_str("  ");
            }
        }

        output.push_str("|\n");
    }

    output.push_str(" -------------------\n");
    output
}
