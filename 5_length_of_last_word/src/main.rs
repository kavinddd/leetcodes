fn main() {
    let tc1_s = String::from("Hello World");
    let tc1_expect = 5;
    let tc1_result = length_of_last_word(tc1_s);
    assert_eq!(tc1_expect, tc1_result);
    let tc2_s = String::from("   fly me   to   the moon  ");
    let tc2_expect = 4;
    let tc2_result = length_of_last_word(tc2_s);
    assert_eq!(tc2_expect, tc2_result);
}

fn length_of_last_word(s: String) -> i32 {
    if let Some(last_word) = s.trim().split(" ").last() {
        return last_word.len() as i32;
    }
    0
}
