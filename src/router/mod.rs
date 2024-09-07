use axum::{
    http::{StatusCode, Uri},
    routing::{get, on, MethodFilter},
    Extension, Router,
};
use context::AppContext;
use logger::logger_trace::ReloadLogLevelHandle;
use router::init_crate_router;
use tokio::net::TcpListener;

use crate::handler::http::system::{info_checker, log_level};

pub async fn init_router(
    ctx: AppContext,
    log_handle: ReloadLogLevelHandle,
) -> Result<(TcpListener, Router), anyhow::Error> {
    let cfg = config::get_config();
    let addr = format!("{}:{}", cfg.web_listen_addr, cfg.web_listen_port);
    tracing::info!("start web server {}", addr);
    let listener = TcpListener::bind(addr).await?;
    let crate_router = init_crate_router().await?;
    Ok((
        listener,
        Router::new()
            .nest(
                "/api/v2",
                Router::new()
                    .route("/info", on(MethodFilter::GET, info_checker))
                    .layer(Extension(ctx)),
            )
            .nest(
                "/api/v2",
                Router::new()
                    .route("/log-level", get(log_level))
                    .layer(Extension(log_handle)),
            )
            .nest("/api/v2", crate_router)
            .fallback(fallback),
    ))
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
