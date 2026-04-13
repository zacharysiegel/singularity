use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::FromRepr;
use uuid::Uuid;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, FromRepr)]
pub enum GameStatus {
    Pending = 0,
    Active = 1,
    Completed = 2,
}

crate::try_from_repr!(GameStatus<i32>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSerial {
    pub id: Uuid,
    pub name: String,
    pub creator_id: Uuid,
    pub status: GameStatus,
    pub max_players: i32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateGameRequest {
    pub name: String,
    pub max_players: Option<i32>,
}
