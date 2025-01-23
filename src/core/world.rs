use crate::core::character::Character;
use crate::core::world::world_time::WorldTime;

mod world_time;
mod weather;

pub struct World {
    pg: Character,
    npc_list: Vec<Character>,
    world_time: WorldTime,
}

impl World {
    pub fn new() -> Self {
        let pg = Character::new("Player".to_string());
        World {
            pg,
            npc_list: Vec::new(),
            world_time: WorldTime::new(),
        }
    }
}
