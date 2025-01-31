use objs::character::traits::CharacterEntity;
use objs::character::{Character, CharacterAttributes};
use objs::{Build, ObjAttributes};
use map::world_grid::WorldGrid;
use crate::core::command::GameCommand;
use crate::core::world::world_time::WorldTime;

mod world_time;
mod weather;
mod map;
pub mod objs;

pub struct World {
    pg: Character,
    npc_list: Vec<dyn CharacterEntity>,
    world_map: WorldGrid,
    world_time: WorldTime,
    buffered_command: Option<GameCommand>,
}

impl World {
    pub fn new() -> Self {
        let pg = Character::build(
            CharacterAttributes {
                obj_attributes: ObjAttributes {
                    id: 0,
                    name: "Player".to_string(),
                },
                health: 10,
            }
        );
        
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
                GameCommand::Interact(interactable) => {}
                GameCommand::Pause => {}
            }
        }
    }

    pub fn command(&mut self, command: GameCommand) {
        self.buffered_command = Some(command);
    }
}
