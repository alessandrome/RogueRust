use super::tile::Tile;
use std::collections::HashMap;

// Dimension types for the single map chunk
pub type ChunkWidth = u16;
pub type ChunkHeight = u16;
pub type ChunkSize = (ChunkWidth, ChunkHeight);

// Dimension types for the world map
pub type MapWidth = u32;
pub type MapHeight = u32;
pub type MapSize = (MapWidth, MapHeight);

#[derive(Eq, PartialEq, Hash)]
pub struct ChunkCoords {
    x: MapWidth,
    y: MapHeight,
}

pub struct Chunk {
    width: ChunkWidth,
    height: ChunkHeight,
    tiles: Vec<Tile>,
}

impl Chunk {
    pub fn new(width: ChunkWidth, height: ChunkHeight) -> Chunk {
        let tiles = vec![Tile::new_empty(); (width as usize) * (height as usize)];
        Chunk {
            width,
            height,
            tiles,
        }
    }

    pub fn width(&self) -> ChunkWidth {
        self.width
    }
    pub fn height(&self) -> ChunkHeight {
        self.height
    }

    pub fn tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[y * self.width as usize + x]
    }

    pub fn tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.tiles[y * self.width as usize + x]
    }
}

pub struct WorldGrid {
    grid: HashMap<ChunkCoords, Chunk>,
}

impl WorldGrid {
    pub fn new() -> Self {
        Self {
            grid: HashMap::new(),
        }
    }

    pub fn chunk(&self, x: MapWidth, y: MapHeight) -> &Chunk {
        let chunk_coords = ChunkCoords { x, y };
        let chunk_result = self.grid.get(&chunk_coords);
        if chunk_result.is_none() {
            // TODO: call to generate chunk and return it
        }
        chunk_result.unwrap()
    }

    pub fn chunk_mut(&mut self, x: MapWidth, y: MapHeight) -> &mut Chunk {
        let chunk_coords = ChunkCoords { x, y };
        let chunk_result = self.grid.get_mut(&chunk_coords);
        if chunk_result.is_none() {
            // TODO: call to generate chunk and return it
        }
        chunk_result.unwrap()
    }
}
