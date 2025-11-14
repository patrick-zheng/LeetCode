pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = triangle[n - 1].clone();

        for r in (0..n - 1).rev() {
            for c in 0..triangle[r].len() {
                dp[c] = triangle[r][c] + dp[c].min(dp[c + 1]);
            }
        }

        dp[0]
    }
}
