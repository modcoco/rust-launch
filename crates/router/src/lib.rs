use axum::{routing::get, Router};

pub async fn init_crate_router() -> Result<Router, anyhow::Error> {
    let router = Router::new()
        .nest(
            "/",
            Router::new().route("/test-1", get(|| async { "Hello, World!" })),
        )
        .nest(
            "/",
            Router::new().route("/test-2", get(|| async { "Hello, World!" })),
        );
    Ok(router)
}
