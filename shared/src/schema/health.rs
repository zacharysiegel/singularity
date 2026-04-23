use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponseSerial {
    pub status: HealthStatusSerial,
    pub database: DatabaseStatusSerial,
}

impl HealthResponseSerial {
    pub fn nominal() -> Self {
        Self {
            status: HealthStatusSerial::Ok,
            database: DatabaseStatusSerial::Connected,
        }
    }

    pub fn database_unreachable() -> Self {
        Self {
            status: HealthStatusSerial::Degraded,
            database: DatabaseStatusSerial::Unreachable,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatusSerial {
    Ok,
    Degraded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseStatusSerial {
    Connected,
    Unreachable,
}
