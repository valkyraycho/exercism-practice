/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| if c.is_ascii_digit() { c } else { transpose(c) })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| if c.is_ascii_digit() { c } else { transpose(c) })
        .collect()
}

fn transpose(c: char) -> char {
    (b'z' - (c.to_ascii_lowercase() as u8 - b'a')) as char
}
