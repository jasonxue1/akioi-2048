use akioi_2048::{Direction, State, step};

#[test]
fn invalid_move_triggers_failure() {
    let board = [[2, 4, 2, 4], [4, 2, 4, 2], [2, 4, 2, 4], [4, 2, 4, 2]];
    let (new_board, delta, msg) = step(board, Direction::Down).expect("step should succeed");
    assert_eq!(new_board, board);
    assert_eq!(delta, 0);
    assert!(matches!(msg, State::GameOver));
}

#[test]
fn invalid_move_no_failure_if_other_moves_exist() {
    let board = [[2, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    let (new_board, delta, msg) = step(board, Direction::Up).expect("step should succeed");
    assert_eq!(new_board, board);
    assert_eq!(delta, 0);
    assert!(matches!(msg, State::Continue));
}
