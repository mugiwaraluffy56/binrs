pub fn byte_to_binary(byte: u8) -> String {
    let mut result = String::with_capacity(8);
    for i in (0..8).rev() {
        result.push(if (byte >> i) & 1 == 1 { '1' } else { '0' });
    }
    result
}

pub fn encode(input: &str) -> String {
    input
        .bytes()              // UTF-8 bytes, handles full Unicode
        .map(byte_to_binary)
        .collect::<Vec<_>>()
        .join(" ")            // "01001000 01101001"
}
