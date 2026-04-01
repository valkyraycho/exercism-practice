pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        self.to_string()
            .chars()
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
}
