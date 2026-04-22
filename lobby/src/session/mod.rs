pub(crate) mod session_db;

pub mod session_api;
pub mod session_extractor;
pub mod session_model;

pub const SESSION_DURATION: chrono::Duration = chrono::Duration::days(14);
pub const SESSION_REFRESH_THRESHOLD: chrono::Duration = chrono::Duration::days(13);
