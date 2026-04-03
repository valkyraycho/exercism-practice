pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0u32; size as usize]; size as usize];
    const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut dir = 0;
    let (mut r, mut c) = (0, 0);
    for i in 1..=size * size {
        matrix[r as usize][c as usize] = i;
        let (nr, nc) = (r + DIRS[dir].0, c + DIRS[dir].1);
        if is_out_of_bound(size, nr, nc) || matrix[nr as usize][nc as usize] != 0 {
            dir = (dir + 1) % 4
        }
        r += DIRS[dir].0;
        c += DIRS[dir].1
    }
    matrix
}

fn is_out_of_bound(size: u32, row: i32, col: i32) -> bool {
    let range = 0..size as i32;
    !range.contains(&row) || !range.contains(&col)
}
