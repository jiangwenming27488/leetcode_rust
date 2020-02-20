struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut res = String::with_capacity(s.len());
        let mut sum = -1;
        s.chars().for_each(|ch| {
            let pos = res.find(ch);
            if pos.is_some() {
                sum = std::cmp::max(sum, res.len() as i32);
                res.drain(0..=pos.unwrap());
            }
            res.push(ch);
        });
        std::cmp::max(res.len() as i32, sum)
    }
}

fn main() {
    let test_str = "abcabcbb".to_string();
    assert_eq!(3, Solution::length_of_longest_substring(test_str));
    let test_str = "bbbbb".to_string();
    assert_eq!(1, Solution::length_of_longest_substring(test_str));
    let test_str = "pwwkew".to_string();
    assert_eq!(3, Solution::length_of_longest_substring(test_str));
    let test_str = " ".to_string();
    assert_eq!(1, Solution::length_of_longest_substring(test_str));
}
