/// What should the type of function be?
pub fn map<T, U, F: FnMut(T) -> U>(input: Vec<T>, function: F) -> Vec<U> {
    input.into_iter().map(function).collect()
}
