pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut xor_all = 0i32;
        for &x in &nums {
            xor_all ^= x;
        }
        let bit = xor_all & -xor_all;
        let mut a = 0i32;
        let mut b = 0i32;
        for &x in &nums {
            if x & bit != 0 {
                a ^= x;
            } else {
                b ^= x;
            }
        }
        vec![a, b]
    }
}
