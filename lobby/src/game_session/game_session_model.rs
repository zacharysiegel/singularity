use chrono::{DateTime, Utc};
use shared::schema::game_session::GameSessionSerial;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GameSession {
    pub id: Uuid,
    pub game_id: Uuid,
    pub account_id: Uuid,
    pub session_id: Uuid,
    pub entered: DateTime<Utc>,
    pub exited: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct GameSessionEntity {
    pub id: Uuid,
    pub game_id: Uuid,
    pub account_id: Uuid,
    pub session_id: Uuid,
    pub entered: DateTime<Utc>,
    pub exited: Option<DateTime<Utc>>,
}

impl GameSession {
    pub fn is_active(&self) -> bool {
        self.exited.is_none()
    }
}

impl From<GameSessionEntity> for GameSession {
    fn from(entity: GameSessionEntity) -> Self {
        GameSession {
            id: entity.id,
            game_id: entity.game_id,
            account_id: entity.account_id,
            session_id: entity.session_id,
            entered: entity.entered,
            exited: entity.exited,
        }
    }
}

impl From<&GameSession> for GameSessionSerial {
    fn from(model: &GameSession) -> Self {
        GameSessionSerial {
            id: model.id,
            game_id: model.game_id,
            account_id: model.account_id,
            session_id: model.session_id,
            entered: model.entered,
            exited: model.exited,
        }
    }
}
