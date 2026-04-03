use std::{collections::HashMap, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    let handles: Vec<_> = input
        .chunks(input.len().div_ceil(worker_count))
        .map(|chunk| {
            let owned: Vec<String> = chunk.iter().map(|s| s.to_string()).collect();
            thread::spawn(move || {
                owned
                    .iter()
                    .flat_map(|s| s.chars())
                    .filter(|c| c.is_alphabetic())
                    .flat_map(|c| c.to_lowercase())
                    .fold(HashMap::new(), |mut map: HashMap<char, usize>, c| {
                        *map.entry(c).or_default() += 1;
                        map
                    })
            })
        })
        .collect();
    handles
        .into_iter()
        .flat_map(|handle| handle.join().unwrap())
        .fold(
            HashMap::new(),
            |mut map: HashMap<char, usize>, (c, count)| {
                *map.entry(c).or_default() += count;
                map
            },
        )
}
