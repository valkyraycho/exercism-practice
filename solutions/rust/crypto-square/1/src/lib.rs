pub fn encrypt(input: &str) -> String {
    let normalized = input
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<Vec<char>>();

    if normalized.is_empty() {
        return String::new();
    }

    let c = (normalized.len() as f64).sqrt().ceil() as usize;
    let r = normalized.len().div_ceil(c);

    (0..c)
        .map(|col| {
            (0..r)
                .map(|row| normalized.get(row * c + col).copied().unwrap_or(' '))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}
