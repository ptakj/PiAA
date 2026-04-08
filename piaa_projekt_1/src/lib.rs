pub fn split_parts<T>(to_split: &[T]) -> (&[T], &[T]) {
    let mid = to_split.len() / 2;
    let left = &to_split[..mid];
    let right = &to_split[mid..];

    (left, right)
}

pub fn median_of_3<T: std::cmp::PartialOrd>(slice: &[T], a: usize, b: usize, c: usize) -> usize {
    let va = &slice[a];
    let vb = &slice[b];
    let vc = &slice[c];

    if va < vb {
        if vb < vc {
            b
        } else if va < vc {
            c
        } else {
            a
        }
    } else {
        if va < vc {
            a
        } else if vb < vc {
            c
        } else {
            b
        }
    }
}

pub fn get_ninther_index<T: std::cmp::PartialOrd>(slice: &[T]) -> usize {
    let n = slice.len();
    if n < 9 {
        return n / 2;
    }

    let s = n / 8;
    let mid = n / 2;

    let m1 = median_of_3(slice, 0, s, 2 * s);
    let m2 = median_of_3(slice, mid - s, mid, mid + s);
    let m3 = median_of_3(slice, n - 1 - 2 * s, n - 1 - s, n - 1);

    median_of_3(slice, m1, m2, m3)
}
