struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        if citations.is_empty() {
            return 0;
        }
        let mut left = *citations.first().unwrap();
        let mut right = *citations.last().unwrap();
        let mut res = 0;
        while left <= right {
            let mid = (left + right) / 2;
            let counts = citations.iter().filter(|citation| {
                **citation >= mid
            }).count() as i32;
            if counts <= mid {
                res = counts.max(res);
                right = mid - 1;
            } else {
                res = mid.max(res);
                left = mid + 1;
            }
        }
        res
    }
}


fn main() {
    let citations = vec![100, ];
    assert_eq!(1, Solution::h_index(citations));
    let citations = vec![0, 1, 3, 5, 6];
    assert_eq!(3, Solution::h_index(citations));
    let citations = vec![0];
    assert_eq!(0, Solution::h_index(citations));
    let citations = vec![1, 1];
    assert_eq!(1, Solution::h_index(citations));
    let citations = vec![0, 1];
    assert_eq!(1, Solution::h_index(citations));
}
