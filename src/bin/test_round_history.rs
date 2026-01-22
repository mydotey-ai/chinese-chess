use chinese_chess::game::GameStateManager;

fn main() {
    println!("Testing round history integration...");

    let mut manager = GameStateManager::new();

    // Test 1: Initial state should have empty history
    println!("Test 1: Initial empty history");
    assert!(manager.history.is_empty());
    println!("✓ History is empty as expected");

    // Test 2: Make a red move (red chariot is at (0,9), can only move straight)
    println!("\nTest 2: Make a red move");
    let result = manager.make_move(0, 9, 1, 9); // Red chariot moves right
    assert!(result.is_ok());
    println!("✓ Red move successful");

    // Check that history has one round with red move only
    assert_eq!(manager.history.len(), 1);
    let round = manager.history.get_round(0).unwrap();
    assert_eq!(round.round_number, 1);
    assert!(round.black_move.is_none());
    println!("✓ History has round 1 with red move only");

    // Test 3: Make a black move (black chariot at (8,0) can move left)
    println!("\nTest 3: Make a black move");
    let result = manager.make_move(8, 0, 7, 0); // Black chariot moves left
    assert!(result.is_ok());
    println!("✓ Black move successful");

    // Check that history still has one round, now with black move
    assert_eq!(manager.history.len(), 1);
    let round = manager.history.get_round(0).unwrap();
    assert_eq!(round.round_number, 1);
    assert!(round.black_move.is_some());
    println!("✓ History round 1 now has both red and black moves");

    // Test 4: Make another red move (should create new round)
    println!("\nTest 4: Make another red move");
    let result = manager.make_move(1, 9, 2, 9); // Red chariot moves right again
    assert!(result.is_ok());
    println!("✓ Second red move successful");

    // Check that history has two rounds
    assert_eq!(manager.history.len(), 2);
    let round2 = manager.history.get_round(1).unwrap();
    assert_eq!(round2.round_number, 2);
    assert!(round2.black_move.is_none());
    println!("✓ History has round 2 with red move only");

    // Test 5: Undo move
    println!("\nTest 5: Undo move");
    let result = manager.undo_move();
    assert!(result.is_ok());
    println!("✓ Undo successful");

    // Check that history has one round again (black move removed)
    assert_eq!(manager.history.len(), 1);
    let round = manager.history.get_round(0).unwrap();
    assert!(round.black_move.is_some()); // Should still have black move after undo
    println!("✓ History back to one round with black move");

    // Test 6: Undo again to remove red move
    println!("\nTest 6: Undo again");
    let result = manager.undo_move();
    assert!(result.is_ok());
    println!("✓ Second undo successful");

    // Check that history is empty
    assert!(manager.history.is_empty());
    println!("✓ History empty as expected");

    println!("\n✅ All round history tests passed!");

    // Test serialization
    println!("\nTest 7: Serialization test");
    let state_with_history = chinese_chess::game_with_history::GameStateWithHistory::new(
        manager.state.clone(),
        manager.history.clone(),
    );

    // Serialize to JSON
    let json_result = serde_json::to_string(&state_with_history);
    assert!(json_result.is_ok());
    println!("✓ Serialization successful");

    // Deserialize back
    let json_str = json_result.unwrap();
    let deserialized: Result<chinese_chess::game_with_history::GameStateWithHistory, _> =
        serde_json::from_str(&json_str);
    assert!(deserialized.is_ok());
    println!("✓ Deserialization successful");

    println!("\n🎉 All integration tests completed successfully!");
}
