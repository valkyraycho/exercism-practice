pub fn collatz(n: u64) -> Option<u64> {
    let count = 0u64;
    inner(n, count)
}

fn inner(n: u64, count: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    if n == 1 {
        return Some(count);
    }

    if n.is_multiple_of(2) {
        return inner(n / 2, count + 1);
    }

    inner(n * 3 + 1, count + 1)
}
