use sysinfo::{
    Components, CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System,
};

use crate::{
    models::{
        builders::{
            component_info::ComponentReportInfoBuilder, cpu_info::CpuInfoBuilder,
            disk_info::DiskReportInfoBuilder, system_info_builder::SystemInfoBuilder,
        },
        data_objects::{
            component_info::ComponentReportInfo, cpu_info::CpuReportInfo,
            disk_info::DiskReportInfo, system_info::SystemReportInfo,
        },
        temperature_severity_status::TemperatureSeverityStatus,
    },
    utils,
};

pub struct NetworkReportInfo {}

pub struct SystemReport {
    components_report_info: Vec<ComponentReportInfo>,
    cpu_report_info: Vec<CpuReportInfo>,
    disks_report_info: Vec<DiskReportInfo>,
    network_report_info: NetworkReportInfo,
    system_info: SystemReportInfo,
}

impl SystemReport {
    pub fn into_html(&self) -> String {
        format!(
            r#"
                <div class="cards-container">
                    <!-- Platform Info Card -->
                    {}

                    <!-- Memory Card -->
                    {}

                    <!-- CPU Cards -->
                    {}

                    <!-- Temperature Card -->
                    {}

                    <!-- Disks Cards -->
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
        for cpu in &self.cpu_report_info {
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
                &cpu.brand, &cpu.name, &cpu.usage_percent, &cpu.usage_percent,
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
            &self.system_info.system_os_version,
            &self.system_info.system_kernal_version,
            &self.system_info.num_cpus,
            &self.system_info.cpu_arch,
            &self.system_info.system_host_name,
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
            utils::convert_bytes_to_gbs(self.system_info.total_memory),
            utils::convert_bytes_to_gbs(self.system_info.used_memory),
            utils::convert_to_percent(self.system_info.used_memory, self.system_info.total_memory),
            utils::convert_bytes_to_gbs(self.system_info.available_memory),
            utils::convert_bytes_to_mbs(self.system_info.used_swap),
            utils::convert_bytes_to_gbs(self.system_info.total_swap),
        )
    }

    fn convert_disks_to_markup_card(&self) -> String {
        let mut cards = String::new();

        let _ = &self.disks_report_info.iter().for_each(|disk| {
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
                disk.name,
                disk.mount_point,
                disk.file_system,
                utils::convert_bytes_to_gbs(disk.total_space),
                utils::convert_bytes_to_gbs(disk.available_space),
                utils::convert_to_percent(disk.available_space, disk.total_space),
                100.0_f64 - utils::convert_to_percent(disk.available_space, disk.total_space),
                disk.usage_total_read_bytes,
                disk.usage_total_write_bytes
            );

            cards.push_str(&card);
        });

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

        let _ = &self.components_report_info.iter().for_each(|component| {
            let severity = TemperatureSeverityStatus::get_severity_color_based_on_temperature_status(
                component.status.clone()
            );

            let card = format!(
                r#"
                    <div class="info-row">
                        <span class="info-label">{} CPU Temperature</span>
                        <div class="temperature-indicator">
                            <span class="temperature-value {}">{}°C</span>
                        </div>
                    </div>
                "#,
                &component.label,
                severity,
                &component.temperature,
            );

            cards.push_str(&card);
        });

        cards.push_str(
            r#"
            </div>
        "#,
        );

        cards
    }
}

pub struct SystemReporter<'a> {
    components: &'a mut Components,
    disks: &'a mut Disks,
    networks: &'a Networks,
    system: &'a mut System,
}

impl<'a> SystemReporter<'a> {
    pub fn new(
        components: &'a mut Components,
        disks: &'a mut Disks,
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
        
        // Refresh system handle specifics
        self.system.refresh_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );

        self.system.refresh_cpu_usage();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        self.system.refresh_cpu_usage();

        let platform = System::distribution_id();
        let cpu_arch = System::cpu_arch();
        let num_cpus: u8 = self.system.cpus().len().try_into().expect("(Error): This machine supercedes 256 individual cpu units");

