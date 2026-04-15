use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut window = HashSet::with_capacity((k + 1).min(nums.len()));
        for (i, &x) in nums.iter().enumerate() {
            if !window.insert(x) {
                return true;
            }
            if i >= k {
                window.remove(&nums[i - k]);
            }
        }
        false
    }
}
