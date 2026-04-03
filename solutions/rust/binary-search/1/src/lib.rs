pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut l = 0;
    let mut r = array.len().checked_sub(1)?;

    while l <= r {
        let m = l + (r - l) / 2;

        if array[m] == key {
            return Some(m);
        }

        if array[m] > key {
            r = m.checked_sub(1)?;
        } else {
            l = m + 1;
        }
    }

    None
}
