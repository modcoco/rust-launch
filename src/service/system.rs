use context::AppContext;
use sqlx::Row;

use crate::utils::health_check::{SystemDependencies, SystemInfo, SystemResources, SystemStatus};

pub async fn info_checker_logic(ctx: AppContext) -> Result<SystemStatus, anyhow::Error> {
    let uptime = ctx.running_time.elapsed();
    let start_time = ctx.start_time.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let process_info = SystemInfo::new();
    let pkg_name = std::env::var("CARGO_MAIN_PKG_NAME").unwrap_or("unknown".to_string());
    let pkg_version = std::env::var("CARGO_MAIN_PKG_VERSION").unwrap_or("unknown".to_owned());
    let client = ctx.kube_client;
    let kube_version_info: Option<kube::k8s_openapi::apimachinery::pkg::version::Info> =
        match client.apiserver_version().await {
            Ok(info) => Some(info),
            Err(_) => None,
        };
    let pg_version = match sqlx::query("SELECT version()")
        .fetch_one(&ctx.pg_pool)
        .await
    {
        Ok(row) => {
            let version: String = row.get(0);
            Some(version)
        }
        Err(_) => None,
    };
    let status = SystemStatus {
        name: pkg_name,
        version: pkg_version,
        pid: process_info.pid,
        status: "healthy".to_string(),
        start_time,
        uptime_seconds: format!("{}s", uptime.as_secs()),
        resources: SystemResources {
            total_cpu: process_info.cpu_count,
            total_memory: process_info.total_memory_gb,
            process_cpu: process_info.process_cpu_usage,
            process_memory: process_info.process_memory_mb,
        },
        dependencies: SystemDependencies {
            database: pg_version,
            kubernetes: kube_version_info,
        },
    };

    Ok(status)
}
