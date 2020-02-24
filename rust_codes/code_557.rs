struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().into_iter().map(|word| {
            word.chars().rev().collect()
        }).collect::<Vec<String>>().join(" ")
    }
}

fn main() {
    let test_str = "Let's take LeetCode contest".to_string();
    let res = "s'teL ekat edoCteeL tsetnoc".to_string();
    assert_eq!(res, Solution::reverse_words(test_str));
}
