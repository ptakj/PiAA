use rand::prelude::*;

pub fn generate_test_table(
    number_of_elements: i32,
    percentile_of_sorted_at_front: i32,
    max_number: i32,
    reversed: bool,
) -> Vec<i32> {
    let mut rng = rand::rng();
    let mut generated_test_table = (0..number_of_elements).fold(Vec::new(), |mut gtt, _| {
        gtt.push(rng.random_range(0..max_number));
        gtt
    });

    let slc = &mut generated_test_table
        [..(number_of_elements * percentile_of_sorted_at_front / 100) as i32 as usize];
    slc.sort();

    if reversed == true {
        generated_test_table.reverse();
    }
    return generated_test_table;
}

pub fn is_sorted(tab: &[i32]) -> bool {
    for i in 0..tab.len() - 1 {
        if tab[i] > tab[i + 1] {
            return false;
        }
    }
    true
}
