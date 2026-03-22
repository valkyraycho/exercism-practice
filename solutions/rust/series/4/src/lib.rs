pub fn series(digits: &str, len: usize) -> Vec<String> {
    (0..=digits.len().saturating_sub(len))
        .map_while(|i| digits.get(i..i + len))
        .map(String::from)
        .collect()
}
