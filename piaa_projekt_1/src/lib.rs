pub fn split_parts(to_split: &[i32]) -> (&[i32], &[i32]) {
    let mid = to_split.len() / 2;
    let left = &to_split[..mid];
    let right = &to_split[mid..];

    (left, right)
}
