#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    let mut counts = [0u8; 7];
    for &d in &dice {
        counts[d as usize] += 1;
    }

    match category {
        Category::Ones => counts[1],
        Category::Twos => counts[2] * 2,
        Category::Threes => counts[3] * 3,
        Category::Fours => counts[4] * 4,
        Category::Fives => counts[5] * 5,
        Category::Sixes => counts[6] * 6,
        Category::FullHouse => {
            if counts.contains(&3) && counts.contains(&2) {
                dice.iter().sum()
            } else {
                0
            }
        }
        Category::FourOfAKind => counts
            .iter()
            .position(|&count| count >= 4)
            .map(|face| face as u8 * 4)
            .unwrap_or(0),
        Category::LittleStraight => {
            if counts[1..=5].iter().all(|&count| count == 1) {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            if counts[2..=6].iter().all(|&count| count == 1) {
                30
            } else {
                0
            }
        }
        Category::Choice => dice.iter().sum(),
        Category::Yacht => {
            if counts.contains(&5) {
                50
            } else {
                0
            }
        }
    }
}
