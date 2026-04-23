use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub account_id: Uuid,
    pub token: String,
    pub created: DateTime<Utc>,
    pub expiry: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SessionEntity {
    pub id: Uuid,
    pub account_id: Uuid,
    pub token: String,
    pub created: DateTime<Utc>,
    pub expiry: DateTime<Utc>,
}

impl From<SessionEntity> for Session {
    fn from(entity: SessionEntity) -> Self {
        Session {
            id: entity.id,
            account_id: entity.account_id,
            token: entity.token,
            created: entity.created,
            expiry: entity.expiry,
        }
    }
}
