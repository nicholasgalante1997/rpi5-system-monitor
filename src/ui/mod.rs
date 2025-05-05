use crate::models::data_objects::component_info::ComponentReportInfo;
use crate::models::data_objects::{
    cpu_info::CpuReportInfo, disk_info::DiskReportInfo, network_info::NetworkReportInfo,
    system_info::SystemReportInfo,
};
use crate::utils;

pub struct HttpViews;

impl HttpViews {
    pub fn get_overview_view(
        system_info: &SystemReportInfo,
        disks_info: &Vec<DiskReportInfo>,
    ) -> String {
        format!(
            r#"
            <div id="overview" class="tab-content active">
                <div class="section-header">
                    <h2 class="section-title">System Overview</h2>
                </div>

                <div class="card-grid">
                    <div class="card">
                        <div class="card-header">
                            <div class="card-title">
                                <div class="card-icon"></div>
                                CPU Usage
                            </div>
                        </div>
                        <div class="card-value" id="cpu-usage">{:.1}%</div>
                        <div class="progress-container">
                            <div class="progress-bar">
                                <div class="progress-fill" id="cpu-progress" style="width: {:.1}%"></div>
                            </div>
                            <div class="progress-stats">
                                <span>{:.1}%</span>
                                <span>100%</span>
                            </div>
                        </div>
                        <div class="card-description">Current CPU utilization across all cores</div>
                    </div>

                    <div class="card">
                        <div class="card-header">
                            <div class="card-title">
                                <div class="card-icon"></div>
                                Memory Usage
                            </div>
                        </div>
                        <div class="card-value" id="memory-usage">{:.1} GB</div>
                        <div class="progress-container">
                            <div class="progress-bar">
                                <div class="progress-fill" id="memory-progress" style="width: {:.1}%"></div>
                            </div>
                            <div class="progress-stats">
                                <span>{:.1} GB</span>
                                <span id="total-memory">{:.1} GB</span>
                            </div>
                        </div>
                        <div class="card-description">Physical memory currently in use</div>
                    </div>

                    <div class="card">
                        <div class="card-header">
                            <div class="card-title">
                                <div class="card-icon"></div>
                                Disk Usage
                            </div>
                        </div>
                        <div class="card-value" id="disk-usage">{:.1} GB</div>
                        <div class="progress-container">
                            <div class="progress-bar">
                                <div class="progress-fill" id="disk-progress" style="width: {:.1}%"></div>
                            </div>
                            <div class="progress-stats">
                                <span>{:.1} GB</span>
                                <span id="total-disk">{:.1} GB</span>
                            </div>
                        </div>
                        <div class="card-description">Primary storage utilization</div>
                    </div>
                </div>

                <div class="card">
                    <div class="card-header">
                        <div class="card-title">
                            <div class="card-icon"></div>
                            System Information
                        </div>
                    </div>
                    <div>
                        <div class="detail-row">
                            <div class="detail-label">Hostname</div>
                            <div class="detail-value" id="hostname">{}</div>
                        </div>
                        <div class="detail-row">
                            <div class="detail-label">Kernel Version</div>
                            <div class="detail-value" id="kernel-version">{}</div>
                        </div>
                        <div class="detail-row">
                            <div class="detail-label">Uptime</div>
                            <div class="detail-value" id="uptime">{}</div>
                        </div>
                        <div class="detail-row">
                            <div class="detail-label">OS</div>
                            <div class="detail-value" id="os-info">{}</div>
                        </div>
                    </div>
                </div>
            </div>
            "#,
            system_info.total_cpu_usage,
            system_info.total_cpu_usage,
            system_info.total_cpu_usage,
            utils::convert_bytes_to_gbs(system_info.used_memory),
            utils::convert_to_percent(system_info.used_memory, system_info.total_memory),
            utils::convert_bytes_to_gbs(system_info.used_memory),
            utils::convert_bytes_to_gbs(system_info.total_memory),
            utils::convert_bytes_to_gbs(utils::get_total_disk_usage_across_all_disks(disks_info)),
            utils::convert_to_percent(
                utils::get_total_disk_usage_across_all_disks(disks_info),
                utils::get_total_disk_space_across_all_disks(disks_info)
            ),
            utils::convert_bytes_to_gbs(utils::get_total_disk_usage_across_all_disks(disks_info)),
            utils::convert_bytes_to_gbs(utils::get_total_disk_space_across_all_disks(disks_info)),
            system_info.system_host_name,
            system_info.system_kernal_version,
            utils::format_uptime(system_info.uptime_in_seconds),
            system_info.system_os_version,
        )
    }

