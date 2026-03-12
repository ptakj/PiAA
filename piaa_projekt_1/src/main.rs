#![allow(unused)]
mod test_numbers_generator;
mod lib;
use core::slice;
use std::vec;



fn my_merge_sort(data_to_sort: &[i32]) -> Vec<i32> {
    if data_to_sort.len() == 1 {
        data_to_sort.to_vec()
    } else {
        let (left_part, right_part) = lib::split_parts(&data_to_sort);
        return lib::merge_parts(&my_merge_sort(left_part), &my_merge_sort(right_part));
    }
}

fn main() {
    test_numbers_generator::generate_test_table(50, 100,300, true);
    let wynik = lib::merge_parts(&[1, 3, 5, 7, 9], &[0, 2, 4, 6, 8, 10]);
    let w = my_merge_sort(&[2, 6, 8, 10, 90, 201031, 66, 87, 42]);
    println!("{:?}", w);
}
