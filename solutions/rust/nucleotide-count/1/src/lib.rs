use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)?
        .get(&nucleotide)
        .copied()
        .ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    dna.chars().try_fold(
        HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]),
        |mut acc, c| match c {
            'A' | 'C' | 'G' | 'T' => {
                *acc.entry(c).or_default() += 1;
                Ok(acc)
            }
            _ => Err(c),
        },
    )
}
