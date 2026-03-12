use piaa_projekt_1::*;

pub fn my_merge_sort(data_to_sort: &[i32]) -> Vec<i32> {
    if data_to_sort.len() == 1 {
        data_to_sort.to_vec()
    } else {
        let (left_part, right_part) = split_parts(&data_to_sort);
        return merge_parts(&my_merge_sort(left_part), &my_merge_sort(right_part));
    }
}