const A_LCASE: u8 = b'a';
const ALL_ALPHABETS: u32 = (1 << 26) - 1;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .fold(0u32, |seen, c| {
            seen | 1 << (c.to_ascii_lowercase() as u8 - A_LCASE)
        })
        == ALL_ALPHABETS
}
