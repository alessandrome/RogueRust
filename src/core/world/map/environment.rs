use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvRoomAttributes {
    tiles: Vec<String>,
    min_width: u8,
    max_width: u8,
    min_height: u8,
    max_height: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvTileAttributes {
    id: String,
    probability: u16, // Cumulative probability
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnvType {
    OpenWorld {
        tiles: Vec<EnvTileAttributes>,
    },
    Dungeon {
        max_rooms: u8,
        rooms: Vec<EnvRoomAttributes>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    id: String,
    name: String,
    attributes: EnvType,
}

impl Environment {
    pub fn id(&self) -> &String {
        &self.id
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }
    
    pub fn attributes(&self) -> &EnvType {
        &self.attributes
    }
}
