pub struct RailFence {
    rails: u32,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence { rails }
    }

    fn generate_index_char_pairs(&self, text: &str) -> Vec<(u32, char)> {
        (0..self.rails)
            .chain((1..self.rails - 1).rev())
            .cycle()
            .zip(text.chars())
            .collect::<Vec<_>>()
    }

    pub fn encode(&self, text: &str) -> String {
        let pairs = self.generate_index_char_pairs(text);

        (0..self.rails)
            .flat_map(|rail| {
                pairs
                    .iter()
                    .filter(move |&&(index, _)| index == rail)
                    .map(|&(_, c)| c)
            })
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut positions = (0..cipher.len()).collect::<Vec<usize>>();
        let indices = (0..self.rails)
            .chain((1..self.rails - 1).rev())
            .cycle()
            .take(cipher.len())
            .collect::<Vec<u32>>();

        positions.sort_by_key(|&i| indices[i]);

        let mut result = vec![' '; cipher.len()];
        for (cipher_char, original_pos) in cipher.chars().zip(positions) {
            result[original_pos] = cipher_char;
        }

        result.iter().collect()
    }
}
