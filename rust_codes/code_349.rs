struct Solution {}


impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res = nums1.into_iter().filter(|num| {
            nums2.contains(num)
        }).collect::<Vec<i32>>();
        res.sort();
        res.dedup();
        res
    }
}

fn main() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    assert_eq!(vec![2], Solution::intersection(nums1, nums2));
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![1, 2];
    assert_eq!(vec![1, 2], Solution::intersection(nums1, nums2));
}
