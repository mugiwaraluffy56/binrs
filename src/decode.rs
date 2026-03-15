pub fn binary_to_byte(s: &str) -> Result<u8, String> {
    if s.len() != 8 {
        return Err(format!("expected 8-bit group, got: '{}'", s));
    }
    s.chars().try_fold(0u8, |acc, c| match c {
        '0' => Ok(acc << 1),
        '1' => Ok((acc << 1) | 1),
        _   => Err(format!("invalid char: '{}'", c)),
    })
}

pub fn decode(input: &str) -> Result<String, String> {
    let bytes: Result<Vec<u8>, _> = input
        .split_whitespace()
        .map(binary_to_byte)
        .collect();
    String::from_utf8(bytes?).map_err(|e| format!("invalid UTF-8: {}", e))
}
