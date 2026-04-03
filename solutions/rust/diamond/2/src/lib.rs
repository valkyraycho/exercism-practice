pub fn get_diamond(c: char) -> Vec<String> {
    let index = c as u8 - b'A';
    (0..=index)
        .chain((0..index).rev())
        .map(|i| build_row(i, index))
        .collect()
}

fn build_row(row: u8, index: u8) -> String {
    let letter = (b'A' + row) as char;
    let padding = " ".repeat((index - row) as usize);
    if row == 0 {
        format!("{padding}{letter}{padding}")
    } else {
        let inner = " ".repeat((2 * row - 1) as usize);
        format!("{padding}{letter}{inner}{letter}{padding}")
    }
}
