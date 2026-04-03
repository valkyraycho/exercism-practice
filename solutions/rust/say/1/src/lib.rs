use std::iter::successors;

const ONES: [&str; 20] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SCALES: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn encode(n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }
    let mut result_rev = successors(Some(n), |&val| (val >= 1000).then_some(val / 1000))
        .map(|val| val % 1000)
        .enumerate()
        .filter(|&(_, chunk)| chunk != 0)
        .map(|(i, chunk)| match SCALES[i] {
            "" => encode_chunk(chunk),
            scale => format!("{} {}", encode_chunk(chunk), scale),
        })
        .collect::<Vec<_>>();

    result_rev.reverse();
    result_rev.join(" ")
}

fn encode_chunk(n: u64) -> String {
    let mut parts = Vec::new();

    if n / 100 > 0 {
        parts.push(format!("{} hundred", ONES[(n / 100) as usize]));
    }

    let remainder = n % 100;
    if remainder >= 20 {
        if !remainder.is_multiple_of(10) {
            parts.push(format!(
                "{}-{}",
                TENS[(remainder / 10) as usize],
                ONES[(remainder % 10) as usize]
            ));
        } else {
            parts.push(TENS[(remainder / 10) as usize].to_string());
        }
    } else if remainder > 0 {
        parts.push(ONES[remainder as usize].to_string());
    }

    parts.join(" ")
}
