use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowSerial {
    pub source_account_id: Uuid,
    pub target_account_id: Uuid,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowingSummarySerial {
    pub account_id: Uuid,
    pub username: String,
    pub is_mutual: bool,
}
