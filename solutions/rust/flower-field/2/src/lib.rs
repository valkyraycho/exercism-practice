pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(row, &row_str)| {
            row_str
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(col, &el)| {
                    if el == b'*' {
                        return '*';
                    }
                    match count_adjacent_flowers(garden, row, col) {
                        0 => ' ',
                        n => char::from_digit(n, 10).unwrap(),
                    }
                })
                .collect()
        })
        .collect()
}

fn count_adjacent_flowers(garden: &[&str], row: usize, col: usize) -> u32 {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    directions
        .iter()
        .filter(|(dr, dc)| {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;
            new_row >= 0
                && new_row < garden.len() as isize
                && new_col >= 0
                && new_col < garden[0].len() as isize
                && garden[new_row as usize].as_bytes()[new_col as usize] == b'*'
        })
        .count() as u32
}
