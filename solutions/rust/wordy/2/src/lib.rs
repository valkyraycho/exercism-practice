pub fn answer(command: &str) -> Option<i32> {
    let mut words = command
        .strip_prefix("What is")?
        .strip_suffix("?")?
        .split_whitespace();

    let mut result: i32 = words.next()?.parse::<i32>().ok()?;
    while let Some(op) = words.next() {
        let op_fn: fn(a: i32, b: i32) -> i32 = match op {
            "plus" => |a, b| a + b,
            "minus" => |a, b| a - b,
            "multiplied" => {
                words.next();
                |a, b| a * b
            }
            "divided" => {
                words.next();
                |a, b| a / b
            }
            _ => return None,
        };
        let num = words.next()?.parse::<i32>().ok()?;
        result = op_fn(result, num)
    }
    Some(result)
}
