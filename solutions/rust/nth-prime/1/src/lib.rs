pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2];
    let mut candidate = 3u32;
    let idx = n as usize;
    while primes.len() <= idx {
        let max_factor = (candidate as f64).sqrt() as u32 + 1;
        if primes
            .iter()
            .take_while(|&&prime| prime < max_factor)
            .all(|&prime| !candidate.is_multiple_of(prime))
        {
            primes.push(candidate);
        }
        candidate += 2;
    }

    primes[idx]
}
