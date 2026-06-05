pub fn format_bytes(bytes: u64) -> String {
    match bytes {
        b if b >= 1_073_741_824 => format!("{:.1} GB", bytes_to_gb(b)),
        b if b >= 1_048_576     => format!("{:.1} MB", bytes_to_mb(b)),
        b if b >= 1_024         => format!("{:.1} KB", bytes_to_kb(b)),
        b                       => format!("{} B", b),
    }
}

pub fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1_073_741_824.0
}

pub fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / 1_048_576.0
}

pub fn bytes_to_kb(bytes: u64) -> f64 {
    bytes as f64 / 1_024.0
}