import akioi_2048 as ak
import pytest


def test_step_rejects_invalid_direction():
    board = ak.init()
    with pytest.raises((ValueError, OverflowError)):
        ak.step(board, -1)
    with pytest.raises((ValueError, OverflowError)):
        ak.step(board, 4)


def test_step_rejects_non_integer_direction():
    board = ak.init()
    with pytest.raises((TypeError, ValueError)):
        ak.step(board, 1.5)


def test_step_rejects_non_4x4_board():
    board = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    with pytest.raises(ValueError):
        ak.step(board, 0)


def test_step_rejects_non_power_of_two():
    board = [
        [3, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    with pytest.raises(ValueError):
        ak.step(board, 0)


def test_step_rejects_too_large_value():
    board = [
        [131072, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    with pytest.raises(ValueError):
        ak.step(board, 0)


def test_step_rejects_unknown_negative_multiplier():
    board = [
        [-3, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    with pytest.raises(ValueError):
        ak.step(board, 0)
