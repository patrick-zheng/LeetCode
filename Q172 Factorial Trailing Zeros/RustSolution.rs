pub struct Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut x = n as i64;
        let mut ans: i64 = 0;
        while x > 0 {
            x /= 5;
            ans += x;
        }
        ans as i32
    }
}
