pub struct Tile {
    id: usize,
    name: String,
    symbol: char,
    traversal_cost: Option<u16>,
}

impl Tile {
    pub fn new_empty() -> Tile {
        Tile {
            id: 0,
            name: "Empty".to_string(),
            symbol: 'X',
            traversal_cost: None,
        }
    }
}
