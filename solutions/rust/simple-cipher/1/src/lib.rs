use rand::RngExt;

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }
    if key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    }
    Some(
        s.chars()
            .zip(key.chars().cycle())
            .map(|(c, k)| ((c as u8 - b'a' + k as u8 - b'a') % 26 + b'a') as char)
            .collect(),
    )
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }
    if key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    }
    Some(
        s.chars()
            .zip(key.chars().cycle())
            .map(|(c, k)| ((c as u8 - b'a' + 26 - (k as u8 - b'a')) % 26 + b'a') as char)
            .collect(),
    )
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::rng();

    let key: String = (0..100)
        .map(|_| (rng.random_range(b'a'..=b'z')) as char)
        .collect();

    let encoded = encode(&key, s).unwrap();
    (key, encoded)
}
