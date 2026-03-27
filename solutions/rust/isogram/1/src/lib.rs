use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    candidate
        .chars()
        .filter(|&c| c.is_alphabetic())
        .all(|c| set.insert(c.to_ascii_lowercase()))
}
