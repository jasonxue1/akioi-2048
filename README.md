# akioi-2048

Pure Rust 2048 engine with multiplier tiles and score tracking.

## Features

- Classic 2048 mechanics
- Special multiplier tiles: -1 (x1), -2 (x2), -4 (x4)
- Simple functional API for simulations/AI
- Detects victory (`65536` tile) and game over

## Rust API

### `init() -> [[i32; 4]; 4]`

Create a new 4x4 board with two starting tiles.

### `step(board: [[i32; 4]; 4], dir: Direction) -> Result<([[i32; 4]; 4], i32, State), String>`

- Apply a move.
- On success returns `(new_board, delta_score, state)`
- `state` is an enum: `State::{Victory, GameOver, Continue}`.
- Use `Direction::{Up,Down,Left,Right}` for `dir`.

### Example

```rust
use akioi_2048::{init, step, Direction, State};

fn main() {
    let board = init();
    let (next, delta, state) = step(board, Direction::Down).unwrap();
    println!("delta={delta}, state={state:?}, next={next:?}");
}
```

## Publishing

This library is intended to be published to crates.io. See `.github/workflows/publish.yml`.