        let system_name = System::name().unwrap_or_else(|| "Undetermined".to_string());
        let system_kernal_version =
            System::kernel_version().unwrap_or_else(|| "Undetermined".to_string());
        let system_os_version =
            System::long_os_version().unwrap_or_else(|| "Undetermined".to_string());
        let system_host_name = System::host_name().unwrap_or_else(|| "Undetermined".to_string());

        let mut system_info_builder = SystemInfoBuilder::new();

        let system_info = system_info_builder
            .set_available_memory(self.system.available_memory())
            .set_used_memory(self.system.used_memory())
            .set_total_memory(self.system.total_memory())
            .set_available_swap(self.system.total_swap() - self.system.used_swap())
            .set_used_swap(self.system.used_swap())
            .set_total_swap(self.system.total_swap())
            .set_platform(&platform)
            .set_system_name(&system_name)
            .set_system_host_name(&system_host_name)
            .set_system_kernal_version(&system_kernal_version)
            .set_system_os_version(&system_os_version)
            .set_cpu_arch(&cpu_arch)
            .set_num_cpus(num_cpus)
            .build();

        let mut cpu_info_reports: Vec<CpuReportInfo> = Vec::new();
        let cpus = self.system.cpus();
        for cpu in cpus {
            let cpu_brand = cpu.brand();
            let cpu_frequency = cpu.frequency();
            let cpu_name = cpu.name();
            let cpu_usage = cpu.cpu_usage();
            let cpu_vendor_id = cpu.vendor_id();

            let cpu_report_info_builder = CpuInfoBuilder::new();
            let cpu_report_info = cpu_report_info_builder
                .set_brand(cpu_brand.to_string())
                .set_frequency(cpu_frequency)
                .set_name(cpu_name.to_string())
                .set_usage_percent(cpu_usage)
                .set_vendor_id(cpu_vendor_id.to_string())
                .build();

            cpu_info_reports.push(cpu_report_info);
        }

        let mut disks_info_reports: Vec<DiskReportInfo> = Vec::new();
        for disk in self.disks.iter_mut() {
            disk.refresh();

            let name = disk.name();
            let mount_point = disk.mount_point();
            let file_system = disk.file_system();
            let kind = disk.kind();
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space - available_space;
            let usage = disk.usage();
            let total_read_bytes = usage.total_read_bytes;
            let total_written_bytes = usage.total_written_bytes;

            let disk_report_info_builder = DiskReportInfoBuilder::new();
            let disk_report_info = disk_report_info_builder
                .set_available_space(available_space)
                .set_used_space(used_space)
                .set_total_space(total_space)
                .set_percentage_free(utils::convert_to_percent(available_space, total_space))
                .set_usage_total_read_bytes(total_read_bytes)
                .set_usage_total_write_bytes(total_written_bytes)
                .set_kind(format!("{:#?}", kind).as_str())
                .set_file_system(format!("{:#?}", file_system).as_str())
                .set_mount_point(format!("{:#?}", mount_point).as_str())
                .set_name(format!("{:#?}", name).as_str())
                .build();

            disks_info_reports.push(disk_report_info);
        }

        let mut component_info_reports: Vec<ComponentReportInfo> = Vec::new();
        for component in self.components.iter_mut() {
            component.refresh();

            let label = component.label();
            let temperature = match component.temperature() {
                Some(temp) => temp,
                None => 0.0_f32,
            };
            let critical = component.critical();

            let component_report_info_builder = ComponentReportInfoBuilder::new();
            let component_report_info = component_report_info_builder
                .set_label(label.to_string())
                .set_temperature(temperature)
                .set_critical_temperature(critical)
                .set_status(TemperatureSeverityStatus::get_temperature_status(
                    temperature,
                    critical,
                ))
                .build();

            component_info_reports.push(component_report_info);
        }

        SystemReport {
            components_report_info: component_info_reports,
            cpu_report_info: cpu_info_reports,
            disks_report_info: disks_info_reports,
            network_report_info: NetworkReportInfo {},
            system_info,
        }
    }
}
