#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    }

    let digits = string_digits
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or(Error::InvalidDigit(c))
                .map(|d| d as u64)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(digits
        .windows(span)
        .map(|window| window.iter().product::<u64>())
        .max()
        .unwrap())
}
