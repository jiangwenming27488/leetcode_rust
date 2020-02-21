struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut close = 1000000;
        let mut nums = nums;
        nums.sort();
        let len = nums.len();
        for idx in 0..len {
            let (mut left, mut right) = (idx + 1, len - 1);
            while left < right {
                let sum = nums[idx] + nums[left] + nums[right];
                if (target - close).abs() > (target - sum).abs() {
                    close = sum;
                }
                if sum == target {
                    break;
                } else if sum > target {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }
        close
    }
}


fn main() {
    let nums = vec![-1, 2, 1, -4];
    let target = 1;
    assert_eq!(2, Solution::three_sum_closest(nums, target));
}
