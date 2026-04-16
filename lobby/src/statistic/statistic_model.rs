use chrono::{DateTime, Utc};
use shared::schema::statistic::{StatisticSerial, StatisticType};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Statistic {
    pub id: Uuid,
    pub account_id: Uuid,
    pub game_id: Option<Uuid>,
    pub statistic_type: StatisticType,
    pub value: f64,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StatisticEntity {
    pub id: Uuid,
    pub account_id: Uuid,
    pub game_id: Option<Uuid>,
    pub statistic_type: String,
    pub value: f64,
    pub updated: DateTime<Utc>,
}

impl TryFrom<StatisticEntity> for Statistic {
    type Error = shared::error::AppErrorStatic;

    fn try_from(entity: StatisticEntity) -> Result<Self, Self::Error> {
        Ok(Statistic {
            id: entity.id,
            account_id: entity.account_id,
            game_id: entity.game_id,
            statistic_type: StatisticType::try_from_str(&entity.statistic_type)?,
            value: entity.value,
            updated: entity.updated,
        })
    }
}

impl From<&Statistic> for StatisticSerial {
    fn from(model: &Statistic) -> Self {
        StatisticSerial {
            id: model.id,
            account_id: model.account_id,
            game_id: model.game_id,
            statistic_type: model.statistic_type.clone(),
            value: model.value,
            updated: model.updated,
        }
    }
}
