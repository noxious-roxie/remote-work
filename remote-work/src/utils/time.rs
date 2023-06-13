use chrono::{FixedOffset, NaiveTime, TimeZone, Utc};

pub fn convert_to_tz(time: NaiveTime, target_tz: &FixedOffset) -> NaiveTime {
    // Create a DateTime from NaiveTime in UTC
    let utc_date_time = Utc::now().date_naive().and_time(time);

    // Convert to the target timezone
    let target_date_time = target_tz.from_utc_datetime(&utc_date_time.into());

    // Get the NaiveTime from DateTime
    target_date_time.time()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{FixedOffset, NaiveTime};

    #[test]
    fn test_convert_to_tz() {
        let original_time = NaiveTime::from_hms_opt(12, 0, 0).unwrap();

        // Convert from UTC to UTC
        let converted_time = convert_to_tz(original_time, &FixedOffset::east_opt(0).unwrap());
        assert_eq!(converted_time, original_time);

        // Convert from UTC to Asia/Kolkata (UTC+5:30)
        let converted_time = convert_to_tz(
            original_time,
            &FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap(),
        );
        assert_eq!(converted_time, NaiveTime::from_hms_opt(17, 30, 0).unwrap());

        // Convert from UTC to America/Los_Angeles (UTC-8:00)
        let converted_time =
            convert_to_tz(original_time, &FixedOffset::west_opt(8 * 3600).unwrap());
        assert_eq!(converted_time, NaiveTime::from_hms_opt(4, 0, 0).unwrap());
    }
}
