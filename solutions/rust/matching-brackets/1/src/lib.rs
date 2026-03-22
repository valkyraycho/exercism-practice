use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    let bracket_match = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                if stack.pop() != bracket_match.get(&c).copied() {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}
