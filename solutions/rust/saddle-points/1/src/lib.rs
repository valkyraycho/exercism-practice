pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let row_maxes = input
        .iter()
        .map(|row| row.iter().max().copied().unwrap_or(0))
        .collect::<Vec<u64>>();

    let cols = input.first().map_or(0, |row| row.len());
    let col_mins = (0..cols)
        .map(|c| input.iter().map(|row| row[c]).min().unwrap())
        .collect::<Vec<u64>>();

    (0..input.len())
        .flat_map(|r| (0..cols).map(move |c| (r, c)))
        .filter(|&(r, c)| input[r][c] == row_maxes[r] && input[r][c] == col_mins[c])
        .collect()
}
