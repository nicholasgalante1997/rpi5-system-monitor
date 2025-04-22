use sysinfo::{Component, Components, Cpu, Disk, Disks, Networks, System};

use crate::utils;

enum TemperatureSeverityStatus {
    Normal,
    High,
    Critical,
    Unknown(String),
}

pub struct SystemReport<'a, 'b, 'c> {
    pub platform: String,
    pub available_swap: u64,
    pub system_name: String,

    pub total_memory: u64,
    pub used_memory: u64,
    pub available_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub system_kernal_version: String,
    pub system_os_version: String,
    pub system_host_name: String,
    pub cpus: &'a [Cpu],
    pub num_cpus: u8,
    pub cpu_arch: String,
    pub components: Vec<&'b Component>,
    pub disks: Vec<&'c Disk>,
}

impl<'a, 'b, 'c> SystemReport<'_, '_, '_> {
    pub fn into_html(&self) -> String {
        format!(
            r#"
                <div class="cards-container">
                    <!-- Platform Info Card -->
                    {}

                    <!-- Memory Card -->
                    {}

                    <!-- CPU Card -->
                    {}

                    <!-- Temperature Card -->
                    {}

                    <!-- Disk Card -->
                    {}
                </div>
            "#,
            &self.convert_platform_data_to_markup_card(),
            &self.convert_memory_info_to_markup_card(),
            &self.convert_cpus_to_markup_cards(),
            &self.convert_components_info_into_temperature_markup_cards(),
            &self.convert_disks_to_markup_card(),
        )
    }

    fn convert_cpus_to_markup_cards(&self) -> String {
        let mut cpu_markup = String::new();
        for cpu in self.cpus {
            let card = format!(
                r#"
                    <!-- CPU Card -->
                    <div class="card">
                        <div class="card-title">
                            <i class="fas fa-server"></i>
                            CPU
                        </div>
                        <div class="info-row">
                            <span class="info-label">Model</span>
                            <span class="info-value">{}</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Core Name</span>
                            <span class="info-value">{}</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Clock Speed</span>
                            <span class="info-value">1.8 GHz</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">CPU Usage</span>
                            <span class="info-value">{}%</span>
                        </div>
                        <div class="progress-container">
                            <div class="progress-bar" style="width: {}%"></div>
                        </div>
                    </div>
                "#,
                &cpu.brand(),
                &cpu.name(),
                &cpu.cpu_usage(),
                &cpu.cpu_usage(),
            );

            cpu_markup.push_str(&card);
        }

        cpu_markup
    }

    fn convert_platform_data_to_markup_card(&self) -> String {
        format!(
            r#"
                <div class="card">
                    <div class="card-title">
                        <i class="fas fa-microchip"></i>
                        Platform Information
                    </div>
                    <div class="info-row">
                        <span class="info-label">Operating System</span>
                        <span class="info-value">{}</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">Kernel Version</span>
                        <span class="info-value">{}</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">Num Cpus</span>
                        <span class="info-value">{}</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">Architecture</span>
                        <span class="info-value">{}</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">Hostname</span>
                        <span class="info-value">{}</span>
                    </div>
                </div>
            "#,
            &self.system_os_version,
            &self.system_kernal_version,
            &self.num_cpus,
            &self.cpu_arch,
            &self.system_host_name,
        )
    }

    fn convert_memory_info_to_markup_card(&self) -> String {
        format!(
            r#"
                <div class="card">
                    <div class="card-title">
                        <i class="fas fa-memory"></i>
                        Memory
                    </div>
                    <div class="info-row">
                        <span class="info-label">Total Memory</span>
                        <span class="info-value">{:.3} GB</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">Used Memory</span>
                        <span class="info-value">{:.3} GB</span>
                    </div>
                    <div class="progress-container">
                        <div class="progress-bar" style="width: {}%"></div>
                    </div>
                    <div class="info-row">
                        <span class="info-label">Available Memory</span>
                        <span class="info-value">{:.3} GB</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">Swap Usage</span>
                        <span class="info-value">{:.3} MB / {:.3} GB</span>
                    </div>
                </div>
            "#,
            utils::convert_bytes_to_gbs(self.total_memory),
            utils::convert_bytes_to_gbs(self.used_memory),
            utils::convert_to_percent(self.used_memory, self.total_memory),
            utils::convert_bytes_to_gbs(self.available_memory),
            utils::convert_bytes_to_mbs(self.used_swap),
            utils::convert_bytes_to_gbs(self.total_swap),
        )
    }

