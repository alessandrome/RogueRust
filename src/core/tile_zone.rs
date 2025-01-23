use super::tile::{Tile};

pub struct TileZone {
    width: usize,
    height: usize,
    tiles_grid: Vec<Vec<Tile>>
}

impl TileZone {
    pub fn new(width: usize, height: usize) -> TileZone {
        let tiles_grid = vec![vec![Tile::new_empty(); width]; height];
        TileZone {
            width,
            height,
            tiles_grid,
        }
    }
}
