use std::collections::VecDeque;

fn main() {
    let tc1_digits = vec![1, 2, 3];
    let tc1_expect = vec![1, 2, 4];
    let tc1_result = plus_one(tc1_digits);
    assert_eq!(tc1_expect, tc1_result);
}

fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut result = VecDeque::new();

    let mut plus_one = true;
    for i in (0..digits.len()).rev() {
        let mut digit = digits[i];

        if plus_one {
            digit = (digit + 1) % 10;

            result.push_front(digit);

            if digit != 0 {
                plus_one = false;
            }
        } else {
            result.push_front(digit);
        }
    }

    if plus_one {
        result.push_front(1);
    }

    println!("{result:?}");
    result.into()
}