    pub fn get_cpu_view(system_info: &SystemReportInfo, cpu_info: &Vec<CpuReportInfo>) -> String {
        let brand = format!("{} ({})", cpu_info[0].brand, cpu_info[0].name);
        format!(
            r#"
            <div id="cpu" class="tab-content active">
                <div class="section-header">
                    <h2 class="section-title">CPU Information</h2>
                </div>

                <div class="card-grid">
                    <div class="card">
                        <div class="card-header">
                            <div class="card-title">
                                <div class="card-icon"></div>
                                CPU Usage
                            </div>
                        </div>
                        <div class="card-value" id="cpu-usage-detail">{:.1}%</div>
                        <div class="progress-container">
                            <div class="progress-bar">
                                <div class="progress-fill" id="cpu-progress-detail" style="width: {:.1}%"></div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="card">
                    <div class="card-header">
                        <div class="card-title">
                            <div class="card-icon"></div>
                            CPU Details
                        </div>
                    </div>
                    <div>
                        <div class="detail-row">
                            <div class="detail-label">Model</div>
                            <div class="detail-value" id="cpu-model">{}</div>
                        </div>
                        <div class="detail-row">
                            <div class="detail-label">Cores</div>
                            <div class="detail-value" id="cpu-cores">{}</div>
                        </div>
                    </div>
                </div>

                <div class="card">
                    <div class="card-header">
                        <div class="card-title">
                            <div class="card-icon"></div>
                            Core Utilization
                        </div>
                    </div>
                    <div id="core-utilization">
                        {}
                    </div>
                </div>
            </div>
            "#,
            &system_info.total_cpu_usage,
            &system_info.total_cpu_usage,
            brand,
            &system_info.num_cpus,
            &HttpViews::get_cpu_list_view(cpu_info)
        )
    }

    pub fn get_memory_view(system_info: &SystemReportInfo) -> String {
        format!(
            r#"
            <div id="memory" class="tab-content active">
                <div class="section-header">
                    <h2 class="section-title">Memory Statistics</h2>
                </div>

                <div class="card-grid">
                    <div class="card">
                        <div class="card-header">
                            <div class="card-title">
                                <div class="card-icon"></div>
                                Physical Memory
                            </div>
                        </div>
                        <div class="card-value" id="memory-usage-detail">{:.1} GB / {:.1} GB</div>
                        <div class="progress-container">
                            <div class="progress-bar">
                                <div class="progress-fill" id="memory-progress-detail" style="width: {:.1}%"></div>
                            </div>
                        </div>
                    </div>

                    <div class="card">
                        <div class="card-header">
                            <div class="card-title">
                                <div class="card-icon"></div>
                                Swap
                            </div>
                        </div>
                        <div class="card-value" id="swap-usage">{:.1} GB / {:.1} GB</div>
                        <div class="progress-container">
                            <div class="progress-bar">
                                <div class="progress-fill" id="swap-progress" style="width: {:.1}%"></div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="card">
                    <div class="card-header">
                        <div class="card-title">
                            <div class="card-icon"></div>
                            Memory Distribution
                        </div>
                    </div>
                    <div>
                        <div class="detail-row">
                            <div class="detail-label">Used</div>
                            <div class="detail-value" id="mem-used">{:.1} GB</div>
                        </div>
                        <div class="detail-row">
                            <div class="detail-label">Free</div>
                            <div class="detail-value" id="mem-free">{:.1} GB</div>
                        </div>
                    </div>
                </div>
            </div>
            "#,
            utils::convert_bytes_to_gbs(system_info.used_memory),
            utils::convert_bytes_to_gbs(system_info.total_memory),
            utils::convert_to_percent(system_info.used_memory, system_info.total_memory),
            utils::convert_bytes_to_gbs(system_info.used_swap),
            utils::convert_bytes_to_gbs(system_info.total_swap),
            utils::convert_to_percent(system_info.used_swap, system_info.total_swap),
            utils::convert_bytes_to_gbs(system_info.used_memory),
            utils::convert_bytes_to_gbs(system_info.available_memory)
        )
    }

    pub fn get_disks_view(disks_info: &Vec<DiskReportInfo>) -> String {
        format!(
            r#"
            <div id="disk" class="tab-content active">
                <div class="section-header">
                    <h2 class="section-title">Disk Usage</h2>
                </div>

                <div class="card">
                    <div class="card-header">
                        <div class="card-title">
                            <div class="card-icon"></div>
                            Storage Devices
                        </div>
                    </div>
                    <table class="data-table">
                        <thead>
                            <tr>
                                <th>Mount Point</th>
                                <th>Size</th>
                                <th>Used</th>
                                <th>Available</th>
                                <th>Usage</th>
                                <th>Write Rate</th>
                                <th>Read Rate</th>
                            </tr>
                        </thead>
                        <tbody id="disk-table-body">
                            {}
                        </tbody>
                    </table>
                </div>
            </div>
            "#,
            &HttpViews::get_disk_table_view(disks_info)
        )
    }

    pub fn get_cpu_list_view(cpu_info: &Vec<CpuReportInfo>) -> String {
        let mut cpu_table = String::new();

        cpu_table.push_str(
            r#"
            <table>
                <thead>
                    <tr>
                        <th>Model</th>
                        <th>Core Num</th>
                        <th>Usage</th>
                        <th>Frequency</th>
                        <th>Vendor ID</th>
                    </tr>
                </thead>
                <tbody>
            "#,
        );

        for cpu in cpu_info {
            cpu_table.push_str(&HttpViews::get_single_cpu_view(cpu));
        }

        cpu_table.push_str(
            r#"
                </tbody>
            </table>
            "#,
        );

        cpu_table
    }

