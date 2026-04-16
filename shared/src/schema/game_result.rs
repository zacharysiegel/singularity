use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResultSerial {
    pub game_id: Uuid,
    pub account_id: Uuid,
    pub placement: i32,
}
