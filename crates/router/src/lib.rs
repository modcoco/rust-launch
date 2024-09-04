use axum::{routing::get, Extension, Router};
use context::AppContext;

pub async fn init_router() -> Result<Router, anyhow::Error> {
    let ctx = AppContext::new().await?;
    Ok(Router::new()
        .route("/health", get(|| async { "Hello, World!" }))
        .layer(Extension(ctx)))
}
