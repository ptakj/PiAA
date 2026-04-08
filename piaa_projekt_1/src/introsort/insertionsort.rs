pub fn my_insertion_sort <T: std::cmp::PartialOrd + Clone + Default + Copy>(data_to_sort: &mut [T]){
   let ln = data_to_sort.len(); 
    for i in (1..ln){
        let mut key = data_to_sort[i];
        let mut j = i-1;

        while data_to_sort[j] > key {
            data_to_sort[j + 1] = data_to_sort[j];
            j = j - 1;
        }
        data_to_sort[j+1] = key;
    }
}