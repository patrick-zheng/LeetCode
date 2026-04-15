use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        if index_diff < 0 || value_diff < 0 {
            return false;
        }
        let k = index_diff as usize;
        let width = i64::from(value_diff) + 1;
        let mut buckets: HashMap<i64, i64> = HashMap::new();
        for (i, &x) in nums.iter().enumerate() {
            if i > k {
                let left = i64::from(nums[i - k - 1]);
                buckets.remove(&left.div_euclid(width));
            }
            let num = i64::from(x);
            let b = num.div_euclid(width);
            if buckets.contains_key(&b) {
                return true;
            }
            if let Some(&v) = buckets.get(&(b - 1)) {
                if num - v <= i64::from(value_diff) {
                    return true;
                }
            }
            if let Some(&v) = buckets.get(&(b + 1)) {
                if v - num <= i64::from(value_diff) {
                    return true;
                }
            }
            buckets.insert(b, num);
        }
        false
    }
}
