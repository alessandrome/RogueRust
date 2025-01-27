use super::ObjAttributes;

pub mod traits;
pub mod structure;

#[derive(Clone, Debug)]
pub struct Place {
    pub obj_attributes: ObjAttributes,
}
