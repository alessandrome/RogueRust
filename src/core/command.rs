use super::world::objs::interaction::Interactable;

#[derive(Copy, Clone, Debug)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub enum GameCommand<'a> {
    Move(MoveDirection),
    MoveTo(i32, i32),
    Interact(&'a dyn Interactable),
    Pause,
}
