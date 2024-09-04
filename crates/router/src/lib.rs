use axum::{routing::get, Extension, Router};
use context::AppContext;

pub async fn init_router() -> Result<Router, anyhow::Error> {
    let ctx = AppContext::new().await?;
    Ok(Router::new()
        .route("/health", get(|| async { "Hello, World!" }))
        .layer(Extension(ctx)))
}

// async fn health_handler(Extension(ctx): Extension<Arc<AppContext>>) -> Result<(), AxumErr> {
//     let uptime = ctx.start_time.elapsed();

//     let status = json!({
//         "status": "healthy",
//         "uptime_seconds": uptime.as_secs(),
//         "message": "Service is running"
//     });

//     Ok(())
// }
