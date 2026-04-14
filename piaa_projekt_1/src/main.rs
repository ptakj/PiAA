#![allow(unused)]
mod test_numbers_generator;
mod introsort;
mod mergesort;
mod quick_sort;

use piaa_projekt_1::*;
use test_numbers_generator::*;

use crate::introsort::introsort::my_introsort;
use crate::mergesort::my_merge_sort;
use crate::quick_sort::my_quick_sort;
use crate::test_numbers_generator::{generate_test_table, is_sorted};

use std::time::{Instant, Duration};
use std::fs::File;
use std::io::Write;

fn run_full_suite(size: usize, p: i32, reversed: bool, iterations: usize, file: &mut File) {
    let label = if reversed {
        format!("Odwrócone (z {}% posortowanych)", p)
    } else {
        format!("{}% posortowane", p)
    };

    let mut test_sets = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        test_sets.push(generate_test_table(size as i32, p, 1_000_000, reversed));
    }

    let t_intro = measure_on_set(&test_sets, "Introsort", |data| {
        let depth = (2.0 * (data.len() as f64).log2()).floor() as usize;
        my_introsort(data, depth);
    });

let t_merge = measure_on_set(&test_sets, "MergeSort", |data| {
    let sorted_result = my_merge_sort(data); 
    data.copy_from_slice(&sorted_result);
});

    let t_quick = measure_on_set(&test_sets, "QuickSort", |data| {
        my_quick_sort(data);
    });

    let results = [
        ("Introsort", t_intro),
        ("MergeSort", t_merge),
        ("QuickSort", t_quick),
    ];

    for (name, time) in results {
        let line = format!("{};{};{};{:.4}", name, size, label, time);
        println!("{}", line); 
        writeln!(file, "{}", line).expect("Nie udało się zapisać do pliku"); 
    }
}

fn measure_on_set<T>(sets: &Vec<Vec<i32>>, name: &str, mut sort_func: T) -> f64
where
    T: FnMut(&mut [i32]),
{
    let mut total_duration = Duration::new(0, 0);

    for (i, data) in sets.iter().enumerate() {
        let mut copy = data.clone();
        
        let start = Instant::now();
        sort_func(&mut copy);
        total_duration += start.elapsed();

        if i == 0 && !is_sorted(&copy) {
            eprintln!("BŁĄD: Algorytm {} nie posortował danych poprawnie!", name);
        }
    }

    total_duration.as_secs_f64() * 1000.0 / sets.len() as f64
}

fn main() {
    let sizes = [10_000, 50_000, 100_000, 500_000, 1_000_000];
    let sorted_percentages = [0, 25, 50, 75, 95, 99, 100]; 
    let iterations = 100;

    let mut file = File::create("wyniki_sortowania.csv").expect("Nie można utworzyć pliku");
    
    let header = "Algorytm;Rozmiar;Warunek;Sredni_Czas_ms";
    println!("{}", header);
    writeln!(file, "{}", header).unwrap();

    for &size in &sizes {
        for &p in &sorted_percentages {
            run_full_suite(size, p, false, iterations, &mut file);
            run_full_suite(size, p, true, iterations, &mut file);
        }
    }
    
    println!("--- KONIEC ---");
}