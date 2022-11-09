pub fn format_size(size: u64) -> String {
    const DIV: f64 = 1024.0;
    if size > (DIV*DIV*DIV) as u64 {
        return format!("{:.2} Gb", size as f64 / DIV / DIV / DIV);
    }
    if size > (DIV*DIV) as u64 {
        return format!("{:.2} Mb", size as f64 / DIV / DIV);
    }
    if size > DIV as u64 {
        return format!("{:.2} Kb", size as f64 / DIV);
    }

    return format!("{} b", size);
}

pub fn format_time(millis: u128) -> String {
    const DIV: f64 = 1000.0;
    
    if millis > DIV as u128 {
        return format!("{:.1} s", millis as f64 / DIV as f64);
    }

    return format!("{} ms", millis);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_size_byte() {
        let formatted = format_size(32);
        assert_eq!("32 b", formatted);
    }

    #[test]
    fn format_size_kilo_byte() {
        let formatted = format_size(12000);
        assert_eq!("11.72 Kb", formatted);
    }

    #[test]
    fn format_size_mega_byte() {
        let formatted = format_size(15000000);
        assert_eq!("14.31 Mb", formatted);
    }

    #[test]
    fn format_size_giga_byte() {
        let formatted = format_size(28000000000);
        assert_eq!("26.08 Gb", formatted);
    }

    #[test]
    fn format_time_millis() {
        let formatted = format_time(100);
        assert_eq!("100 ms", formatted);
    }

    #[test]
    fn format_time_sec() {
        let formatted = format_time(2200);
        assert_eq!("2.2 s", formatted);
    }
}