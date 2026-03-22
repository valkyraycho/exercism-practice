use std::iter::successors;

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.checked_ilog10().unwrap_or(0) + 1;

    successors(Some(num), |&n| (n >= 10).then_some(n / 10))
        .map(|n| (n % 10).pow(digits))
        .sum::<u32>()
        == num
}
