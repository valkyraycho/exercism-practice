pub fn actions(n: u8) -> Vec<&'static str> {
    const ACTIONS: [&str; 4] = ["wink", "double blink", "close your eyes", "jump"];

    let mut result: Vec<&str> = ACTIONS
        .iter()
        .enumerate()
        .filter(|&(i, _)| n & (1 << i) != 0)
        .map(|(_, &action)| action)
        .collect();

    if n & 16 != 0 {
        result.reverse();
    }

    result
}
