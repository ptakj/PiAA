#![allow(unused)]
mod test_numbers_generator;
use piaa_projekt_1::*;
mod mergesort;
use core::slice;
use std::vec;

fn main() {
    test_numbers_generator::generate_test_table(50, 100,300, true);
    let wynik = merge_parts(&[1, 3, 5, 7, 9], &[0, 2, 4, 6, 8, 10]);
    let w = mergesort::my_merge_sort(&[2, 6, 8, 10, 90, 201031, 66, 87, 42]);
    println!("{:?}", w);
}
