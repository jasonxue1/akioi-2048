use akioi_2048::{Direction, step};

#[test]
fn up_number_merges_and_positive_score() {
    let board = [[4, 0, 0, 0], [4, 0, 0, 0], [2, 0, 0, 0], [2, 0, 0, 0]];
    let (new_board, delta, _msg) = step(board, Direction::Up).unwrap();
    assert_eq!(new_board[0][0], 8);
    assert_eq!(new_board[1][0], 4);
    assert_eq!(delta, 12);
}

#[test]
fn up_multiplier_merges_and_negative_score() {
    let board = [[-2, 0, 0, 0], [-2, 0, 0, 0], [-1, 0, 0, 0], [-1, 0, 0, 0]];
    let (new_board, delta, _msg) = step(board, Direction::Up).unwrap();
    assert_eq!(new_board[0][0], -4);
    assert_eq!(new_board[1][0], -2);
    assert_eq!(delta, -6);
}

#[test]
fn up_number_multiplier_merges() {
    let board = [[-2, 0, 0, 0], [2, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    let (new_board, delta, _msg) = step(board, Direction::Up).unwrap();
    assert_eq!(new_board[0][0], 4);
    assert_eq!(delta, 4);
}

#[test]
fn up_number_and_multiplier_do_not_merge_without_tiles_above() {
    let board = [[0, 0, 0, 0], [0, 0, 0, 0], [-2, 0, 0, 0], [2, 0, 0, 0]];
    let (new_board, delta, _msg) = step(board, Direction::Up).unwrap();
    assert_eq!(new_board[0][0], -2);
    assert_eq!(new_board[1][0], 2);
    assert_eq!(delta, 0);
}

#[test]
fn up_number_and_multiplier_no_merge_with_gap() {
    let board = [[16, 0, 0, 0], [0, 0, 0, 0], [-2, 0, 0, 0], [2, 0, 0, 0]];
    let (new_board, delta, _msg) = step(board, Direction::Up).unwrap();
    assert_eq!(new_board[0][0], 16);
    assert_eq!(new_board[1][0], -2);
    assert_eq!(new_board[2][0], 2);
    assert_eq!(delta, 0);
}

#[test]
fn down_number_merges_and_positive_score() {
    let board = [[2, 0, 0, 0], [2, 0, 0, 0], [4, 0, 0, 0], [4, 0, 0, 0]];
    let (new_board, delta, _msg) = step(board, Direction::Down).unwrap();
    assert_eq!(new_board[3][0], 8);
    assert_eq!(new_board[2][0], 4);
    assert_eq!(delta, 12);
}

#[test]
fn down_multiplier_merges_and_negative_score() {
    let board = [[-1, 0, 0, 0], [-1, 0, 0, 0], [-2, 0, 0, 0], [-2, 0, 0, 0]];
    let (new_board, delta, _msg) = step(board, Direction::Down).unwrap();
    assert_eq!(new_board[3][0], -4);
    assert_eq!(new_board[2][0], -2);
    assert_eq!(delta, -6);
}

#[test]
fn down_number_multiplier_merges() {
    let board = [[0, 0, 0, 0], [0, 0, 0, 0], [2, 0, 0, 0], [-2, 0, 0, 0]];
    let (new_board, delta, _msg) = step(board, Direction::Down).unwrap();
    assert_eq!(new_board[3][0], 4);
    assert_eq!(delta, 4);
}

#[test]
fn down_number_and_multiplier_do_not_merge_without_tiles_below() {
    let board = [[2, 0, 0, 0], [-2, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    let (new_board, delta, _msg) = step(board, Direction::Down).unwrap();
    assert_eq!(new_board[2][0], 2);
    assert_eq!(new_board[3][0], -2);
    assert_eq!(delta, 0);
}

#[test]
fn down_number_and_multiplier_no_merge_with_gap() {
    let board = [[2, 0, 0, 0], [-2, 0, 0, 0], [0, 0, 0, 0], [16, 0, 0, 0]];
    let (new_board, delta, _msg) = step(board, Direction::Down).unwrap();
    assert_eq!(new_board[1][0], 2);
    assert_eq!(new_board[2][0], -2);
    assert_eq!(new_board[3][0], 16);
    assert_eq!(delta, 0);
}

#[test]
fn down_move_without_merge() {
    let board = [[-1, 2, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    let (new_board, delta, _msg) = step(board, Direction::Down).unwrap();
    assert_eq!(new_board[3][0], -1);
    assert_eq!(new_board[3][1], 2);
    assert_eq!(delta, 0);
}

#[test]
fn no_merge_for_negative_four() {
    let board = [[0, 0, 0, 0], [0, 0, 0, 0], [-4, 0, 0, 0], [-4, 0, 0, 0]];
    let (new_board, delta, _msg) = step(board, Direction::Down).unwrap();
    assert_eq!(new_board, board);
    assert_eq!(delta, 0);
}

#[test]
fn left_number_merges_and_positive_score() {
    let board = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [4, 4, 2, 2]];
    let (new_board, delta, _msg) = step(board, Direction::Left).unwrap();
    assert_eq!(new_board[3][0], 8);
    assert_eq!(new_board[3][1], 4);
    assert_eq!(delta, 12);
}

#[test]
fn left_multiplier_merges_and_negative_score() {
    let board = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [-2, -2, -1, -1]];
    let (new_board, delta, _msg) = step(board, Direction::Left).unwrap();
    assert_eq!(new_board[3][0], -4);
    assert_eq!(new_board[3][1], -2);
    assert_eq!(delta, -6);
}

#[test]
fn left_number_multiplier_merges() {
    let board = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [-2, 2, 0, 0]];
    let (new_board, delta, _msg) = step(board, Direction::Left).unwrap();
    assert_eq!(new_board[3][0], 4);
    assert_eq!(delta, 4);
}

#[test]
fn left_number_and_multiplier_do_not_merge_without_tiles_left() {
    let board = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, -2, 2, 0]];
    let (new_board, delta, _msg) = step(board, Direction::Left).unwrap();
    assert_eq!(new_board[3][0], -2);
    assert_eq!(new_board[3][1], 2);
    assert_eq!(delta, 0);
}

#[test]
fn left_number_and_multiplier_no_merge_with_gap() {
    let board = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [16, 0, -2, 2]];
    let (new_board, delta, _msg) = step(board, Direction::Left).unwrap();
    assert_eq!(new_board[3][0], 16);
    assert_eq!(new_board[3][1], -2);
    assert_eq!(new_board[3][2], 2);
    assert_eq!(delta, 0);
}

