pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l: usize = 0;
        let mut r: usize = numbers.len() - 1;

        while l < r {
            let s = numbers[l] as i64 + numbers[r] as i64; // avoid overflow
            if s == target as i64 {
                return vec![(l + 1) as i32, (r + 1) as i32]; // 1-indexed
            }
            if s < target as i64 {
                l += 1;
            } else {
                r -= 1;
            }
        }
        vec![]
    }
}