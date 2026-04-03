pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen {
    const ALL: [Allergen; 8] = [
        Allergen::Eggs,
        Allergen::Peanuts,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];

    fn value(&self) -> u32 {
        1 << Allergen::ALL
            .iter()
            .position(|allergen| allergen == self)
            .unwrap()
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        allergen.value() & self.0 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::ALL
            .iter()
            .filter(|&allergen| self.is_allergic_to(allergen))
            .copied()
            .collect()
    }
}
