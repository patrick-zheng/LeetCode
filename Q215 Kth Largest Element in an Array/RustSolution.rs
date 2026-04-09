pub struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        let k = k as usize;
        let (_, kth, _) = nums.select_nth_unstable(n - k);
        *kth
    }
}
