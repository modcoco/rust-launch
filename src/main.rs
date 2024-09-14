use axum::{self};
use context::AppContext;
use logger::logger_trace::init_logger;
use rust_boot::{router::init_router, utils};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (_guard, handle, _file_log_hande) = init_logger("rust-boot", true);
    utils::build::show_build_info();
    let ctx = AppContext::new().await?;
    let (listener, router) = init_router(ctx, handle).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
