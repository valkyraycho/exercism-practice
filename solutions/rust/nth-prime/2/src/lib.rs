pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    (2..)
        .filter(|&candidate: &u32| {
            let max_factor = (candidate as f64).sqrt() as u32 + 1;
            if primes
                .iter()
                .take_while(|&&prime| prime < max_factor)
                .any(|&prime| candidate.is_multiple_of(prime))
            {
                return false;
            }
            primes.push(candidate);
            true
        })
        .nth(n as usize)
        .unwrap()
}
