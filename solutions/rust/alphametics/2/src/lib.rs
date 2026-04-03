use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left, right) = input.split_once("==").unwrap();
    let left_hand_sides: Vec<&str> = left.split('+').map(|n| n.trim()).collect();
    let right_hand_side = right.trim();
    let alphabets: HashSet<char> = left_hand_sides
        .iter()
        .flat_map(|&w| w.chars())
        .chain(right_hand_side.chars())
        .collect();

    let leading: HashSet<char> = left_hand_sides
        .iter()
        .chain(std::iter::once(&right_hand_side))
        .filter_map(|&w| w.chars().next())
        .collect();

    let letters: Vec<char> = alphabets.iter().copied().collect();
    let puzzle = Puzzle {
        leading,
        left: left_hand_sides,
        right: right_hand_side,
    };
    puzzle.backtrack(&letters, &mut HashMap::new(), &mut [false; 10])
}

fn word_to_number(word: &str, mapping: &HashMap<char, u8>) -> u64 {
    word.chars()
        .fold(0u64, |acc, c| acc * 10 + mapping[&c] as u64)
}

struct Puzzle<'a> {
    leading: HashSet<char>,
    left: Vec<&'a str>,
    right: &'a str,
}

impl Puzzle<'_> {
    fn backtrack(
        &self,
        letters_to_assign: &[char],
        mapping: &mut HashMap<char, u8>,
        used: &mut [bool; 10],
    ) -> Option<HashMap<char, u8>> {
        let (&letter, remaining) = match letters_to_assign.split_first() {
            Some(pair) => pair,
            None => {
                if self
                    .left
                    .iter()
                    .map(|&w| word_to_number(w, mapping))
                    .sum::<u64>()
                    == word_to_number(self.right, mapping)
                {
                    return Some(mapping.to_owned());
                }
                return None;
            }
        };
        for digit in 0..=9u8 {
            if used[digit as usize] || (digit == 0 && self.leading.contains(&letter)) {
                continue;
            }
            mapping.insert(letter, digit);
            used[digit as usize] = true;

            if let Some(solution) = self.backtrack(remaining, mapping, used) {
                return Some(solution);
            }

            mapping.remove(&letter);
            used[digit as usize] = false;
        }
        None
    }
}
