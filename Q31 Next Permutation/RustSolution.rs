pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n <= 1 {
            return;
        }

        // Step 1: Find the first decreasing element from the right
        let mut i = (n - 2) as isize;
        while i >= 0 && nums[i as usize] >= nums[(i + 1) as usize] {
            i -= 1;
        }

        if i >= 0 {
            let mut j = (n - 1) as isize;
            // Step 2: Find element just larger than nums[i]
            while nums[j as usize] <= nums[i as usize] {
                j -= 1;
            }
            // Step 3: Swap
            nums.swap(i as usize, j as usize);
        }

        // Step 4: Reverse the suffix
        nums[((i + 1) as usize)..].reverse();
    }
}

