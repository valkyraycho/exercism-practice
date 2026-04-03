pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0u32; size as usize]; size as usize];
    const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut dir = 0;
    let mut r = 0;
    let mut c = 0;
    for i in 1..=size * size {
        matrix[r as usize][c as usize] = i;
        if is_out_of_bound(size, r + DIRS[dir].0, c + DIRS[dir].1)
            || matrix[(r + DIRS[dir].0) as usize][(c + DIRS[dir].1) as usize] != 0
        {
            dir = (dir + 1) % 4
        }
        r += DIRS[dir].0;
        c += DIRS[dir].1
    }
    matrix
}

fn is_out_of_bound(size: u32, row: i32, col: i32) -> bool {
    row < 0 || row >= size as i32 || col < 0 || col >= size as i32
}
