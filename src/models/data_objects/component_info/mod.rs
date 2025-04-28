use serde::{Serialize, Deserialize};
use crate::models::temperature_severity_status::TemperatureSeverityStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentReportInfo {
    pub label: String,
    pub status: TemperatureSeverityStatus,
    pub temperature: f32,
    pub critical_temperature: Option<f32>,
}