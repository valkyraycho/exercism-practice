pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    let split = find_split(word);
    format!("{}{}ay", &word[split..], &word[..split])
}

fn find_split(word: &str) -> usize {
    let bytes = word.as_bytes();

    if is_vowel(bytes[0]) || bytes.starts_with(b"xr") || bytes.starts_with(b"yt") {
        return 0;
    }

    for i in 1..bytes.len() {
        if bytes[i - 1] == b'q' && bytes[i] == b'u' {
            return i + 1;
        }

        if bytes[i] == b'y' {
            return i;
        }

        if is_vowel(bytes[i]) {
            return i;
        }
    }
    0
}

fn is_vowel(b: u8) -> bool {
    matches!(b, b'a' | b'e' | b'i' | b'o' | b'u')
}
