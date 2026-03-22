pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| !c.is_alphabetic() && c != '\'')
        .filter(|w| !w.is_empty())
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.as_bytes()
                    .windows(2)
                    .filter(|pair| pair[0].is_ascii_lowercase() && pair[1].is_ascii_uppercase())
                    .map(|pair| pair[1] as char),
            )
        })
        .collect::<String>()
        .to_uppercase()
}
