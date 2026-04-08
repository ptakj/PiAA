use piaa_projekt_1::*;
use crate::introsort::insertionsort::my_insertion_sort;
use crate::introsort::heapsort::my_heap_sort;

pub fn partition<T: std::cmp::PartialOrd + Clone + Default + Copy>(partitioned: &mut [T]) -> usize{
 let ln = partitioned.len();
 let mut pivot = partitioned[ln-1];
 
 let mut i = 0;

 for j in (0..ln-1){
    if partitioned[j] <= pivot{
        partitioned.swap(i, j);
        i = i+1;

        
    }
 }
 partitioned.swap(i, ln-1);
  i
}


pub fn calculate_depth_limit(n: usize) -> usize {
    if n <= 1 {
        return 0;
    }
    (2.0 * (n as f64).log2()).floor() as usize
}

pub fn my_introsort <T: std::cmp::PartialOrd + Clone + Default + Copy>(
    data_to_sort: &mut [T],
    depth_limit: usize){
    
    let ln = data_to_sort.len();
    if ln < 2 {
        return;
    }
    if ln <= 16 {
        my_insertion_sort(data_to_sort);
        return;
    }

    if depth_limit == 0 {
        my_heap_sort(data_to_sort);
        return;
    }

    let piv_indx = partition(data_to_sort);
    let (left, right) = data_to_sort.split_at_mut(piv_indx);


let right_side = &mut right[1..];

    my_introsort(left, depth_limit - 1);
    my_introsort(right_side, depth_limit - 1);
}
