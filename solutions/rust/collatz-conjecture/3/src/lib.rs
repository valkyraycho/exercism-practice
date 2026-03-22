use std::iter::successors;

pub fn collatz(n: u64) -> Option<u64> {
    (n > 0).then(|| {
        successors(Some(n), |&n| {
            (n != 1).then(|| {
                if n.is_multiple_of(2) {
                    n / 2
                } else {
                    n * 3 + 1
                }
            })
        })
        .count() as u64
            - 1
    })
}
