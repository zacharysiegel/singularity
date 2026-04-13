use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub account_id: Uuid,
    pub token: String,
    pub created: DateTime<Utc>,
    pub expires: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SessionEntity {
    pub id: Uuid,
    pub account_id: Uuid,
    pub token: String,
    pub created: DateTime<Utc>,
    pub expires: DateTime<Utc>,
}

impl From<SessionEntity> for Session {
    fn from(entity: SessionEntity) -> Self {
        Session {
            id: entity.id,
            account_id: entity.account_id,
            token: entity.token,
            created: entity.created,
            expires: entity.expires,
        }
    }
}
