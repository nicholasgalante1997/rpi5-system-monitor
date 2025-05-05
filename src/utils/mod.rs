use crate::models::data_objects::disk_info::DiskReportInfo;

pub fn convert_bytes_to_mbs(bytes: u64) -> f64 {
    bytes as f64 / 1_048_576.0
}

pub fn convert_bytes_to_gbs(bytes: u64) -> f64 {
    bytes as f64 / 1_073_741_824.0
}

pub fn convert_to_percent(value: u64, total: u64) -> f64 {
    let div = value as f64 / total as f64;
    let percentage = div * 100.0 as f64;
    percentage.round()
}

pub fn get_total_disk_space_across_all_disks(disks: &Vec<DiskReportInfo>) -> u64 {
    let mut total_disk_space = 0_u64;
    for disk in disks {
        total_disk_space += disk.total_space;
    }

    total_disk_space
}

pub fn get_total_disk_usage_across_all_disks(disks: &Vec<DiskReportInfo>) -> u64 {
    let mut total_disk_usage = 0_u64;
    for disk in disks {
        total_disk_usage += disk.used_space;
    }

    total_disk_usage
}

pub fn format_uptime(uptime_in_seconds: u64) -> String {
    // Convert to more readable format
    let days = uptime_in_seconds / (24 * 3600);
    let hours = (uptime_in_seconds % (24 * 3600)) / 3600;
    let minutes = (uptime_in_seconds % 3600) / 60;
    let seconds = uptime_in_seconds % 60;

    format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
}
