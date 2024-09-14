use std::{path::PathBuf, sync::Arc};

use chrono::Local;
use tokio::time::Instant;
use tracing::Level;
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{
    fmt,
    fmt::{format::Writer, time::FormatTime},
    layer::{Layered, SubscriberExt as _},
    util::SubscriberInitExt as _,
    Registry,
};
pub use tracing_subscriber::{reload::Handle, EnvFilter};

pub type FileLogReloadLayer = Layered<
    fmt::Layer<
        Layered<tracing_subscriber::reload::Layer<EnvFilter, Registry>, Registry>,
        fmt::format::DefaultFields,
        fmt::format::Format<fmt::format::Full, LocalTimer>,
    >,
    Layered<tracing_subscriber::reload::Layer<EnvFilter, Registry>, Registry>,
>;

pub type StdoutReloadLogLevelHandle =
    tracing_subscriber::reload::Handle<tracing_subscriber::EnvFilter, tracing_subscriber::Registry>;

pub type FileReloadLogLevelHandle = Handle<EnvFilter, FileLogReloadLayer>;

pub struct LogLevelHandles {
    pub stdout_handle: StdoutReloadLogLevelHandle,
    pub file_handle: FileReloadLogLevelHandle,
}

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

pub fn init_logger(app_name: &str, log_to_file: bool) -> (Option<WorkerGuard>, LogLevelHandles) {
    let stdout_default_filter = tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()), // "info, my_crate=debug
    );
    let file_default_filter = tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
    );
    let (stdout_filter, reload_handle) =
        tracing_subscriber::reload::Layer::new(stdout_default_filter);
    let (file_filter, file_reload_handle) =
        tracing_subscriber::reload::Layer::new(file_default_filter);

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_line_number(true)
        .with_level(true)
        .with_target(true)
        .with_ansi(true)
        .with_timer(LocalTimer);

    let registry = tracing_subscriber::registry()
        .with(stdout_filter)
        .with(stdout_layer);

    let guard = if log_to_file {
        let file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            get_os_log_directory(app_name),
            to_snake_case(app_name),
        );
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        let file_layer = tracing_subscriber::fmt::layer()
            .with_line_number(true)
            // .with_thread_ids(true)
            // .with_thread_names(true)
            .with_level(true)
            .with_target(true)
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_timer(LocalTimer);
        _ = registry.with(file_filter).with(file_layer).try_init();
        Some(guard)
    } else {
        _ = registry.try_init();
        None
    };

    tracing::info!("Logger initialized");
    (
        guard,
        LogLevelHandles {
            stdout_handle: reload_handle,
            file_handle: file_reload_handle,
        },
    )
}

pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub async fn setup_stdout_log_level(
        level: LogLevel,
        reload_log_handle: StdoutReloadLogLevelHandle,
    ) -> Result<String, anyhow::Error> {
        let level_flag = match level {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        };

        let env_filter = EnvFilter::try_new(level_flag)?;
        reload_log_handle.modify(|filter| *filter = env_filter)?;

        std::env::set_var("RUST_LOG", level_flag);
        let rust_log = match std::env::var("RUST_LOG") {
            Ok(current_log_level) => current_log_level,
            Err(_) => "unknown".to_owned(),
        };

        Ok(rust_log)
    }

    pub async fn setup_file_log_level(
        level: LogLevel,
        filelog_reload_handle: Handle<EnvFilter, FileLogReloadLayer>,
    ) -> Result<String, anyhow::Error> {
        let level_flag = match level {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "info",
            LogLevel::Error => "info",
        };

        let env_filter = EnvFilter::try_new(level_flag)?;
        filelog_reload_handle.modify(|filter| *filter = env_filter)?;

        std::env::set_var("RUST_FILE_LOG", level_flag);
        let rust_file_log = match std::env::var("RUST_FILE_LOG") {
            Ok(current_log_level) => current_log_level,
            Err(_) => "unknown".to_owned(),
        };

        Ok(rust_file_log)
    }

    pub fn decode_log_level(level: &str) -> Self {
        match level.to_lowercase().as_str() {
            "trace" => LogLevel::Trace,
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => LogLevel::Info,
        }
    }
}

#[cfg(target_os = "linux")]
fn is_root() -> bool {
    unsafe { libc::getuid() == 0 }
}

#[cfg(target_os = "windows")]
fn is_root() -> bool {
    true
}

#[cfg(target_os = "macos")]
fn is_root() -> bool {
    true
}

pub fn get_os_log_directory(app_name: &str) -> PathBuf {
    let _snake_name = to_snake_case(app_name);
    let _camel_case_name = to_camel_case(app_name);

    if !is_root() {
        return PathBuf::from("logs");
    }

    #[cfg(target_os = "linux")]
    {
        PathBuf::from(format!("/var/log/{}", _snake_name))
    }
    #[cfg(target_os = "windows")]
    {
        PathBuf::from(format!("C:\\ProgramData\\{}\\Logs", _camel_case_name))
    }
    #[cfg(target_os = "macos")]
    {
        PathBuf::from(format!("/Library/Logs/{}", _camel_case_name))
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        PathBuf::from("logs")
    }
}

pub fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let mut prev_char_was_upper = false;

    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !prev_char_was_upper {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap_or(c));
            prev_char_was_upper = true;
        } else {
            result.push(c);
            prev_char_was_upper = false;
        }
    }

    result.to_lowercase().replace(' ', "")
}

pub fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_alphanumeric() {
            if capitalize_next {
                result.push(c.to_uppercase().next().unwrap_or(c));
                capitalize_next = false;
            } else {
                result.push(c);
            }
        } else {
            capitalize_next = true;
        }
    }

    result
}
