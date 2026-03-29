pub fn split_parts<T>(to_split: &[T]) -> (&[T], &[T]) {
    let mid = to_split.len() / 2;
    let left = &to_split[..mid];
    let right = &to_split[mid..];

    (left, right)
}
