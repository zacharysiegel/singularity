use chrono::Duration;

pub(crate) mod session_db;

pub mod session_api;
pub mod session_extractor;
pub mod session_model;

pub const SESSION_DURATION: Duration = Duration::days(14);
pub const SESSION_REFRESH_THRESHOLD: Duration = Duration::days(13);
