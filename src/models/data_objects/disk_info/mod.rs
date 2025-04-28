use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiskReportInfo {
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub kind: String,
    pub total_space: u64,
    pub used_space: u64,
    pub available_space: u64,
    pub percentage_free: f64,
    pub usage_total_read_bytes: u64,
    pub usage_total_write_bytes: u64,
}
