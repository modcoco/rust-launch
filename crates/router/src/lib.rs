use axum::{
    response::IntoResponse,
    routing::{on, MethodFilter},
    Extension, Router,
};
use context::AppContext;
use utils::{err::AxumErr, rsp::Rsp};

pub async fn init_router() -> Result<Router, anyhow::Error> {
    let ctx = AppContext::new().await?;
    Ok(Router::new()
        .route("/health", on(MethodFilter::GET, health_checker))
        .layer(Extension(ctx)))
}

pub async fn health_checker(
    Extension(ctx): Extension<AppContext>,
) -> Result<impl IntoResponse, AxumErr> {
    let uptime = ctx.start_time.elapsed();
    tracing::info!("Get container list");

    let status = serde_json::json!({
        "status": "healthy",
        "uptime_seconds": uptime.as_secs(),
        "message": "Service is running"
    });

    Ok(Rsp::success_with_optional_biz_status(
        status,
        "Data fetched successfully.",
        Some(1),
    ))
}
