use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMembershipSerial {
    pub game_id: Uuid,
    pub account_id: Uuid,
    pub joined: DateTime<Utc>,
}
