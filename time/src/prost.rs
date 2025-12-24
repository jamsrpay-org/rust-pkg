use chrono::{DateTime, FixedOffset};
use prost_types::Timestamp;

pub struct DateTimeFixedOffset(pub DateTime<FixedOffset>);
impl From<DateTimeFixedOffset> for Timestamp {
    fn from(value: DateTimeFixedOffset) -> Self {
        Timestamp {
            seconds: value.0.timestamp(),
            nanos: value.0.timestamp_subsec_nanos() as i32,
        }
    }
}
