pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn str_trimmed() {
        let str = "nvidia.com";
        let trimmed_str = str.trim_end_matches(".com");
        println!("{}", trimmed_str);
    }

    fn naive_datetime_with_offset(start_time: NaiveDateTime, offset_hours: i32) -> NaiveDateTime {
        let start_time_utc: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(start_time, Utc);
        let offset = FixedOffset::east_opt(offset_hours * 3600).expect("Failed to create offset");
        let start_time_with_offset = start_time_utc.with_timezone(&offset);

        start_time_with_offset.naive_local()
    }

    #[test]
    fn data_without_zone() {
        let start_time = NaiveDateTime::parse_from_str("2024-07-08 12:00:00", "%Y-%m-%d %H:%M:%S")
            .expect("Failed to parse date");

        let naive_start_time_utc_plus_8 = naive_datetime_with_offset(start_time, 8);

        println!("NaiveDateTime with UTC+8: {}", naive_start_time_utc_plus_8);
    }
}
