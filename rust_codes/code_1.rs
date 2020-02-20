pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len: usize = nums.len();
    let mut res: Vec<i32> = Vec::with_capacity(2);
    for first in 0..len {
        for second in (first + 1)..len {
            if nums[first] + nums[second] == target {
                res.append([first as i32, second as i32].to_vec().as_mut());
                break;
            }
        }
    }
    res
}


fn main() {
    let vec: Vec<i32> = vec![2, 7, 11, 15];
    let res = two_sum(vec, 9);
    assert_eq!([0, 1].to_vec(), res);
}
