mod context;

use axum::{routing::get, Router};
use common::{
    axum::{self, Extension},
    tracing,
};
use context::Context;

pub async fn init_router() -> Router {
    let ctx = Context::new()
        .await
        .map_err(|err| {
            tracing::error!("Get context err, {}", err);
        })
        .unwrap();

    Router::new()
        .route("/health", get(|| async { "Hello, World!" }))
        .layer(Extension(ctx))
}
