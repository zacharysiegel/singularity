use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResultSerial {
    pub game_id: Uuid,
    pub account_id: Uuid,
    pub placement: i32,
    pub accolades: Value,
    pub stats: Value,
}
