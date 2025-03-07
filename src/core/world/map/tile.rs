mod types;
mod structures;
mod effect;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tile {
    id: String,
    name: String,
    symbol: char,
    walking_cost: Option<u16>,
}

impl Tile {
    pub fn new_empty() -> Tile {
        Tile {
            id: "null".to_string(),
            name: "Empty".to_string(),
            symbol: 'X',
            walking_cost: None,
        }
    }
}
