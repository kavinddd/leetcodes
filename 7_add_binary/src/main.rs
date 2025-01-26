use std::collections::VecDeque;

fn main() {
    let tc1_a = String::from("11");
    let tc1_b = String::from("1");
    let tc1_expect = String::from("100");
    let tc1_result = add_binary(tc1_a, tc1_b);
    assert_eq!(tc1_expect, tc1_result);

    let tc2_a = String::from("1010");
    let tc2_b = String::from("1011");
    let tc2_expect = String::from("10101");
    let tc2_result = add_binary(tc2_a, tc2_b);
    assert_eq!(tc2_expect, tc2_result);
}

pub fn add_binary(a: String, b: String) -> String {
    let mut carry_flag = false;
    let mut result: VecDeque<u8> = VecDeque::new();

    let max_len = std::cmp::max(a.len(), b.len());
    let a_bytes: Vec<u8> = a.bytes().rev().collect();
    let b_bytes: Vec<u8> = b.bytes().rev().collect();
    for i in 0..max_len {
        let a_num = a_bytes.get(i).unwrap_or(&b'0') - b'0';
        let b_num = b_bytes.get(i).unwrap_or(&b'0') - b'0';

        let mut sum = a_num + b_num;

        if carry_flag {
            sum += 1;
            carry_flag = false;
        }

        if sum > 1 {
            carry_flag = true;
            sum = sum % 2;
        }

        result.push_front(sum + b'0')
    }

    if carry_flag {
        result.push_front(b'1');
    }

    String::from_utf8(result.into()).unwrap()
}
