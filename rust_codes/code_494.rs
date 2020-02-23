struct Solution {}

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        if nums.is_empty() || s > 1000 {
            return 0;
        }
        let (rows, cols) = (nums.len(), 2001usize);
        let mut datas: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
        datas[0][nums[0] as usize + 1000] = 1;
        datas[0][1000 - nums[0] as usize] += 1;//针对0
        for idx in 1..rows {
            for idy in 0..cols as i32 {
                let (col, left, right) = (idy as usize, idy - nums[idx], idy + nums[idx]);
                if left < 0 {
                    if right < cols as i32 {
                        datas[idx][col] = datas[idx - 1][right as usize];
                    }
                } else if right >= cols as i32 {
                    datas[idx][col] = datas[idx - 1][left as usize];
                } else {
                    datas[idx][col] = datas[idx - 1][left as usize] + datas[idx - 1][right as usize];
                }
            }
        }
        datas[rows - 1][(s + 1000) as usize]
    }
}


fn main() {
    let nums = vec![1, 1, 1, 1, 1];
    let sum = 3;
    assert_eq!(5, Solution::find_target_sum_ways(nums, sum));
}
