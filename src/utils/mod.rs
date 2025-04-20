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
