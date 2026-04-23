use chrono::{DateTime, Utc};
use shared::schema::follow::{FollowSerial, FollowingSummarySerial};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Follow {
    pub follower_account_id: Uuid,
    pub followed_account_id: Uuid,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct FollowEntity {
    pub follower_account_id: Uuid,
    pub followed_account_id: Uuid,
    pub created: DateTime<Utc>,
}

impl From<FollowEntity> for Follow {
    fn from(entity: FollowEntity) -> Self {
        Follow {
            follower_account_id: entity.follower_account_id,
            followed_account_id: entity.followed_account_id,
            created: entity.created,
        }
    }
}

impl From<&Follow> for FollowSerial {
    fn from(model: &Follow) -> Self {
        FollowSerial {
            follower_account_id: model.follower_account_id,
            followed_account_id: model.followed_account_id,
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
