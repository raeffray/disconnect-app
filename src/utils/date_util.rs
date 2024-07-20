use chrono::{DateTime, NaiveDateTime, Utc};

pub fn timestamp_to_readable(timestamp: usize) -> String {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp as i64, 0)
        .expect("Invalid timestamp");
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}