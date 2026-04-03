use std::cmp::max;

#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut max_values = vec![0; max_weight as usize + 1];
    for item in items {
        for w in (item.weight as usize..max_values.len()).rev() {
            max_values[w] = max(
                max_values[w],
                max_values[w - item.weight as usize] + item.value,
            );
        }
    }
    max_values[max_weight as usize]
}
