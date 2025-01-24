fn main() {
    let tc1_nums = vec![1, 3, 5, 6];
    let tc1_target = 5;
    let tc1_expect = 2;
    let tc1_result = search_insert(tc1_nums, tc1_target);

    let tc2_nums = vec![1, 3, 5, 6];
    let tc2_target = 2;
    let tc2_expect = 1;
    let tc2_result = search_insert(tc2_nums, tc2_target);

    let tc3_nums = vec![1, 3, 5, 6];
    let tc3_target = 7;
    let tc3_expect = 4;
    let tc3_result = search_insert(tc3_nums, tc3_target);

    println!("===============================");
    println!("{}", tc1_result);
    println!("{}", tc2_result);
    println!("{}", tc3_result);
}

// need to implmeent with O(log n) runtime complexity
fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 1 {
        if target <= nums[0] {
            return 0;
        } else {
            return 1;
        }
    }

    let mut low = 0;
    let mut high: i32 = nums.len() as i32 - 1;
    while low <= high {
        let mid = low + (high - low) / 2;

        println!("mid = low + (high - low) / 2");
        println!("{mid} = {low} + ({high} - {low}) / 2");

        let mid_val = nums[mid as usize];

        println!("nums[{mid}]: {mid_val}");
        if target == mid_val {
            return mid as i32;
        } else if target < mid_val {
            high = mid - 1;
        } else if target > mid_val {
            low = mid + 1;
        }
    }

    if target < nums[0] {
        return 0;
    }
    low as i32
}
