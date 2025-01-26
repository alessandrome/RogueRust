pub mod map_object;
pub mod character;
pub mod interaction;

#[derive(Clone, Debug)]
pub struct ObjAttributes {
    id: u64,
    name: String,
}
