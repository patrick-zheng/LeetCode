pub struct Solution;

impl Solution {
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        let mut power = n as i64;
        if power < 0 {
            x = 1.0 / x;
            power = -power;
        }

        let mut result = 1.0;
        while power > 0 {
            if power % 2 == 1 {
                result *= x;
            }
            x *= x;
            power /= 2;
        }
        result
    }
}
