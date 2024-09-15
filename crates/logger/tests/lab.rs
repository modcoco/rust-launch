use logger::logger_trace::init_logger;
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[test]
fn logs_file() {
    let (_guard, _handle) = logger::logger_trace::init_logger("test_boot", true);
    tracing::info!("Client test");
}

#[allow(dead_code)]
#[test]
fn main() {
    let (_guard, _handle) = init_logger("test_boot", false);

    tracing::info!("Client test");

    // drop(guard);
    tracing::info!("This is not an example");
}

#[test]
fn init_logger_t() {
    let stdout_default_filter = tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()), // "info, my_crate=debug
    );
    let (stdout_filter, _stdout_reload_handle) =
        tracing_subscriber::reload::Layer::new(stdout_default_filter);

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_line_number(true)
        .with_level(true)
        .with_target(true)
        .with_ansi(true)
        .with_timer(logger::logger_trace::LocalTimer);

    let file_appender = RollingFileAppender::new(Rotation::DAILY, "tests", "test_log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
            .with_line_number(true)
            // .with_thread_ids(true)
            // .with_thread_names(true)
            .with_level(true)
            .with_target(true)
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_timer(logger::logger_trace::LocalTimer);

    let registry = tracing_subscriber::registry()
        .with(stdout_filter)
        .with(stdout_layer);
    _ = registry.with(file_layer).try_init();

    tracing::info!("Logger initialized");
}
