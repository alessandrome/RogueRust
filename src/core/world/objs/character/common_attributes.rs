#[derive(Copy, Clone, Debug)]
pub struct Attributes {
    pub strength: u32,
    pub dexterity: u32,
    pub intellect: u32,
    pub charm: u32,
}

#[derive(Clone, Debug)]
pub struct CommonAttributes {
    pub name: String,
    pub health: i32,
    pub level: u32,
    pub exp: u32,
    pub attributes: Attributes,
}
