pub mod traits;
mod common_attributes;

use traits::{CharacterEntity};
use crate::core::world::map::objs::ObjAttributes;

#[derive(Debug)]
pub struct Character<T: CharacterEntity> {
    // obj_attributes: ObjAttributes,
    entity: T,
    health: i32,
    mana: i32,
    stamina: i32,
}

impl<T: CharacterEntity> Character<T> {
    pub fn new(name: String, entity: T) -> Self {
        // let ObjAttributes =
        Self {
            // obj_attributes:
            entity,
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
