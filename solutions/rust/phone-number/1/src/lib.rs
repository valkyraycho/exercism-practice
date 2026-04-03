pub fn number(user_number: &str) -> Option<String> {
    if user_number
        .chars()
        .any(|c| !c.is_ascii_digit() && !"()+-. ".contains(c))
    {
        return None;
    }

    let mut numbers: String = user_number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();

    if numbers.len() == 11 {
        numbers = numbers.strip_prefix("1")?.to_string();
    }

    if numbers.len() != 10 {
        return None;
    }

    if !matches!(numbers.as_bytes().first(), Some(b'2'..=b'9')) {
        return None;
    }

    if !matches!(numbers.as_bytes().get(3), Some(b'2'..=b'9')) {
        return None;
    }

    Some(numbers)
}
