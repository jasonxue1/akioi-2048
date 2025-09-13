#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// All directions in the engine’s canonical order.
/// Order matters only for checking if any move is possible.
pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Down,
    Direction::Right,
    Direction::Up,
    Direction::Left,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Victory,
    GameOver,
    Continue,
}
