use axum::{self};
use logger::logger_trace::init_logger;
use router::init_router;
use rust_boot::utils::build::show_build_info;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (_handle, _guard) = init_logger("rust-boot", true);
    show_build_info();
    let cfg = config::get_config();
    let router = init_router().await?;
    let addr = format!("{}:{}", cfg.web_listen_addr, cfg.web_listen_port);
    tracing::info!("start web server {}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
