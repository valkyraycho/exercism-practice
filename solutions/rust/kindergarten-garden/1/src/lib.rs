pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let index = (student.as_bytes()[0] - b'A') as usize * 2;

    diagram
        .lines()
        .take(2)
        .flat_map(|r| {
            r[index..index + 2].chars().map(|c| match c {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                'V' => "violets",
                _ => unreachable!(),
            })
        })
        .collect()
}
