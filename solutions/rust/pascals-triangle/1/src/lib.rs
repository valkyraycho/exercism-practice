pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row_count).fold(Vec::new(), |mut result, _| {
            let row = match result.last() {
                None => vec![1],
                Some(prev) => std::iter::once(1)
                    .chain(prev.windows(2).map(|pair| pair[0] + pair[1]))
                    .chain(std::iter::once(1))
                    .collect(),
            };
            result.push(row);
            result
        })
    }
}
