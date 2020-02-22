struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().unwrap_or("").len() as i32
    }
}

fn main() {
    let test_str = "hello world".to_string();
    assert_eq!(5, Solution::length_of_last_word(test_str));
}
