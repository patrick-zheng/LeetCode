pub struct Solution;

impl Solution {
    pub fn functionName() -> void {
        if (x < 0) { return false; }

        let mut rev = 0;
        let mut n = x;

        while (n > 0) {
            rev = rev * 10 + n % 10;
            n /= 10;
        }

        rev == x
    }
}

