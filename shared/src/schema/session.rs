use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl LoginRequest {
    pub fn is_valid(&self) -> bool {
        !self.email.is_empty() && !self.password.is_empty()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub token: String,
}
