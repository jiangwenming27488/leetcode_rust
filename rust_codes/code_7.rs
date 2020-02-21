struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = (x as i64).abs().to_string().chars().rev().collect::<String>().parse::<i64>().unwrap();
        if x.is_negative() {
            num = -num;
        }
        if num > i32::max_value() as i64 || num < i32::min_value() as i64 {
            return 0;
        }
        return num as i32;
    }
}

fn main() {
    let x = 123;
    assert_eq!(321, Solution::reverse(x));
    let x = -123;
    assert_eq!(-321, Solution::reverse(x));
    let x = 120;
    assert_eq!(21, Solution::reverse(x));
    let x = -2147483648;
    assert_eq!(0, Solution::reverse(x));
}
