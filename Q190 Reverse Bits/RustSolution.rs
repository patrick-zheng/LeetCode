pub struct Solution;

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        (n as u32).reverse_bits() as i32
    }
}
