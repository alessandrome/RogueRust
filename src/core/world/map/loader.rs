use std::path::PathBuf;
use crate::core::world::map::environment::Environment;
use crate::core::world::map::biome::Biome;
use crate::core::world::map::tile::Tile;

pub struct MapItemPrototypesLoader {
    paths: Vec<PathBuf>,
    tiles: Vec<Tile>,
    environments: Vec<Environment>,
    biomes: Vec<Biome>,
}

impl MapItemPrototypesLoader {
    pub fn new() -> MapItemPrototypesLoader {
        MapItemPrototypesLoader {
            paths: Vec::new(),
            tiles: vec![],
            environments: vec![],
            biomes: vec![],
        }
    }
    
    pub fn add_path<P: Into<PathBuf>>(&mut self, path: P) {
        self.paths.push(path.into());
    }
    
    pub fn load(&mut self) {
        for path in &self.paths {
            // TODO: load first Tiles, then Envs and lastly Biomes
        }
    }
}
