pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut is_prime = vec![true; (upper_bound + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..upper_bound {
        if i * i > upper_bound {
            break;
        }
        if !is_prime[i as usize] {
            continue;
        }

        for multiple in (i * i..=upper_bound).step_by(i as usize) {
            is_prime[multiple as usize] = false;
        }
    }
    is_prime
        .iter()
        .enumerate()
        .filter(|&(_, b)| *b)
        .map(|(i, _)| i as u64)
        .collect()
}
