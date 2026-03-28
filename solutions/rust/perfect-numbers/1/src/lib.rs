use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let sum = if num > 1 { 1 } else { 0 }
        + (2..)
            .take_while(|&i| i * i <= num)
            .filter(|&i| num.is_multiple_of(i))
            .map(|i| if i != num / i { i + num / i } else { i })
            .sum::<u64>();

    match sum.cmp(&num) {
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Abundant),
        Ordering::Less => Some(Classification::Deficient),
    }
}
