pub fn get_diamond(c: char) -> Vec<String> {
    let index = c as u8 - b'A';
    let width = 2 * index + 1;
    let mut first_half: Vec<String> = (0..=index)
        .map(|i| {
            let mut s = String::with_capacity(width as usize);
            s.push_str(" ".repeat((index - i) as usize).as_str());
            if i == 0 {
                s.push('A');
            } else {
                s.push((i + b'A') as char);
                s.push_str(" ".repeat((2 * i - 1) as usize).as_str());
                s.push((i + b'A') as char);
            }
            s.push_str(" ".repeat((index - i) as usize).as_str());
            s
        })
        .collect();

    let second_half: Vec<String> = first_half[..first_half.len() - 1]
        .iter()
        .rev()
        .cloned()
        .collect();
    first_half.extend(second_half);
    first_half
}
