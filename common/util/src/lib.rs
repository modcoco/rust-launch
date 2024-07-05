#![deny(unused_crate_dependencies)]
use common::chrono::{self, NaiveDateTime, TimeZone as _};
use common::serde::{self, Deserialize, Deserializer, Serializer};
use std::{fmt::Display, str::FromStr};

// 通用的从字符串反序列化为类型 T
pub fn deserialize_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<T>().map_err(serde::de::Error::custom)
}

// 特定类型的反序列化函数
pub fn deserialize_u64_from_str<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_from_str(deserializer)
}

pub fn deserialize_i64_from_str<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_from_str(deserializer)
}

// 通用的序列化类型 T 成字符串
pub fn serialize_to_str<S, T>(x: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    serializer.serialize_str(&x.to_string())
}

// 特定类型的序列化函数
pub fn serialize_u64_to_string<S>(x: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serialize_to_str(x, serializer)
}

pub fn serialize_i64_to_string<S>(x: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serialize_to_str(x, serializer)
}

pub fn serialize_i16_to_string<S>(x: &i16, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serialize_to_str(x, serializer)
}

// 日期时间类型的特定序列化函数
pub fn serialize_datetime_to_ymd_hms<S>(x: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let dt: chrono::DateTime<chrono::Local> = chrono::Local.from_utc_datetime(x);
    serializer.serialize_str(&dt.format("%F %T").to_string())
}

pub fn serialize_datetime_to_local_string<S>(
    x: &chrono::DateTime<chrono::Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let local_time = x.with_timezone(&chrono::Local);
    let formatted_dt = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
    serializer.serialize_str(&formatted_dt)
}
