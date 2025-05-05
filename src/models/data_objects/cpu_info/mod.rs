use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CpuReportInfo {
    pub brand: String,
    pub frequency: u64,
    pub name: String,
    pub usage_percent: f32,
    pub vendor_id: String,
}
