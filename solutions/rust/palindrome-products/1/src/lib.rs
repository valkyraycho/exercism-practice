use std::{collections::HashSet, u64};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factor_pairs: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factor_pairs
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max || min == u64::MAX {
        return None;
    }

    let mut min_val = u64::MAX;
    let mut max_val = 0u64;

    let mut min_factors = HashSet::new();
    let mut max_factors = HashSet::new();

    for i in min..=max {
        for j in i..=max {
            let product = i * j;
            if is_palindrome(product) {
                if product == min_val {
                    min_factors.insert((i, j));
                } else if product < min_val {
                    min_val = product;
                    min_factors = HashSet::from([(i, j)]);
                }
                if product == max_val {
                    max_factors.insert((i, j));
                } else if product > max_val {
                    max_val = product;
                    max_factors = HashSet::from([(i, j)]);
                }
            }
        }
    }
    if min_val == u64::MAX {
        return None;
    }

    Some((
        Palindrome {
            value: min_val,
            factor_pairs: min_factors,
        },
        Palindrome {
            value: max_val,
            factor_pairs: max_factors,
        },
    ))
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s.bytes().eq(s.bytes().rev())
}
