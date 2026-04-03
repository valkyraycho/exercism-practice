pub fn answer(command: &str) -> Option<i32> {
    let mut words = command
        .strip_prefix("What is")?
        .strip_suffix("?")?
        .split_whitespace();

    let mut result: i32 = words.next()?.parse::<i32>().ok()?;
    while let Some(op) = words.next() {
        let op = match op {
            "plus" | "minus" => op,
            "multiplied" | "divided" => {
                words.next();
                op
            }
            _ => return None,
        };
        let num = words.next()?.parse::<i32>().ok()?;
        match op {
            "plus" => result += num,
            "minus" => result -= num,
            "multiplied" => result *= num,
            "divided" => result /= num,
            _ => return None,
        }
    }
    Some(result)
}
