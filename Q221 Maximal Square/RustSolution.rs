pub struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dp = vec![0i32; cols];
        let mut best_side = 0i32;

        for i in 0..rows {
            let mut northwest = 0i32;
            for j in 0..cols {
                let above = dp[j];
                if matrix[i][j] == '1' {
                    let left = if j > 0 { dp[j - 1] } else { 0 };
                    dp[j] = above.min(left).min(northwest) + 1;
                    best_side = best_side.max(dp[j]);
                } else {
                    dp[j] = 0;
                }
                northwest = above;
            }
        }
        best_side * best_side
    }
}
