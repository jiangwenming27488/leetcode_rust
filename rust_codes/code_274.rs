struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.sort();
        let mut left = *citations.first().unwrap_or(&0);
        let mut right = *citations.last().unwrap_or(&0);
        let mut res = 0i32;
        while left <= right {
            let mid = (left + right) / 2;
            let counts = citations.iter().filter(|citation| {
                **citation >= mid
            }).count() as i32;
            if counts <= mid {
                res = res.max(counts);
                right = mid - 1;
            } else {
                res = res.max(mid);
                left = mid + 1;
            }
        }
        res
    }
}

fn main() {
    let citations = vec![3, 0, 6, 1, 5];
    assert_eq!(3, Solution::h_index(citations));
}
