use chrono::{DateTime, Utc};
use shared::schema::follow::{FollowSerial, FollowingSummarySerial};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Follow {
    pub source_account_id: Uuid,
    pub target_account_id: Uuid,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct FollowEntity {
    pub source_account_id: Uuid,
    pub target_account_id: Uuid,
    pub created: DateTime<Utc>,
}

impl From<FollowEntity> for Follow {
    fn from(entity: FollowEntity) -> Self {
        Follow {
            source_account_id: entity.source_account_id,
            target_account_id: entity.target_account_id,
            created: entity.created,
        }
    }
}

impl From<&Follow> for FollowSerial {
    fn from(model: &Follow) -> Self {
        FollowSerial {
            source_account_id: model.source_account_id,
            target_account_id: model.target_account_id,
            created: model.created,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FollowingSummaryRow {
    pub account_id: Uuid,
    pub username: String,
    pub is_mutual: bool,
}

impl From<&FollowingSummaryRow> for FollowingSummarySerial {
    fn from(row: &FollowingSummaryRow) -> Self {
        FollowingSummarySerial {
            account_id: row.account_id,
            username: row.username.clone(),
            is_mutual: row.is_mutual,
        }
    }
}
