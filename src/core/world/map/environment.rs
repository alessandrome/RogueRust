use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "env_type")]
pub enum EnvType {
    OpenWorld {},
    Dungeon { max_rooms: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    id: String,
    name: String,
    attributes: EnvType,
}
