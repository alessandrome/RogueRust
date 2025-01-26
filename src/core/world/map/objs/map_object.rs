use crate::core::world::map::objs::ObjAttributes;

pub mod traits;
mod structure;

#[derive(Clone, Debug)]
pub struct MapObject {
    pub obj_attributes: ObjAttributes,
}
