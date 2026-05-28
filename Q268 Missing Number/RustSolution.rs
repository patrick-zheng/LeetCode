pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut missing = nums.len() as i32;
        for (i, &num) in nums.iter().enumerate() {
            missing ^= i as i32 ^ num;
        }
        missing
    }
}
