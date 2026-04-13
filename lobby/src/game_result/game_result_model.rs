use serde_json::Value;
use shared::schema::game_result::GameResultSerial;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GameResult {
    pub game_id: Uuid,
    pub account_id: Uuid,
    pub placement: i32,
    pub accolades: Value,
    pub stats: Value,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct GameResultEntity {
    pub game_id: Uuid,
    pub account_id: Uuid,
    pub placement: i32,
    pub accolades: Value,
    pub stats: Value,
}

impl From<GameResultEntity> for GameResult {
    fn from(entity: GameResultEntity) -> Self {
        GameResult {
            game_id: entity.game_id,
            account_id: entity.account_id,
            placement: entity.placement,
            accolades: entity.accolades,
            stats: entity.stats,
        }
    }
}

impl From<&GameResult> for GameResultSerial {
    fn from(model: &GameResult) -> Self {
        GameResultSerial {
            game_id: model.game_id,
            account_id: model.account_id,
            placement: model.placement,
            accolades: model.accolades.clone(),
            stats: model.stats.clone(),
        }
    }
}
