use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
    word_chars.sort_unstable();
    possible_anagrams
        .iter()
        .filter(|&&candidate| {
            let mut candidate_chars: Vec<char> = candidate.to_lowercase().chars().collect();
            candidate_chars.sort_unstable();
            candidate_chars == word_chars
        })
        .cloned()
        .collect()
}
