struct Solution {}


impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by(|lhs, rhs| {
            if lhs[0] != rhs[0] {
                return lhs[0].cmp(&rhs[0]);
            } else {
                return lhs[1].cmp(&rhs[1]);
            }
        });
        let (mut idx, mut len) = (1usize, intervals.len());
        loop {
            if idx >= len {
                break;
            }
            if intervals[idx][0] > intervals[idx - 1][1] {
                idx += 1;
            } else {
                intervals[idx - 1][1] = intervals[idx][1].max(intervals[idx - 1][1]);
                intervals[idx - 1][0] = intervals[idx][0].min(intervals[idx - 1][0]);
                intervals.remove(idx);
                len -= 1;
            }
        }
        intervals
    }
}

fn main() {
    let test_vec = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    assert_eq!(vec![vec![1, 6], vec![8, 10], vec![15, 18]], Solution::merge(test_vec));
    let test_vec = vec![vec![1, 4], vec![4, 5]];
    assert_eq!(vec![vec![1, 5]], Solution::merge(test_vec));
}
