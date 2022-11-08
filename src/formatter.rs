pub fn format_size(size: u64) -> String {
    if size > 1_000_000_000 {
        return format!("{:.2} Go", size as f64 / 1024.0 / 1024.0 / 1024.0);
    }
    if size > 1_000_000 {
        return format!("{:.2} Mo", size as f64 / 1024.0 / 1024.0);
    }
    if size > 1_000 {
        return format!("{:.2} Ko", size as f64 / 1024.0);
    }

    return format!("{} o", size);
}