use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSessionSerial {
    pub id: Uuid,
    pub game_id: Uuid,
    pub account_id: Uuid,
    pub session_id: Uuid,
    pub entered: DateTime<Utc>,
    pub exited: Option<DateTime<Utc>>,
}
