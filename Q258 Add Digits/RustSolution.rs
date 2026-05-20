pub struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        1 + (num - 1) % 9
    }
}
