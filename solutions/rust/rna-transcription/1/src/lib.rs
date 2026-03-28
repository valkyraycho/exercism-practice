#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    seq: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    seq: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.chars().position(|c| !"GCTA".contains(c)).map_or(
            Ok(Dna {
                seq: dna.to_string(),
            }),
            Err,
        )
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            seq: self
                .seq
                .chars()
                .map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => unreachable!(),
                })
                .collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.chars().position(|c| !"GCAU".contains(c)).map_or(
            Ok(Rna {
                seq: rna.to_string(),
            }),
            Err,
        )
    }
}
