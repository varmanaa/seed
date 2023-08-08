pub fn abbreviate(decimal: i64) -> String {
    match decimal {
        i64::MIN ..= -1_000_000 | 1_000_000 ..= i64::MAX => {
            format!("{:.1}m", decimal as f64 / 1_000_000.0)
        }
        -999_999 ..= -1_000 | 1_000 ..= 999_999 => format!("{:.1}k", decimal as f64 / 1_000.0),
        _ => decimal.to_string(),
    }
}

pub fn modulo(
    a: usize,
    n: usize,
) -> usize {
    ((a % n) + n) % n
}
