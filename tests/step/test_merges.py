import akioi_2048 as ak


def test_number_merges_and_positive_score():
    board = [
        [2, 0, 0, 0],
        [2, 0, 0, 0],
        [4, 0, 0, 0],
        [4, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 0)
    assert new_board[3][0] == 8
    assert new_board[2][0] == 4
    assert delta == 12


def test_multiplier_merges_and_negative_score():
    board = [
        [-1, 0, 0, 0],
        [-1, 0, 0, 0],
        [-2, 0, 0, 0],
        [-2, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 0)
    assert new_board[3][0] == -4
    assert new_board[2][0] == -2
    assert delta == -6


def test_no_merge_for_negative_four():
    board = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [-4, 0, 0, 0],
        [-4, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 0)
    assert new_board == board
    assert delta == 0


def test_number_and_multiplier_merge_with_tiles_below():
    board = [
        [0, 0, 0, 0],
        [2, 0, 0, 0],
        [-2, 0, 0, 0],
        [4, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 0)
    assert new_board[3][0] == 8
    assert new_board[2][0] == 2
    assert delta == 8


def test_number_and_multiplier_do_not_merge_without_tiles_below():
    board = [
        [2, 0, 0, 0],
        [-2, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 0)
    assert new_board[2][0] == 2
    assert new_board[3][0] == -2
    assert delta == 0


def test_number_and_multiplier_no_merge_with_gap():
    board = [
        [2, 0, 0, 0],
        [-2, 0, 0, 0],
        [0, 0, 0, 0],
        [16, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 0)
    assert new_board[1][0] == 2
    assert new_board[2][0] == -2
    assert new_board[3][0] == 16
    assert delta == 0


def test_single_tile_merges_only_once():
    board = [
        [2, 2, 2, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 3)
    assert new_board[0][0] == 4
    assert new_board[0][1] == 2
    assert delta == 4
