use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSerial {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAccountRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}
