use serde::{Deserialize, Serialize};
use sysinfo::{
    Components, CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System,
};

use crate::{
    models::{
        builders::{
            component_info::ComponentReportInfoBuilder, cpu_info::CpuInfoBuilder,
            disk_info::DiskReportInfoBuilder, network_info::NetworkReportInfoBuilder,
            system_info_builder::SystemInfoBuilder,
        },
        data_objects::{
            component_info::ComponentReportInfo, cpu_info::CpuReportInfo,
            disk_info::DiskReportInfo, network_info::NetworkReportInfo,
            system_info::SystemReportInfo,
        },
        temperature_severity_status::TemperatureSeverityStatus,
    },
    utils,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemReport {
    pub components_report_info: Vec<ComponentReportInfo>,
    pub cpu_report_info: Vec<CpuReportInfo>,
    pub disks_report_info: Vec<DiskReportInfo>,
    pub network_report_info: Vec<NetworkReportInfo>,
    pub system_info: SystemReportInfo,
}

pub struct SystemReporter<'a> {
    components: &'a mut Components,
    disks: &'a mut Disks,
    networks: &'a mut Networks,
    system: &'a mut System,
}

impl<'a> SystemReporter<'a> {
    pub fn new(
        components: &'a mut Components,
        disks: &'a mut Disks,
        networks: &'a mut Networks,
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

        self.networks.refresh(true);

        let platform = System::distribution_id();
        let cpu_arch = System::cpu_arch();
        let num_cpus: u8 = self
            .system
            .cpus()
            .len()
            .try_into()
            .expect("(Error): This machine supercedes 256 individual cpu units");

        let system_name = System::name().unwrap_or_else(|| "Undetermined".to_string());
        let system_kernal_version =
            System::kernel_version().unwrap_or_else(|| "Undetermined".to_string());
        let system_os_version =
            System::long_os_version().unwrap_or_else(|| "Undetermined".to_string());
        let system_host_name = System::host_name().unwrap_or_else(|| "Undetermined".to_string());

        let uptime = System::uptime();

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
            .set_total_cpu_usage(self.system.global_cpu_usage())
            .set_uptime_in_seconds(uptime)
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

        let mut network_info_reports: Vec<NetworkReportInfo> = Vec::new();
        for (interface, network_data) in self.networks.iter() {
            let interface_name = interface.clone();
            let rx_bytes = network_data.received();
            let tx_bytes = network_data.transmitted();
            let rx_packets = network_data.packets_received();
            let tx_packets = network_data.packets_transmitted();
            let rx_errors = network_data.total_errors_on_received();
            let tx_errors = network_data.total_errors_on_transmitted();

            let network_report_info_builder = NetworkReportInfoBuilder::new();

            let network_report_info = network_report_info_builder
                .set_interface_name(interface_name)
                .set_rx_bytes(rx_bytes)
                .set_tx_bytes(tx_bytes)
                .set_rx_packets(rx_packets)
                .set_tx_packets(tx_packets)
                .set_rx_errors(rx_errors)
                .set_tx_errors(tx_errors)
                .build();

            network_info_reports.push(network_report_info);
        }

        SystemReport {
            components_report_info: component_info_reports,
            cpu_report_info: cpu_info_reports,
            disks_report_info: disks_info_reports,
            network_report_info: network_info_reports,
            system_info,
        }
    }
}
