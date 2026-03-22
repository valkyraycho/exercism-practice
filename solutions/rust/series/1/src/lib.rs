pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .char_indices()
        .map_while(|(i, _)| digits.get(i..i + len))
        .map(String::from)
        .collect()
}
