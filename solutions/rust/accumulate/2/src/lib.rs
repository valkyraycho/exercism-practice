/// What should the type of function be?
pub fn map<T, U, F: FnMut(T) -> U>(input: Vec<T>, mut function: F) -> Vec<U> {
    let mut result = Vec::with_capacity(input.len());
    for el in input {
        result.push(function(el));
    }
    result
}
