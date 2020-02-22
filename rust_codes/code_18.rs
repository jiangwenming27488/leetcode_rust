struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut result: Vec<Vec<i32>> = vec![];
        if len < 4 {
            return result;
        }
        let mut nums = nums;
        nums.sort();
        for idx in 0..(len - 3) {
            if idx > 0 && nums[idx] == nums[idx - 1] {
                continue;
            }
            for idy in (idx + 1)..(len - 2) {
                if idy > idx + 1 && nums[idy - 1] == nums[idy] {
                    continue;
                }
                let (mut left, mut right) = (idy + 1, len - 1);
                while left < right {
                    let sum = nums[idx] + nums[idy] + nums[left] + nums[right];
                    if sum == target {
                        let tmp = vec![nums[idx], nums[idy], nums[left], nums[right]];
                        if !result.contains(&tmp) {
                            result.push(tmp);
                        }
                        while left < right && nums[left] == nums[left + 1] { left += 1; }
                        while left < right && nums[right - 1] == nums[right] { right -= 1; }
                        left += 1;
                        right -= 1;
                    } else if sum > target {
                        right -= 1;
                    } else {
                        left += 1;
                    }
                }
            }
        }
        result
    }
}


fn main() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    assert_eq!(vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]], Solution::four_sum(nums, target));
}
