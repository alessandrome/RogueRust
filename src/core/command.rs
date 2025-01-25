#[derive(Copy, Clone, Debug)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub enum GameCommand {
    Move(MoveDirection),
    MoveTo(i32, i32),
    Pause,
}
