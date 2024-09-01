use axum::{routing::get, Router};
use common::{
    anyhow,
    axum::{self, Extension},
};
use context::context::Context;

pub async fn init_router() -> Result<Router, anyhow::Error> {
    let ctx = Context::new().await?;
    Ok(Router::new()
        .route("/health", get(|| async { "Hello, World!" }))
        .layer(Extension(ctx)))
}
