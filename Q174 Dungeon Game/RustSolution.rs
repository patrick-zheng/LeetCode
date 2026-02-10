pub struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();
        let inf: i64 = i64::MAX / 4;

        // dp[j] = min health needed to ENTER current cell in current row at column j
        let mut dp = vec![inf; n + 1];
        dp[n - 1] = 1; // sentinel

        for i in (0..m).rev() {
            dp[n] = inf; // right boundary sentinel for this row
            for j in (0..n).rev() {
                let need_next = dp[j].min(dp[j + 1]);
                let need_here = need_next - dungeon[i][j] as i64;
                dp[j] = if need_here <= 1 { 1 } else { need_here };
            }
        }

        dp[0] as i32
    }
}
