use piaa_projekt_1::*;


fn median_of_3<T: std::cmp::PartialOrd>(slice: &[T], a: usize, b: usize, c: usize) -> usize {
    let va = &slice[a];
    let vb = &slice[b];
    let vc = &slice[c];

    if va < vb {
        if vb < vc { b } else if va < vc { c } else { a }
    } else {
        if va < vc { a } else if vb < vc { c } else { b }
    }
}

fn get_ninther_index<T: std::cmp::PartialOrd>(slice: &[T]) -> usize {
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

pub fn my_quick_sort<T: std::cmp::PartialOrd + Clone>(data_to_sort: &mut[T]) {
    if data_to_sort.len() > 1{
    let mut left = 0;
    let dtslen = data_to_sort.len();

    let mut pivot = get_ninther_index(data_to_sort);
    let pivot_val = data_to_sort[pivot].clone();
    data_to_sort.swap(data_to_sort.len()-1, pivot);
    
    for i in 0..data_to_sort.len(){
        if data_to_sort[i]< pivot_val{
            data_to_sort.swap(left,i);
            left += 1;
        }
    }
    data_to_sort.swap(pivot, left);


    my_quick_sort(data_to_sort[0..pivot].as_mut());

    my_quick_sort(data_to_sort[pivot..dtslen].as_mut());
}

}