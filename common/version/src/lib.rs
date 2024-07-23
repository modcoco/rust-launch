pub use anyhow;
pub use axum;
pub use base64;
pub use chrono;
pub use dotenv;
pub use futures_util;
pub use libc;
pub use native_tls;
pub use reqwest;
pub use rustls_pemfile;
pub use serde;
pub use serde_json;
pub use sqlx;
pub use tokio;
pub use tokio_tungstenite;
pub use toml;
pub use tracing;
pub use tracing_appender;

pub mod constants;
pub mod err;
pub mod rsp;

use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use constants::*;

pub fn url_https_builder(domain: &str, port: &str, path: Option<&str>) -> String {
    base_http_builder(URL_HTTPS, domain, port, path)
}

pub fn url_http_builder(domain: &str, port: &str, path: Option<&str>) -> String {
    base_http_builder(URL_HTTP, domain, port, path)
}

fn base_http_builder(http_header: &str, domain: &str, port: &str, path: Option<&str>) -> String {
    match path {
        Some(p) => [http_header, domain, COLON, port, p].concat(),
        None => [http_header, domain, COLON, port].concat(),
    }
}

pub fn naive_datetime_with_offset(start_time: NaiveDateTime, offset_hours: i32) -> NaiveDateTime {
    let start_time_utc: DateTime<Utc> = DateTime::<Utc>::from_naive_utc_and_offset(start_time, Utc);
    let offset = FixedOffset::east_opt(offset_hours * 3600).expect("Failed to create offset");
    let start_time_with_offset = start_time_utc.with_timezone(&offset);

    start_time_with_offset.naive_local()
}
