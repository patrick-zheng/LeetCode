pub struct Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut x = n;
        let mut count = 0;

        while x != 0 {
            x &= x - 1;
            count += 1;
        }

        count
    }
}
