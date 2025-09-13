use akioi_2048::step;

#[test]
fn invalid_move_triggers_failure() {
    let board = [[2, 4, 2, 4], [4, 2, 4, 2], [2, 4, 2, 4], [4, 2, 4, 2]];
    let (new_board, delta, msg) = step(board, 0).expect("step should succeed");
    assert_eq!(new_board, board);
    assert_eq!(delta, 0);
    assert_eq!(msg, -1);
}

#[test]
fn invalid_move_no_failure_if_other_moves_exist() {
    let board = [[2, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    let (new_board, delta, msg) = step(board, 2).expect("step should succeed");
    assert_eq!(new_board, board);
    assert_eq!(delta, 0);
    assert_eq!(msg, 0);
}
