use akioi_2048::{init, step};

#[test]
fn init_board_has_two_tiles() {
    let board = init();
    let flat: Vec<i32> = board.into_iter().flatten().collect();
    let non_zero: Vec<i32> = flat.iter().copied().filter(|x| *x != 0).collect();
    assert_eq!(flat.len(), 16);
    assert_eq!(non_zero.len(), 2);
    for x in non_zero {
        assert!(matches!(x, -2 | -1 | 2 | 4));
    }
}

#[test]
fn step_smoke() {
    let board = init();
    let (new_board, _delta, msg) = step(board, 0).expect("step should succeed");
    assert_eq!(new_board.len(), 4);
    assert!(new_board.iter().all(|r| r.len() == 4));
    assert!(matches!(msg, -1..=1));
}
