pub fn parse_hex(s: &str) -> Option<usize> {
    let digits = s.strip_prefix("0x").unwrap_or(s);
    usize::from_str_radix(digits, 16).ok()
}
