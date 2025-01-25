pub mod traits;
mod common_attributes;

use traits::{CharacterEntity};

#[derive(Debug)]
pub struct Character<T: CharacterEntity> {
    id: usize,
    entity: T,
    name: String,
    health: i32,
    mana: i32,
    stamina: i32,
}

impl<T: CharacterEntity> Character<T> {
    pub fn new(name: String, entity: T) -> Self {
        Self {
            id: 0,
            entity,
            name,
            health: 0,
            mana: 0,
            stamina: 0,
        }
    }
}

impl<T> std::fmt::Display for Character<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}
