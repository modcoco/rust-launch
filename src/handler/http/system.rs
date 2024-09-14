use axum::{extract::Query, response::IntoResponse, Extension, Json};
use context::AppContext;
use logger::logger_trace::{FileReloadLogLevelHandle, LogLevel, StdoutReloadLogLevelHandle};
use utils::err::AxumErr;

use crate::service::system::info_checker_logic;

#[derive(serde::Deserialize)]
pub struct RustLogLevel {
    pub level: String,
}
pub async fn stdout_log_level(
    Query(req): Query<RustLogLevel>,
    Extension(reload_log_handle): Extension<StdoutReloadLogLevelHandle>,
) -> Result<impl IntoResponse, AxumErr> {
    let level = LogLevel::decode_log_level(&req.level);
    let current_log_level = LogLevel::setup_stdout_log_level(level, reload_log_handle).await?;
    Ok(Json(current_log_level))
}

pub async fn file_log_level(
    Query(req): Query<RustLogLevel>,
    Extension(reload_log_handle): Extension<FileReloadLogLevelHandle>,
) -> Result<impl IntoResponse, AxumErr> {
    let level = LogLevel::decode_log_level(&req.level);
    let current_file_log_level = LogLevel::setup_file_log_level(level, reload_log_handle).await?;
    Ok(Json(current_file_log_level))
}

pub async fn info_checker(
    Extension(ctx): Extension<AppContext>,
) -> Result<impl IntoResponse, AxumErr> {
    let status = info_checker_logic(ctx).await?;
    Ok(Json(status))
}
