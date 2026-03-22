def to_rna(dna_strand):
    RNAs = {
        "G": "C",
        "C": "G",
        "T": "A",
        "A": "U"
    }

    return "".join([RNAs[c] for c in dna_strand])
