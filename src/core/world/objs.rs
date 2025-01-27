pub mod character;
pub mod interaction;
pub mod place;

pub trait Build {
    type Params: Clone;

    fn build(obj: Self::Params) -> Self;
}

pub trait GameObject {
    fn id(&self) -> u64;
    fn name(&self) -> String;
}

#[derive(Clone, Debug)]
pub struct ObjAttributes {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Obj {
    id: u64,
    name: String,
}

impl Build for Obj {
    type Params = ObjAttributes;

    fn build(params: Self::Params) -> Self {
        Self {
            id: params.id,
            name: params.name,
        }
    }
}

impl GameObject for Obj {
    fn id(&self) -> u64 { self.id }
    fn name(&self) -> String { self.name.clone() }
}
