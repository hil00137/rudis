use chrono::{DateTime, Utc};
use chrono_tz::Tz;

pub fn gettimeofday(timezone: Option<Tz>) -> DateTime<Tz> {
    let utc_now = Utc::now();
    return match timezone {
        Some(tz) => utc_now.with_timezone(&tz),
        None => utc_now.with_timezone(&Tz::UTC)
    };
}
