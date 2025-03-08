use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use crate::core::world::map::environment::Environment;
use crate::core::world::map::biome::Biome;
use crate::core::world::map::tile::Tile;


const TILES_JSON_NAME: &str = "tiles.json";
const ENVS_JSON_NAME: &str = "environments.json";
const BIOMES_JSON_NAME: &str = "biomes.json";
const CONF_DIR: &str = "./conf";
const OBJS_DIR: &str = "./conf/objs";
const CORE_OBJS_DIR: &str = "./conf/objs/core";

fn get_conf_core_path() -> PathBuf {
    #[cfg(test)]
    {
        return PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src/core/world/map").join(CORE_OBJS_DIR);
    }

    #[cfg(not(test))]
    {
        let exe_dir = std::env::current_exe()
            .expect("Failed to get current executable path")
            .parent()
            .expect("Executable has no parent directory")
            .to_path_buf();
        return exe_dir.join("conf");
    }
}

pub struct MapItemPrototypesLoader {
    paths: Vec<PathBuf>,
    tiles: Vec<Tile>,
    environments: Vec<Environment>,
    biomes: Vec<Biome>,
}

impl MapItemPrototypesLoader {
    pub fn new() -> MapItemPrototypesLoader {
        let core_path = get_conf_core_path();
        MapItemPrototypesLoader {
            paths: vec![core_path],
            tiles: vec![],
            environments: vec![],
            biomes: vec![],
        }
    }
    
    pub fn add_path<P: Into<PathBuf>>(&mut self, path: P) {
        self.paths.push(path.into());
    }
    
    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        for path in &self.paths {
            // TODO: load first Tiles, then Envs and lastly Biomes
            let tiles_json = path.join("tiles.json");
            let tiles_file = File::open(&tiles_json).expect(format!("Tiles file \"{}\" not found!", tiles_json.display()).as_str());
            let file_reader = BufReader::new(tiles_file);
            // let json: serde_json::Value = serde_json::from_reader(file_reader)?;
            self.tiles = serde_json::from_reader(file_reader)?;
        }
        Ok(())
    }
    
    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
}
