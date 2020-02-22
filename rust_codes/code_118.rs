struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = vec![];
        for index in 0..num_rows as usize {
            let mut tmp: Vec<i32> = vec![1; index + 1];
            for idy in 1..index as usize {
                tmp[idy] = results[index - 1][idy - 1] + results[index - 1][idy];
            }
            results.push(tmp);
        }
        results
    }
}


fn main() {
    let num_rows = 5;
    let target = vec![vec![1], vec![1, 1], vec![1, 2, 1],
                      vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]];
    assert_eq!(target, Solution::generate(num_rows));
}
