use piaa_projekt_1::*;



pub fn my_quick_sort<T: std::cmp::PartialOrd + Clone>(data_to_sort: &mut [T]) {
    if data_to_sort.len() > 1 {
        let mut left = 0;
        let dtslen = data_to_sort.len();

        let mut pivot = get_ninther_index(data_to_sort);
        let pivot_val = data_to_sort[pivot].clone();
        data_to_sort.swap(data_to_sort.len() - 1, pivot);

        for i in 0..data_to_sort.len() {
            if data_to_sort[i] < pivot_val {
                data_to_sort.swap(left, i);
                left += 1;
            }
        }
        data_to_sort.swap(pivot, left);

        my_quick_sort(data_to_sort[0..pivot].as_mut());

        my_quick_sort(data_to_sort[pivot..dtslen].as_mut());
    }
}
