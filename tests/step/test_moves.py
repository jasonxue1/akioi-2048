import akioi_2048 as ak
import pytest


def count_non_zero(board):
    return sum(1 for row in board for cell in row if cell)


@pytest.mark.parametrize("direction", [0, 1, 2, 3])
def test_step_smoke_all_directions(direction):
    board = ak.init()
    new_board, delta, msg = ak.step(board, direction)
    assert len(new_board) == 4 and all(len(r) == 4 for r in new_board)
    assert isinstance(delta, int)
    assert msg in (-1, 0, 1)


def test_spawn_occurs_after_valid_move():
    board = [
        [0, 0, 0, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    new_board, delta, msg = ak.step(board, 3)
    assert count_non_zero(new_board) == 2
    assert delta == 0
    assert msg in (-1, 0, 1)


def test_no_spawn_when_move_invalid():
    board = [
        [2, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    new_board, delta, msg = ak.step(board, 2)
    assert new_board == board
    assert delta == 0
    assert msg == 0
    assert count_non_zero(new_board) == 1


def test_down_move_without_merge():
    board = [
        [-1, 2, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 0)
    assert new_board[3][0] == -1
    assert new_board[3][1] == 2
    assert delta == 0


def test_move_left_merges_adjacent_pairs_correctly():
    board = [
        [2, 2, 4, 4],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 3)
    assert new_board[0][0] == 4
    assert new_board[0][1] == 8
    assert delta == 12


def test_move_up_combines_tiles():
    board = [
        [0, 0, 0, 0],
        [2, 0, 0, 0],
        [2, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 2)
    assert new_board[0][0] == 4
    assert delta == 4
