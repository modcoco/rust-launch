pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn to_gb(value: i64) -> i64 {
    ((value as f64) / (1000.0 * 1000.0)).round() as i64
}

#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hash, Hasher};

    use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
    use hex::encode;
    use sha2::{Digest, Sha256};

    use super::*;

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

    #[test]
    fn calculate() {
        let memory: i64 = 1055924872;
        let memory = ((memory as f64) / (1000.0 * 1000.0)).round() as i64;
        println!("{}", memory);

        let memory: i64 = 1055924872;
        let memory = memory / (1000 * 1000);
        println!("{}", memory);

        let memory: i64 = 1055924872;
        let memory = to_gb(memory);
        println!("{}", memory);
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

    #[test]
    fn logs_file() {
        let appender = tracing_appender::rolling::never(".", "cluster.log");
        let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);
        tracing_subscriber::fmt()
            .with_writer(non_blocking_appender)
            .with_ansi(false)
            .init();
        tracing::info!("test")
    }

    #[test]
    fn remove_utc() {
        let start_time: NaiveDateTime =
            chrono::NaiveDateTime::parse_from_str("2024-07-16 17:44:14", "%Y-%m-%d %H:%M:%S")
                .map_err(|_| sqlx::Error::Configuration("invalid start_time format".into()))
                .unwrap();

        let start_time = start_time - chrono::Duration::hours(8);
        println!("{}", start_time);
        let now = chrono::Local::now().naive_local() - chrono::Duration::hours(8);

        println!("{}", now);
    }

    fn generate_unique_key_only_numb(ip: &str, hostname: &str, mac: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        ip.hash(&mut hasher);
        hostname.hash(&mut hasher);
        mac.hash(&mut hasher);
        hasher.finish()
    }

    fn generate_unique_key_sha256(ip: &str, hostname: &str, mac: &str) -> String {
        let mut hasher = Sha256::new();

        hasher.update(ip);
        hasher.update(hostname);
        hasher.update(mac);

        let result = hasher.finalize();
        let hex_string = hex::encode(result);

        hex_string[..18].to_string()
    }

    #[test]
    fn test_generate_unique_key_only_numb() {
        let ip = "192.168.1.1";
        let hostname = "host1";
        let mac = "00:00:00:00:00:01";

        let unique_key = generate_unique_key_only_numb(ip, hostname, mac);
        println!("Unique Key: {}", unique_key);
    }

    #[test]
    fn test_generate_unique_key_sha256() {
        let ip = "192.168.1.1";
        let hostname = "host1";
        let mac = "00:00:00:00:00:01";

        let unique_key = generate_unique_key_sha256(ip, hostname, mac);
        println!("Unique Key: {}", unique_key);
    }
}