#[test]
fn right_number_merges_and_positive_score() {
    let board = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [2, 2, 4, 4]];
    let (new_board, delta, _msg) = step(board, Direction::Right).unwrap();
    assert_eq!(new_board[3][3], 8);
    assert_eq!(new_board[3][2], 4);
    assert_eq!(delta, 12);
}

#[test]
fn right_multiplier_merges_and_negative_score() {
    let board = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [-1, -1, -2, -2]];
    let (new_board, delta, _msg) = step(board, Direction::Right).unwrap();
    assert_eq!(new_board[3][3], -4);
    assert_eq!(new_board[3][2], -2);
    assert_eq!(delta, -6);
}

#[test]
fn right_number_multiplier_merges() {
    let board = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 2, -2]];
    let (new_board, delta, _msg) = step(board, Direction::Right).unwrap();
    assert_eq!(new_board[3][3], 4);
    assert_eq!(delta, 4);
}

#[test]
fn right_number_and_multiplier_do_not_merge_without_tiles_right() {
    let board = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [2, -2, 0, 0]];
    let (new_board, delta, _msg) = step(board, Direction::Right).unwrap();
    assert_eq!(new_board[3][2], 2);
    assert_eq!(new_board[3][3], -2);
    assert_eq!(delta, 0);
}

#[test]
fn right_number_and_multiplier_no_merge_with_gap() {
    let board = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [2, -2, 0, 16]];
    let (new_board, delta, _msg) = step(board, Direction::Right).unwrap();
    assert_eq!(new_board[3][1], 2);
    assert_eq!(new_board[3][2], -2);
    assert_eq!(new_board[3][3], 16);
    assert_eq!(delta, 0);
}
