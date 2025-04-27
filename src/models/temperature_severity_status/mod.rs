#[derive(Clone)]
pub enum TemperatureSeverityStatus {
    Normal,
    High,
    Critical,
    Unknown(String),
}

impl TemperatureSeverityStatus {
    pub fn get_temperature_status(
        temperature: f32,
        critical: Option<f32>,
    ) -> TemperatureSeverityStatus {
        if temperature == 0.0_f32 {
            return TemperatureSeverityStatus::Unknown("Missing-Temperature".to_string());
        };

        match critical {
            Some(critical_temp) => {
                if temperature < (critical_temp - 14.0_f32) {
                    TemperatureSeverityStatus::Normal
                } else if temperature < (critical_temp - 5.0_f32) {
                    TemperatureSeverityStatus::High
                } else {
                    TemperatureSeverityStatus::Critical
                }
            }
            None => TemperatureSeverityStatus::Unknown("Missing-Critical".to_string()),
        }
    }

    pub fn get_severity_color_based_on_temperature_status(
        status: TemperatureSeverityStatus,
    ) -> String {
        match status {
            TemperatureSeverityStatus::Critical => String::from("danger"),
            TemperatureSeverityStatus::High => String::from("warning"),
            TemperatureSeverityStatus::Normal => String::from("normal"),
            TemperatureSeverityStatus::Unknown(msg) => {
                crate::log::logger()
                    .extend("component-temperature-error".to_string())
                    .write(format!("Error: {}", msg));

                String::from("normal")
            }
        }
    }
}
