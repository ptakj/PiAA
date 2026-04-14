pub fn my_insertion_sort<T: std::cmp::PartialOrd + Clone + Default + Copy>(data_to_sort: &mut [T]) {
    let ln = data_to_sort.len();
    if ln < 2 {
        return;
    }

    for i in 1..ln {
        let key = data_to_sort[i];
        let mut j = i;

        while j > 0 && data_to_sort[j - 1] > key {
            data_to_sort[j] = data_to_sort[j - 1];
            j -= 1;
        }
        data_to_sort[j] = key;
    }
}
