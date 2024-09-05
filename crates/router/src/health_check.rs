use std::thread;

use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

pub struct SystemInfo {
    pub pid: String,
    pub cpu_count: String,
    pub total_memory_gb: String,
    pub process_cpu_usage: String,
    pub process_memory_mb: String,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_processes_specifics(
            ProcessesToUpdate::All,
            ProcessRefreshKind::new().with_cpu(),
        );

        let cpu_count = sys.cpus().len();
        let pid = sysinfo::get_current_pid().expect("Failed to get current PID");
        let total_memory_gb = format!("{:.2}Gi", sys.total_memory() as f32 / 1_073_741_824.0); // 1 GB = 1024^3
        let (process_cpu_usage, process_memory_mb) =
            sys.process(pid)
                .map_or(("unknown".to_string(), "unknown".to_string()), |process| {
                    let cpu_usage = process.cpu_usage();
                    let memory_usage_mb = process.memory() as f32 / 1_048_576.0; // 1 MB = 1024^2
                    (
                        format!("{:.6}%", cpu_usage),
                        format!("{:.2}Mi", memory_usage_mb),
                    )
                });
        Self {
            pid: pid.to_string(),
            cpu_count: format!("{}cores", cpu_count),
            total_memory_gb,
            process_cpu_usage,
            process_memory_mb,
        }
    }
}
