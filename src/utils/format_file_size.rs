const SUFFIXES: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];

pub fn format_file_size(file_size_bytes: i32) -> String {
    for (i, &suffix) in SUFFIXES.iter().enumerate() {
        let i = i as u32;
        let lower_bound = i32::pow(1024, i);
        let higher_bound = i32::pow(1024, i + 1);
        if file_size_bytes >= lower_bound && file_size_bytes < higher_bound {
            let val = f64::from(file_size_bytes) / f64::from(lower_bound);
            return format!("{:.2} {}", val, suffix);
        }
    }
    format!("{} {}", file_size_bytes, SUFFIXES[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_proper_string_representation() {
        assert_eq!(&format_file_size(10485760), "10.00 MiB")
    }
}
