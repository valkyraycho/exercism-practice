/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .filter(|&c| c != '-')
        .enumerate()
        .map(|(i, c)| {
            if i == 9 && c == 'X' {
                (i, Some(10))
            } else if i <= 9 {
                (i, c.to_digit(10))
            } else {
                (i, None)
            }
        })
        .try_fold((0u32, 0u32), |(sum, count), (i, digit)| {
            digit.map(|d| (sum + (10 - i as u32) * d, count + 1))
        })
        .is_some_and(|(sum, count)| count == 10 && sum.is_multiple_of(11))
}