    fn convert_disks_to_markup_card(&self) -> String {
        let mut cards = String::new();

        for disk in &self.disks {
            let card = format!(
                r#"
                    <div class="card">
                        <div class="card-title">
                            <i class="fas fa-hdd"></i>
                            Storage
                        </div>
                        <div class="info-row">
                            <span class="info-label">Device</span>
                            <span class="info-value">{:?} {:#?}</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">FS</span>
                            <span class="info-value">{:#?}</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Total Space</span>
                            <span class="info-value">{:.3} GB</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Available</span>
                            <span class="info-value">{:.2} GB ({:.1}%)</span>
                        </div>
                        <div class="progress-container">
                            <div class="progress-bar" style="width: {}%"></div>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Bytes Read / Bytes Written</span>
                            <span class="info-value">{}B / {}B</span>
                        </div>
                    </div>        
                "#,
                disk.name(),
                disk.mount_point(),
                disk.file_system(),
                utils::convert_bytes_to_gbs(disk.total_space()),
                utils::convert_bytes_to_gbs(disk.available_space()),
                utils::convert_to_percent(disk.available_space(), disk.total_space()),
                100.0_f64 - utils::convert_to_percent(disk.available_space(), disk.total_space()),
                disk.usage().total_read_bytes,
                disk.usage().total_written_bytes
            );

            cards.push_str(&card);
        }

        cards
    }

    fn convert_components_info_into_temperature_markup_cards(&self) -> String {
        let mut cards = format!(
            r#"
                <div class="card">
                    <div class="card-title">
                        <i class="fas fa-thermometer-half"></i>
                        System Temperatures
                    </div>
            "#
        );

        for component in &self.components[..] {
            let temp = match component.temperature() {
                Some(value) => value,
                None => 0.0_f32,
            };

            let critical = component.critical();

            let card = format!(
                r#"
                    <div class="info-row">
                        <span class="info-label">{} CPU Temperature</span>
                        <div class="temperature-indicator">
                            <span class="temperature-value {}">{}°C</span>
                        </div>
                    </div>
                "#,
                component.label(),
                SystemReport::get_severity_color_based_on_temperature_status(
                    SystemReport::get_temperature_status(temp, critical)
                ),
                temp
            );

            cards.push_str(&card);
        }

        cards.push_str(
            r#"
            </div>
        "#,
        );

        cards
    }

    fn get_temperature_status(
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

    fn get_severity_color_based_on_temperature_status(status: TemperatureSeverityStatus) -> String {
        match status {
            TemperatureSeverityStatus::Critical => String::from("danger"),
            TemperatureSeverityStatus::High => String::from("warning"),
            TemperatureSeverityStatus::Normal => String::from("normal"),
            TemperatureSeverityStatus::Unknown(_) => String::from("normal"),
        }
    }
}

pub struct SystemReporter<'a> {
    components: &'a Components,
    disks: &'a Disks,
    networks: &'a Networks,
    system: &'a mut System,
}

impl<'a> SystemReporter<'a> {
    pub fn new(
        components: &'a Components,
        disks: &'a Disks,
        networks: &'a Networks,
        system: &'a mut System,
    ) -> Self {
        SystemReporter {
            components,
            disks,
            networks,
            system,
        }
    }

    pub fn build_report(&mut self) -> SystemReport {
        self.system.refresh_all(); // Switch to refresh_specifics
        self.system.refresh_cpu_usage();

        // RAM and swap information:
        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let total_swap = self.system.total_swap();
        let used_swap = self.system.used_swap();

        // System information:
        let distribution_id = System::distribution_id();
        let cpu_arch = System::cpu_arch();
        let system_name = System::name().unwrap_or_else(|| "Undetermined".to_string());
        let system_kernal_version =
            System::kernel_version().unwrap_or_else(|| "Undetermined".to_string());
        let system_os_version =
            System::long_os_version().unwrap_or_else(|| "Undetermined".to_string());
        let system_host_name = System::host_name().unwrap_or_else(|| "Undetermined".to_string());

        // Number of CPUs:
        let num_cpus = self.system.cpus().len();
        let cpus = self.system.cpus();

        let disks: Vec<_> = self.disks.into_iter().collect();
        let components: Vec<_> = self.components.into_iter().collect();

        SystemReport {
            components,
            disks,
            total_memory,
            used_memory,
            available_memory: total_memory - used_memory,
            total_swap,
            used_swap,
            available_swap: total_swap - used_swap,
            system_name,
            system_kernal_version,
            system_os_version,
            system_host_name,
            cpus,
            num_cpus: num_cpus.try_into().unwrap(),
            cpu_arch,
            platform: distribution_id,
        }
    }
}
