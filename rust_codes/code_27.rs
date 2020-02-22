struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|value| {
            *value != val
        });
        nums.len() as i32
    }
}


fn main() {
    let mut nums = vec![3, 2, 2, 3];
    assert_eq!(2, Solution::remove_element(&mut nums, 3));
}
