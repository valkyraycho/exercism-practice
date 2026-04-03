/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const MODULAR: i32 = 26;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}

fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    (1..m).find(|&x| (a * x) % m == 1)
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, MODULAR) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let chars: Vec<char> = plaintext
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| {
            if c.is_ascii_digit() {
                c
            } else {
                let x = (c.to_ascii_lowercase() as u8 - b'a') as i32;
                (((a * x + b) % MODULAR) as u8 + b'a') as char
            }
        })
        .collect();
    Ok(chars
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" "))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, MODULAR) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(ciphertext
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            if c.is_ascii_digit() {
                c
            } else {
                let y = (c as u8 - b'a') as i32;
                (((mod_inverse(a, MODULAR).unwrap() * (y - b)).rem_euclid(MODULAR)) as u8 + b'a')
                    as char
            }
        })
        .collect())
}
