use chrono::{DateTime, Utc};

/// Formats `timestamp` as a coarse relative duration string for display
/// (e.g. `5s ago`, `12m ago`, `3h ago`, `2d ago`). Always uses the largest unit
/// that produces a non-zero count and clamps future timestamps to `0s ago`.
pub fn format_relative_time(timestamp: DateTime<Utc>) -> String {
    let elapsed: chrono::Duration = Utc::now() - timestamp;
    let seconds: i64 = elapsed.num_seconds().max(0);
    if seconds < 60 {
        format!("{seconds}s ago")
    } else if seconds < 3600 {
        format!("{}m ago", seconds / 60)
    } else if seconds < 86_400 {
        format!("{}h ago", seconds / 3600)
    } else {
        format!("{}d ago", seconds / 86_400)
    }
}
