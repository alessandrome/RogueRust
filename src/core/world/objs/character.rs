pub mod traits;
mod common_attributes;

use traits::{CharacterEntity};
use super::{ObjAttributes, GameObject, Build};

#[derive(Clone)]
pub struct CharacterAttributes {
    pub obj_attributes: ObjAttributes,
    pub health: i32,
}

#[derive(Clone, Debug)]
pub struct Character {
    id: u64,
    name: String,
    health: i32,
}

impl Build for Character {
    type Params = CharacterAttributes;

    fn build(obj: Self::Params) -> Self {
        Self {
            id: obj.obj_attributes.id,
            name: obj.obj_attributes.name,
            health: obj.health,
        }
    }
}
