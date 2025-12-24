use chrono::{DateTime, FixedOffset, Utc};

pub fn utc_to_fixed_offset(time: DateTime<Utc>) -> DateTime<FixedOffset> {
    let fixed_offset = FixedOffset::east_opt(0).unwrap();
    let time_with_offset = time.with_timezone(&fixed_offset);
    time_with_offset
}

pub fn fixed_offset_to_utc(time: DateTime<FixedOffset>) -> DateTime<Utc> {
    time.with_timezone(&Utc)
}
