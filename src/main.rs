use axum::{self};
use logger::logger_trace::init_logger;
use router::init_router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let (_handle, _guard) = init_logger("rust-boot", true);
    let router = init_router().await?;
    tracing::info!("start web server...");
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, router).await?;
    Ok(())
}