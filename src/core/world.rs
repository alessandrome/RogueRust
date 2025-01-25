use map::objs::character::traits::{CharacterEntity};
use map::objs::character::Character;
use crate::core::command::GameCommand;
use map::tile_zone::WorldGrid;
use crate::core::world::world_time::WorldTime;

mod world_time;
mod weather;
mod map;

pub struct World {
    pg: Character,
    npc_list: Vec<dyn CharacterEntity>,
    world_map: WorldGrid,
    world_time: WorldTime,
    buffered_command: Option<GameCommand>,
}

impl World {
    pub fn new() -> Self {
        let pg = Character::new("Player".to_string());
        World {
            pg,
            npc_list: Vec::new(),
            world_map: WorldGrid::new(),
            world_time: WorldTime::new(),
            buffered_command: None,
        }
    }

    pub fn cycle(&mut self) {
        if let Some(command) = self.buffered_command.take() {
            match command {
                GameCommand::Move(direction) => {}
                GameCommand::MoveTo(x, y) => {}
                GameCommand::Pause => {}
            }
        }
    }

    pub fn command(&mut self, command: GameCommand) {
        self.buffered_command = Some(command);
    }
}
