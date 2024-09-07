use axum::{self};
use context::AppContext;
use logger::logger_trace::init_logger;
use router::init_router;
use rust_boot::utils;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (_guard, handle) = init_logger("rust-boot", true);
    utils::build::show_build_info();
    let ctx = AppContext::new().await?;
    let (listener, router) = init_router(ctx, handle).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
