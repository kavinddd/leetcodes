fn main() {
    let mut tc1 = vec![3, 2, 2, 3];
    let tc1_result = remove_element(&mut tc1, 3);
    println!("{tc1:?} -> {tc1_result}");

    let mut tc2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let tc2_result = remove_element(&mut tc2, 2);
    println!("{tc2:?} -> {tc2_result}");
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut count = 0;

    for i in (0..nums.len()).rev() {
        let num = nums[i];

        if num == val {
            nums.swap_remove(i);
        } else {
            count += 1;
        }
    }

    count
}
