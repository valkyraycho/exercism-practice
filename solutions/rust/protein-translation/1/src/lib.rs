pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut proteins = Vec::new();

    for i in (0..rna.len()).step_by(3) {
        match rna.get(i..i + 3) {
            Some("AUG") => proteins.push("Methionine"),
            Some("UUU") | Some("UUC") => proteins.push("Phenylalanine"),
            Some("UUA") | Some("UUG") => proteins.push("Leucine"),
            Some("UCU") | Some("UCC") | Some("UCA") | Some("UCG") => proteins.push("Serine"),
            Some("UAU") | Some("UAC") => proteins.push("Tyrosine"),
            Some("UGU") | Some("UGC") => proteins.push("Cysteine"),
            Some("UGG") => proteins.push("Tryptophan"),
            Some("UAA") | Some("UAG") | Some("UGA") => return Some(proteins),
            _ => return None,
        }
    }

    Some(proteins)
}
