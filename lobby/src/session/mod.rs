pub(crate) mod session_db;

pub mod session_api;
pub mod session_extractor;
pub mod session_model;

pub const SESSION_DURATION_DAYS: i64 = 14;
pub const SESSION_REFRESH_THRESHOLD_DAYS: i64 = SESSION_DURATION_DAYS - 1;
