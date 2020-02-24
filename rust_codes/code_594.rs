struct Solution {}

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut elements: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        let mut res = 0;
        for num in nums {
            *elements.entry(num).or_insert(0) += 1;
        }
        for (key, value) in &elements {
            let next = *key + 1;
            if elements.contains_key(&next) {
                res = res.max(elements[&next] + *value);
            }
        }
        res
    }
}

fn main() {
    let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
    assert_eq!(5, Solution::find_lhs(nums));
}
