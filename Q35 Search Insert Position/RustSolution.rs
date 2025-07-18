pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() as i32 - 1);

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid as usize];

            if mid_val == target {
                return mid;
            } else if mid_val < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        left
    }
}
