use chrono::{DateTime, Utc};
use shared::schema::accolade::{AccoladeSerial, AccoladeType};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Accolade {
    pub id: Uuid,
    pub account_id: Uuid,
    pub game_id: Uuid,
    pub accolade_type: AccoladeType,
    pub awarded: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct AccoladeEntity {
    pub id: Uuid,
    pub account_id: Uuid,
    pub game_id: Uuid,
    pub accolade_type: String,
    pub awarded: DateTime<Utc>,
}

impl TryFrom<AccoladeEntity> for Accolade {
    type Error = shared::error::AppErrorStatic;

    fn try_from(entity: AccoladeEntity) -> Result<Self, Self::Error> {
        Ok(Accolade {
            id: entity.id,
            account_id: entity.account_id,
            game_id: entity.game_id,
            accolade_type: AccoladeType::try_from_str(&entity.accolade_type)?,
            awarded: entity.awarded,
        })
    }
}

impl From<&Accolade> for AccoladeSerial {
    fn from(model: &Accolade) -> Self {
        AccoladeSerial {
            id: model.id,
            account_id: model.account_id,
            game_id: model.game_id,
            accolade_type: model.accolade_type.clone(),
            awarded: model.awarded,
        }
    }
}
