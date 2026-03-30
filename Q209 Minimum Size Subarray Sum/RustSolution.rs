pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut sum: i64 = 0;
        let mut ans = usize::MAX;

        for right in 0..nums.len() {
            sum += nums[right] as i64;
            while sum >= target as i64 {
                ans = ans.min(right - left + 1);
                sum -= nums[left] as i64;
                left += 1;
            }
        }

        if ans == usize::MAX { 0 } else { ans as i32 }
    }
}
