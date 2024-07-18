use axum::extract::Extension;
use axum::extract::Query;
use common::axum;
use common::chrono::Local;
use common::tokio::{self, time::Instant};
use common::tracing_appender;
use common::tracing_appender::non_blocking::WorkerGuard;
use common::tracing_appender::rolling::RollingFileAppender;
use common::tracing_appender::rolling::Rotation;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::Level;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;
use tracing_subscriber::EnvFilter;

pub type ReloadLogLevelHandle =
    tracing_subscriber::reload::Handle<tracing_subscriber::EnvFilter, tracing_subscriber::Registry>;

pub fn setup_logger() -> Arc<tokio::time::Instant> {
    let start_time = Instant::now();
    let start_time: Arc<Instant> = Arc::new(start_time);

    pub struct LocalTimer;
    impl FormatTime for LocalTimer {
        fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
            write!(w, "{}", Local::now().format("%FT%T%.3f"))
        }
    }

    let format = tracing_subscriber::fmt::format()
        .with_line_number(true)
        .with_level(true)
        .with_target(true)
        .with_timer(LocalTimer);

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        // .with_writer(io::stdout) // 写入标准输出
        // .with_writer(non_blocking)
        .with_ansi(true) // 如果日志是写入文件，应将ansi的颜色输出功能关掉
        .event_format(format)
        .init();

    start_time
}

pub struct LocalTimer;
impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%FT%T%.3f"))
    }
}

pub fn init_logger() -> (ReloadLogLevelHandle, WorkerGuard) {
    let default_filter = tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()), // sqlx::query=debug
    );
    let (filter, reload_handle) = tracing_subscriber::reload::Layer::new(default_filter);

    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "rustboot");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_line_number(true)
        .with_level(true)
        .with_target(true)
        .with_ansi(true)
        .with_timer(LocalTimer);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_level(true)
        .with_target(true)
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_timer(LocalTimer);

    _ = tracing_subscriber::registry()
        .with(filter)
        .with(stdout_layer)
        .with(file_layer)
        .try_init();

    tracing::info!("Logger initialized");

    (reload_handle, guard)
}

#[allow(dead_code)]
#[test]
fn main() {
    let (_handle, _guard) = init_logger();

    // 记录日志
    tracing::info!("Client test");

    // 显式刷新缓冲区，确保日志写入文件
    // drop(guard);
    tracing::info!("This is not an example");
}

// WEB
#[derive(serde::Deserialize)]
pub struct LevelFlag {
    level: String,
}
pub async fn change_log_level(
    Query(flag): Query<LevelFlag>,
    Extension(reload_handle): Extension<ReloadLogLevelHandle>,
) -> String {
    match flag.level.parse::<EnvFilter>() {
        Ok(env_filter) => {
            reload_handle.modify(|filter| *filter = env_filter).unwrap();
            "ok".to_string()
        }
        Err(err) => err.to_string(),
    }
}

// CMD
// export RUST_LOG=debug
// let reload_handle = logger::logger_trace::init_logger();
// std::env::set_var("RUST_LOG", "info");
// let new_filter = tracing_subscriber::EnvFilter::from_default_env();
// _ = reload_handle.reload(new_filter);

pub fn get_log_directory() -> PathBuf {
    #[cfg(target_os = "linux")]
    {
        PathBuf::from("/var/log/your_app_name")
    }
    #[cfg(target_os = "windows")]
    {
        PathBuf::from("C:\\ProgramData\\YourAppName\\Logs")
    }
    #[cfg(target_os = "macos")]
    {
        PathBuf::from("/Library/Logs/YourAppName")
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        // 默认情况下，如果不是上述操作系统，可以选择一个合适的默认路径
        PathBuf::from("./logs")
    }
}