    pub fn get_single_cpu_view(cpu_info: &CpuReportInfo) -> String {
        format!(
            r#"
            <tr>
                <td>{}</td>
                <td>{}</td>
                <td>{:.1}%</td>
                <td>{} MHz</td>
                <td>{}</td>
            </tr>
            "#,
            cpu_info.brand,
            cpu_info.name,
            cpu_info.usage_percent,
            cpu_info.frequency,
            cpu_info.vendor_id
        )
    }

    pub fn get_disk_table_view(disks_info: &Vec<DiskReportInfo>) -> String {
        let mut disk_table = String::new();
        for disk in disks_info {
            disk_table.push_str(&HttpViews::get_disk_usage_table_row_view(disk));
        }
        disk_table
    }

    pub fn get_disk_usage_table_row_view(disk_info: &DiskReportInfo) -> String {
        format!(
            r#"
            <tr>
                <td>{}</td>
                <td>{:.1} GB</td>
                <td>{:.1} GB</td>
                <td>{:.1} GB</td>
                <td>{:.1}%</td>
                <td>{:.1} MB</td>
                <td>{:.1} MB</td>
            </tr>
            "#,
            disk_info.mount_point,
            utils::convert_bytes_to_gbs(disk_info.total_space),
            utils::convert_bytes_to_gbs(disk_info.used_space),
            utils::convert_bytes_to_gbs(disk_info.total_space - disk_info.used_space),
            utils::convert_to_percent(disk_info.used_space, disk_info.total_space),
            utils::convert_bytes_to_mbs(disk_info.usage_total_write_bytes),
            utils::convert_bytes_to_mbs(disk_info.usage_total_read_bytes)
        )
    }

    pub fn get_components_view(components: &Vec<ComponentReportInfo>) -> String {
        format!(
            r#"
            <div id="components" class="tab-content active">
                <div class="section-header">
                    <h2 class="section-title">System Components</h2>
                </div>

                <div class="card">
                    <div class="card-header">
                        <div class="card-title">
                            <div class="card-icon"></div>
                            Hardware Overview
                        </div>
                    </div>
                    <table class="data-table">
                        <thead>
                            <tr>
                                <th>Component Name</th>
                                <th>Status</th>
                                <th>Temperature</th>
                                <th>Critical Temperature
                            </tr>
                        </thead>
                        <tbody>
                            {}
                        </tbody>
                    </table>
                </div>
            </div>
            "#,
            &HttpViews::get_components_table_view(components)
        )
    }

    pub fn get_components_table_view(components: &Vec<ComponentReportInfo>) -> String {
        let mut components_table = String::new();
        for component in components {
            components_table.push_str(&HttpViews::get_component_table_row(component));
        }
        components_table
    }

    pub fn get_component_table_row(component: &ComponentReportInfo) -> String {
        format!(
            r#"
            <tr>
                <td>{}</td>
                <td>{:#?}</td>
                <td>{:.1}°C</td>
                <td>{:.1}°C</td>
            </tr>
            "#,
            component.label,
            component.status,
            component.temperature,
            component.critical_temperature.unwrap_or_else(|| -1.0_f32)
        )
    }

    pub fn get_network_view(network_info: &Vec<NetworkReportInfo>) -> String {
        format!(
            r#"
            <div id="network" class="tab-content active">
                <div class="section-header">
                    <h2 class="section-title">Network Usage</h2>
                </div>

                <div class="card">
                    <div class="card-header">
                        <div class="card-title">
                            <div class="card-icon"></div>
                            Network Interfaces
                        </div>
                    </div>
                    <table class="data-table">
                        <thead>
                            <tr>
                                <th>Name</th>
                                <th>Packets Received</th>
                                <th>Packets Sent</th>
                                <th>Bytes Received</th>
                                <th>Bytes Sent</th>
                            </tr>
                        </thead>
                        <tbody>
                            {}
                        </tbody>
                    </table>
                </div>
            </div>
            "#,
            &HttpViews::get_network_table_view(network_info)
        )
    }

    pub fn get_network_table_view(network_info: &Vec<NetworkReportInfo>) -> String {
        let mut network_table = String::new();
        for network in network_info {
            network_table.push_str(&HttpViews::get_single_network_view(network));
        }
        network_table
    }

    pub fn get_single_network_view(network: &NetworkReportInfo) -> String {
        format!(
            r#"
            <tr>
                <td>{}</td>
                <td>{:.1}</td>
                <td>{:.1}</td>
                <td>{:.1} MB</td>
                <td>{:.1} MB</td>
            </tr>
            "#,
            network.interface_name,
            network.rx_packets,
            network.tx_packets,
            utils::convert_bytes_to_mbs(network.rx_bytes),
            utils::convert_bytes_to_mbs(network.tx_bytes)
        )
    }
}
