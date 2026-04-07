use piaa_projekt_1::*;


pub fn my_introsort <T: std::cmp::PartialOrd + Clone + Default + Copy>(data_to_sort: &mut [T], depth_limit: usize){
    let ln = data_to_sort.len();

    if ln <= 16 {
        my_insertion_sort(data_to_sort);
    }

    if depth_limit == 0 {
        my_heap_sort(data_to_sort);
    }   else {

    }

    
    my_introsort(data_to_sort)
}