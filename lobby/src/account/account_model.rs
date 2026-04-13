use chrono::{DateTime, Utc};
use shared::schema::account::{AccountPublicSerial, AccountSerial};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AccountEntity {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<AccountEntity> for Account {
    fn from(entity: AccountEntity) -> Self {
        Account {
            id: entity.id,
            email: entity.email,
            username: entity.username,
            password_hash: entity.password_hash,
            created: entity.created,
            updated: entity.updated,
            deleted_at: entity.deleted_at,
        }
    }
}

impl From<&Account> for AccountSerial {
    fn from(model: &Account) -> Self {
        AccountSerial {
            id: model.id,
            email: model.email.clone(),
            username: model.username.clone(),
            created: model.created,
            updated: model.updated,
        }
    }
}

impl From<&Account> for AccountPublicSerial {
    fn from(model: &Account) -> Self {
        AccountPublicSerial {
            id: model.id,
            username: model.username.clone(),
        }
    }
}
