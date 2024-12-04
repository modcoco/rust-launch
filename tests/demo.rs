use serde::{Deserialize, Serialize};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn to_gb(value: i64) -> i64 {
    ((value as f64) / (1000.0 * 1000.0)).round() as i64
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        hash::{DefaultHasher, Hash, Hasher},
        io::Read,
    };

    use anyhow::Ok;
    use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
    use clap::Parser;
    use sha2::{Digest, Sha256};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn str_trimmed() {
        let str = "nvidia.com";
        let trimmed_str = str.trim_end_matches(".com");
        println!("{}", trimmed_str);
    }

    #[test]
    fn calculate() {
        let memory: i64 = 1055924872;
        let memory = ((memory as f64) / (1000.0 * 1000.0)).round() as i64;
        println!("{}", memory);

        let memory: i64 = 1055924872;
        let memory = memory / (1000 * 1000);
        println!("{}", memory);

        let memory: i64 = 1055924872;
        let memory = to_gb(memory);
        println!("{}", memory);
    }

    fn naive_datetime_with_offset(start_time: NaiveDateTime, offset_hours: i32) -> NaiveDateTime {
        let start_time_utc: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(start_time, Utc);
        let offset = FixedOffset::east_opt(offset_hours * 3600).expect("Failed to create offset");
        let start_time_with_offset = start_time_utc.with_timezone(&offset);

        start_time_with_offset.naive_local()
    }

    #[test]
    fn data_without_zone() {
        let start_time = NaiveDateTime::parse_from_str("2024-07-08 12:00:00", "%Y-%m-%d %H:%M:%S")
            .expect("Failed to parse date");

        let naive_start_time_utc_plus_8 = naive_datetime_with_offset(start_time, 8);

        println!("NaiveDateTime with UTC+8: {}", naive_start_time_utc_plus_8);
    }

    #[test]
    fn logs_file() {
        let appender = tracing_appender::rolling::never(".", "cluster.log");
        let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);
        tracing_subscriber::fmt()
            .with_writer(non_blocking_appender)
            .with_ansi(false)
            .init();
        tracing::info!("test")
    }

    #[test]
    fn remove_utc() {
        let start_time: NaiveDateTime =
            chrono::NaiveDateTime::parse_from_str("2024-07-16 17:44:14", "%Y-%m-%d %H:%M:%S")
                .map_err(|_| sqlx::Error::Configuration("invalid start_time format".into()))
                .unwrap();

        let start_time = start_time - chrono::Duration::hours(8);
        println!("{}", start_time);
        let now = chrono::Local::now().naive_local() - chrono::Duration::hours(8);

        println!("{}", now);
    }

    fn generate_unique_key_only_numb(ip: &str, hostname: &str, mac: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        ip.hash(&mut hasher);
        hostname.hash(&mut hasher);
        mac.hash(&mut hasher);
        hasher.finish()
    }

    fn generate_unique_key_sha256(ip: &str, hostname: &str, mac: &str) -> String {
        let mut hasher = Sha256::new();

        hasher.update(ip);
        hasher.update(hostname);
        hasher.update(mac);

        let result = hasher.finalize();
        let hex_string = hex::encode(result);

        hex_string[..18].to_string()
    }

    #[test]
    fn test_generate_unique_key_only_numb() {
        let ip = "192.168.1.1";
        let hostname = "host1";
        let mac = "00:00:00:00:00:01";

        let unique_key = generate_unique_key_only_numb(ip, hostname, mac);
        println!("Unique Key: {}", unique_key);
    }

    #[test]
    fn test_generate_unique_key_sha256() {
        let ip = "192.168.1.1";
        let hostname = "host1";
        let mac = "00:00:00:00:00:01";

        let unique_key = generate_unique_key_sha256(ip, hostname, mac);
        println!("Unique Key: {}", unique_key);
    }

    #[test]
    fn test_ref_string() {
        let ip: Option<String> = Some("192.168.1.1".to_string());
        // 主要用于模式匹配中借用值的引用，而不是获取值的所有权,能够避免值的移动
        if let Some(ref flag) = ip {
            // if let Some(true) = is_multi_arch { 如果判断bool类型更简单
            if flag == "true" {
                // request_json["test"] = serde_json::json!("test");
            }
        }
    }

    #[test]
    fn clap_test() {
        #[derive(Parser, Debug)]
        struct Args {
            #[arg(short, long)]
            name: String,
            #[arg(short, long, default_value_t = 1)]
            count: u8,
        }

        let args = Args::parse_from(["test", "--name", "Alice", "--count", "3"]);
        assert_eq!(args.name, "Alice");
        assert_eq!(args.count, 3);
    }

    #[test]
    fn test_idle() -> Result<(), anyhow::Error> {
        let queue_name = "volcano-queue-idp-b-1795635752299212800";
        struct MatchEntry {
            node_id: usize,
            is_match: bool,
            match_score: f64,
        }
        let mut file = File::open("test.json").expect("无法打开文件");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("无法读取文件内容");
        let rsp: NodeAndQueueResource = serde_json::from_str(&contents)?;
        assert!(!rsp.node_idle_resources.is_empty());

        let team = rsp
            .queue_idle_resources
            .iter()
            .find(|queue| queue.name == queue_name)
            .ok_or_else(|| println!("test"))
            .unwrap();

        let mut nodes_match = rsp
            .node_idle_resources
            .iter()
            .enumerate()
            .map(|(node_id, node)| {
                let match_score = node.gpu().num - team.gpu().num;
                let is_match = node.cpu >= team.cpu
                    && node.mem >= team.mem
                    && node.gpu().num >= team.gpu().num;
                MatchEntry {
                    node_id,
                    is_match,
                    match_score,
                }
            })
            .collect::<Vec<_>>();
        if nodes_match.iter().any(|match_| match_.is_match) {
            println!("1,{:?}", team.to_resource())
        }
        tracing::info!("not all nodes match queue {queue_name} idle resource");
        nodes_match.sort_unstable_by(|a, b| a.match_score.total_cmp(&b.match_score));

        println!(
            "2,{:?}",
            rsp.node_idle_resources[nodes_match.last().unwrap().node_id].to_resource()
        );
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct NodeAndQueueResource {
    node_idle_resources: Vec<IdleResource>,
    queue_idle_resources: Vec<IdleResource>,
}
#[derive(Deserialize, Debug, Clone)]
struct IdleResource {
    #[serde(default)]
    name: String,
    cpu: f64,
    mem: f64,
    #[serde(default)]
    gpu: Vec<GPU>,
}

impl IdleResource {
    /// if both mthreads and vcuda, use mthreads first
    fn gpu(&self) -> GPU {
        self.gpu
            .iter()
            .max_by_key(|gpu| gpu.num as u32)
            .map(|gpu| gpu.to_owned())
            .unwrap_or_default()
    }
    fn to_resource(&self) -> Resource {
        Resource {
            memory: self.mem,
            cpu: self.cpu,
            gpu: self.gpu(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Resource {
    /// memory GB
    /// current setting memory limit is 1/3 of memory request
    /// (spec.containers[].resources.requests.cpu)
    pub memory: f64,
    /// core count of cpu, 0.25 means 25% of one core same as 250mi in K8s resource limit
    #[serde(rename = "numCpu")]
    #[serde(alias = "cpu")]
    pub cpu: f64,
    /// if gpu<1: float, if gpu>=1: int
    /// saas version   : memory gb
    /// private version: device count of gpu cards
    #[serde(flatten)]
    pub gpu: GPU,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GPU {
    #[serde(rename = "gpuVendor")]
    #[serde(alias = "vendor")]
    #[serde(default = "gpu_vendor_default")]
    pub vendor: String,
    #[serde(rename = "gpuType")]
    #[serde(alias = "type")]
    #[serde(default = "gpu_type_default")]
    pub r#type: String,
    #[serde(rename = "numGpu")]
    #[serde(alias = "gpu")]
    // pub gpu: f64,
    #[serde(alias = "num")]
    pub num: f64,
}

pub fn gpu_vendor_default() -> String {
    "nvidia.com".to_owned()
}
pub fn gpu_type_default() -> String {
    "gpu".to_owned()
}

impl Default for GPU {
    fn default() -> Self {
        Self {
            vendor: gpu_vendor_default(),
            r#type: gpu_type_default(),
            num: 0.0,
        }
    }
}
