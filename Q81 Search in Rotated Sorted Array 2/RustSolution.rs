pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() { return false; }
        let (mut l, mut r) = (0i32, nums.len() as i32 - 1);

        while l <= r {
            let m = l + (r - l) / 2;
            let (lm, mm, rm) = (nums[l as usize], nums[m as usize], nums[r as usize]);

            if mm == target { return true; }

            // Duplicates: shrink both ends
            if lm == mm && mm == rm {
                l += 1;
                r -= 1;
                continue;
            }

            // Left half is sorted
            if lm <= mm {
                if lm <= target && target < mm {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            } else {
                // Right half is sorted
                if mm < target && target <= rm {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
        }
        false
    }
}
