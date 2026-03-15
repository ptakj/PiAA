#![allow(unused)]
mod test_numbers_generator;
use piaa_projekt_1::*;
mod mergesort;
mod quick_sort;
use core::slice;
use std::vec;

fn main() {
    let mut t = test_numbers_generator::generate_test_table(20, 100, 50, false);
    //let wynik = mergesort::merge_parts(&[1, 3, 5, 7, 9], &[0, 2, 4, 6, 8, 10]);
    //let w = mergesort::my_merge_sort(&[2, 6, 8, 10, 90, 201031, 66, 87, 42]);
    let w = quick_sort::my_quick_sort(t.as_mut_slice());
    println!("{:?}", t);
}
