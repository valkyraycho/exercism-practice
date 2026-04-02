pub fn rotate(input: &str, key: u8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                return ((c as u8 - base + key) % 26 + base) as char;
            }
            c
        })
        .collect()
}
