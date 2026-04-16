use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticSerial {
    pub id: Uuid,
    pub account_id: Uuid,
    pub game_id: Option<Uuid>,
    pub statistic_type: StatisticType,
    pub value: f64,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatisticType {
    HoursInGame,
    Gdp,
    FirstPlaceFinishes,
    SecondPlaceFinishes,
    ThirdPlaceFinishes,
}

impl StatisticType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatisticType::HoursInGame => "hours_in_game",
            StatisticType::Gdp => "gdp",
            StatisticType::FirstPlaceFinishes => "first_place_finishes",
            StatisticType::SecondPlaceFinishes => "second_place_finishes",
            StatisticType::ThirdPlaceFinishes => "third_place_finishes",
        }
    }

    pub fn try_from_str(value: &str) -> Result<Self, crate::error::AppErrorStatic> {
        match value {
            "hours_in_game" => Ok(StatisticType::HoursInGame),
            "gdp" => Ok(StatisticType::Gdp),
            "first_place_finishes" => Ok(StatisticType::FirstPlaceFinishes),
            "second_place_finishes" => Ok(StatisticType::SecondPlaceFinishes),
            "third_place_finishes" => Ok(StatisticType::ThirdPlaceFinishes),
            _ => Err(crate::error::AppErrorStatic::new(
                &format!("Error parsing StatisticType [{}]", value),
            )),
        }
    }
}
