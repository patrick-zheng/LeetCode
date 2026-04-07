pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        Self::rob_linear(&nums, 0, n - 2).max(Self::rob_linear(&nums, 1, n - 1))
    }

    fn rob_linear(nums: &[i32], lo: usize, hi: usize) -> i32 {
        let mut prev2 = 0;
        let mut prev1 = 0;
        for i in lo..=hi {
            let money = nums[i];
            let current = prev1.max(prev2 + money);
            prev2 = prev1;
            prev1 = current;
        }
        prev1
    }
}
