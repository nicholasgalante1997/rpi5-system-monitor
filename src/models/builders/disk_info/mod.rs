use crate::models::data_objects::disk_info::DiskReportInfo;

pub struct DiskReportInfoBuilder {
    name: Option<String>,
    mount_point: Option<String>,
    file_system: Option<String>,
    kind: Option<String>,
    total_space: Option<u64>,
    available_space: Option<u64>,
    used_space: Option<u64>,
    percentage_free: Option<f64>,
    usage_total_read_bytes: Option<u64>,
    usage_total_write_bytes: Option<u64>,
}

impl DiskReportInfoBuilder {
    pub fn new() -> Self {
        DiskReportInfoBuilder {
            name: None,
            mount_point: None,
            file_system: None,
            kind: None,
            total_space: None,
            available_space: None,
            used_space: None,
            percentage_free: None,
            usage_total_read_bytes: None,
            usage_total_write_bytes: None,
        }
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn set_mount_point(mut self, mount_point: &str) -> Self {
        self.mount_point = Some(mount_point.to_string());
        self
    }

    pub fn set_file_system(mut self, file_system: &str) -> Self {
        self.file_system = Some(file_system.to_string());
        self
    }

    pub fn set_total_space(mut self, total_space: u64) -> Self {
        self.total_space = Some(total_space);
        self
    }

    pub fn set_available_space(mut self, available_space: u64) -> Self {
        self.available_space = Some(available_space);
        self
    }

    pub fn set_used_space(mut self, used_space: u64) -> Self {
        self.used_space = Some(used_space);
        self
    }

    pub fn set_percentage_free(mut self, percentage_free: f64) -> Self {
        self.percentage_free = Some(percentage_free);
        self
    }

    pub fn set_usage_total_read_bytes(mut self, usage_total_read_bytes: u64) -> Self {
        self.usage_total_read_bytes = Some(usage_total_read_bytes);
        self
    }

    pub fn set_usage_total_write_bytes(mut self, usage_total_write_bytes: u64) -> Self {
        self.usage_total_write_bytes = Some(usage_total_write_bytes);
        self
    }

    pub fn set_kind(mut self, kind: &str) -> Self {
        self.kind = Some(kind.to_string());
        self
    }

    pub fn build(&self) -> DiskReportInfo {
        DiskReportInfo {
            name: self.name.clone().expect("DiskInfoBuilder -> build() has thrown an error -> Name not set"),
            mount_point: self.mount_point.clone().expect("DiskInfoBuilder -> build() has thrown an error -> Mount Point not set"),
            file_system: self.file_system.clone().expect("DiskInfoBuilder -> build() has thrown an error -> File System not set"),
            kind: self.kind.clone().expect("DiskInfoBuilder -> build() has thrown an error -> Kind not set"),
            total_space: self.total_space.clone().expect("DiskInfoBuilder -> build() has thrown an error -> Total Space not set"),
            available_space: self.available_space.clone().expect("DiskInfoBuilder -> build() has thrown an error -> Available Space not set"),
            used_space: self.used_space.clone().expect("DiskInfoBuilder -> build() has thrown an error -> Used Space not set"),
            percentage_free: self.percentage_free.clone().expect("DiskInfoBuilder -> build() has thrown an error -> Percentage Free not set"),
            usage_total_read_bytes: self.usage_total_read_bytes.clone().expect("DiskInfoBuilder -> build() has thrown an error -> Usage Total Read Bytes not set"),
            usage_total_write_bytes: self.usage_total_write_bytes.clone().expect("DiskInfoBuilder -> build() has thrown an error -> Usage Total Write Bytes not set"),
        }
    }

}