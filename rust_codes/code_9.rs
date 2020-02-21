struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let num = x.to_string();
        num.chars().rev().collect::<String>() == num
    }
}

fn main() {
    let test_num = 121;
    assert_eq!(true, Solution::is_palindrome(test_num));
    let test_num = -121;
    assert_eq!(false, Solution::is_palindrome(test_num));
    let test_num = 10;
    assert_eq!(false, Solution::is_palindrome(test_num));
}
