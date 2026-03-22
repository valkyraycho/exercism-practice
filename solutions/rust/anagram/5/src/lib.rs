use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut word_sorted: Vec<char> = word_lower.chars().collect();
    word_sorted.sort_unstable();
    possible_anagrams
        .iter()
        .filter(|&&candidate| {
            let candidate_lower = candidate.to_lowercase();
            let mut candidate_sorted: Vec<char> = candidate_lower.chars().collect();
            candidate_sorted.sort_unstable();
            candidate.len() == word.len()
                && candidate_lower != word_lower
                && candidate_sorted == word_sorted
        })
        .copied()
        .collect()
}
