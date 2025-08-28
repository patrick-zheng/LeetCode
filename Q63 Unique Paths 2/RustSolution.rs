pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 {
            return 0;
        }

        let mut dp = vec![0i64; n];
        dp[0] = 1;

        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 1 {
                    dp[j] = 0;                 // obstacle
                } else if j > 0 {
                    dp[j] += dp[j - 1];        // from left + from top
                }
            }
        }

        dp[n - 1] as i32
    }
}
