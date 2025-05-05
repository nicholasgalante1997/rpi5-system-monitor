use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemReportInfo {
    pub platform: String,
    pub system_name: String,
    pub system_host_name: String,
    pub system_os_version: String,
    pub system_kernal_version: String,
    pub used_swap: u64,
    pub available_swap: u64,
    pub total_swap: u64,
    pub used_memory: u64,
    pub available_memory: u64,
    pub total_memory: u64,
    pub cpu_arch: String,
    pub num_cpus: u8,
    pub total_cpu_usage: f32,
    pub uptime_in_seconds: u64,
}
