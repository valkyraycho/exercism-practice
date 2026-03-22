/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|&c| !c.is_whitespace())
        .rev()
        .try_fold((0, 0), |(idx, sum), val| {
            val.to_digit(10)
                .map(|num| if idx % 2 == 1 { num * 2 } else { num })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| (idx + 1, sum + num))
        })
        .is_some_and(|(idx, sum)| idx > 1 && sum % 10 == 0)
}
