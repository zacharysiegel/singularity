use shared::schema::game_result::GameResultSerial;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GameResult {
    pub game_id: Uuid,
    pub account_id: Uuid,
    pub placement: i32,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct GameResultEntity {
    pub game_id: Uuid,
    pub account_id: Uuid,
    pub placement: i32,
}

impl From<GameResultEntity> for GameResult {
    fn from(entity: GameResultEntity) -> Self {
        GameResult {
            game_id: entity.game_id,
            account_id: entity.account_id,
            placement: entity.placement,
        }
    }
}

impl From<&GameResult> for GameResultSerial {
    fn from(model: &GameResult) -> Self {
        GameResultSerial {
            game_id: model.game_id,
            account_id: model.account_id,
            placement: model.placement,
        }
    }
}
