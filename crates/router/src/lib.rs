use axum::{
    response::IntoResponse,
    routing::{on, MethodFilter},
    Extension, Json, Router,
};
use context::AppContext;
use utils::err::AxumErr;

pub async fn init_router() -> Result<Router, anyhow::Error> {
    let ctx = AppContext::new().await?;
    Ok(Router::new()
        .route("/info", on(MethodFilter::GET, info_checker))
        .layer(Extension(ctx)))
}

pub async fn info_checker(
    Extension(ctx): Extension<AppContext>,
) -> Result<impl IntoResponse, AxumErr> {
    let uptime = ctx.running_time.elapsed();
    let start_time = ctx.start_time.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let process_info = SystemInfo::new();
    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg_version = env!("CARGO_PKG_VERSION");
    let status = serde_json::json!({
        "name": pkg_name,
        "version": pkg_version,
        "pid": process_info.pid,
        "status": "healthy",
        "startTime": start_time,
        "uptimeSeconds": format!("{}s",uptime.as_secs()),

        "totalCpu": process_info.cpu_count,
        "totalMemory": process_info.total_memory_gb,
        "processCpu": process_info.process_cpu_usage,
        "processMemory": process_info.process_memory_mb,
    });

    Ok(Json(status))
}

// {
//     "status": "healthy",
//     "uptime": "123456",
//     "timestamp": "2024-09-04T12:34:56Z",
//     "version": "1.0.0",
//     "resources": {
//       "cpu_usage": "25%",
//       "memory_usage": "512MB",
//       "disk_usage": "40%",
//       "network": "connected"
//     },
//     "dependencies": {
//       "database": "connected",
//       "redis": "connected",
//       "external_api": "reachable"
//     },
//     "errors": []
//   }

struct SystemInfo {
    pid: String,
    cpu_count: String,
    total_memory_gb: String,
    process_cpu_usage: String,
    process_memory_mb: String,
}

impl SystemInfo {
    fn new() -> Self {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();

        let cpu_count = sys.cpus().len();
        let pid = sysinfo::get_current_pid().expect("Failed to get current PID");
        let total_memory = sys.total_memory() as f32 / 1_073_741_824.0; // 1 GB = 1024 * 1024 * 1024 字节
        let total_memory = format!("{:.2}Gi", total_memory);

        let (process_cpu_usage, process_memory_mb) = if let Some(process) = sys.process(pid) {
            let cpu_usage = process.cpu_usage();
            let memory_usage_bytes = process.memory();
            let actual_cpu_usage = cpu_usage / 100.0 * cpu_count as f32;
            let memory_usage_mb = memory_usage_bytes as f32 / 1_048_576.0; // 1 MB = 1024 * 1024 字节
            let process_cpu_usage = format!("{:.2}m", actual_cpu_usage);
            let process_memory_mb = format!("{:.2}Mi", memory_usage_mb);
            (process_cpu_usage, process_memory_mb)
        } else {
            ("unknown".to_string(), "unknown".to_string())
        };

        Self {
            pid: pid.to_string(),
            cpu_count: format!("{}m", cpu_count),
            total_memory_gb: total_memory,
            process_cpu_usage,
            process_memory_mb,
        }
    }
}
