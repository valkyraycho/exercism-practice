pub struct Matrix {
    rows: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        Self {
            rows: input
                .lines()
                .map(|r| {
                    r.split_ascii_whitespace()
                        .map(|c| c.parse::<u32>().unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.rows.get(row_no - 1).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        self.rows
            .iter()
            .map(|r| r.get(col_no - 1).copied())
            .collect()
    }
}
