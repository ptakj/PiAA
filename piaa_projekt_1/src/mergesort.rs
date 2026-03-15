use piaa_projekt_1::*;

pub fn merge_parts(first_part: &[i32], second_part: &[i32]) -> Vec<i32> {
    let combined_lenght: i32 = (first_part.len() + second_part.len()).try_into().unwrap();
    let mut ans = vec![0; combined_lenght as usize];
    let mut first_marker = 0;
    let mut second_marker = 0;
    let mut ans_marker = 0;

    for _ in 0..combined_lenght {
        if first_marker == first_part.len() {
            ans[ans_marker as usize] = second_part[second_marker as usize];
            ans_marker = ans_marker + 1;
            second_marker = second_marker + 1;
        } else if second_marker == second_part.len() {
            ans[ans_marker as usize] = first_part[first_marker as usize];
            ans_marker = ans_marker + 1;
            first_marker = first_marker + 1;
        } else if first_part[first_marker as usize] <= second_part[second_marker as usize] {
            ans[ans_marker as usize] = first_part[first_marker as usize];
            ans_marker = ans_marker + 1;
            first_marker = first_marker + 1;
        } else {
            ans[ans_marker as usize] = second_part[second_marker as usize];
            ans_marker = ans_marker + 1;
            second_marker = second_marker + 1;
        }
    }

    ans
}
pub fn my_merge_sort(data_to_sort: &[i32]) -> Vec<i32> {
    if data_to_sort.len() == 1 {
        data_to_sort.to_vec()
    } else {
        let (left_part, right_part) = split_parts(&data_to_sort);
        return merge_parts(&my_merge_sort(left_part), &my_merge_sort(right_part));
    }
}
