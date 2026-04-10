use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::with_capacity(nums.len());
        for x in nums {
            if !seen.insert(x) {
                return true;
            }
        }
        false
    }
}
