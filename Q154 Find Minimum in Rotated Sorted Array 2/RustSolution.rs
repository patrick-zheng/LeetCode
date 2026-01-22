pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = nums.len() - 1;

        while l < r {
            let mid = l + (r - l) / 2;

            if nums[mid] < nums[r] {
                r = mid;
            } else if nums[mid] > nums[r] {
                l = mid + 1;
            } else {
                r -= 1; // duplicates -> shrink
            }
        }

        nums[l]
    }
}
