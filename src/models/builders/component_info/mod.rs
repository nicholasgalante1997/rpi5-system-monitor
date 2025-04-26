use crate::models::{
    data_objects::component_info::ComponentReportInfo,
    temperature_severity_status::TemperatureSeverityStatus,
};

pub struct ComponentReportInfoBuilder {
    label: Option<String>,
    status: Option<TemperatureSeverityStatus>,
    temperature: Option<f32>,
    critical_temperature: Option<f32>,
}

impl ComponentReportInfoBuilder {
    pub fn new() -> Self {
        ComponentReportInfoBuilder {
            label: None,
            status: None,
            temperature: None,
            critical_temperature: None,
        }
    }

    pub fn set_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn set_status(mut self, status: TemperatureSeverityStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn set_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn set_critical_temperature(mut self, critical_temperature: Option<f32>) -> Self {
        self.critical_temperature = critical_temperature;
        self
    }

    pub fn build(self) -> ComponentReportInfo {
        ComponentReportInfo {
            label: self.label.expect(
                "ComponentReportInfoBuilder -> build() has thrown an error -> Label not set",
            ),
            status: self.status.expect(
                "ComponentReportInfoBuilder -> build() has thrown an error -> Status not set",
            ),
            temperature: self.temperature.expect(
                "ComponentReportInfoBuilder -> build() has thrown an error -> Temperature not set",
            ),
            critical_temperature: self.critical_temperature,
        }
    }
}
