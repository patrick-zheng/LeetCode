pub struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n as u32) & ((n as u32) - 1) == 0
    }
}
