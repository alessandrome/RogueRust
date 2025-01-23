mod traits;

#[derive(Debug)]
pub struct Character {
    id: usize,
    name: String,
    health: i32,
    mana: i32,
    stamina: i32,
}

impl Character {
    pub fn new(name: String) -> Self {
        Self {
            id: 0,
            name,
            health: 0,
            mana: 0,
            stamina: 0,
        }
    }
}

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}
