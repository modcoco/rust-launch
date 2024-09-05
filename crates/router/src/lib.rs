mod health_check;

use axum::{
    response::IntoResponse,
    routing::{on, MethodFilter},
    Extension, Json, Router,
};
use context::AppContext;
use health_check::SystemInfo;
use utils::err::AxumErr;

pub async fn init_router() -> Result<Router, anyhow::Error> {
    let ctx = AppContext::new().await?;
    Ok(Router::new()
        .route("/info", on(MethodFilter::GET, info_checker))
        .layer(Extension(ctx)))
}

pub async fn info_checker(
    Extension(ctx): Extension<AppContext>,
) -> Result<impl IntoResponse, AxumErr> {
    let uptime = ctx.running_time.elapsed();
    let start_time = ctx.start_time.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let process_info = SystemInfo::new();
    let pkg_name = std::env::var("CARGO_MAIN_PKG_NAME").unwrap_or("unknown".to_string());
    let pkg_version = std::env::var("CARGO_MAIN_PKG_VERSION").unwrap_or("unknown".to_owned());
    let status = serde_json::json!({
        "name": pkg_name,
        "version": pkg_version,
        "pid": process_info.pid,
        "status": "healthy",
        "startTime": start_time,
        "uptimeSeconds": format!("{}s",uptime.as_secs()),
        "resources": {
            "totalCpu": process_info.cpu_count,
            "totalMemory": process_info.total_memory_gb,
            "processCpu": process_info.process_cpu_usage,
            "processMemory": process_info.process_memory_mb,
        },
        "dependencies": {
            "database": "connected",
            "kubernetes": "v23",
        },
    });

    Ok(Json(status))
}
