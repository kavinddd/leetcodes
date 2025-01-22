fn main() {
    let first_occured_index = str_str(String::from("aaa"), String::from("aaaa"));

    println!("{}", first_occured_index);
}

fn str_str(haystack: String, needle: String) -> i32 {
    if haystack.len() < needle.len() {
        return -1;
    }

    let mut i = 0;
    while i <= haystack.len() - needle.len() {
        let mut count = 0;
        let mut needle_chars = needle.chars();

        for c in haystack[i..].chars() {
            if c == needle_chars.next().unwrap() {
                count += 1;

                if count == needle.len() {
                    return i as i32;
                }
            } else {
                break;
            }
        }

        i += 1;
    }

    -1
}
