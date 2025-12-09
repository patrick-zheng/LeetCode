pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ones: i32 = 0; // bits seen once
        let mut twos: i32 = 0; // bits seen twice

        for x in nums {
            ones = (ones ^ x) & !twos;
            twos = (twos ^ x) & !ones;
        }

        ones // bits that appeared 1 mod 3 times
    }
}
