pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut result: i32 = 0;

        while x != 0 {
            let digit = x % 10;
            x /= 10;

            // Check for overflow/underflow before it happens
            if result > i32::MAX / 10 || (result == i32::MAX / 10 && digit > 7) {
                return 0;
            }
            if result < i32::MIN / 10 || (result == i32::MIN / 10 && digit < -8) {
                return 0;
            }

            result = result * 10 + digit;
        }

        result
    }
}

