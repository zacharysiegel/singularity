use chrono::{DateTime, Utc};
use shared::schema::game_membership::GameMembershipSerial;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GameMembership {
    pub game_id: Uuid,
    pub account_id: Uuid,
    pub joined: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct GameMembershipEntity {
    pub game_id: Uuid,
    pub account_id: Uuid,
    pub joined: DateTime<Utc>,
}

impl From<GameMembershipEntity> for GameMembership {
    fn from(entity: GameMembershipEntity) -> Self {
        GameMembership {
            game_id: entity.game_id,
            account_id: entity.account_id,
            joined: entity.joined,
        }
    }
}

impl From<&GameMembership> for GameMembershipSerial {
    fn from(model: &GameMembership) -> Self {
        GameMembershipSerial {
            game_id: model.game_id,
            account_id: model.account_id,
            joined: model.joined,
        }
    }
}
