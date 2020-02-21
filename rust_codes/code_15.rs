struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut results: Vec<Vec<i32>> = vec![];
        let length: usize = nums.len();
        if length <= 2 {
            return results;
        }
        for index in 0..(length - 2) {
            let (mut left, mut right) = (index + 1, length - 1);
            while left < right {
                let sum = nums[left] + nums[right] + nums[index];
                if sum == 0 {
                    let tmp: Vec<i32> = vec![nums[index], nums[left], nums[right]];
                    if !results.contains(&tmp) {
                        results.push(tmp);
                    }
                }
                if sum > 0 {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }
        results
    }
}

fn main() {
    let nums: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];
    assert_eq!([vec![-1, -1, 2], vec![-1, 0, 1]].to_vec(), Solution::three_sum(nums));
}
