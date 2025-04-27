use crate::models::data_objects::system_info::SystemReportInfo;

pub struct SystemInfoBuilder {
    platform: Option<String>,
    system_name: Option<String>,
    system_host_name: Option<String>,
    system_os_version: Option<String>,
    system_kernal_version: Option<String>,
    used_swap: Option<u64>,
    available_swap: Option<u64>,
    total_swap: Option<u64>,
    used_memory: Option<u64>,
    available_memory: Option<u64>,
    total_memory: Option<u64>,
    cpu_arch: Option<String>,
    num_cpus: Option<u8>,
}

impl SystemInfoBuilder {
    pub fn new() -> Self {
        SystemInfoBuilder {
            platform: None,
            system_name: None,
            system_host_name: None,
            system_os_version: None,
            system_kernal_version: None,
            used_swap: None,
            available_swap: None,
            total_swap: None,
            used_memory: None,
            available_memory: None,
            total_memory: None,
            cpu_arch: None,
            num_cpus: None,
        }
    }

    pub fn set_platform(mut self, platform: &str) -> Self {
        self.platform = Some(platform.to_string());
        self
    }

    pub fn set_system_name(mut self, system_name: &str) -> Self {
        self.system_name = Some(system_name.to_string());
        self
    }

    pub fn set_system_host_name(mut self, system_host_name: &str) -> Self {
        self.system_host_name = Some(system_host_name.to_string());
        self
    }

    pub fn set_system_os_version(mut self, system_os_version: &str) -> Self {
        self.system_os_version = Some(system_os_version.to_string());
        self
    }

    pub fn set_system_kernal_version(mut self, system_kernal_version: &str) -> Self {
        self.system_kernal_version = Some(system_kernal_version.to_string());
        self
    }

    pub fn set_used_swap(mut self, used_swap: u64) -> Self {
        self.used_swap = Some(used_swap);
        self
    }

    pub fn set_available_swap(mut self, available_swap: u64) -> Self {
        self.available_swap = Some(available_swap);
        self
    }

    pub fn set_total_swap(mut self, total_swap: u64) -> Self {
        self.total_swap = Some(total_swap);
        self
    }

    pub fn set_used_memory(mut self, used_memory: u64) -> Self {
        self.used_memory = Some(used_memory);
        self
    }

    pub fn set_available_memory(mut self, available_memory: u64) -> Self {
        self.available_memory = Some(available_memory);
        self
    }

    pub fn set_total_memory(mut self, total_memory: u64) -> Self {
        self.total_memory = Some(total_memory);
        self
    }

    pub fn set_cpu_arch(mut self, cpu_arch: &str) -> Self {
        self.cpu_arch = Some(cpu_arch.to_string());
        self
    }

    pub fn set_num_cpus(mut self, num_cpus: u8) -> Self {
        self.num_cpus = Some(num_cpus);
        self
    }

    pub fn build(&self) -> SystemReportInfo {
        SystemReportInfo {
            platform: self
                .platform
                .clone()
                .expect("SystemInfoBuilder -> build() has thrown an error -> Platform not set"),
            system_name: self
                .system_name
                .clone()
                .expect("SystemInfoBuilder -> build() has thrown an error -> System Name not set"),
            system_host_name: self.system_host_name.clone().expect(
                "SystemInfoBuilder -> build() has thrown an error -> System Host Name not set",
            ),
            system_kernal_version: self.system_kernal_version.clone().expect(
                "SystemInfoBuilder -> build() has thrown an error -> System Kernal Version not set",
            ),
            system_os_version: self.system_os_version.clone().expect(
                "SystemInfoBuilder -> build() has thrown an error -> System OS Version not set",
            ),
            used_swap: self
                .used_swap
                .clone()
                .expect("SystemInfoBuilder -> build() has thrown an error -> Used Swap not set"),
            available_swap: self.available_swap.clone().expect(
                "SystemInfoBuilder -> build() has thrown an error -> Available Swap not set",
            ),
            total_swap: self
                .total_swap
                .clone()
                .expect("SystemInfoBuilder -> build() has thrown an error -> Total Swap not set"),
            used_memory: self
                .used_memory
                .clone()
                .expect("SystemInfoBuilder -> build() has thrown an error -> Used Memory not set"),
            available_memory: self.available_memory.clone().expect(
                "SystemInfoBuilder -> build() has thrown an error -> Available Memory not set",
            ),
            total_memory: self
                .total_memory
                .clone()
                .expect("SystemInfoBuilder -> build() has thrown an error -> Total Memory not set"),
            cpu_arch: self.cpu_arch.clone().expect(
                "SystemInfoBuilder -> build() has thrown an error -> CPU Architecture not set",
            ),
            num_cpus: self.num_cpus.clone().expect(
                "SystemInfoBuilder -> build() has thrown an error -> Number of CPUs not set",
            ),
        }
    }
}
