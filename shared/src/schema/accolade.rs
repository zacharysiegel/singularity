use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Accolades are the generalized store for discrete game outcome associations.
/// Each accolade ties an account to a specific game with a typed award (e.g. placement,
/// achievements). This replaces a dedicated game_result table — placement finishes
/// are accolades, not a separate concept. Accolades are always game-scoped.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccoladeSerial {
    pub id: Uuid,
    pub account_id: Uuid,
    pub game_id: Uuid,
    pub accolade_type: AccoladeType,
    pub awarded: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccoladeType {
    FirstPlace,
    SecondPlace,
    ThirdPlace,
}

impl AccoladeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccoladeType::FirstPlace => "first_place",
            AccoladeType::SecondPlace => "second_place",
            AccoladeType::ThirdPlace => "third_place",
        }
    }

    pub fn try_from_str(value: &str) -> Result<Self, crate::error::AppErrorStatic> {
        match value {
            "first_place" => Ok(AccoladeType::FirstPlace),
            "second_place" => Ok(AccoladeType::SecondPlace),
            "third_place" => Ok(AccoladeType::ThirdPlace),
            _ => Err(crate::error::AppErrorStatic::new(
                &format!("Error parsing AccoladeType [{}]", value),
            )),
        }
    }
}
