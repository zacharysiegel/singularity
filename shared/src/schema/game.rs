use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::FromRepr;
use uuid::Uuid;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, FromRepr)]
pub enum GameStatus {
    Pending = 0,
    Active = 1,
    Completed = 2,
}

impl Display for GameStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_string: &'static str = match self {
            GameStatus::Pending => "pending",
            GameStatus::Active => "active",
            GameStatus::Completed => "completed",
        };
        write!(f, "{:?}", display_string)
    }
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

impl CreateGameRequest {
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GameBrowserEntry {
    pub id: Uuid,
    pub name: String,
    pub creator_id: Uuid,
    pub status: GameStatus,
    pub max_players: i32,
    pub member_count: i64,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameBrowserQuery {
    pub status: Option<i32>,
}
