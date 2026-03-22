pub fn factors(n: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut val = n;

    let mut factor: u64 = 2;
    while factor.pow(2) <= val {
        while val.is_multiple_of(factor) {
            result.push(factor);
            val /= factor;
        }
        factor += 1;
    }

    if val > 1 {
        result.push(val);
    }

    result
}
