use chrono::Utc;

pub fn current_time_iso8601() -> String {
    Utc::now().to_rfc3339()
}
