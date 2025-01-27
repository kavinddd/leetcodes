fn main() {
    assert_eq!(6, my_sqrt(36));
    assert_eq!(6, my_sqrt(40));
    assert_eq!(46339, my_sqrt(2147395599));
}

fn my_sqrt(x: i32) -> i32 {
    if x == 0 || x == 1 {
        return x;
    }

    let mut left: u128 = 0;
    let mut right: u128 = x as u128;

    while left <= right {
        let mid = left + ((right - left) / 2);
        let mid_squared = (mid * mid) as u128;

        if mid_squared == x as u128 {
            return mid as i32;
        } else if mid_squared > x as u128 {
            right = mid - 1;
        } else if mid_squared < x as u128 {
            left = mid + 1;
        }
    }

    right as i32
}
