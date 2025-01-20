fn main() {
    let mut tc1 = vec![1, 1, 1, 2];
    let tc1_unique = remove_duplicates(&mut tc1);
    println!("{:?}, unique: {}", tc1, tc1_unique);

    let mut tc2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let tc2_unique = remove_duplicates(&mut tc2);
    println!("{:?}, unique: {}", tc2, tc2_unique);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut prev_num = nums.first().unwrap().to_owned();
    let mut count_unique = 1;

    for num in nums.to_owned().get(1..).unwrap().iter() {
        let num = *num;
        if num != prev_num {
            count_unique += 1;
            nums[count_unique - 1] = num;
        }

        prev_num = num
    }

    count_unique as i32
}
