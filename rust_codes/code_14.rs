struct Solution {}

impl Solution {
    pub fn longest_common_prefix(nums: Vec<String>) -> String {
        let mut res = String::new();
        if !nums.is_empty() {
            res = nums[0].clone();
            for idx in 1..nums.len() {
                while !nums[idx].starts_with(&res) {
                    res.pop();
                }
            }
        }
        res
    }
}

fn main() {
    let nums = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    assert_eq!("fl".to_string(), Solution::longest_common_prefix(nums));
    let nums = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    assert_eq!("", Solution::longest_common_prefix(nums));
}
