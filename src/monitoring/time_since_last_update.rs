use crate::types::Entry;
use chrono::{DateTime, TimeZone, Utc};
use std::time::SystemTime;

/// Calculate the time since the last update in seconds.
pub fn time_since_last_update<T: Entry>(query: &T) -> u64 {
    let datetime: DateTime<Utc> = TimeZone::from_utc_datetime(&Utc, &query.timestamp());
    let timestamp = datetime.timestamp();
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);

    now.unwrap().as_secs() - timestamp as u64
}
