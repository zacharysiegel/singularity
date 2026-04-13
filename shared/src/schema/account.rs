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

/// Limited public view of an account (no email)
#[derive(Debug, Clone, Serialize)]
pub struct AccountPublicSerial {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAccountRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAccountRequest {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}
