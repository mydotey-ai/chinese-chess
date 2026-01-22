use chinese_chess::game::GameStateManager;
use chinese_chess::game_with_history::GameStateWithHistory;

fn main() {
    println!("=== 验证回合制走子记录 ===\n");

    let mut manager = GameStateManager::new();

    // 第一步：红方走子
    println!("1. 红方第一步 (红车)");
    let result1 = manager.make_move(0, 9, 0, 8);
    assert!(result1.is_ok());

    let state1 = GameStateWithHistory::new(manager.state.clone(), manager.history.clone());
    println!("   回合数: {}", state1.history.rounds.len());
    println!(
        "   第一个回合的回合号: {}",
        state1.history.rounds[0].round_number
    );
    println!(
        "   第一个回合的红方走子: from=({},{}) to=({},{})",
        state1.history.rounds[0].red_move.from_x,
        state1.history.rounds[0].red_move.from_y,
        state1.history.rounds[0].red_move.to_x,
        state1.history.rounds[0].red_move.to_y
    );

    // 第二步：黑方走子
    println!("\n2. 黑方第一步 (黑车)");
    let result2 = manager.make_move(0, 0, 0, 1);
    assert!(result2.is_ok());

    let state2 = GameStateWithHistory::new(manager.state.clone(), manager.history.clone());
    println!("   回合数: {}", state2.history.rounds.len());
    println!(
        "   第一个回合的黑方走子: from=({},{}) to=({},{})",
        state2.history.rounds[0].black_move.as_ref().unwrap().from_x,
        state2.history.rounds[0].black_move.as_ref().unwrap().from_y,
        state2.history.rounds[0].black_move.as_ref().unwrap().to_x,
        state2.history.rounds[0].black_move.as_ref().unwrap().to_y
    );

    // 第三步：红方第二次走子
    println!("\n3. 红方第二步 (红马)");
    let result3 = manager.make_move(1, 9, 2, 7);
    assert!(result3.is_ok());

    let state3 = GameStateWithHistory::new(manager.state.clone(), manager.history.clone());
    println!("   回合数: {}", state3.history.rounds.len());
    println!(
        "   第二个回合的回合号: {}",
        state3.history.rounds[1].round_number
    );
    println!(
        "   第二个回合的红方走子: from=({},{}) to=({},{})",
        state3.history.rounds[1].red_move.from_x,
        state3.history.rounds[1].red_move.from_y,
        state3.history.rounds[1].red_move.to_x,
        state3.history.rounds[1].red_move.to_y
    );

    // 检查序列化
    println!("\n4. 检查序列化");
    let json = serde_json::to_string(&state3).unwrap();
    println!("   序列化成功: {}", json.len() > 0);

    let deserialized: GameStateWithHistory = serde_json::from_str(&json).unwrap();
    println!("   反序列化成功");
    println!(
        "   反序列化后的回合数: {}",
        deserialized.history.rounds.len()
    );

    // 验证数据完整性
    println!("\n5. 验证数据完整性");
    assert_eq!(deserialized.history.rounds.len(), 2);
    assert_eq!(deserialized.history.rounds[0].round_number, 1);
    assert_eq!(deserialized.history.rounds[1].round_number, 2);
    assert!(deserialized.history.rounds[0].black_move.is_some());
    assert!(deserialized.history.rounds[1].black_move.is_none());

    println!("\n=== 回合制走子记录验证成功！ ===");
    println!("\n总结:");
    println!("- 红方第一步: 创建回合1 (红方走子完成，黑方走子为空)");
    println!("- 黑方第一步: 添加到回合1 (红黑双方走子都完成)");
    println!("- 红方第二步: 创建回合2 (红方走子完成，黑方走子为空)");
    println!("- 序列化/反序列化正常");
    println!("- 回合编号正确: 1, 2");
}
