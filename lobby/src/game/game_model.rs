use chrono::{DateTime, Utc};
use shared::schema::game::{GameBrowserEntry, GameSerial, GameStatus};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Game {
    pub id: Uuid,
    pub name: String,
    pub creator_id: Uuid,
    pub status: GameStatus,
    pub max_players: i32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct GameEntity {
    pub id: Uuid,
    pub name: String,
    pub creator_id: Uuid,
    pub status: i32,
    pub max_players: i32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct GameBrowserRow {
    pub id: Uuid,
    pub name: String,
    pub creator_id: Uuid,
    pub status: i32,
    pub max_players: i32,
    pub member_count: i64,
    pub created: DateTime<Utc>,
}

impl TryFrom<GameEntity> for Game {
    type Error = shared::error::AppErrorStatic;

    fn try_from(entity: GameEntity) -> Result<Self, Self::Error> {
        Ok(Game {
            id: entity.id,
            name: entity.name,
            creator_id: entity.creator_id,
            status: GameStatus::try_from(entity.status)?,
            max_players: entity.max_players,
            created: entity.created,
            updated: entity.updated,
        })
    }
}

impl TryFrom<GameBrowserRow> for GameBrowserEntry {
    type Error = shared::error::AppErrorStatic;

    fn try_from(row: GameBrowserRow) -> Result<Self, Self::Error> {
        Ok(GameBrowserEntry {
            id: row.id,
            name: row.name,
            creator_id: row.creator_id,
            status: GameStatus::try_from(row.status)?,
            max_players: row.max_players,
            member_count: row.member_count,
            created: row.created,
        })
    }
}

impl From<&Game> for GameSerial {
    fn from(model: &Game) -> Self {
        GameSerial {
            id: model.id,
            name: model.name.clone(),
            creator_id: model.creator_id,
            status: model.status,
            max_players: model.max_players,
            created: model.created,
            updated: model.updated,
        }
    }
}
