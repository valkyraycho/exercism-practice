use std::{fmt::Write, iter::repeat_n};
pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    let mut prev_char: char = source.chars().next().unwrap();
    let mut count = 1u32;
    for c in source.chars().skip(1) {
        if prev_char == c {
            count += 1
        } else {
            push_current_encoding(&mut result, count, prev_char);
            prev_char = c;
            count = 1;
        }
    }
    push_current_encoding(&mut result, count, prev_char);
    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut count = 0u32;
    for c in source.chars() {
        if c.is_ascii_digit() {
            count = count * 10 + c.to_digit(10).unwrap();
        } else {
            result.extend(repeat_n(c, count.max(1) as usize));
            count = 0;
        }
    }

    result
}

fn push_current_encoding(s: &mut String, count: u32, c: char) {
    match count {
        1 => write!(s, "{c}"),
        _ => write!(s, "{count}{c}"),
    }
    .unwrap()
}
