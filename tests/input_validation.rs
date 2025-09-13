use akioi_2048::{Direction, step};

fn to_board(v: [[i32; 4]; 4]) -> [[i32; 4]; 4] {
    v
}

#[test]
fn rejects_non_power_of_two() {
    let board = to_board([[3, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
    let err = step(board, Direction::Down).unwrap_err();
    assert_eq!(err, "invalid tile value: 3");
}

#[test]
fn rejects_too_large_value() {
    let board = to_board([[131072, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
    let err = step(board, Direction::Down).unwrap_err();
    assert_eq!(err, "invalid tile value: 131072");
}

#[test]
fn rejects_unknown_negative_multiplier() {
    let board = to_board([[-3, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
    let err = step(board, Direction::Down).unwrap_err();
    assert_eq!(err, "invalid tile value: -3");
}

#[test]
fn rejects_one() {
    let board = to_board([[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
    let err = step(board, Direction::Down).unwrap_err();
    assert_eq!(err, "invalid tile value: 1");
}
