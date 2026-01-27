pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left = 0;
        let mut right = n - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] > nums[mid + 1] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left as i32
    }
}
