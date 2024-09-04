use axum::{routing::get, Extension, Router};
use context::context::{KubeContext, PgContext};

pub async fn init_router() -> Result<Router, anyhow::Error> {
    let kube_ctx = KubeContext::new().await?;
    let pg_ctx = PgContext::new().await;
    Ok(Router::new()
        .route("/health", get(|| async { "Hello, World!" }))
        .layer(Extension(kube_ctx))
        .layer(Extension(pg_ctx)))
}
