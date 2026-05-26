pub struct Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut value = n;
        for p in [2, 3, 5] {
            while value % p == 0 {
                value /= p;
            }
        }
        value == 1
    }
}
