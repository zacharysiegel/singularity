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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountPublicSerial {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

impl CreateAccountRequest {
    pub fn is_valid(&self) -> bool {
        !self.email.is_empty() && !self.username.is_empty() && !self.password.is_empty()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAccountRequest {
    pub username: Option<String>,
    pub email: Option<String>,
}

impl UpdateAccountRequest {
    pub fn is_valid(&self) -> bool {
        self.username.is_some() || self.email.is_some()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

impl ChangePasswordRequest {
    pub fn is_valid(&self) -> bool {
        !self.new_password.is_empty()
    }
}
