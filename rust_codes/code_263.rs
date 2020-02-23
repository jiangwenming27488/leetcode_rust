struct Solution {}

impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        let mut num = num;
        while num > 0 {
            if num % 2 == 0 {
                num = num >> 1;
            } else if num % 3 == 0 {
                num = num / 3;
            } else if num % 5 == 0 {
                num = num / 5;
            } else {
                break;
            }
        }
        num == 1
    }
}

fn main() {
    let num = 6;
    assert_eq!(true, Solution::is_ugly(num));
    let num = 8;
    assert_eq!(true, Solution::is_ugly(num));
    let num = 14;
    assert_eq!(false, Solution::is_ugly(num));
}
