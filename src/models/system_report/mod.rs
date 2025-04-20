use sysinfo::{Components, Disks, Networks, System};

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
}

impl SystemReport {
    pub fn into_html(&self) -> String {
        r#""#.to_string()
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
        let system_name = System::name().unwrap_or_else(|| "Undetermined".to_string());
        let system_kernal_version =
            System::kernel_version().unwrap_or_else(|| "Undetermined".to_string());
        let system_os_version = System::os_version().unwrap_or_else(|| "Undetermined".to_string());
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
            num_cpus: num_cpus.try_into().unwrap()
        }
    }
}
