trait Enemy {}
trait Playable {}
trait Character {}
pub trait CharacterEntity {
    fn get_position(&self) -> (i32, i32);
    fn get_health(&self) -> i32;
    fn get_name(&self) -> String;
    fn move_to(&mut self, x: i32, y: i32);
    fn attributes() -> &CommonAttributes;
}
