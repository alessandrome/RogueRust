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
#[serde(tag = "env_type")]
pub enum EnvType {
    OpenWorld {
        tiles: Vec<String>
    },
    Dungeon {
        max_rooms: u8,
        rooms: Vec<EnvRoomAttributes>
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    id: String,
    name: String,
    attributes: EnvType,
}
