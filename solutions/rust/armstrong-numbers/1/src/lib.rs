pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.checked_ilog10().unwrap_or(0) + 1;

    let (sum, _) = (0..digits).fold((0, num), |(sum, cur_num), _| {
        (sum + (cur_num % 10).pow(digits), cur_num / 10)
    });
    sum == num
}
