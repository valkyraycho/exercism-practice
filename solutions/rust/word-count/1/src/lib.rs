use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut result = HashMap::new();
    for word in words
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|w| !w.is_empty())
    {
        let word = word.trim_matches('\'').to_ascii_lowercase();
        if !word.is_empty() {
            *result.entry(word).or_default() += 1
        }
    }

    result
}
