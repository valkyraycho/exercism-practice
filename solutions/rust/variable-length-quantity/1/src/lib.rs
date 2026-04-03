#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|&val| encode_one(val)).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    if bytes.last().is_some_and(|&b| b & 0x80 != 0) {
        return Err(Error::IncompleteNumber);
    }
    let mut result = Vec::new();
    let mut value = 0u32;

    for &byte in bytes {
        value = (value << 7) | (byte & 0x7F) as u32;
        if byte & 0x80 == 0 {
            result.push(value);
            value = 0u32;
        }
    }
    Ok(result)
}

fn encode_one(mut value: u32) -> Vec<u8> {
    let mut bytes = vec![(value & 0x7F) as u8];
    value >>= 7;
    while value > 0 {
        bytes.push((value & 0x7F) as u8 | 0x80);
        value >>= 7;
    }
    bytes.reverse();
    bytes
}
