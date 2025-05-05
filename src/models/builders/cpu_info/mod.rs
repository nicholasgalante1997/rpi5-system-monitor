use crate::models::data_objects::cpu_info::CpuReportInfo;

pub struct CpuInfoBuilder {
    brand: Option<String>,
    frequency: Option<u64>,
    name: Option<String>,
    usage_percent: Option<f32>,
    vendor_id: Option<String>,
}

impl CpuInfoBuilder {
    pub fn new() -> Self {
        CpuInfoBuilder {
            brand: None,
            frequency: None,
            name: None,
            usage_percent: None,
            vendor_id: None,
        }
    }

    pub fn set_brand(mut self, brand: String) -> Self {
        self.brand = Some(brand);
        self
    }

    pub fn set_frequency(mut self, frequency: u64) -> Self {
        self.frequency = Some(frequency);
        self
    }

    pub fn set_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn set_usage_percent(mut self, usage_percent: f32) -> Self {
        self.usage_percent = Some(usage_percent);
        self
    }

    pub fn set_vendor_id(mut self, vendor_id: String) -> Self {
        self.vendor_id = Some(vendor_id);
        self
    }

    pub fn build(self) -> CpuReportInfo {
        CpuReportInfo {
            brand: self
                .brand
                .expect("CpuInfoBuilder -> build() has thrown an error -> Brand not set"),
            frequency: self
                .frequency
                .expect("CpuInfoBuilder -> build() has thrown an error -> Frequency not set"),
            name: self
                .name
                .expect("CpuInfoBuilder -> build() has thrown an error -> Name not set"),
            usage_percent: self
                .usage_percent
                .expect("CpuInfoBuilder -> build() has thrown an error -> Usage Percent not set"),
            vendor_id: self
                .vendor_id
                .expect("CpuInfoBuilder -> build() has thrown an error -> Vendor ID not set"),
        }
    }
}
