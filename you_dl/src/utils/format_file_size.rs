const SUFFIXES: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];

fn default_formatting(invalid_file_size_bytes: &str) -> String {
    format!("{} {}", invalid_file_size_bytes, SUFFIXES[0])
}

pub fn format_file_size(file_size_bytes: &str) -> String {
    let file_size_bytes = match file_size_bytes.parse::<i32>() {
        Ok(v) => v,
        Err(_) => return default_formatting(file_size_bytes),
    };

    for (i, &suffix) in SUFFIXES.iter().enumerate() {
        let i = i as u32;
        let lower_bound = i32::pow(1024, i);
        let higher_bound = i32::pow(1024, i + 1);
        if file_size_bytes >= lower_bound && file_size_bytes < higher_bound {
            let val = f64::from(file_size_bytes) / f64::from(lower_bound);
            return format!("{:.2} {}", val, suffix);
        }
    }
    default_formatting(&file_size_bytes.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_value_as_is_if_cannot_be_parsed_into_i32() {
        assert_eq!(&format_file_size("1abc"), "1abc B")
    }

    #[test]
    fn creates_proper_string_representation() {
        assert_eq!(&format_file_size("10485760"), "10.00 MiB")
    }
}
