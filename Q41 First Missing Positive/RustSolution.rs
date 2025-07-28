pub struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();

        let mut i = 0;
        while i < n {
            let val = nums[i];
            if val > 0 && (val as usize) <= n && nums[i] != nums[val as usize - 1] {
                nums.swap(i, val as usize - 1);
            } else {
                i += 1;
            }
        }

        for (i, &num) in nums.iter().enumerate() {
            if num != (i as i32 + 1) {
                return i as i32 + 1;
            }
        }

        n as i32 + 1
    }
}
