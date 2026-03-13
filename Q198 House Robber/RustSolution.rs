pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut prev2 = 0; // max up to i-2
        let mut prev1 = 0; // max up to i-1

        for money in nums {
            let current = prev1.max(prev2 + money);
            prev2 = prev1;
            prev1 = current;
        }

        prev1
    }
}
