import akioi_2048 as ak


def test_game_over_message_on_full_board_no_moves():
    board = [
        [2, 4, 8, 16],
        [32, 64, 128, 256],
        [512, 1024, 2048, 4096],
        [8192, 16384, 32768, 8192],
    ]
    new_board, delta, msg = ak.step(board, 0)
    assert new_board == board
    assert delta == 0
    assert msg == -1


def test_victory_message_when_creating_65536():
    board = [
        [32768, 0, 0, 0],
        [32768, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    new_board, delta, msg = ak.step(board, 0)
    flat = [c for row in new_board for c in row]
    assert 65536 in flat
    assert delta == 65536
    assert msg == 1


def test_full_board_with_merge_available_not_game_over():
    board = [
        [2, 2, 4, 8],
        [16, 32, 64, 128],
        [256, 512, 1024, 2048],
        [4096, 8192, 16384, 32768],
    ]
    new_board, _, msg = ak.step(board, 3)
    assert msg == 0
    assert new_board != board
