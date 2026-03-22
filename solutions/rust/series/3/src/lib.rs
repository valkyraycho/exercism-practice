pub fn series(digits: &str, len: usize) -> Vec<String> {
    (0..=digits.len().saturating_sub(len))
        .map(|i| digits[i..i + len].to_string())
        .collect()
}
