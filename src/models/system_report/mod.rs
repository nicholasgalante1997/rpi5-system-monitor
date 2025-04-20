use sysinfo::{Components, Disks, Networks, System};

use crate::utils;

pub struct SystemReport {
    pub total_memory: u64,
    pub used_memory: u64,
    pub available_memory: u64,

    pub total_swap: u64,
    pub used_swap: u64,
    pub available_swap: u64,

    pub system_name: String,
    pub system_kernal_version: String,
    pub system_os_version: String,
    pub system_host_name: String,

    pub num_cpus: u8,
    pub cpu_arch: String,

    pub platform: String,
}

impl SystemReport {
    pub fn into_html(&self) -> String {
        format!(
            r#"
                <div class="cards-container">
                    <!-- Platform Info Card -->
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
                            <span class="info-label">Architecture</span>
                            <span class="info-value">{}</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Hostname</span>
                            <span class="info-value">{}</span>
                        </div>
                    </div>

                    <!-- Memory Card -->
                    <div class="card">
                        <div class="card-title">
                            <i class="fas fa-memory"></i>
                            Memory
                        </div>
                        <div class="info-row">
                            <span class="info-label">Total Memory</span>
                            <span class="info-value">{}</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Used Memory</span>
                            <span class="info-value">{}</span>
                        </div>
                        <div class="progress-container">
                            <div class="progress-bar" style="width: {}%"></div>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Available Memory</span>
                            <span class="info-value">{}</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Swap Usage</span>
                            <span class="info-value">{} MB / {} GB</span>
                        </div>
                    </div>

                    <!-- CPU Card -->
                    <div class="card">
                        <div class="card-title">
                            <i class="fas fa-server"></i>
                            CPU
                        </div>
                        <div class="info-row">
                            <span class="info-label">Model</span>
                            <span class="info-value">Broadcom BCM2711</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Cores</span>
                            <span class="info-value">4 x Cortex-A72</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Clock Speed</span>
                            <span class="info-value">1.8 GHz</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Load Average</span>
                            <span class="info-value">0.75, 0.93, 0.81</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">CPU Usage</span>
                            <span class="info-value">18%</span>
                        </div>
                        <div class="progress-container">
                            <div class="progress-bar" style="width: 18%"></div>
                        </div>
                    </div>

                    <!-- Temperature Card -->
                    <div class="card">
                        <div class="card-title">
                            <i class="fas fa-thermometer-half"></i>
                            System Temperatures
                        </div>
                        <div class="info-row">
                            <span class="info-label">CPU Temperature</span>
                            <div class="temperature-indicator">
                                <span class="temperature-value normal">42.5°C</span>
                            </div>
                        </div>
                        <div class="info-row">
                            <span class="info-label">GPU Temperature</span>
                            <div class="temperature-indicator">
                                <span class="temperature-value normal">38.2°C</span>
                            </div>
                        </div>
                    </div>

                    <!-- GPU Card -->
                    <div class="card">
                        <div class="card-title">
                            <i class="fas fa-tv"></i>
                            GPU
                        </div>
                        <div class="info-row">
                            <span class="info-label">Model</span>
                            <span class="info-value">VideoCore VI</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Memory</span>
                            <span class="info-value">512 MB</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Frequency</span>
                            <span class="info-value">500 MHz</span>
                        </div>
                    </div>

                    <!-- Disk Card -->
                    <div class="card">
                        <div class="card-title">
                            <i class="fas fa-hdd"></i>
                            Storage
                        </div>
                        <div class="info-row">
                            <span class="info-label">Device</span>
                            <span class="info-value">/dev/mmcblk0p2</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Total</span>
                            <span class="info-value">32 GB</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Used</span>
                            <span class="info-value">12.4 GB (39%)</span>
                        </div>
                        <div class="progress-container">
                            <div class="progress-bar" style="width: 39%"></div>
                        </div>
                        <div class="info-row">
                            <span class="info-label">Available</span>
                            <span class="info-value">19.6 GB</span>
                        </div>
                    </div>
                </div>
        
            "#,
            &self.system_os_version,
            &self.system_kernal_version,
            &self.cpu_arch,
            &self.system_host_name,
            utils::convert_bytes_to_gbs(self.total_memory),
            utils::convert_bytes_to_gbs(self.used_memory),
            utils::convert_to_percent(self.used_memory, self.total_memory),
            utils::convert_bytes_to_gbs(self.available_memory),
            utils::convert_bytes_to_mbs(self.used_swap),
            utils::convert_bytes_to_gbs(self.total_swap)
        )
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

        SystemReport {
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
            num_cpus: num_cpus.try_into().unwrap(),
            cpu_arch,
            platform: distribution_id,
        }
    }
}
